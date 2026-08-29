use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};

use crate::app::App;
use crate::exercise::types::ExerciseStatus;
use crate::game::xp;
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
        if let Some(ex) = self.app.catalog.get_exercise(exercise_id) {
            self.screen = Screen::ExerciseView(ex.id.clone());
        } else {
            self.screen = Screen::Home;
        }
    }

    /// Run verification for watch mode. Returns immediately with result.
    /// Uses run_id gating: only applies the result if run_id matches current.
    fn run_watch_verify(&mut self, exercise_id: &str) {
        let exercise = match self.app.catalog.get_exercise(exercise_id) {
            Some(e) => e.clone(),
            None => return,
        };

        // Check that the student's working copy still exists
        let source = self.app.workspace.working_path(&exercise);
        if !source.exists() {
            self.watch_status = WatchStatus::Failed(
                "Exercise file not found. Press 'x' to reset it.".into(),
            );
            return;
        }

        self.current_run_id = pipeline::next_run_id();
        let run_id = self.current_run_id;
        self.watch_status = WatchStatus::Verifying;

        let result = match pipeline::verify_exercise(&exercise, &source, &self.app.cache_dir, run_id) {
            Ok(r) => r,
            Err(e) => {
                self.watch_status = WatchStatus::Failed(format!("{:#}", e));
                return;
            }
        };

        // Run ID check: discard if a newer run started while we were verifying
        if run_id != self.current_run_id {
            return; // Stale result — discard
        }

        if result.passed() {
            // Record attempt + complete
            self.app.state.record_attempt(exercise_id);
            let attempts = self.app.state.exercises.get(exercise_id).unwrap().attempts;

            let time_taken = exercise.time_limit_secs.map(|_| {
                self.watch_start.map(|s| s.elapsed()).unwrap_or_default()
            });

            let old_level = self.app.state.player.level;
            let award = xp::calculate_xp(&exercise, attempts, time_taken, self.app.streak.current);
            let is_new = self.app.state.complete_exercise(
                exercise_id,
                award.total,
                self.watch_start.map(|s| s.elapsed().as_secs()),
            );

            if is_new {
                self.app.streak.increment();
                let new_level = self.app.state.player.level;
                if new_level > old_level {
                    self.level_up_from = Some((old_level, new_level));
                    self.level_up_time = Some(Instant::now());
                }
                self.check_module_completion(exercise_id);
            }

            let msg = format!(
                "PASSED! +{} XP (base: {}, first-try: {}, time: {}, streak: {:.2}x)\nLevel {} | {}/{} exercises | Streak: {}",
                award.total, award.base, award.first_try_bonus,
                award.time_trial_bonus, award.streak_multiplier,
                self.app.state.player.level,
                self.app.state.exercises_completed(),
                self.app.catalog.total_exercises(),
                self.app.streak.current,
            );
            self.watch_status = WatchStatus::Passed(msg);
            let _ = self.app.save();
        } else {
            // Record attempt
            self.app.state.record_attempt(exercise_id);
            let _ = self.app.save();

            let output = result.first_error().unwrap_or("Verification failed").to_string();
            self.watch_status = WatchStatus::Failed(output);
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

    /// Check if the module containing `exercise_id` just became fully completed.
    /// Should be called after marking an exercise complete.
    fn check_module_completion(&mut self, exercise_id: &str) {
        if let Some(ex) = self.app.catalog.get_exercise(exercise_id) {
            let module_id = ex.module_id.clone();
            let exercises = self.app.catalog.exercises_for_module(&module_id);
            let all_complete = exercises.iter().all(|e| {
                self.app.exercise_status(&e.id) == ExerciseStatus::Completed
            });
            if all_complete && !exercises.is_empty() {
                // Check if we haven't already celebrated this module
                let module_state = self.app.state.modules.get(&module_id);
                let was_already_complete = module_state
                    .map(|ms| ms.completed)
                    .unwrap_or(false);
                if !was_already_complete {
                    if let Some(m) = self.app.catalog.get_module(&module_id) {
                        self.module_complete_info =
                            Some((m.name.clone(), m.theme_name.clone()));
                        self.module_complete_time = Some(Instant::now());
                    }
                    // Mark module as completed in state
                    let ms = self.app.state.modules
                        .entry(module_id)
                        .or_insert_with(|| crate::state::game_state::ModuleState {
                            unlocked: true,
                            completed: false,
                            unlocked_at: None,
                            completed_at: None,
                        });
                    ms.completed = true;
                    ms.completed_at = Some(chrono::Utc::now());
                }
            }
        }
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
                    tui.run_watch_verify(&id);
                }
            }
        }

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
            // Temporarily enter watch mode logic for a single verify
            if let Some(ex) = tui.app.catalog.get_exercise(&exercise_id) {
                let ex = ex.clone();
                let source = match tui.app.workspace.ensure_materialized(&ex) {
                    Ok(p) => p,
                    Err(e) => {
                        tui.verify_passed = Some(false);
                        tui.verify_output = Some(format!("{:#}", e));
                        return;
                    }
                };
                let run_id = pipeline::next_run_id();
                match pipeline::verify_exercise(&ex, &source, &tui.app.cache_dir, run_id) {
                    Ok(result) => {
                        if result.passed() {
                            tui.app.state.record_attempt(&exercise_id);
                            let attempts = tui.app.state.exercises.get(&exercise_id).unwrap().attempts;
                            let old_level = tui.app.state.player.level;
                            let award = xp::calculate_xp(&ex, attempts, None, tui.app.streak.current);
                            let is_new = tui.app.state.complete_exercise(&exercise_id, award.total, None);
                            if is_new {
                                tui.app.streak.increment();
                                let new_level = tui.app.state.player.level;
                                if new_level > old_level {
                                    tui.level_up_from = Some((old_level, new_level));
                                    tui.level_up_time = Some(Instant::now());
                                }
                                tui.check_module_completion(&exercise_id);
                            }
                            tui.verify_passed = Some(true);
                            tui.verify_output = Some(format!(
                                "PASSED! +{} XP (base: {}, first-try: {}, streak: {:.2}x)",
                                award.total, award.base, award.first_try_bonus, award.streak_multiplier
                            ));
                            let _ = tui.app.save();
                        } else {
                            tui.app.state.record_attempt(&exercise_id);
                            let _ = tui.app.save();
                            tui.verify_passed = Some(false);
                            tui.verify_output = Some(
                                result.first_error().unwrap_or("Failed").to_string(),
                            );
                        }
                    }
                    Err(e) => {
                        tui.verify_passed = Some(false);
                        tui.verify_output = Some(format!("{:#}", e));
                    }
                }
            }
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
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            tui.exit_watch_mode(exercise_id);
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
            // Restore the pristine template over the student's working copy.
            if let Some(ex) = tui.app.catalog.get_exercise(exercise_id) {
                let ex = ex.clone();
                match tui.app.workspace.reset(&ex) {
                    Ok(p) => {
                        tui.status_message =
                            Some(format!("Reset to the original exercise: {}", p.display()));
                        tui.watch_status = WatchStatus::Watching;
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
                    // Correct! Award XP
                    tui.app.state.record_attempt(exercise_id);
                    let attempts = tui.app.state.exercises.get(exercise_id).unwrap().attempts;
                    let old_level = tui.app.state.player.level;
                    let award = xp::calculate_xp(&exercise, attempts, None, tui.app.streak.current);
                    let is_new = tui.app.state.complete_exercise(exercise_id, award.total, None);
                    if is_new {
                        tui.app.streak.increment();
                        let new_level = tui.app.state.player.level;
                        if new_level > old_level {
                            tui.level_up_from = Some((old_level, new_level));
                            tui.level_up_time = Some(Instant::now());
                        }
                        tui.check_module_completion(exercise_id);
                    }
                    tui.mcq_feedback = Some(format!(
                        "Correct! +{} XP (base: {}, first-try: {}, streak: {:.2}x)",
                        award.total, award.base, award.first_try_bonus, award.streak_multiplier
                    ));
                    tui.mcq_correct = Some(true);
                    let _ = tui.app.save();
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
            tui.screen = Screen::ModuleView(module_id.to_string());
        }
        KeyCode::Char('q') => tui.should_quit = true,
        KeyCode::Char('m') => {
            tui.app.state.mark_lesson_read(module_id);
            let _ = tui.app.save();
            tui.current_lesson = None;
            tui.status_message = Some("Lesson marked as read.".into());
            tui.screen = Screen::ModuleView(module_id.to_string());
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
            tui.screen = Screen::ExerciseView(exercise_id.to_string());
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
