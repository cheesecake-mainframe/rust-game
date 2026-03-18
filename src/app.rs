use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{bail, Context, Result};

use crate::exercise::catalog::ExerciseCatalog;
use crate::exercise::types::*;
use crate::game::{progress, streak::StreakState, xp};
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
}

impl App {
    /// Initialize the app: load catalog and state.
    pub fn init() -> Result<Self> {
        let exercises_dir = PathBuf::from("exercises");
        let cache_dir = PathBuf::from(".rust-game-cache");

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
        })
    }

    /// Create an App for testing — loads the real catalog but uses fresh state.
    /// Does not load or save persistent state.
    pub fn new_for_testing() -> Result<Self> {
        let exercises_dir = PathBuf::from("exercises");
        let cache_dir = PathBuf::from(".rust-game-test-cache");

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
        })
    }

    /// Save the current state to disk.
    pub fn save(&mut self) -> Result<()> {
        // Sync streak back to state before saving
        self.state.player.current_streak = self.streak.current;
        self.state.player.best_streak = self.streak.best;
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

        let run_id = pipeline::next_run_id();
        let start = Instant::now();
        let result = pipeline::verify_exercise(&exercise, &self.cache_dir, run_id)?;

        if result.passed() {
            let elapsed_secs = start.elapsed().as_secs();

            // Record attempt and complete
            self.state.record_attempt(exercise_id);
            let attempts = self.state.exercises.get(exercise_id).unwrap().attempts;

            let time_taken = exercise
                .time_limit_secs
                .map(|_| start.elapsed());

            let award = xp::calculate_xp(&exercise, attempts, time_taken, self.streak.current);
            let is_new = self.state.complete_exercise(exercise_id, award.total, Some(elapsed_secs));

            if is_new {
                self.streak.increment();
            }

            println!("  PASSED!");
            println!("  +{} XP (base: {}, first-try: {}, time: {}, streak: {:.2}x)",
                award.total, award.base, award.first_try_bonus,
                award.time_trial_bonus, award.streak_multiplier
            );
            println!(
                "  Level {} | {}/{} exercises | Streak: {}",
                self.state.player.level,
                self.state.exercises_completed(),
                self.catalog.total_exercises(),
                self.streak.current,
            );

            self.save()?;
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
                println!("Next exercise: {} ({})", ex.name, ex.id);
                println!("  Type: {:?}", ex.exercise_type);
                println!("  File: {}", ex.file_path.display());
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
            return Ok(());
        }

        if let Some(id) = exercise_id {
            self.state.exercises.remove(id);
            self.save()?;
            println!("Exercise '{}' has been reset.", id);
            return Ok(());
        }

        println!("Specify --exercise <ID> or --all to reset.");
        Ok(())
    }

    /// Format exercise context for an AI tutor.
    pub fn cmd_hint_ai(&self, exercise_id: &str) -> Result<()> {
        let exercise = self
            .catalog
            .get_exercise(exercise_id)
            .context(format!("Exercise not found: '{}'", exercise_id))?;

        let source = std::fs::read_to_string(&exercise.file_path)
            .with_context(|| format!("Failed to read: {}", exercise.file_path.display()))?;

        let context = crate::ai::context_formatter::format_ai_context(
            exercise,
            &source,
            None, // no verification result for CLI
            &self.catalog,
            &self.state,
        );

        println!("{}", context);
        Ok(())
    }

    /// Show the reference solution for an exercise.
    pub fn cmd_solution(&self, exercise_id: &str) -> Result<()> {
        let exercise = self
            .catalog
            .get_exercise(exercise_id)
            .context(format!("Exercise not found: '{}'", exercise_id))?;

        let solution = std::fs::read_to_string(&exercise.solution_path)
            .with_context(|| format!("Solution not found: {}", exercise.solution_path.display()))?;

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
