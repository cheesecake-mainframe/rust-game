use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::exercise::catalog::ExerciseCatalog;
use crate::exercise::types::*;
use crate::exercise::workspace::Workspace;
use crate::game::{award, progress, streak::StreakState};
use crate::runner::{pipeline, sandbox};
use crate::state::game_state::GameState;
use crate::state::persistence;

/// Top-level application state.
pub struct App {
    pub catalog: ExerciseCatalog,
    pub state: GameState,
    pub streak: StreakState,
    pub exercises_dir: PathBuf,
    pub cache_dir: PathBuf,
    /// Maps tracked exercise templates to the student's gitignored working copies.
    pub workspace: Workspace,
    /// Whether [`App::save`] actually writes to disk.
    ///
    /// `false` under `new_for_testing`. `save_state` resolves a process-global
    /// path, so without this any test that completes an exercise would overwrite
    /// the player's real `state.json` — including in parallel with other tests.
    persist: bool,
    /// The most recent verification result per exercise ID, so the AI context
    /// block can carry the compiler error it promises.
    pub last_result: HashMap<String, pipeline::VerificationResult>,
}

impl App {
    /// Find the exercises directory by searching multiple locations.
    ///
    /// Priority:
    /// 1. `RUST_GAME_DIR` env var (explicit override — looks for `$RUST_GAME_DIR/exercises/`)
    /// 2. `./exercises/` in the current working directory (primary workflow)
    /// 3. Relative to the executable (handles PATH / symlink scenarios)
    ///
    /// The `workspace/` directory always sits *beside* whichever `exercises/`
    /// directory this resolves to — not in the current working directory. If
    /// resolution falls through to the executable-relative branch, that may be
    /// a read-only install location; `RUST_GAME_DIR` is the escape hatch.
    fn find_exercises_dir() -> Result<PathBuf> {
        // 1. Env var override
        if let Ok(root) = std::env::var("RUST_GAME_DIR") {
            let dir = PathBuf::from(&root).join("exercises");
            if dir.is_dir() {
                return Ok(dir);
            }
        }

        // 2. Current working directory
        let cwd = PathBuf::from("exercises");
        if cwd.is_dir() {
            return Ok(cwd);
        }

        // 3. Relative to executable
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                // Check sibling (exe next to exercises/)
                let sibling = exe_dir.join("exercises");
                if sibling.is_dir() {
                    return Ok(sibling);
                }
                // Check parent (exe in target/debug/, exercises/ at repo root)
                if let Some(grandparent) = exe_dir.parent().and_then(|p| p.parent()) {
                    let repo_root = grandparent.join("exercises");
                    if repo_root.is_dir() {
                        return Ok(repo_root);
                    }
                }
            }
        }

        anyhow::bail!(
            "Could not find the exercises/ directory.\n\n\
             rust-game must be run from the repository root:\n\
             \n\
             \x20   cd /path/to/rust-game\n\
             \x20   cargo run\n\
             \n\
             Or set the RUST_GAME_DIR environment variable:\n\
             \n\
             \x20   RUST_GAME_DIR=/path/to/rust-game rust-game"
        )
    }

    /// The sandbox cache directory, anchored beside the resolved `exercises/`
    /// rather than the current working directory — otherwise running with
    /// `RUST_GAME_DIR` from elsewhere strands orphan cache trees.
    pub fn cache_dir_for(exercises_dir: &Path) -> PathBuf {
        exercises_dir
            .parent()
            .unwrap_or(Path::new("."))
            .join(".rust-game-cache")
    }

    /// Resolve the exercises directory for callers that do not need a full
    /// `App` (notably `clean-cache`, which runs before `init`).
    pub fn exercises_dir_or_cwd() -> PathBuf {
        Self::find_exercises_dir().unwrap_or_else(|_| PathBuf::from("exercises"))
    }

    /// Initialize the app: load catalog and state.
    pub fn init() -> Result<Self> {
        let exercises_dir = Self::find_exercises_dir()?;
        let cache_dir = Self::cache_dir_for(&exercises_dir);
        let workspace = Workspace::new(
            exercises_dir
                .parent()
                .unwrap_or(Path::new("."))
                .join("workspace"),
        );

        let catalog = ExerciseCatalog::load(&exercises_dir)
            .context("Failed to load exercise catalog")?;

        let state = persistence::load_state()
            .context("Failed to load game state")?;

        let mut streak = StreakState::from_saved(
            state.player.current_streak,
            state.player.best_streak,
        );
        streak.check_grace_period(state.player.last_exercise_at);

        Ok(Self {
            catalog,
            state,
            streak,
            exercises_dir,
            cache_dir,
            workspace,
            persist: true,
            last_result: HashMap::new(),
        })
    }

    /// Create an App for testing — loads the real catalog but uses fresh state.
    ///
    /// Persistence is disabled, so `save()` is a no-op: the state path is
    /// process-global and a test that completed an exercise would otherwise
    /// overwrite the player's real progress.
    pub fn new_for_testing() -> Result<Self> {
        let exercises_dir = PathBuf::from("exercises");
        let cache_dir = PathBuf::from(".rust-game-test-cache");
        let workspace = Workspace::new(PathBuf::from(".rust-game-test-workspace"));

        let catalog = ExerciseCatalog::load(&exercises_dir)
            .context("Failed to load exercise catalog")?;

        let state = GameState::new();
        let streak = StreakState::new();

        Ok(Self {
            catalog,
            state,
            streak,
            exercises_dir,
            cache_dir,
            workspace,
            persist: false,
            last_result: HashMap::new(),
        })
    }

    /// Save the current state to disk.
    ///
    /// A no-op when `persist` is false (see [`App::new_for_testing`]).
    pub fn save(&mut self) -> Result<()> {
        // Sync streak back to state before saving
        self.state.player.current_streak = self.streak.current;
        self.state.player.best_streak = self.streak.best;
        if !self.persist {
            return Ok(());
        }
        persistence::save_state(&self.state)
    }

    // ─── Exercise Status ────────────────────────────────────

    /// Compute the effective status of an exercise.
    pub fn exercise_status(&self, exercise_id: &str) -> ExerciseStatus {
        // Check if already tracked in state
        if let Some(ex_state) = self.state.exercises.get(exercise_id) {
            return ex_state.status;
        }

        // Check if the module is unlocked
        let exercise = match self.catalog.get_exercise(exercise_id) {
            Some(e) => e,
            None => return ExerciseStatus::Locked,
        };

        let unlocked = self.compute_unlocked_modules();
        if unlocked.get(&exercise.module_id).copied().unwrap_or(false) {
            ExerciseStatus::Available
        } else {
            ExerciseStatus::Locked
        }
    }

    /// Compute which modules are unlocked.
    pub fn compute_unlocked_modules(&self) -> HashMap<String, bool> {
        let module_completion: HashMap<String, bool> = self
            .catalog
            .modules()
            .iter()
            .map(|m| {
                let exercise_ids: Vec<String> = self
                    .catalog
                    .exercises_for_module(&m.id)
                    .iter()
                    .map(|e| e.id.clone())
                    .collect();
                let exercise_statuses: HashMap<String, ExerciseStatus> = exercise_ids
                    .iter()
                    .map(|id| (id.clone(), self.exercise_status_raw(id)))
                    .collect();
                let complete =
                    progress::is_module_complete(&m.id, &exercise_ids, &exercise_statuses);
                (m.id.clone(), complete)
            })
            .collect();

        progress::compute_unlocked_modules(self.catalog.modules(), &module_completion)
    }

    /// Raw status from state (without module lock check).
    fn exercise_status_raw(&self, exercise_id: &str) -> ExerciseStatus {
        self.state
            .exercises
            .get(exercise_id)
            .map(|s| s.status)
            .unwrap_or(ExerciseStatus::Available)
    }

    // ─── Commands ───────────────────────────────────────────

    /// List all exercises with their status.
    pub fn cmd_list(&self, module_filter: Option<&str>) {
        let unlocked = self.compute_unlocked_modules();

        for module in self.catalog.modules() {
            if let Some(filter) = module_filter {
                if !module.id.contains(filter) && !module.name.to_lowercase().contains(&filter.to_lowercase()) {
                    continue;
                }
            }

            let is_unlocked = unlocked.get(&module.id).copied().unwrap_or(false);
            let lock_icon = if is_unlocked { " " } else { "🔒" };

            println!(
                "\n{} {} — {}",
                lock_icon, module.name, module.theme_name
            );

            let exercises = self.catalog.exercises_for_module(&module.id);
            if exercises.is_empty() {
                println!("    (no exercises yet)");
                continue;
            }

            for ex in exercises {
                let status = self.exercise_status(&ex.id);
                let status_icon = match status {
                    ExerciseStatus::Completed => "  [x]",
                    ExerciseStatus::InProgress => "  [>]",
                    ExerciseStatus::Available => "  [ ]",
                    ExerciseStatus::Locked => "  [.]",
                };
                let xp_earned = self
                    .state
                    .exercises
                    .get(&ex.id)
                    .map(|s| format!("+{}xp", s.xp_earned))
                    .unwrap_or_default();
                let type_label = format!("{:?}", ex.exercise_type);
                println!(
                    "{} {} ({}) {}",
                    status_icon, ex.name, type_label, xp_earned
                );
            }
        }
        println!();
    }

    /// Verify a specific exercise.
    pub fn cmd_verify(&mut self, exercise_id: &str) -> Result<()> {
        let exercise = self
            .catalog
            .get_exercise(exercise_id)
            .context(format!("Exercise not found: '{}'", exercise_id))?
            .clone();

        let status = self.exercise_status(exercise_id);
        if status == ExerciseStatus::Locked {
            bail!(
                "Exercise '{}' is locked. Complete its prerequisites first.",
                exercise_id
            );
        }

        println!("Verifying: {} ...", exercise.name);

        // Verify the student's working copy, materializing it on first use.
        let source = self.workspace.ensure_materialized(&exercise)?;

        let run_id = pipeline::next_run_id();
        let result = pipeline::verify_exercise(&exercise, &source, &self.cache_dir, run_id)?;
        self.last_result
            .insert(exercise_id.to_string(), result.clone());

        if result.passed() {
            // The CLI has no meaningful start time — a one-shot verify measures
            // compile duration, which always beats a 60-180s limit and so used
            // to grant the time-trial bonus unconditionally. `None` is honest.
            let outcome = award::award_completion(self, &exercise, None);

            println!("  PASSED!");
            if outcome.is_new {
                let a = &outcome.award;
                println!(
                    "  +{} XP (base: {}, first-try: {}, time: {}, streak: {:.2}x)",
                    a.total, a.base, a.first_try_bonus, a.time_trial_bonus, a.streak_multiplier
                );
            } else {
                println!("  (already completed — no additional XP)");
            }
            if let Some((old, new)) = outcome.level_up {
                println!("  LEVEL UP!  {} -> {}", old, new);
            }
            if let Some((name, theme)) = &outcome.module_completed {
                println!("  MODULE COMPLETE: {} — {}", name, theme);
            }
            if let Some(err) = &outcome.save_error {
                eprintln!(
                    "  WARNING: progress could not be saved ({}). \
                     This XP will be lost when you quit.",
                    err
                );
            }
            println!(
                "  Level {} | {}/{} exercises | Streak: {}",
                self.state.player.level,
                self.state.exercises_completed(),
                self.catalog.total_exercises(),
                self.streak.current,
            );
        } else {
            // Record attempt
            self.state.record_attempt(exercise_id);
            self.save()?;

            println!("  FAILED");
            if let Some(err) = result.first_error() {
                println!("\n{}", err);
            }
        }

        Ok(())
    }

    /// Show hints for an exercise.
    pub fn cmd_hint(&self, exercise_id: &str) -> Result<()> {
        let exercise = self
            .catalog
            .get_exercise(exercise_id)
            .context(format!("Exercise not found: '{}'", exercise_id))?;

        if exercise.hints.is_empty() {
            println!("No hints available for '{}'.", exercise.name);
            return Ok(());
        }

        println!("Hints for: {}\n", exercise.name);
        for (i, hint) in exercise.hints.iter().enumerate() {
            println!("  {}. {}", i + 1, hint);
        }
        println!();
        Ok(())
    }

    /// Show player stats.
    pub fn cmd_stats(&self) {
        let p = &self.state.player;
        let completed = self.state.exercises_completed();
        let total = self.catalog.total_exercises();
        let progress_pct = if total > 0 {
            (completed as f64 / total as f64 * 100.0) as u32
        } else {
            0
        };

        let level_prog = crate::game::level::level_progress(p.xp);
        let xp_next = crate::game::level::xp_to_next_level(p.xp);

        println!("\n  rust-game Stats");
        println!("  ───────────────────────");
        println!("  Level:      {}", p.level);
        println!("  XP:         {} ({:.0}% to next level, {} XP needed)", p.xp, level_prog * 100.0, xp_next);
        println!("  Streak:     {} (best: {})", self.streak.current, self.streak.best);
        println!("  Completed:  {}/{} exercises ({}%)", completed, total, progress_pct);
        println!("  Time:       {} minutes", p.total_time_played_secs / 60);
        println!();
    }

    /// Jump to the next available exercise.
    pub fn cmd_next(&self) -> Result<()> {
        // Find first Available or InProgress exercise
        for ex in self.catalog.exercises() {
            let status = self.exercise_status(&ex.id);
            if status == ExerciseStatus::Available || status == ExerciseStatus::InProgress {
                // Materialize so the path we print actually exists.
                let working = self.workspace.ensure_materialized(ex)?;
                println!("Next exercise: {} ({})", ex.name, ex.id);
                println!("  Type: {:?}", ex.exercise_type);
                println!("  File: {}", working.display());
                println!("\n  Run: rust-game verify {}", ex.id);
                if !ex.hints.is_empty() {
                    println!("  Hints: rust-game hint {}", ex.id);
                }
                return Ok(());
            }
        }

        println!("All exercises completed! Congratulations!");
        Ok(())
    }

    /// Reset an exercise or all progress.
    pub fn cmd_reset(&mut self, exercise_id: Option<&str>, all: bool) -> Result<()> {
        if all {
            self.state = GameState::new();
            self.streak = StreakState::new();
            self.save()?;
            println!("All progress has been reset.");
            println!(
                "Your code in {} was left untouched — delete it manually if you want a clean slate.",
                self.workspace.root().display()
            );
            return Ok(());
        }

        if let Some(id) = exercise_id {
            let exercise = self
                .catalog
                .get_exercise(id)
                .context(format!("Exercise not found: '{}'", id))?
                .clone();

            // Say what is about to happen before doing it — this destroys the
            // student's edits for that exercise.
            let target = self.workspace.working_path(&exercise);
            if self.workspace.is_materialized(&exercise) {
                println!(
                    "Overwriting your working copy with the original exercise:\n  {}",
                    target.display()
                );
            } else {
                println!("Creating a fresh copy at:\n  {}", target.display());
            }

            let restored = self.workspace.reset(&exercise)?;
            self.state.forget_exercise(id);
            self.save()?;

            println!("Done. Progress for '{}' has been cleared.", id);
            debug_assert_eq!(restored, target);
            return Ok(());
        }

        println!("Specify --exercise <ID> or --all to reset.");
        Ok(())
    }

    /// Format exercise context for an AI tutor.
    pub fn cmd_hint_ai(&mut self, exercise_id: &str) -> Result<()> {
        let exercise = self
            .catalog
            .get_exercise(exercise_id)
            .context(format!("Exercise not found: '{}'", exercise_id))?;

        let exercise = exercise.clone();
        let source_path = self.workspace.source_path(&exercise);
        let source = std::fs::read_to_string(&source_path)
            .with_context(|| format!("Failed to read: {}", source_path.display()))?;

        // The compiler error is the single most useful thing a Rust tutor can
        // be handed, so never emit the block without one. If nothing is cached,
        // verify now — reading `source_path`, which is the same file printed
        // above, so this stays side-effect-free.
        if !self.last_result.contains_key(exercise_id) {
            eprintln!("Verifying to capture the current compiler output...");
            let run_id = pipeline::next_run_id();
            match pipeline::verify_exercise(&exercise, &source_path, &self.cache_dir, run_id) {
                Ok(result) => {
                    self.last_result.insert(exercise_id.to_string(), result);
                }
                Err(e) => eprintln!("Could not capture compiler output: {:#}", e),
            }
        }

        let context = crate::ai::context_formatter::format_ai_context(
            &exercise,
            &source,
            self.last_result.get(exercise_id),
            &self.catalog,
            &self.state,
        );

        println!("{}", context);
        Ok(())
    }

    /// Print a module's lesson.
    ///
    /// Accepts a module ID or any exercise ID inside that module, so an AI
    /// tutor can pass through whatever the student typed.
    pub fn cmd_lesson(&mut self, module_ref: &str, mark_read: bool) -> Result<()> {
        let module_id = self.resolve_module_id(module_ref)?;
        let module = self
            .catalog
            .get_module(&module_id)
            .context(format!("Module not found: '{}'", module_ref))?
            .clone();

        let loaded = crate::lesson::load(&module)?;
        match &loaded {
            Some(lesson) => {
                println!("{}", lesson.body);
            }
            None => {
                println!(
                    "No lesson written yet for {} — {}.",
                    module.name, module.theme_name
                );
                match &module.book_url {
                    Some(url) => println!("Read the Rust Book chapter instead: {}", url),
                    None => println!("See https://doc.rust-lang.org/book/ for background."),
                }
            }
        }

        if mark_read {
            self.state.mark_lesson_read(&module_id);
            self.save()?;
            let label = loaded
                .as_ref()
                .map(|l| l.title.clone())
                .unwrap_or_else(|| module.name.clone());
            println!("\n(Lesson \"{}\" marked as read.)", label);
        }

        Ok(())
    }

    /// Resolve a module ID or an exercise ID to a module ID.
    fn resolve_module_id(&self, module_ref: &str) -> Result<String> {
        if self.catalog.get_module(module_ref).is_some() {
            return Ok(module_ref.to_string());
        }
        if let Some(ex) = self.catalog.get_exercise(module_ref) {
            return Ok(ex.module_id.clone());
        }
        bail!(
            "No module or exercise matches '{}'. Try `rust-game list` to see the module IDs.",
            module_ref
        )
    }

    /// Show the reference solution for an exercise.
    pub fn cmd_solution(&mut self, exercise_id: &str) -> Result<()> {
        let exercise = self
            .catalog
            .get_exercise(exercise_id)
            .context(format!("Exercise not found: '{}'", exercise_id))?
            .clone();

        let solution = std::fs::read_to_string(&exercise.solution_path)
            .with_context(|| format!("Solution not found: {}", exercise.solution_path.display()))?;

        // Recorded here as well as in the TUI. Leaving this path silent would
        // make the marker and the stats line trivially bypassable, which is the
        // whole point of recording it.
        self.state.mark_solution_viewed(exercise_id);
        if let Err(e) = self.save() {
            eprintln!("Warning: could not record that you viewed this: {:#}", e);
        }

        println!("Solution for: {}\n", exercise.name);
        println!("{}", solution);
        Ok(())
    }

    /// Wipe sandbox compilation caches.
    pub fn cmd_clean_cache(&self) -> Result<()> {
        sandbox::clean_all_sandboxes(&self.cache_dir)?;
        println!("Sandbox cache cleaned.");
        Ok(())
    }
}
