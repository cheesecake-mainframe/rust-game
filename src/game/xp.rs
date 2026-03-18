use std::time::Duration;

use crate::exercise::types::Exercise;

/// Breakdown of how XP was calculated for an exercise completion.
#[derive(Debug, Clone)]
pub struct XpAward {
    pub base: u32,
    pub first_try_bonus: u32,
    pub time_trial_bonus: u32,
    pub streak_multiplier: f64,
    pub total: u32,
}

/// Calculate XP earned for completing an exercise.
///
/// Formula: total = (base + first_try_bonus + time_trial_bonus) * streak_multiplier
pub fn calculate_xp(
    exercise: &Exercise,
    attempts: u32,
    time_taken: Option<Duration>,
    current_streak: u32,
) -> XpAward {
    let base = exercise.base_xp;

    // First-try bonus: +50% of base if solved on attempt 1
    let first_try_bonus = if attempts == 1 { base / 2 } else { 0 };

    // Time trial bonus: +15 XP if exercise has a time limit and solved under it
    let time_trial_bonus = match (exercise.time_limit_secs, time_taken) {
        (Some(limit), Some(taken)) if taken.as_secs() < limit as u64 => 15,
        _ => 0,
    };

    let streak_multiplier = streak_multiplier_for(current_streak);

    let subtotal = base + first_try_bonus + time_trial_bonus;
    let total = (subtotal as f64 * streak_multiplier).round() as u32;

    XpAward {
        base,
        first_try_bonus,
        time_trial_bonus,
        streak_multiplier,
        total,
    }
}

/// Get the XP multiplier for the current streak count.
pub fn streak_multiplier_for(streak: u32) -> f64 {
    match streak {
        0..=1 => 1.0,
        2 => 1.25,
        3..=4 => 1.5,
        5..=9 => 2.0,
        _ => 3.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercise::types::*;
    use std::path::PathBuf;

    fn make_exercise(base_xp: u32, time_limit: Option<u32>) -> Exercise {
        Exercise {
            id: "test/ex1".into(),
            name: "Test".into(),
            module_id: "test".into(),
            exercise_type: ExerciseType::FixCompilerError,
            difficulty: Difficulty::Beginner,
            base_xp,
            file_path: PathBuf::from("test.rs"),
            solution_path: PathBuf::from("test_solution.rs"),
            hints: vec![],
            description: "test".into(),
            flavor_text: None,
            time_limit_secs: time_limit,
            custom_checks: vec![],
            multiple_choice_options: vec![],
            extra_files: vec![],
            order: 1,
            ci: false,
        }
    }

    #[test]
    fn test_base_xp_no_bonuses() {
        let ex = make_exercise(10, None);
        let award = calculate_xp(&ex, 2, None, 0);
        assert_eq!(award.base, 10);
        assert_eq!(award.first_try_bonus, 0);
        assert_eq!(award.time_trial_bonus, 0);
        assert_eq!(award.streak_multiplier, 1.0);
        assert_eq!(award.total, 10);
    }

    #[test]
    fn test_first_try_bonus() {
        let ex = make_exercise(20, None);
        let award = calculate_xp(&ex, 1, None, 0);
        assert_eq!(award.first_try_bonus, 10); // 50% of 20
        assert_eq!(award.total, 30); // 20 + 10
    }

    #[test]
    fn test_time_trial_under_limit() {
        let ex = make_exercise(10, Some(120));
        let award = calculate_xp(&ex, 2, Some(Duration::from_secs(60)), 0);
        assert_eq!(award.time_trial_bonus, 15);
        assert_eq!(award.total, 25); // 10 + 15
    }

    #[test]
    fn test_time_trial_over_limit() {
        let ex = make_exercise(10, Some(120));
        let award = calculate_xp(&ex, 2, Some(Duration::from_secs(200)), 0);
        assert_eq!(award.time_trial_bonus, 0);
        assert_eq!(award.total, 10);
    }

    #[test]
    fn test_streak_multipliers() {
        assert_eq!(streak_multiplier_for(0), 1.0);
        assert_eq!(streak_multiplier_for(1), 1.0);
        assert_eq!(streak_multiplier_for(2), 1.25);
        assert_eq!(streak_multiplier_for(3), 1.5);
        assert_eq!(streak_multiplier_for(4), 1.5);
        assert_eq!(streak_multiplier_for(5), 2.0);
        assert_eq!(streak_multiplier_for(9), 2.0);
        assert_eq!(streak_multiplier_for(10), 3.0);
        assert_eq!(streak_multiplier_for(100), 3.0);
    }

    #[test]
    fn test_all_bonuses_combined() {
        let ex = make_exercise(20, Some(120));
        // First try, under time, streak of 5
        let award = calculate_xp(&ex, 1, Some(Duration::from_secs(60)), 5);
        // base=20, first_try=10, time_trial=15 → subtotal=45 × 2.0 = 90
        assert_eq!(award.total, 90);
    }

    #[test]
    fn test_xp_idempotent_same_inputs() {
        let ex = make_exercise(15, None);
        let a1 = calculate_xp(&ex, 1, None, 3);
        let a2 = calculate_xp(&ex, 1, None, 3);
        assert_eq!(a1.total, a2.total);
    }
}
