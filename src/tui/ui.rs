use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};

use crate::app::App;
use crate::exercise::types::ExerciseStatus;
use crate::game::award;
use crate::runner::pipeline;
use crate::watcher::file_watcher::ExerciseWatcher;
use super::event::{self, AppEvent};
use super::terminal;
use super::screens;

/// TUI-specific navigation state layered on top of the business logic App.
pub struct TuiApp {
    pub app: App,
    pub screen: Screen,
    pub selected_module: usize,
    pub selected_exercise: usize,
    pub should_quit: bool,
    pub hints_revealed: usize,
    pub verify_output: Option<String>,
    pub verify_passed: Option<bool>,
    pub status_message: Option<String>,
    // Watch mode state
    watcher: Option<ExerciseWatcher>,
    pub watch_status: WatchStatus,
    current_run_id: u64,
    watch_start: Option<Instant>,
    // Level-up animation
    pub level_up_from: Option<(u32, u32)>, // (old_level, new_level)
    level_up_time: Option<Instant>,
    // Module completion animation
    pub module_complete_info: Option<(String, String)>, // (module_name, theme_name)
    module_complete_time: Option<Instant>,
    // Multiple choice state
    pub selected_mcq_option: usize,
    pub mcq_feedback: Option<String>,
    pub mcq_correct: Option<bool>,
    // AI context screen state
    pub ai_context_scroll: u16,
    // Lesson screen state
    pub lesson_scroll: u16,
    /// Largest useful scroll offset, written by the lesson renderer (which is
    /// the only place the laid-out viewport size and the wrapped line count are
    /// both known) and read by the key handler so scrolling stops at the end
    /// instead of running into blank space.
    pub lesson_max_scroll: std::cell::Cell<u16>,
    /// The lesson currently being read. Cached here rather than loaded per
    /// frame: `tui_markdown::from_str` borrows its input, and the render loop
    /// ticks every 200ms.
    pub current_lesson: Option<crate::lesson::Lesson>,
    // Watch-mode failure output
    pub watch_scroll: u16,
    /// Largest useful scroll offset for the failure pane, published by the
    /// renderer (the only place the wrapped line count is known).
    pub watch_scroll_max: std::cell::Cell<u16>,
    /// Where Esc should return to. Set when `a`/`l` are pressed from watch
    /// mode, so those screens come back instead of stranding the session.
    pub return_to: Option<Screen>,
    /// Armed by the first `x` in watch mode; the second one performs the reset.
    pub pending_reset: bool,
    // Async verification
    /// Receiver for the in-flight verification, if any. The `u64` is the run
    /// ID: `anyhow::Error` carries no ID of its own, so pairing it here is what
    /// lets a stale *error* be discarded as well as a stale success.
    verify_rx: Option<std::sync::mpsc::Receiver<(u64, anyhow::Result<pipeline::VerificationResult>)>>,
    /// Which screen dispatched the in-flight verification.
    verify_kind: Option<VerifyKind>,
    /// A save arrived while a verification was in flight; re-dispatch on completion.
    verify_pending: bool,
}

/// Which caller a pending verification belongs to, so its result is applied to
/// the right screen state even if the student navigated away.
#[derive(Clone, Copy, PartialEq)]
enum VerifyKind {
    Watch,
    OneShot,
}

#[derive(Clone, PartialEq)]
pub enum Screen {
    Home,
    ModuleView(String),
    ExerciseView(String),
    WatchMode(String),
    Stats,
    MultipleChoice(String),
    AiContext(String),
    /// Reading a module's lesson. Holds the module ID.
    Lesson(String),
}

#[derive(Clone, PartialEq)]
pub enum WatchStatus {
    Watching,
    Verifying,
    Passed(String),  // XP breakdown message
    Failed(String),  // error output
}

impl TuiApp {
    pub fn new(app: App) -> Self {
        Self {
            app,
            screen: Screen::Home,
            selected_module: 0,
            selected_exercise: 0,
            should_quit: false,
            hints_revealed: 0,
            verify_output: None,
            verify_passed: None,
            status_message: None,
            watcher: None,
            watch_status: WatchStatus::Watching,
            current_run_id: 0,
            watch_start: None,
            level_up_from: None,
            level_up_time: None,
            module_complete_info: None,
            module_complete_time: None,
            selected_mcq_option: 0,
            mcq_feedback: None,
            mcq_correct: None,
            ai_context_scroll: 0,
            lesson_scroll: 0,
            lesson_max_scroll: std::cell::Cell::new(0),
            current_lesson: None,
            watch_scroll: 0,
            watch_scroll_max: std::cell::Cell::new(0),
            return_to: None,
            pending_reset: false,
            verify_rx: None,
            verify_kind: None,
            verify_pending: false,
        }
    }

    /// Open a module's lesson, loading and caching it.
    /// Returns false (and sets a status message) when the module has no lesson.
    fn open_lesson(&mut self, module_id: &str) -> bool {
        let module = match self.app.catalog.get_module(module_id) {
            Some(m) => m.clone(),
            None => return false,
        };
        match crate::lesson::load(&module) {
            Ok(Some(lesson)) => {
                self.current_lesson = Some(lesson);
                self.lesson_scroll = 0;
                self.lesson_max_scroll.set(0);
                self.screen = Screen::Lesson(module_id.to_string());
                true
            }
            Ok(None) => {
                let hint = module
                    .book_url
                    .as_deref()
                    .unwrap_or("https://doc.rust-lang.org/book/");
                self.status_message =
                    Some(format!("No lesson yet for this module. See {}", hint));
                false
            }
            Err(e) => {
                self.status_message = Some(format!("Could not load lesson: {:#}", e));
                false
            }
        }
    }

    pub fn selected_module_id(&self) -> Option<String> {
        self.app
            .catalog
            .modules()
            .get(self.selected_module)
            .map(|m| m.id.clone())
    }

    fn enter_watch_mode(&mut self, exercise_id: &str) {
        let ex = match self.app.catalog.get_exercise(exercise_id) {
            Some(e) => e.clone(),
            None => return,
        };

        // Materialize first. The watcher canonicalizes the file and watches its
        // parent directory, so if the working copy does not exist yet the
        // canonicalize silently falls back and no event would ever match.
        let working = match self.app.workspace.ensure_materialized(&ex) {
            Ok(p) => p,
            Err(e) => {
                self.status_message = Some(format!("Could not prepare exercise: {:#}", e));
                return;
            }
        };

        // Invalidate anything already in flight. Without this, `current_run_id`
        // only ever changes at dispatch, so the staleness gate can never fire —
        // and a verification started for the previous exercise would land on
        // this one's screen.
        self.current_run_id = pipeline::next_run_id();

        match ExerciseWatcher::new(&working) {
            Ok(w) => {
                self.watcher = Some(w);
                self.screen = Screen::WatchMode(exercise_id.to_string());
                self.watch_status = WatchStatus::Watching;
                self.watch_start = Some(Instant::now());
                self.hints_revealed = 0;
                self.verify_output = None;
                self.verify_passed = None;
            }
            Err(e) => {
                self.status_message = Some(format!("Watch failed: {:#}", e));
            }
        }
    }

    fn exit_watch_mode(&mut self, exercise_id: &str) {
        self.watcher = None;
        self.watch_status = WatchStatus::Watching;
        self.return_to = None;
        self.pending_reset = false;
        // Any verification still running belongs to the session being left.
        self.current_run_id = pipeline::next_run_id();
        if let Some(ex) = self.app.catalog.get_exercise(exercise_id) {
            self.screen = Screen::ExerciseView(ex.id.clone());
        } else {
            self.screen = Screen::Home;
        }
    }

    /// Dispatch a verification onto a worker thread and return immediately.
    ///
    /// Verification used to run inline in the render loop, which froze the UI
    /// for the whole compile — up to the 30s boss-battle timeout — and made
    /// `WatchStatus::Verifying` unreachable, because it was set and overwritten
    /// before the next `terminal.draw`. Worse, the student saw a stale green
    /// `PASSED!` while their newly-broken code was compiling.
    fn dispatch_verify(&mut self, exercise_id: &str, kind: VerifyKind) {
        // One verification at a time against a given sandbox: `Sandbox::prepare`
        // rewrites `src/main.rs`, so a second run would edit the sources the
        // first is still compiling. Coalescing also debounces save storms from
        // format-on-save editors.
        if self.verify_rx.is_some() {
            self.verify_pending = true;
            return;
        }

        let exercise = match self.app.catalog.get_exercise(exercise_id) {
            Some(e) => e.clone(),
            None => return,
        };

        let source = match kind {
            VerifyKind::Watch => {
                let p = self.app.workspace.working_path(&exercise);
                if !p.exists() {
                    self.watch_status = WatchStatus::Failed(
                        "Exercise file not found. Press 'x' to reset it.".into(),
                    );
                    return;
                }
                p
            }
            VerifyKind::OneShot => match self.app.workspace.ensure_materialized(&exercise) {
                Ok(p) => p,
                Err(e) => {
                    self.verify_passed = Some(false);
                    self.verify_output = Some(format!("{:#}", e));
                    return;
                }
            },
        };

        self.current_run_id = pipeline::next_run_id();
        let run_id = self.current_run_id;
        match kind {
            VerifyKind::Watch => {
                self.watch_status = WatchStatus::Verifying;
                self.watch_scroll = 0;
                // A verify landing between the two `x` presses would otherwise
                // overwrite the confirmation prompt while leaving it armed.
                self.pending_reset = false;
            }
            VerifyKind::OneShot => {
                self.verify_passed = None;
                self.verify_output = Some("Verifying...".to_string());
            }
        }

        let cache_dir = self.app.cache_dir.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = pipeline::verify_exercise(&exercise, &source, &cache_dir, run_id);
            let _ = tx.send((run_id, result));
        });
        self.verify_rx = Some(rx);
        self.verify_kind = Some(kind);
    }

    /// Collect a finished verification, if one has arrived. Non-blocking.
    ///
    /// Quitting mid-verify leaves the worker's `cargo` running to completion as
    /// an orphan — it lives in its own `setsid` session, finishes in seconds,
    /// and writes only into `.rust-game-cache/`. Deliberate, for a local tool.
    fn poll_verify(&mut self) {
        use std::sync::mpsc::TryRecvError;

        let received = match self.verify_rx.as_ref() {
            Some(rx) => match rx.try_recv() {
                Ok(msg) => Some(Some(msg)),
                // The worker vanished without sending; clear the slot rather
                // than wedging every future dispatch behind the in-flight guard.
                Err(TryRecvError::Disconnected) => Some(None),
                Err(TryRecvError::Empty) => None,
            },
            None => None,
        };

        let Some(message) = received else { return };
        self.verify_rx = None;
        let kind = self.verify_kind.take();

        match (message, kind) {
            (Some((run_id, result)), Some(kind)) => {
                // Stale results — including errors — are discarded.
                if run_id == self.current_run_id {
                    self.apply_verify_result(kind, result);
                }
            }
            (None, Some(kind)) => {
                let msg = "Verification worker stopped unexpectedly.".to_string();
                match kind {
                    VerifyKind::Watch => self.watch_status = WatchStatus::Failed(msg),
                    VerifyKind::OneShot => {
                        self.verify_passed = Some(false);
                        self.verify_output = Some(msg);
                    }
                }
            }
            _ => {}
        }

        if self.verify_pending {
            self.verify_pending = false;
            // Also honor `return_to`: the student may have pressed `a` or `l`
            // while the verify was running, and their newest save still needs
            // checking.
            let target = match (&self.screen, &self.return_to) {
                (Screen::WatchMode(id), _) => Some(id.clone()),
                (_, Some(Screen::WatchMode(id))) => Some(id.clone()),
                _ => None,
            };
            if let Some(id) = target {
                self.dispatch_verify(&id, VerifyKind::Watch);
            }
        }
    }

    /// Apply a completed verification to the screen state that asked for it.
    fn apply_verify_result(
        &mut self,
        kind: VerifyKind,
        result: anyhow::Result<pipeline::VerificationResult>,
    ) {
        let result = match result {
            Ok(r) => r,
            Err(e) => {
                let msg = format!("{:#}", e);
                match kind {
                    VerifyKind::Watch => self.watch_status = WatchStatus::Failed(msg),
                    VerifyKind::OneShot => {
                        self.verify_passed = Some(false);
                        self.verify_output = Some(msg);
                    }
                }
                return;
            }
        };

        let exercise_id = result.exercise_id.clone();
        let exercise = match self.app.catalog.get_exercise(&exercise_id) {
            Some(e) => e.clone(),
            None => return,
        };
        self.app
            .last_result
            .insert(exercise_id.clone(), result.clone());

        // Screen state is only safe to write while the student is still looking
        // at *this* exercise. The award below is keyed by `result.exercise_id`
        // and stays correct either way.
        let on_this_exercise = match &self.screen {
            Screen::WatchMode(id) | Screen::ExerciseView(id) => id == &exercise_id,
            _ => false,
        };

        if result.passed() {
            // Only watch mode tracks when the student started, so it is the
            // only path that can honestly award the time-trial bonus. If they
            // have moved on, there is no honest elapsed time to use.
            let started_at = match kind {
                VerifyKind::Watch if on_this_exercise => self.watch_start,
                _ => None,
            };
            let outcome = award::award_completion(&mut self.app, &exercise, started_at);
            self.apply_outcome(&outcome);
            let msg = self.format_outcome(&outcome);
            if on_this_exercise {
                match kind {
                    VerifyKind::Watch => self.watch_status = WatchStatus::Passed(msg),
                    VerifyKind::OneShot => {
                        self.verify_passed = Some(true);
                        self.verify_output = Some(msg);
                    }
                }
            }
        } else {
            self.app.state.record_attempt(&exercise_id);
            let _ = self.app.save();
            let output = result
                .first_error()
                .unwrap_or("Verification failed")
                .to_string();
            if on_this_exercise {
                match kind {
                    VerifyKind::Watch => self.watch_status = WatchStatus::Failed(output),
                    VerifyKind::OneShot => {
                        self.verify_passed = Some(false);
                        self.verify_output = Some(output);
                    }
                }
            }
        }
    }

    /// Check if the level-up animation should still be showing.
    pub fn is_level_up_visible(&self) -> bool {
        if let Some(t) = self.level_up_time {
            t.elapsed() < Duration::from_secs(3)
        } else {
            false
        }
    }

    /// Dismiss the level-up animation.
    fn dismiss_level_up(&mut self) {
        self.level_up_from = None;
        self.level_up_time = None;
    }

    /// Check if the module completion animation should still be showing.
    pub fn is_module_complete_visible(&self) -> bool {
        if let Some(t) = self.module_complete_time {
            t.elapsed() < Duration::from_secs(3)
        } else {
            false
        }
    }

    /// Dismiss the module completion animation.
    fn dismiss_module_complete(&mut self) {
        self.module_complete_info = None;
        self.module_complete_time = None;
    }

    /// Apply the parts of an [`Outcome`] that drive TUI animations.
    fn apply_outcome(&mut self, outcome: &award::Outcome) {
        if let Some((old, new)) = outcome.level_up {
            self.level_up_from = Some((old, new));
            self.level_up_time = Some(Instant::now());
        }
        if let Some((name, theme)) = &outcome.module_completed {
            self.module_complete_info = Some((name.clone(), theme.clone()));
            self.module_complete_time = Some(Instant::now());
        }
    }

    /// Render an [`Outcome`] for display. Reads the award off the outcome, so
    /// it cannot claim XP the state layer declined.
    fn format_outcome(&self, outcome: &award::Outcome) -> String {
        let head = if outcome.is_new {
            let a = &outcome.award;
            format!(
                "PASSED! +{} XP (base: {}, first-try: {}, time: {}, streak: {:.2}x)",
                a.total, a.base, a.first_try_bonus, a.time_trial_bonus, a.streak_multiplier
            )
        } else {
            "PASSED! (already completed — no additional XP)".to_string()
        };
        format!(
            "{}\nLevel {} | {}/{} exercises | Streak: {}",
            head,
            self.app.state.player.level,
            self.app.state.exercises_completed(),
            self.app.catalog.total_exercises(),
            self.app.streak.current,
        )
    }
}

/// Main TUI entry point.
pub fn run(app: App) -> Result<()> {
    terminal::install_panic_hook();
    let mut terminal = terminal::setup()?;
    let mut tui = TuiApp::new(app);

    let tick_rate = Duration::from_millis(200);

    loop {
        // Render
        terminal.draw(|frame| screens::render(frame, &tui))?;

        // Check file watcher (non-blocking).
        //
        // This loop ticks every 200ms, so nothing here should allocate on the
        // common path. `matches!` tests the screen without holding a borrow;
        // the exercise ID is only cloned once a change has actually fired, and
        // the clone is what releases the borrow so `run_watch_verify` can take
        // `&mut tui`.
        if matches!(tui.screen, Screen::WatchMode(_)) {
            let file_changed = tui.watcher.as_mut().map(|w| w.poll_change()).unwrap_or(false);
            if file_changed {
                if let Screen::WatchMode(id) = &tui.screen {
                    let id = id.clone();
                    tui.dispatch_verify(&id, VerifyKind::Watch);
                }
            }
        }

        // Collect a finished verification regardless of screen — the student
        // may have pressed `a` or `l` while it was running.
        tui.poll_verify();

        // Auto-dismiss celebration overlays
        if tui.level_up_time.is_some() && !tui.is_level_up_visible() {
            tui.dismiss_level_up();
        }
        if tui.module_complete_time.is_some() && !tui.is_module_complete_visible() {
            tui.dismiss_module_complete();
        }

        // Handle events
        match event::poll_event(tick_rate)? {
            AppEvent::Key(key) => {
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('c')
                {
                    tui.should_quit = true;
                } else if tui.is_level_up_visible() {
                    // Any key dismisses level-up
                    tui.dismiss_level_up();
                } else if tui.is_module_complete_visible() {
                    // Any key dismisses module completion
                    tui.dismiss_module_complete();
                } else {
                    handle_key(key, &mut tui);
                }
            }
            AppEvent::Tick => {}
        }

        if tui.should_quit {
            break;
        }
    }

    terminal::teardown(terminal)?;
    tui.app.save()?;
    Ok(())
}

fn handle_key(key: crossterm::event::KeyEvent, tui: &mut TuiApp) {
    tui.status_message = None;

    match &tui.screen.clone() {
        Screen::Home => handle_home_key(key, tui),
        Screen::ModuleView(_) => handle_module_view_key(key, tui),
        Screen::ExerciseView(_) => handle_exercise_view_key(key, tui),
        Screen::WatchMode(id) => handle_watch_mode_key(key, tui, &id.clone()),
        Screen::Stats => handle_stats_key(key, tui),
        Screen::MultipleChoice(id) => handle_mcq_key(key, tui, &id.clone()),
        Screen::AiContext(id) => handle_ai_context_key(key, tui, &id.clone()),
        Screen::Lesson(id) => handle_lesson_key(key, tui, &id.clone()),
    }
}

fn handle_home_key(key: crossterm::event::KeyEvent, tui: &mut TuiApp) {
    match key.code {
        KeyCode::Char('q') => tui.should_quit = true,
        KeyCode::Char('s') => tui.screen = Screen::Stats,
        KeyCode::Char('n') => {
            let next_id = tui.app.catalog.exercises().iter()
                .find(|ex| {
                    let status = tui.app.exercise_status(&ex.id);
                    status == ExerciseStatus::Available || status == ExerciseStatus::InProgress
                })
                .map(|ex| ex.id.clone());
            if let Some(id) = next_id {
                tui.enter_watch_mode(&id);
            } else {
                tui.status_message = Some("All exercises completed!".into());
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let max = tui.app.catalog.modules().len().saturating_sub(1);
            if tui.selected_module < max {
                tui.selected_module += 1;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if tui.selected_module > 0 {
                tui.selected_module -= 1;
            }
        }
        KeyCode::Enter => {
            if let Some(module_id) = tui.selected_module_id() {
                tui.screen = Screen::ModuleView(module_id);
                tui.selected_exercise = 0;
            }
        }
        _ => {}
    }
}

fn handle_module_view_key(key: crossterm::event::KeyEvent, tui: &mut TuiApp) {
    let module_id = match &tui.screen {
        Screen::ModuleView(id) => id.clone(),
        _ => return,
    };

    let exercises = tui.app.catalog.exercises_for_module(&module_id);
    let max_ex = exercises.len().saturating_sub(1);

    match key.code {
        KeyCode::Esc => tui.screen = Screen::Home,
        KeyCode::Char('q') => tui.should_quit = true,
        KeyCode::Char('l') => {
            tui.open_lesson(&module_id);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if tui.selected_exercise < max_ex {
                tui.selected_exercise += 1;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if tui.selected_exercise > 0 {
                tui.selected_exercise -= 1;
            }
        }
        KeyCode::Enter => {
            if let Some(ex) = exercises.get(tui.selected_exercise) {
                tui.screen = Screen::ExerciseView(ex.id.clone());
                tui.hints_revealed = 0;
                tui.verify_output = None;
                tui.verify_passed = None;
            }
        }
        _ => {}
    }
}

fn handle_exercise_view_key(key: crossterm::event::KeyEvent, tui: &mut TuiApp) {
    let exercise_id = match &tui.screen {
        Screen::ExerciseView(id) => id.clone(),
        _ => return,
    };

    match key.code {
        KeyCode::Esc => {
            if let Some(ex) = tui.app.catalog.get_exercise(&exercise_id) {
                tui.screen = Screen::ModuleView(ex.module_id.clone());
            } else {
                tui.screen = Screen::Home;
            }
        }
        KeyCode::Char('q') => tui.should_quit = true,
        KeyCode::Char('h') => {
            if let Some(ex) = tui.app.catalog.get_exercise(&exercise_id) {
                if tui.hints_revealed < ex.hints.len() {
                    tui.hints_revealed += 1;
                }
            }
        }
        KeyCode::Char('w') => {
            // Enter watch mode
            let status = tui.app.exercise_status(&exercise_id);
            if status == ExerciseStatus::Locked {
                tui.status_message = Some("Exercise is locked.".into());
            } else {
                tui.enter_watch_mode(&exercise_id);
            }
        }
        KeyCode::Char('a') => {
            // Open AI context screen
            tui.ai_context_scroll = 0;
            tui.screen = Screen::AiContext(exercise_id.clone());
        }
        KeyCode::Char('v') => {
            // One-shot verify (or MCQ screen for multiple choice exercises)
            let status = tui.app.exercise_status(&exercise_id);
            if status == ExerciseStatus::Locked {
                tui.status_message = Some("Exercise is locked.".into());
                return;
            }
            // Check if this is an MCQ exercise
            if let Some(ex) = tui.app.catalog.get_exercise(&exercise_id) {
                if ex.exercise_type == crate::exercise::types::ExerciseType::ReverseEngineeringMultipleChoice {
                    tui.selected_mcq_option = 0;
                    tui.mcq_feedback = None;
                    tui.mcq_correct = None;
                    tui.screen = Screen::MultipleChoice(exercise_id.clone());
                    return;
                }
            }
            tui.dispatch_verify(&exercise_id, VerifyKind::OneShot);
        }
        KeyCode::Char('o') => {
            if let Some(ex) = tui.app.catalog.get_exercise(&exercise_id) {
                let ex = ex.clone();
                match tui.app.workspace.ensure_materialized(&ex) {
                    Ok(p) => tui.status_message = Some(format!("Open: {}", p.display())),
                    Err(e) => tui.status_message = Some(format!("{:#}", e)),
                }
            }
        }
        KeyCode::Char('n') => {
            if let Some(next) = tui.app.catalog.next_exercise_after(&exercise_id) {
                tui.screen = Screen::ExerciseView(next.id.clone());
                tui.hints_revealed = 0;
                tui.verify_output = None;
                tui.verify_passed = None;
            }
        }
        _ => {}
    }
}

fn handle_watch_mode_key(key: crossterm::event::KeyEvent, tui: &mut TuiApp, exercise_id: &str) {
    // Any key other than a second `x` cancels a pending reset.
    if tui.pending_reset && key.code != KeyCode::Char('x') {
        tui.pending_reset = false;
        tui.status_message = Some("Reset cancelled.".into());
        return;
    }

    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            tui.exit_watch_mode(exercise_id);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let max = tui.watch_scroll_max.get();
            tui.watch_scroll = tui.watch_scroll.saturating_add(1).min(max);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            tui.watch_scroll = tui.watch_scroll.saturating_sub(1);
        }
        KeyCode::Char('a') => {
            // Remember where to come back to — Esc from AiContext otherwise
            // lands on ExerciseView and silently ends the watch session.
            tui.return_to = Some(Screen::WatchMode(exercise_id.to_string()));
            tui.ai_context_scroll = 0;
            tui.screen = Screen::AiContext(exercise_id.to_string());
        }
        KeyCode::Char('l') => {
            if let Some(ex) = tui.app.catalog.get_exercise(exercise_id) {
                let module_id = ex.module_id.clone();
                tui.return_to = Some(Screen::WatchMode(exercise_id.to_string()));
                if !tui.open_lesson(&module_id) {
                    tui.return_to = None;
                }
            }
        }
        KeyCode::Char('h') => {
            if let Some(ex) = tui.app.catalog.get_exercise(exercise_id) {
                if tui.hints_revealed < ex.hints.len() {
                    tui.hints_revealed += 1;
                }
            }
        }
        KeyCode::Char('n') => {
            // Next exercise (only if current is completed)
            let status = tui.app.exercise_status(exercise_id);
            if status == ExerciseStatus::Completed {
                if let Some(next) = tui.app.catalog.next_exercise_after(exercise_id) {
                    let next_id = next.id.clone();
                    tui.exit_watch_mode(exercise_id);
                    tui.enter_watch_mode(&next_id);
                }
            }
        }
        KeyCode::Char('x') => {
            // Destructive, so it announces itself first and needs confirming.
            if !tui.pending_reset {
                tui.pending_reset = true;
                tui.status_message = Some(
                    "This discards your work on this exercise. Press x again to confirm, \
                     any other key to cancel."
                        .into(),
                );
                return;
            }
            tui.pending_reset = false;
            if let Some(ex) = tui.app.catalog.get_exercise(exercise_id) {
                let ex = ex.clone();
                match tui.app.workspace.reset(&ex) {
                    Ok(p) => {
                        tui.status_message = Some(format!(
                            "Reset to the original exercise (previous work saved to .bak): {}",
                            p.display()
                        ));
                        tui.watch_status = WatchStatus::Watching;
                        tui.watch_scroll = 0;
                    }
                    Err(e) => tui.status_message = Some(format!("Reset failed: {:#}", e)),
                }
            }
        }
        _ => {}
    }
}

fn handle_stats_key(key: crossterm::event::KeyEvent, tui: &mut TuiApp) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('s') => tui.screen = Screen::Home,
        KeyCode::Char('q') => tui.should_quit = true,
        _ => {}
    }
}

fn handle_mcq_key(key: crossterm::event::KeyEvent, tui: &mut TuiApp, exercise_id: &str) {
    let exercise = match tui.app.catalog.get_exercise(exercise_id) {
        Some(e) => e.clone(),
        None => return,
    };
    let num_options = exercise.multiple_choice_options.len();

    match key.code {
        KeyCode::Esc => {
            tui.screen = Screen::ExerciseView(exercise_id.to_string());
        }
        KeyCode::Char('q') => tui.should_quit = true,
        KeyCode::Down | KeyCode::Char('j') => {
            if num_options > 0 && tui.selected_mcq_option < num_options - 1 {
                tui.selected_mcq_option += 1;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if tui.selected_mcq_option > 0 {
                tui.selected_mcq_option -= 1;
            }
        }
        KeyCode::Enter => {
            if let Some(opt) = exercise.multiple_choice_options.get(tui.selected_mcq_option) {
                if opt.correct {
                    // MCQ compiles nothing, so there is no verification result
                    // and no start time — only the award path is shared.
                    let outcome = award::award_completion(&mut tui.app, &exercise, None);
                    tui.apply_outcome(&outcome);
                    tui.mcq_feedback = Some(format!("Correct! {}", tui.format_outcome(&outcome)));
                    tui.mcq_correct = Some(true);
                } else {
                    tui.app.state.record_attempt(exercise_id);
                    let _ = tui.app.save();
                    tui.mcq_feedback = Some(format!(
                        "Incorrect — {} is not right. Try again!",
                        opt.label
                    ));
                    tui.mcq_correct = Some(false);
                }
            }
        }
        _ => {}
    }
}

fn handle_lesson_key(key: crossterm::event::KeyEvent, tui: &mut TuiApp, module_id: &str) {
    match key.code {
        KeyCode::Esc => {
            // Leave without marking read.
            tui.current_lesson = None;
            tui.screen = tui
                .return_to
                .take()
                .unwrap_or_else(|| Screen::ModuleView(module_id.to_string()));
        }
        KeyCode::Char('q') => tui.should_quit = true,
        KeyCode::Char('m') => {
            tui.app.state.mark_lesson_read(module_id);
            let _ = tui.app.save();
            tui.current_lesson = None;
            tui.status_message = Some("Lesson marked as read.".into());
            tui.screen = tui
                .return_to
                .take()
                .unwrap_or_else(|| Screen::ModuleView(module_id.to_string()));
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let max = tui.lesson_max_scroll.get();
            tui.lesson_scroll = tui.lesson_scroll.saturating_add(1).min(max);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            tui.lesson_scroll = tui.lesson_scroll.saturating_sub(1);
        }
        _ => {}
    }
}

fn handle_ai_context_key(key: crossterm::event::KeyEvent, tui: &mut TuiApp, exercise_id: &str) {
    match key.code {
        KeyCode::Esc => {
            tui.screen = tui
                .return_to
                .take()
                .unwrap_or_else(|| Screen::ExerciseView(exercise_id.to_string()));
        }
        KeyCode::Char('q') => tui.should_quit = true,
        KeyCode::Down | KeyCode::Char('j') => {
            tui.ai_context_scroll = tui.ai_context_scroll.saturating_add(1);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            tui.ai_context_scroll = tui.ai_context_scroll.saturating_sub(1);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tui() -> TuiApp {
        TuiApp::new(App::new_for_testing().expect("test app"))
    }

    fn first_exercise_id(tui: &TuiApp) -> String {
        tui.app.catalog.exercises()[0].id.clone()
    }

    /// `WatchStatus::Verifying` used to be unreachable: verification ran inline
    /// between draws, so the state was set and overwritten before it rendered.
    #[test]
    fn verifying_is_observable_while_a_verification_is_in_flight() {
        let mut tui = test_tui();
        let id = first_exercise_id(&tui);
        tui.app.workspace.ensure_materialized(&tui.app.catalog.exercises()[0].clone()).unwrap();
        tui.screen = Screen::WatchMode(id.clone());

        tui.dispatch_verify(&id, VerifyKind::Watch);
        assert!(
            matches!(tui.watch_status, WatchStatus::Verifying),
            "the UI must be able to show that a verification is running"
        );
        assert!(tui.verify_rx.is_some(), "a worker should be in flight");
    }

    /// A result whose run_id no longer matches must be discarded — this is what
    /// stops a verification started for one exercise from landing on another.
    #[test]
    fn a_stale_result_does_not_touch_screen_state() {
        let mut tui = test_tui();
        let id = first_exercise_id(&tui);
        tui.app.workspace.ensure_materialized(&tui.app.catalog.exercises()[0].clone()).unwrap();
        tui.screen = Screen::WatchMode(id.clone());

        tui.dispatch_verify(&id, VerifyKind::Watch);

        // Simulate leaving and re-entering: the run in flight is now stale.
        tui.current_run_id = pipeline::next_run_id();
        tui.watch_status = WatchStatus::Watching;

        // Drain the worker.
        for _ in 0..600 {
            tui.poll_verify();
            if tui.verify_rx.is_none() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        assert!(tui.verify_rx.is_none(), "the worker slot must be released");
        assert!(
            matches!(tui.watch_status, WatchStatus::Watching),
            "a stale result must not overwrite the current screen state"
        );
    }

    /// A second save while a verification is running must be coalesced, not
    /// dispatched against a sandbox the first run is still compiling in.
    #[test]
    fn overlapping_dispatch_is_coalesced() {
        let mut tui = test_tui();
        let id = first_exercise_id(&tui);
        tui.app.workspace.ensure_materialized(&tui.app.catalog.exercises()[0].clone()).unwrap();
        tui.screen = Screen::WatchMode(id.clone());

        tui.dispatch_verify(&id, VerifyKind::Watch);
        let run_id = tui.current_run_id;

        tui.dispatch_verify(&id, VerifyKind::Watch);
        assert!(tui.verify_pending, "the second dispatch should be queued");
        assert_eq!(
            tui.current_run_id, run_id,
            "the queued dispatch must not start a second run"
        );
    }
}
