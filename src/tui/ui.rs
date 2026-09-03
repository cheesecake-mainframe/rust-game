use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};

use crate::app::App;
use crate::exercise::types::{ExerciseStatus, ExerciseType};
use crate::state::game_state::EditorLayout;
use crate::game::award;
use crate::runner::pipeline;
use crate::watcher::file_watcher::ExerciseWatcher;
use super::editor::EditorSession;
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
    // Embedded editor
    /// The buffer for the exercise being watched. `None` outside watch mode —
    /// and also inside it when the file could not be opened, which is a
    /// supported state, not an impossible one.
    pub editor: Option<EditorSession>,
    /// Ctrl+E gives the compiler output the whole screen.
    pub errors_expanded: bool,
    // Lesson-first flow
    /// Modules whose lesson was skipped with Esc during this run. Deliberately
    /// not persisted: a new session should offer an unread lesson again.
    lesson_skipped: std::collections::HashSet<String>,
    /// Where to go once the student leaves a lesson that interrupted them.
    lesson_gate_target: Option<GateTarget>,
    // Layout
    /// Published by the renderer — the only place the frame's width is known —
    /// and read by the key handler so the toggle knows what it is toggling away
    /// from. Same idiom as `watch_scroll_max`.
    pub resolved_layout: std::cell::Cell<EditorLayout>,
    // Compare view
    /// Built once when the screen opens, never in the renderer.
    pub compare_diff: Option<crate::tui::screens::CompareView>,
    pub compare_scroll: u16,
    pub compare_scroll_max: std::cell::Cell<u16>,
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
    /// Comparing the student's code against the reference solution.
    Compare(String),
}

/// What the student was doing when an unread lesson interrupted them, so it can
/// be carried out once they leave the lesson instead of being dropped.
#[derive(Clone)]
pub enum GateTarget {
    /// Show a module's exercise list.
    ModuleList(String),
    /// Open one exercise's detail view.
    ExerciseView(String),
    /// Start editing one exercise.
    Edit(String),
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
            editor: None,
            errors_expanded: false,
            lesson_skipped: std::collections::HashSet::new(),
            lesson_gate_target: None,
            resolved_layout: std::cell::Cell::new(EditorLayout::Stacked),
            compare_diff: None,
            compare_scroll: 0,
            compare_scroll_max: std::cell::Cell::new(0),
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

    /// Should the caller proceed with what the student asked for?
    ///
    /// Returns false when an unread lesson has been opened instead — the caller
    /// must not navigate; `resume_after_lesson` performs the navigation once the
    /// student leaves the lesson.
    fn gate_lesson(&mut self, module_id: &str, target: GateTarget) -> bool {
        if self.app.state.is_lesson_read(module_id) || self.lesson_skipped.contains(module_id) {
            return true;
        }
        self.return_to = None;
        self.lesson_gate_target = Some(target);
        // `open_lesson` returns false, with a status message, for a module that
        // has no lesson — so a lesson-less module can never block progress.
        if self.open_lesson(module_id) {
            return false;
        }
        // Failing open means the caller proceeds now, so nothing is waiting to
        // be resumed. Leaving the target set would strand it until the student
        // next closed *any* lesson, which would then teleport them here.
        self.lesson_gate_target = None;
        true
    }

    /// Carry out whatever the lesson gate interrupted.
    ///
    /// Without this, pressing `n` to reach the next exercise and then Esc-ing
    /// the lesson would land on the module list — five steps from where the
    /// student was going, with nothing saying so.
    fn resume_after_lesson(&mut self) -> bool {
        match self.lesson_gate_target.take() {
            Some(GateTarget::ModuleList(id)) => {
                // The direct path resets this; the gated path skipped that line,
                // and a stale index can point past this module's list.
                self.selected_exercise = 0;
                self.screen = Screen::ModuleView(id);
                true
            }
            Some(GateTarget::ExerciseView(id)) => {
                self.hints_revealed = 0;
                self.verify_output = None;
                self.verify_passed = None;
                self.screen = Screen::ExerciseView(id);
                true
            }
            // Safe to re-enter: the lesson is now read or skipped, so the gate
            // inside `enter_watch_mode` passes and this cannot recurse.
            Some(GateTarget::Edit(id)) => {
                self.enter_watch_mode(&id);
                true
            }
            None => false,
        }
    }

    /// Flip the editor/output split and remember the choice.
    fn toggle_layout(&mut self) {
        let next = match self.resolved_layout.get() {
            EditorLayout::SideBySide => EditorLayout::Stacked,
            EditorLayout::Stacked => EditorLayout::SideBySide,
        };
        self.app.state.preferences.editor_layout = Some(next);
        self.resolved_layout.set(next);
        self.status_message = Some(match self.app.save() {
            Ok(()) => match next {
                EditorLayout::SideBySide => "Layout: side by side.".to_string(),
                EditorLayout::Stacked => "Layout: stacked.".to_string(),
            },
            Err(e) => format!("Layout changed, but could not be saved: {:#}", e),
        });
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

        // The lesson comes first, and it must be checked before the MCQ branch
        // below: that branch navigates and returns, so a gate placed after it
        // would never run for a quiz.
        if !self.gate_lesson(&ex.module_id, GateTarget::Edit(exercise_id.to_string())) {
            return;
        }

        // Multiple-choice exercises have a file, but the task is to predict its
        // output, not to change it. The guard lives here rather than at the key
        // handler because three different paths reach this function.
        if ex.exercise_type == ExerciseType::ReverseEngineeringMultipleChoice {
            self.selected_mcq_option = 0;
            self.mcq_feedback = None;
            self.mcq_correct = None;
            self.screen = Screen::MultipleChoice(exercise_id.to_string());
            return;
        }

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
                self.errors_expanded = false;

                // A file that cannot be opened for editing must not end the
                // watch session — fall back to the read-only behavior and say so.
                match EditorSession::load(&working) {
                    Ok(ed) => self.editor = Some(ed),
                    Err(e) => {
                        self.editor = None;
                        self.status_message = Some(format!(
                            "Could not open for editing (edit it externally): {:#}",
                            e
                        ));
                    }
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Watch failed: {:#}", e));
            }
        }
    }

    fn exit_watch_mode(&mut self, exercise_id: &str) {
        // Saving is explicit while working, but never on the way out: leaving
        // must not be a way to lose work.
        if let Some(ed) = self.editor.as_mut() {
            if ed.is_dirty() {
                if let Err(e) = ed.save() {
                    self.status_message = Some(format!("Could not save your work: {:#}", e));
                }
            }
        }
        self.editor = None;
        self.errors_expanded = false;
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
            // A one-shot press would otherwise look like a dead key: it queues
            // nothing the student can see, and the pending flag only re-fires
            // for watch mode.
            if kind == VerifyKind::OneShot {
                self.verify_output =
                    Some("A verification is already running — waiting for it...".to_string());
            }
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
                        "Exercise file not found. Press Ctrl+X to reset it.".into(),
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
            Screen::WatchMode(id) | Screen::ExerciseView(id) | Screen::Compare(id) => {
                id == &exercise_id
            }
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
        self.format_outcome_with("PASSED!", outcome)
    }

    /// As [`TuiApp::format_outcome`] but with a caller-supplied lead-in, so the
    /// quiz screen does not announce "PASSED!" at someone who answered a
    /// multiple-choice question.
    fn format_outcome_with(&self, lead: &str, outcome: &award::Outcome) -> String {
        let head = if outcome.is_new {
            let a = &outcome.award;
            format!(
                "{} +{} XP (base: {}, first-try: {}, time: {}, streak: {:.2}x)",
                lead, a.total, a.base, a.first_try_bonus, a.time_trial_bonus, a.streak_multiplier
            )
        } else {
            format!("{} (already completed — no additional XP)", lead)
        };
        let body = format!(
            "{}\nLevel {} | {}/{} exercises | Streak: {}",
            head,
            self.app.state.player.level,
            self.app.state.exercises_completed(),
            self.app.catalog.total_exercises(),
            self.app.streak.current,
        );
        match &outcome.save_error {
            Some(e) => format!(
                "{}\nWARNING: progress could not be saved ({}) — this XP will be lost on quit.",
                body, e
            ),
            None => body,
        }
    }
}

/// Main TUI entry point.
pub fn run(app: App) -> Result<()> {
    terminal::install_panic_hook();
    let mut terminal = terminal::setup()?;
    let mut tui = TuiApp::new(app);

    let tick_rate = Duration::from_millis(200);

    // Errors are captured rather than propagated so that the save below still
    // runs. Returning early from inside the loop would lose the buffer.
    let mut loop_error: Option<anyhow::Error> = None;

    loop {
        // Render
        if let Err(e) = terminal.draw(|frame| screens::render(frame, &tui)) {
            loop_error = Some(e.into());
            break;
        }

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
                // Our own save also lands here. Verification must run either
                // way, but only a change we did not make should replace the
                // buffer — otherwise saving would discard whatever was typed
                // while the compile ran. After a failed save the buffer is the
                // only good copy, so it is never reloaded over.
                let reload_error = match tui.editor.as_mut() {
                    Some(ed) if !ed.last_save_failed() && !ed.disk_matches_last_write() => {
                        ed.reload_from_disk().err()
                    }
                    _ => None,
                };
                if let Some(e) = reload_error {
                    tui.status_message = Some(format!("Could not reload the file: {:#}", e));
                }

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
        let next_event = match event::poll_event(tick_rate) {
            Ok(ev) => ev,
            Err(e) => {
                loop_error = Some(e);
                break;
            }
        };
        match next_event {
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
            AppEvent::Paste(text) => {
                // A paste is an edit: it must cancel an armed reset like any
                // other key, and it must not go into a buffer that is currently
                // off screen behind the expanded error pane.
                if tui.pending_reset {
                    tui.pending_reset = false;
                    tui.status_message = Some("Reset cancelled.".into());
                } else if matches!(tui.screen, Screen::WatchMode(_)) && !tui.errors_expanded {
                    if let Some(ed) = tui.editor.as_ref() {
                        ed.on_paste(text);
                    }
                }
            }
            AppEvent::Tick => {}
        }

        if tui.should_quit {
            break;
        }
    }

    // The one place every quit path converges. Saving per-key is not enough:
    // `Ctrl+L` and `Ctrl+A` leave watch mode's handler without ending the
    // session, and both of those screens bind `q` to quit — so a per-key net
    // would miss the student who checks the lesson and then quits.
    if let Some(ed) = tui.editor.as_mut() {
        if ed.is_dirty() {
            let _ = ed.save(); // best effort: the process is on its way out
        }
    }

    terminal::teardown(terminal)?;
    tui.app.save()?;
    match loop_error {
        Some(e) => Err(e),
        None => Ok(()),
    }
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
        Screen::Compare(_) => handle_compare_key(key, tui),
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
                if !tui.gate_lesson(&module_id, GateTarget::ModuleList(module_id.clone())) {
                    return;
                }
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
                let (id, module_id) = (ex.id.clone(), ex.module_id.clone());
                if !tui.gate_lesson(&module_id, GateTarget::ExerciseView(id.clone())) {
                    return;
                }
                tui.screen = Screen::ExerciseView(id);
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
                    Ok(p) => tui.status_message = Some(format!("Path: {}", p.display())),
                    Err(e) => tui.status_message = Some(format!("{:#}", e)),
                }
            }
        }
        KeyCode::Char('n') => {
            if let Some(next) = tui.app.catalog.next_exercise_after(&exercise_id) {
                let (next_id, next_module) = (next.id.clone(), next.module_id.clone());
                let crossing = tui
                    .app
                    .catalog
                    .get_exercise(&exercise_id)
                    .map(|e| e.module_id != next_module)
                    .unwrap_or(false);
                // Walking within a module has already passed a gate; crossing
                // into a new one has not.
                if crossing
                    && !tui.gate_lesson(&next_module, GateTarget::ExerciseView(next_id.clone()))
                {
                    return;
                }
                tui.screen = Screen::ExerciseView(next_id);
                tui.hints_revealed = 0;
                tui.verify_output = None;
                tui.verify_passed = None;
            }
        }
        _ => {}
    }
}

fn handle_watch_mode_key(key: crossterm::event::KeyEvent, tui: &mut TuiApp, exercise_id: &str) {
    // Exact match, not `contains`. Terminals that report AltGr as CONTROL|ALT
    // would otherwise have composed characters swallowed by a command arm
    // instead of reaching edtui's `normalize_altgr` — the same class of bug the
    // brace-typing fix addressed.
    let ctrl = key.modifiers == KeyModifiers::CONTROL;

    // An armed reset owns the keyboard. `Ctrl+X` confirms; anything else cancels
    // with visible feedback and is consumed — a cancelling key must not also
    // type into the buffer, and it must not vanish silently either.
    if tui.pending_reset {
        if ctrl && key.code == KeyCode::Char('x') {
            tui.pending_reset = false;
            perform_reset(tui, exercise_id);
        } else {
            tui.pending_reset = false;
            tui.status_message = Some("Reset cancelled.".into());
        }
        return;
    }

    // The expanded error pane also owns the keyboard: the editor is off screen,
    // so bare keys are free again here.
    if tui.errors_expanded {
        // Save, compare and the layout toggle work from either pane. Dropping
        // them here would make three practised keys silent no-ops.
        if ctrl {
            match key.code {
                KeyCode::Char('s') => {
                    save_buffer(tui);
                    return;
                }
                KeyCode::Char('d') => {
                    open_compare(tui, exercise_id);
                    return;
                }
                KeyCode::Char('j') => {
                    toggle_layout(tui);
                    return;
                }
                // The Passed pane renders full-screen in this mode and says
                // "Press Ctrl+N for the next exercise". Swallowing it here would
                // make the instruction a lie.
                KeyCode::Char('n') => {
                    let status = tui.app.exercise_status(exercise_id);
                    if status == ExerciseStatus::Completed {
                        if let Some(next) = tui.app.catalog.next_exercise_after(exercise_id) {
                            let next_id = next.id.clone();
                            tui.exit_watch_mode(exercise_id);
                            tui.enter_watch_mode(&next_id);
                        }
                    }
                    return;
                }
                _ => {}
            }
        }
        match key.code {
            KeyCode::Esc => tui.errors_expanded = false,
            KeyCode::Char('e') if ctrl => tui.errors_expanded = false,
            KeyCode::Down | KeyCode::Char('j') => {
                let max = tui.watch_scroll_max.get();
                tui.watch_scroll = tui.watch_scroll.saturating_add(1).min(max);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                tui.watch_scroll = tui.watch_scroll.saturating_sub(1);
            }
            _ => {}
        }
        return;
    }

    if key.code == KeyCode::Esc {
        // Esc is unbound inside edtui but is the universal cancel key, so it
        // leaves search before it leaves the session. A second Esc then exits.
        if tui.editor.as_ref().is_some_and(|ed| ed.is_searching()) {
            if let Some(ed) = tui.editor.as_ref() {
                ed.cancel_search();
            }
            return;
        }
        tui.exit_watch_mode(exercise_id);
        return;
    }

    // Reserved Ctrl combinations are the game's; everything else falls through
    // to the editor, so edtui keeps undo (Ctrl+U), redo (Ctrl+R), kill-line
    // (Ctrl+K), paste (Ctrl+Y) and the rest of its motions.
    //
    // Never bound: Ctrl+I and Ctrl+M, which the terminal delivers as Tab and
    // Enter; Ctrl+H, because terminals in backarrow mode send its byte for the
    // Backspace key; and Ctrl+Q/T/W/Z, which Ghostty claims for quit, new tab,
    // close tab and a text macro — a terminal binding is taken before the
    // application ever sees the key, so binding them here would be dead code.
    if ctrl {
        let reserved = matches!(
            key.code,
            KeyCode::Char('s')
                | KeyCode::Char('e')
                | KeyCode::Char('g')
                | KeyCode::Char('a')
                | KeyCode::Char('l')
                | KeyCode::Char('n')
                | KeyCode::Char('x')
                | KeyCode::Char('d')
                | KeyCode::Char('j')
                | KeyCode::Down
                | KeyCode::Up
        );
        if reserved {
            match key.code {
                KeyCode::Char('s') => save_buffer(tui),
                KeyCode::Char('j') => toggle_layout(tui),
                KeyCode::Char('d') => open_compare(tui, exercise_id),
                KeyCode::Char('e') => {
                    tui.errors_expanded = true;
                    tui.watch_scroll = 0;
                }
                KeyCode::Char('g') => {
                    if let Some(ex) = tui.app.catalog.get_exercise(exercise_id) {
                        if tui.hints_revealed < ex.hints.len() {
                            tui.hints_revealed += 1;
                        }
                    }
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
                    tui.pending_reset = true;
                    tui.status_message = Some(
                        "This discards your work on this exercise. Press Ctrl+X again to \
                         confirm, any other key to cancel."
                            .into(),
                    );
                }
                KeyCode::Down => {
                    let max = tui.watch_scroll_max.get();
                    tui.watch_scroll = tui.watch_scroll.saturating_add(1).min(max);
                }
                KeyCode::Up => {
                    tui.watch_scroll = tui.watch_scroll.saturating_sub(1);
                }
                _ => {}
            }
            return;
        }
    }

    // Everything else belongs to the editor. With no buffer open the screen is
    // read-only and ordinary keys simply do nothing.
    if let Some(ed) = tui.editor.as_ref() {
        ed.on_key(key);
    }
}

/// Flip the editor/output split.
fn toggle_layout(tui: &mut TuiApp) {
    tui.toggle_layout();
}

/// Open the side-by-side comparison against the reference solution.
///
/// The order matters: build first, and only record the view once it succeeded —
/// an unreadable solution file must not be logged as "seen".
fn open_compare(tui: &mut TuiApp, exercise_id: &str) {
    let Some(exercise) = tui.app.catalog.get_exercise(exercise_id).cloned() else {
        return;
    };

    // The buffer, not the file: with unsaved edits the disk copy is code the
    // student has already moved past.
    let yours = match tui.editor.as_ref() {
        Some(ed) => ed.buffer_text(),
        None => match std::fs::read_to_string(tui.app.workspace.source_path(&exercise)) {
            Ok(s) => s,
            Err(e) => {
                tui.status_message = Some(format!("Could not read your code: {:#}", e));
                return;
            }
        },
    };
    let reference = match std::fs::read_to_string(&exercise.solution_path) {
        Ok(s) => s,
        Err(e) => {
            tui.status_message = Some(format!("Could not read the solution: {:#}", e));
            return;
        }
    };

    tui.compare_diff = Some(screens::CompareView::build(&yours, &reference));
    tui.compare_scroll = 0;
    tui.compare_scroll_max.set(0);

    tui.app.state.mark_solution_viewed(exercise_id);
    if let Err(e) = tui.app.save() {
        tui.status_message = Some(format!("Could not record that: {:#}", e));
    }

    tui.return_to = Some(Screen::WatchMode(exercise_id.to_string()));
    tui.screen = Screen::Compare(exercise_id.to_string());
}

fn handle_compare_key(key: crossterm::event::KeyEvent, tui: &mut TuiApp) {
    let ctrl = key.modifiers == KeyModifiers::CONTROL;
    let exercise_id = match &tui.screen {
        Screen::Compare(id) => id.clone(),
        _ => return,
    };
    // `open_compare` always sets `return_to`, but falling back to an empty id
    // would be a broken screen rather than a merely wrong one.
    let leave = |tui: &mut TuiApp| {
        tui.compare_diff = None;
        tui.screen = tui
            .return_to
            .take()
            .unwrap_or_else(|| Screen::ExerciseView(exercise_id.clone()));
    };
    match key.code {
        KeyCode::Esc => leave(tui),
        KeyCode::Char('d') if ctrl => leave(tui),
        KeyCode::Down | KeyCode::Char('j') => {
            let max = tui.compare_scroll_max.get();
            tui.compare_scroll = tui.compare_scroll.saturating_add(1).min(max);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            tui.compare_scroll = tui.compare_scroll.saturating_sub(1);
        }
        _ => {}
    }
}

/// Write the buffer to disk, reporting either outcome.
fn save_buffer(tui: &mut TuiApp) {
    let msg = match tui.editor.as_mut() {
        Some(ed) => match ed.save() {
            Ok(()) => "Saved.".to_string(),
            Err(e) => format!("Save failed: {:#}", e),
        },
        None => "No editable buffer — edit the file in your own editor.".to_string(),
    };
    tui.status_message = Some(msg);
}

/// Restore the pristine template and reload the buffer from it.
///
/// The reload is immediate rather than left to the watcher: the student pressed
/// a destructive key and should see the result at once, not 300ms later.
fn perform_reset(tui: &mut TuiApp, exercise_id: &str) {
    let Some(ex) = tui.app.catalog.get_exercise(exercise_id).cloned() else {
        return;
    };
    match tui.app.workspace.reset(&ex) {
        Ok(p) => {
            // Reset genuinely throws the work away, so it may clear the record
            // of having seen the solution. Deliberately *not* `forget_exercise`:
            // that also forfeits XP, which is the CLI reset's behaviour, not
            // this one's.
            tui.app.state.clear_solution_viewed(exercise_id);
            let save_error = tui.app.save().err().map(|e| format!("{:#}", e));
            let reload_error = tui.editor.as_mut().and_then(|ed| ed.reload_from_disk().err());
            tui.status_message = Some(match (reload_error, save_error) {
                (Some(e), _) => format!("Reset the file, but could not reload it: {:#}", e),
                (None, Some(e)) => format!("Reset, but progress could not be saved: {}", e),
                (None, None) => format!(
                    "Reset to the original exercise (previous work saved to .bak): {}",
                    p.display()
                ),
            });
            tui.watch_status = WatchStatus::Watching;
            tui.watch_scroll = 0;
        }
        Err(e) => tui.status_message = Some(format!("Reset failed: {:#}", e)),
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
                    tui.mcq_feedback = Some(tui.format_outcome_with("Correct!", &outcome));
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
            // Leave without marking read. Skipping is remembered for this run so
            // the same lesson does not reappear on the next exercise.
            tui.current_lesson = None;
            tui.lesson_skipped.insert(module_id.to_string());
            if !tui.resume_after_lesson() {
                tui.screen = tui
                    .return_to
                    .take()
                    .unwrap_or_else(|| Screen::ModuleView(module_id.to_string()));
            }
        }
        KeyCode::Char('q') => tui.should_quit = true,
        KeyCode::Char('m') => {
            tui.app.state.mark_lesson_read(module_id);
            let _ = tui.app.save();
            tui.current_lesson = None;
            tui.status_message = Some("Lesson marked as read.".into());
            if !tui.resume_after_lesson() {
                tui.screen = tui
                    .return_to
                    .take()
                    .unwrap_or_else(|| Screen::ModuleView(module_id.to_string()));
            }
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

    /// Most tests are about something other than the lesson gate, so they mark
    /// every lesson read first and reach the exercise directly.
    fn test_tui_past_the_gate() -> TuiApp {
        let mut tui = test_tui();
        let ids: Vec<String> = tui
            .app
            .catalog
            .modules()
            .iter()
            .map(|m| m.id.clone())
            .collect();
        for id in ids {
            tui.app.state.mark_lesson_read(&id);
        }
        tui
    }

    fn ctrl(c: char) -> crossterm::event::KeyEvent {
        crossterm::event::KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
    }

    fn first_exercise_id(tui: &TuiApp) -> String {
        tui.app.catalog.exercises()[0].id.clone()
    }

    // ── The lesson gate ───────────────────────────────────────────────────

    #[test]
    fn an_unread_lesson_gates_entry_to_watch_mode() {
        let mut tui = test_tui();
        let id = first_exercise_id(&tui);

        tui.enter_watch_mode(&id);

        assert!(
            matches!(tui.screen, Screen::Lesson(_)),
            "an unread lesson must come first"
        );
        assert!(tui.editor.is_none(), "the editor should not have opened yet");
    }

    /// Esc must carry on with what the student was doing. Dropping them on the
    /// module list would put five steps between `n` and the exercise.
    #[test]
    fn esc_resumes_the_original_navigation() {
        let mut tui = test_tui();
        let id = first_exercise_id(&tui);
        let module_id = tui.app.catalog.exercises()[0].module_id.clone();

        tui.enter_watch_mode(&id);
        assert!(matches!(tui.screen, Screen::Lesson(_)));

        handle_lesson_key(
            crossterm::event::KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            &mut tui,
            &module_id,
        );

        assert!(
            matches!(tui.screen, Screen::WatchMode(_)),
            "Esc should have resumed into the editor, got {:?}",
            std::mem::discriminant(&tui.screen)
        );
        assert!(tui.editor.is_some());
    }

    #[test]
    fn marking_the_lesson_read_also_resumes() {
        let mut tui = test_tui();
        let id = first_exercise_id(&tui);
        let module_id = tui.app.catalog.exercises()[0].module_id.clone();

        tui.enter_watch_mode(&id);
        handle_lesson_key(
            crossterm::event::KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE),
            &mut tui,
            &module_id,
        );

        assert!(matches!(tui.screen, Screen::WatchMode(_)));
        assert!(tui.app.state.is_lesson_read(&module_id));
    }

    #[test]
    fn esc_suppresses_the_gate_for_the_rest_of_the_session() {
        let mut tui = test_tui();
        let id = first_exercise_id(&tui);
        let module_id = tui.app.catalog.exercises()[0].module_id.clone();

        tui.enter_watch_mode(&id);
        handle_lesson_key(
            crossterm::event::KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            &mut tui,
            &module_id,
        );
        tui.exit_watch_mode(&id);

        // Second time round it must not gate again.
        tui.enter_watch_mode(&id);
        assert!(
            matches!(tui.screen, Screen::WatchMode(_)),
            "a skipped lesson must not reappear in the same session"
        );
        assert!(
            !tui.app.state.is_lesson_read(&module_id),
            "skipping is not reading"
        );
    }

    #[test]
    fn a_read_lesson_does_not_gate() {
        let mut tui = test_tui_past_the_gate();
        let id = first_exercise_id(&tui);

        tui.enter_watch_mode(&id);

        assert!(matches!(tui.screen, Screen::WatchMode(_)));
    }

    /// A module with no lesson must fail open, never block.
    #[test]
    fn a_module_without_a_lesson_never_gates() {
        let mut tui = test_tui();
        assert!(
            tui.gate_lesson("99_no_such_module", GateTarget::ModuleList("x".into())),
            "a missing lesson must not stop the student"
        );
        assert!(
            tui.lesson_gate_target.is_none(),
            "failing open leaves nothing to resume; a stale target would \
             hijack the next lesson the student closes"
        );
    }

    /// The MCQ branch of `enter_watch_mode` navigates and returns, so a gate
    /// placed after it would never run for a quiz.
    #[test]
    fn an_mcq_in_an_unread_module_gates_before_the_quiz() {
        let mut tui = test_tui();

        tui.enter_watch_mode("07_enums_pattern_matching/predict_match");

        assert!(
            matches!(tui.screen, Screen::Lesson(_)),
            "the lesson must come before the quiz"
        );
    }

    /// The gated home-Enter path skips the line that resets the selection, so
    /// the resume has to do it — otherwise Esc lands on a module with the
    /// previous module's index, possibly past the end of its list.
    #[test]
    fn the_module_list_gate_resumes_and_resets_the_selection() {
        let mut tui = test_tui();
        let module_id = tui.app.catalog.modules()[1].id.clone();
        tui.selected_exercise = 7;

        assert!(
            !tui.gate_lesson(&module_id, GateTarget::ModuleList(module_id.clone())),
            "an unread lesson should gate"
        );
        handle_lesson_key(
            crossterm::event::KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            &mut tui,
            &module_id,
        );

        match &tui.screen {
            Screen::ModuleView(id) => assert_eq!(id, &module_id),
            other => panic!("expected the module list, got {:?}", std::mem::discriminant(other)),
        }
        assert_eq!(tui.selected_exercise, 0, "a stale index can point past the list");
    }

    #[test]
    fn the_exercise_view_gate_resumes_to_that_exercise() {
        let mut tui = test_tui();
        let ex = tui.app.catalog.exercises()[0].clone();

        assert!(!tui.gate_lesson(&ex.module_id, GateTarget::ExerciseView(ex.id.clone())));
        handle_lesson_key(
            crossterm::event::KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE),
            &mut tui,
            &ex.module_id,
        );

        match &tui.screen {
            Screen::ExerciseView(id) => assert_eq!(id, &ex.id),
            other => panic!("expected the exercise view, got {:?}", std::mem::discriminant(other)),
        }
    }

    /// The CLI path must record too, or the marker and the stats line are a
    /// `rust-game solution <id>` away from meaningless.
    #[test]
    fn the_cli_solution_command_records_the_view() {
        let mut tui = test_tui();
        let id = first_exercise_id(&tui);

        assert!(!tui.app.state.has_viewed_solution(&id));
        tui.app.cmd_solution(&id).expect("the solution should print");
        assert!(tui.app.state.has_viewed_solution(&id));
    }

    // ── Keymap ────────────────────────────────────────────────────────────

    /// Ctrl+H is not a command: terminals in backarrow mode send its byte for
    /// the Backspace key, so it must reach the editor untouched.
    #[test]
    fn ctrl_h_falls_through_to_the_editor() {
        let mut tui = test_tui_past_the_gate();
        let id = first_exercise_id(&tui);
        tui.enter_watch_mode(&id);

        let before = tui.hints_revealed;
        handle_watch_mode_key(
            crossterm::event::KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE),
            &mut tui,
            &id,
        );
        let typed = tui.editor.as_ref().unwrap().buffer_text();

        handle_watch_mode_key(ctrl('h'), &mut tui, &id);

        assert_eq!(before, tui.hints_revealed, "Ctrl+H must not reveal a hint");
        // edtui binds it to delete-backwards, so it must actually reach the
        // editor — not merely be swallowed somewhere harmless.
        assert_ne!(
            typed,
            tui.editor.as_ref().unwrap().buffer_text(),
            "Ctrl+H must fall through to the editor"
        );
    }

    #[test]
    fn unreserved_ctrl_keys_reach_the_editor() {
        let mut tui = test_tui_past_the_gate();
        let id = first_exercise_id(&tui);
        tui.enter_watch_mode(&id);

        // Type first: Ctrl+B on a fresh buffer is a no-op at (0,0).
        handle_watch_mode_key(
            crossterm::event::KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE),
            &mut tui,
            &id,
        );
        let before = tui.editor.as_ref().unwrap().state_mut().cursor;
        handle_watch_mode_key(ctrl('b'), &mut tui, &id);
        let after = tui.editor.as_ref().unwrap().state_mut().cursor;

        assert_ne!(
            (before.row, before.col),
            (after.row, after.col),
            "Ctrl+B is edtui's back-one-character and must not be swallowed"
        );
    }

    /// Reset throws the work away, so it may clear the record of having seen the
    /// solution — but it must not forfeit XP, which is the CLI reset's job.
    #[test]
    fn reset_clears_the_solution_record_without_touching_xp() {
        let mut tui = test_tui_past_the_gate();
        let id = first_exercise_id(&tui);
        tui.enter_watch_mode(&id);

        tui.app.state.mark_solution_viewed(&id);
        // Give the entry real XP: with `xp_earned` at 0 this assertion could not
        // catch a switch to `forget_exercise`, which is exactly what it guards.
        tui.app.state.get_or_create_exercise(&id).xp_earned = 15;
        tui.app.state.player.xp = 40;
        assert!(tui.app.state.has_viewed_solution(&id));

        handle_watch_mode_key(ctrl('x'), &mut tui, &id); // arm
        handle_watch_mode_key(ctrl('x'), &mut tui, &id); // confirm

        assert!(
            !tui.app.state.has_viewed_solution(&id),
            "reset should clear the record"
        );
        assert_eq!(tui.app.state.player.xp, 40, "reset must not forfeit XP");
    }

    #[test]
    fn the_layout_toggle_round_trips_through_preferences() {
        let mut tui = test_tui_past_the_gate();
        let id = first_exercise_id(&tui);
        tui.enter_watch_mode(&id);
        tui.resolved_layout.set(EditorLayout::Stacked);

        handle_watch_mode_key(ctrl('j'), &mut tui, &id);
        assert_eq!(
            tui.app.state.preferences.editor_layout,
            Some(EditorLayout::SideBySide)
        );

        handle_watch_mode_key(ctrl('j'), &mut tui, &id);
        assert_eq!(
            tui.app.state.preferences.editor_layout,
            Some(EditorLayout::Stacked)
        );
    }

    #[test]
    fn ctrl_d_opens_the_comparison_and_records_it() {
        let mut tui = test_tui_past_the_gate();
        let id = first_exercise_id(&tui);
        tui.enter_watch_mode(&id);

        handle_watch_mode_key(ctrl('d'), &mut tui, &id);

        assert!(matches!(tui.screen, Screen::Compare(_)));
        assert!(tui.compare_diff.is_some());
        assert!(
            tui.app.state.has_viewed_solution(&id),
            "opening the comparison is what the record records"
        );
    }

    /// `WatchStatus::Verifying` used to be unreachable: verification ran inline
    /// between draws, so the state was set and overwritten before it rendered.
    #[test]
    fn verifying_is_observable_while_a_verification_is_in_flight() {
        let mut tui = test_tui_past_the_gate();
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
        let mut tui = test_tui_past_the_gate();
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

    /// The editor-present path. Every other test here assigns `screen`
    /// directly, which leaves `editor` as `None` and never touches it.
    #[test]
    fn entering_watch_mode_opens_an_editable_buffer_and_leaving_drops_it() {
        let mut tui = test_tui_past_the_gate();
        let id = first_exercise_id(&tui);

        tui.enter_watch_mode(&id);
        assert!(matches!(tui.screen, Screen::WatchMode(_)));
        assert!(
            tui.editor.is_some(),
            "watch mode should open the working copy for editing"
        );

        tui.exit_watch_mode(&id);
        assert!(tui.editor.is_none(), "leaving must drop the session");
    }

    /// Alt is vacated entirely now, so edtui's word motions must reach it.
    #[test]
    fn alt_keys_reach_the_editor() {
        let mut tui = test_tui_past_the_gate();
        let id = first_exercise_id(&tui);
        tui.enter_watch_mode(&id);
        assert!(tui.editor.is_some());

        let before = tui.editor.as_ref().unwrap().state_mut().cursor;
        // Alt+f is edtui's "forward one word" and is not a game command.
        handle_watch_mode_key(
            crossterm::event::KeyEvent::new(KeyCode::Char('f'), KeyModifiers::ALT),
            &mut tui,
            &id,
        );
        let after = tui.editor.as_ref().unwrap().state_mut().cursor;

        assert_ne!(
            (before.row, before.col),
            (after.row, after.col),
            "Alt+f should reach edtui now that Alt is vacated"
        );
        assert!(!tui.pending_reset, "it is not a game command");
    }

    /// Save must both write the file and stay out of the buffer.
    #[test]
    fn ctrl_s_saves_without_typing_into_the_buffer() {
        let mut tui = test_tui_past_the_gate();
        let id = first_exercise_id(&tui);
        tui.enter_watch_mode(&id);

        // A plain key types; the buffer becomes dirty.
        handle_watch_mode_key(
            crossterm::event::KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE),
            &mut tui,
            &id,
        );
        assert!(tui.editor.as_ref().unwrap().is_dirty());
        let typed = tui.editor.as_ref().unwrap().buffer_text();

        handle_watch_mode_key(ctrl('s'), &mut tui, &id);

        let ed = tui.editor.as_ref().unwrap();
        assert!(!ed.is_dirty(), "Ctrl+S should have written the file");
        assert_eq!(
            typed,
            ed.buffer_text(),
            "Ctrl+S is reserved and must not insert an 's'"
        );
    }

    /// Ctrl+X is reserved and must arm the reset rather than reach the editor.
    #[test]
    fn reserved_ctrl_keys_do_not_reach_the_editor() {
        let mut tui = test_tui_past_the_gate();
        let id = first_exercise_id(&tui);
        tui.enter_watch_mode(&id);

        let before = tui.editor.as_ref().unwrap().buffer_text();
        handle_watch_mode_key(ctrl('x'), &mut tui, &id);

        assert!(tui.pending_reset, "Ctrl+X arms the reset confirmation");
        assert_eq!(
            before,
            tui.editor.as_ref().unwrap().buffer_text(),
            "a reserved key must not type into the buffer"
        );
    }

    /// Multiple-choice exercises do have a file, but the task is to predict its
    /// output. Three call sites reach `enter_watch_mode`, so the guard lives
    /// inside it rather than at any one of them.
    #[test]
    fn watch_mode_routes_multiple_choice_to_the_answer_screen() {
        let mut tui = test_tui_past_the_gate();
        tui.enter_watch_mode("07_enums_pattern_matching/predict_match");

        assert!(
            matches!(tui.screen, Screen::MultipleChoice(_)),
            "an MCQ exercise must not open an editor"
        );
        assert!(tui.editor.is_none());
    }

    /// A second save while a verification is running must be coalesced, not
    /// dispatched against a sandbox the first run is still compiling in.
    #[test]
    fn overlapping_dispatch_is_coalesced() {
        let mut tui = test_tui_past_the_gate();
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
