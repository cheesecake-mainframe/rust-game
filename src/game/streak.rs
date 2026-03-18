use chrono::{DateTime, Utc};

/// Tracks the player's exercise completion streak.
#[derive(Debug, Clone)]
pub struct StreakState {
    pub current: u32,
    pub best: u32,
}

impl StreakState {
    pub fn new() -> Self {
        Self {
            current: 0,
            best: 0,
        }
    }

    /// Restore from saved state.
    pub fn from_saved(current: u32, best: u32) -> Self {
        Self { current, best }
    }

    /// Called when an exercise is completed successfully.
    pub fn increment(&mut self) {
        self.current += 1;
        if self.current > self.best {
            self.best = self.current;
        }
    }

    /// Called when the student explicitly quits or skips an exercise.
    /// NOT called on failure — failures don't break streaks.
    pub fn break_streak(&mut self) {
        self.current = 0;
    }

    /// Check the 24-hour grace period on app startup.
    /// If last_exercise_at is more than 24 hours ago, reset the streak.
    pub fn check_grace_period(&mut self, last_exercise_at: Option<DateTime<Utc>>) {
        if let Some(last) = last_exercise_at {
            let elapsed = Utc::now().signed_duration_since(last);
            if elapsed.num_hours() >= 24 {
                self.current = 0;
            }
        }
    }

    /// Get the XP multiplier for the current streak.
    pub fn multiplier(&self) -> f64 {
        crate::game::xp::streak_multiplier_for(self.current)
    }
}

impl Default for StreakState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_new_streak() {
        let s = StreakState::new();
        assert_eq!(s.current, 0);
        assert_eq!(s.best, 0);
        assert_eq!(s.multiplier(), 1.0);
    }

    #[test]
    fn test_increment() {
        let mut s = StreakState::new();
        s.increment();
        assert_eq!(s.current, 1);
        assert_eq!(s.best, 1);
        s.increment();
        assert_eq!(s.current, 2);
        assert_eq!(s.best, 2);
    }

    #[test]
    fn test_break_and_best_preserved() {
        let mut s = StreakState::new();
        s.increment();
        s.increment();
        s.increment();
        assert_eq!(s.best, 3);
        s.break_streak();
        assert_eq!(s.current, 0);
        assert_eq!(s.best, 3); // Best is preserved
    }

    #[test]
    fn test_grace_period_within_24h() {
        let mut s = StreakState::from_saved(5, 5);
        let recent = Utc::now() - Duration::hours(12);
        s.check_grace_period(Some(recent));
        assert_eq!(s.current, 5); // Preserved
    }

    #[test]
    fn test_grace_period_expired() {
        let mut s = StreakState::from_saved(5, 10);
        let old = Utc::now() - Duration::hours(25);
        s.check_grace_period(Some(old));
        assert_eq!(s.current, 0); // Reset
        assert_eq!(s.best, 10); // Best preserved
    }

    #[test]
    fn test_grace_period_no_previous() {
        let mut s = StreakState::from_saved(3, 3);
        s.check_grace_period(None);
        assert_eq!(s.current, 3); // No change when no timestamp
    }

    #[test]
    fn test_multiplier_follows_streak() {
        let mut s = StreakState::new();
        assert_eq!(s.multiplier(), 1.0);
        s.increment();
        s.increment();
        assert_eq!(s.multiplier(), 1.25); // streak of 2
        s.increment();
        assert_eq!(s.multiplier(), 1.5); // streak of 3
    }
}
