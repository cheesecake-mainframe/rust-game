/// Calculate the XP required to reach a given level.
///
/// Formula: XP = 25 * (level - 1)^1.8
/// Level 1 requires 0 XP.
pub fn xp_for_level(level: u32) -> u64 {
    if level <= 1 {
        return 0;
    }
    (25.0 * ((level - 1) as f64).powf(1.8)).round() as u64
}

/// Determine the level for a given XP total.
/// Returns the highest level where xp_for_level(level) <= xp.
pub fn level_from_xp(xp: u64) -> u32 {
    let mut level = 1;
    while xp_for_level(level + 1) <= xp {
        level += 1;
    }
    level
}

/// XP needed to reach the next level from the current XP.
pub fn xp_to_next_level(current_xp: u64) -> u64 {
    let current_level = level_from_xp(current_xp);
    let next_threshold = xp_for_level(current_level + 1);
    next_threshold.saturating_sub(current_xp)
}

/// Progress toward the next level as a fraction (0.0 to 1.0).
pub fn level_progress(current_xp: u64) -> f64 {
    let current_level = level_from_xp(current_xp);
    let current_threshold = xp_for_level(current_level);
    let next_threshold = xp_for_level(current_level + 1);
    let range = next_threshold - current_threshold;
    if range == 0 {
        return 1.0;
    }
    (current_xp - current_threshold) as f64 / range as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_1_is_zero() {
        assert_eq!(xp_for_level(1), 0);
    }

    #[test]
    fn test_level_2_is_25() {
        assert_eq!(xp_for_level(2), 25);
    }

    #[test]
    fn test_thresholds_are_monotonically_increasing() {
        for l in 1..=30 {
            assert!(
                xp_for_level(l + 1) > xp_for_level(l),
                "Level {} ({}) should be less than level {} ({})",
                l,
                xp_for_level(l),
                l + 1,
                xp_for_level(l + 1)
            );
        }
    }

    #[test]
    fn test_level_from_xp_basic() {
        assert_eq!(level_from_xp(0), 1);
        assert_eq!(level_from_xp(24), 1);
        assert_eq!(level_from_xp(25), 2);
        assert_eq!(level_from_xp(26), 2);
    }

    #[test]
    fn test_level_from_xp_boundaries() {
        for l in 1..=20 {
            let threshold = xp_for_level(l);
            assert_eq!(
                level_from_xp(threshold),
                l,
                "At XP={}, expected level {}",
                threshold,
                l
            );
        }
    }

    #[test]
    fn test_xp_to_next_level() {
        assert_eq!(xp_to_next_level(0), 25); // Need 25 to reach level 2
        assert_eq!(xp_to_next_level(25), xp_for_level(3) - 25);
    }

    #[test]
    fn test_level_progress() {
        let progress = level_progress(0);
        assert!((progress - 0.0).abs() < f64::EPSILON);

        // At the exact threshold, progress should be 0.0 for the new level
        let progress = level_progress(xp_for_level(3));
        assert!((progress - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_reachable_with_100_exercises() {
        // ~100 exercises averaging 20 XP each = ~2000 base
        // With bonuses, roughly 2500-3500
        let level_at_2500 = level_from_xp(2500);
        let level_at_3500 = level_from_xp(3500);
        // Should reach level 15-19 range
        assert!(level_at_2500 >= 13, "2500 XP should be at least level 13, got {}", level_at_2500);
        assert!(level_at_3500 <= 20, "3500 XP should be at most level 20, got {}", level_at_3500);
    }
}
