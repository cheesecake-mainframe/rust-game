use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::exercise::types::ExerciseStatus;

/// Root state object — persisted to disk as JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub version: u32,
    pub player: PlayerState,
    pub exercises: HashMap<String, ExerciseState>,
    pub modules: HashMap<String, ModuleState>,
    /// Which module lessons have been read, keyed by module ID.
    ///
    /// `#[serde(default)]` so a version-1 state file (which predates lessons)
    /// still parses before the migration in `persistence` runs.
    #[serde(default)]
    pub lessons: HashMap<String, LessonState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub xp: u64,
    pub level: u32,
    pub current_streak: u32,
    pub best_streak: u32,
    pub total_time_played_secs: u64,
    pub last_exercise_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseState {
    pub status: ExerciseStatus,
    pub xp_earned: u32,
    pub attempts: u32,
    pub completed_at: Option<DateTime<Utc>>,
    pub time_taken_secs: Option<u64>,
    pub first_attempted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LessonState {
    pub read: bool,
    pub read_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleState {
    pub unlocked: bool,
    pub completed: bool,
    pub unlocked_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl GameState {
    /// Create a fresh state for a new player.
    pub fn new() -> Self {
        Self {
            version: 1,
            player: PlayerState {
                xp: 0,
                level: 1,
                current_streak: 0,
                best_streak: 0,
                total_time_played_secs: 0,
                last_exercise_at: None,
            },
            exercises: HashMap::new(),
            modules: HashMap::new(),
            lessons: HashMap::new(),
        }
    }

    /// Mark a module's lesson as read. Idempotent — re-marking only refreshes
    /// the timestamp, which keeps the unread indicator honest.
    pub fn mark_lesson_read(&mut self, module_id: &str) {
        let entry = self.lessons.entry(module_id.to_string()).or_default();
        entry.read = true;
        entry.read_at = Some(Utc::now());
    }

    /// Has this module's lesson been read?
    pub fn is_lesson_read(&self, module_id: &str) -> bool {
        self.lessons.get(module_id).map(|l| l.read).unwrap_or(false)
    }

    /// Forget an exercise entirely: drop its record and take back the XP it
    /// awarded, recomputing the level.
    ///
    /// Plain `exercises.remove()` is not enough — `complete_exercise` adds the
    /// award to `player.xp`, so removing only the entry would leave that XP
    /// banked and let a re-solve count it a second time.
    pub fn forget_exercise(&mut self, exercise_id: &str) {
        if let Some(removed) = self.exercises.remove(exercise_id) {
            self.player.xp = self.player.xp.saturating_sub(removed.xp_earned as u64);
            self.player.level = crate::game::level::level_from_xp(self.player.xp);
        }
    }

    /// Count of completed exercises (computed, not stored).
    pub fn exercises_completed(&self) -> usize {
        self.exercises
            .values()
            .filter(|e| e.status == ExerciseStatus::Completed)
            .count()
    }

    /// Get or create the exercise state for an exercise ID.
    pub fn get_or_create_exercise(&mut self, exercise_id: &str) -> &mut ExerciseState {
        self.exercises
            .entry(exercise_id.to_string())
            .or_insert_with(|| ExerciseState {
                status: ExerciseStatus::Available,
                xp_earned: 0,
                attempts: 0,
                completed_at: None,
                time_taken_secs: None,
                first_attempted_at: None,
            })
    }

    /// Record an exercise attempt (increment counter, set first_attempted_at).
    pub fn record_attempt(&mut self, exercise_id: &str) {
        let state = self.get_or_create_exercise(exercise_id);
        state.attempts += 1;
        if state.first_attempted_at.is_none() {
            state.first_attempted_at = Some(Utc::now());
        }
        if state.status == ExerciseStatus::Available {
            state.status = ExerciseStatus::InProgress;
        }
    }

    /// Mark an exercise as completed and award XP.
    /// Returns true if this is a NEW completion (not already completed).
    /// This is the idempotency check — calling this twice for the same
    /// exercise does not double-award XP.
    pub fn complete_exercise(&mut self, exercise_id: &str, xp: u32, time_secs: Option<u64>) -> bool {
        let state = self.get_or_create_exercise(exercise_id);
        if state.status == ExerciseStatus::Completed {
            return false; // Already completed — no double award
        }
        state.status = ExerciseStatus::Completed;
        state.xp_earned = xp;
        state.completed_at = Some(Utc::now());
        state.time_taken_secs = time_secs;

        self.player.xp += xp as u64;
        self.player.level = crate::game::level::level_from_xp(self.player.xp);
        self.player.last_exercise_at = Some(Utc::now());

        true
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state() {
        let state = GameState::new();
        assert_eq!(state.version, 1);
        assert_eq!(state.player.xp, 0);
        assert_eq!(state.player.level, 1);
        assert_eq!(state.exercises_completed(), 0);
    }

    #[test]
    fn test_forget_exercise_takes_back_its_xp() {
        let mut state = GameState::new();
        state.complete_exercise("ex1", 40, None);
        state.complete_exercise("ex2", 60, None);
        assert_eq!(state.player.xp, 100);
        let level_with_both = state.player.level;

        state.forget_exercise("ex2");

        // The entry is gone AND its XP is no longer banked — otherwise
        // re-solving would count the same award twice.
        assert!(!state.exercises.contains_key("ex2"));
        assert_eq!(state.player.xp, 40);
        assert!(state.player.level <= level_with_both);
        assert_eq!(
            state.player.level,
            crate::game::level::level_from_xp(40),
            "level must be recomputed from the reduced XP"
        );
    }

    #[test]
    fn test_forget_unknown_exercise_is_a_no_op() {
        let mut state = GameState::new();
        state.complete_exercise("ex1", 40, None);
        state.forget_exercise("never_started");
        assert_eq!(state.player.xp, 40);
    }

    #[test]
    fn test_lesson_read_tracking() {
        let mut state = GameState::new();
        assert!(!state.is_lesson_read("04_ownership_moves"));
        state.mark_lesson_read("04_ownership_moves");
        assert!(state.is_lesson_read("04_ownership_moves"));
        assert!(state.lessons["04_ownership_moves"].read_at.is_some());
    }

    #[test]
    fn test_record_attempt() {
        let mut state = GameState::new();
        state.record_attempt("ex1");
        let ex = state.exercises.get("ex1").unwrap();
        assert_eq!(ex.attempts, 1);
        assert_eq!(ex.status, ExerciseStatus::InProgress);
        assert!(ex.first_attempted_at.is_some());
    }

    #[test]
    fn test_complete_exercise() {
        let mut state = GameState::new();
        state.record_attempt("ex1");
        let is_new = state.complete_exercise("ex1", 15, Some(30));
        assert!(is_new);
        assert_eq!(state.player.xp, 15);
        assert_eq!(state.exercises_completed(), 1);
        let ex = state.exercises.get("ex1").unwrap();
        assert_eq!(ex.status, ExerciseStatus::Completed);
        assert_eq!(ex.xp_earned, 15);
    }

    #[test]
    fn test_complete_exercise_idempotent() {
        let mut state = GameState::new();
        state.record_attempt("ex1");
        state.complete_exercise("ex1", 15, None);
        let is_new = state.complete_exercise("ex1", 15, None);
        assert!(!is_new); // Second completion returns false
        assert_eq!(state.player.xp, 15); // XP not doubled
    }

    #[test]
    fn test_level_updates_on_completion() {
        let mut state = GameState::new();
        // Award enough XP to reach level 2 (25 XP needed)
        state.complete_exercise("ex1", 30, None);
        assert_eq!(state.player.level, 2);
    }

    #[test]
    fn test_exercises_completed_count() {
        let mut state = GameState::new();
        state.complete_exercise("ex1", 10, None);
        state.complete_exercise("ex2", 10, None);
        state.record_attempt("ex3"); // In progress, not completed
        assert_eq!(state.exercises_completed(), 2);
    }
}
