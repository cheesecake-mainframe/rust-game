// Solution: Unit Tests
// =====================

/// Returns the absolute value of a number.
fn absolute_value(n: i64) -> i64 {
    if n < 0 { -n } else { n }
}

/// Checks if a string is a palindrome (case-insensitive, ignoring spaces).
fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    let reversed: String = cleaned.chars().rev().collect();
    cleaned == reversed
}

/// Returns the factorial of n. Panics if n > 20 (overflow protection).
fn factorial(n: u64) -> u64 {
    if n > 20 {
        panic!("Input too large! Maximum is 20.");
    }
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

/// Finds the maximum value in a slice. Returns None for empty slices.
fn find_max(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

/// Clamps a value between min and max (inclusive).
/// Panics if min > max.
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if min > max {
        panic!("min ({}) must not be greater than max ({})", min, max);
    }
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Splits a string into words and returns the count.
fn word_count(text: &str) -> usize {
    if text.trim().is_empty() {
        return 0;
    }
    text.split_whitespace().count()
}

fn main() {
    println!("This exercise is about writing tests!");
    println!("Run with: rustc --test unit_tests.rs && ./unit_tests");
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- absolute_value tests ---

    #[test]
    fn absolute_value_positive() {
        assert_eq!(absolute_value(42), 42);
    }

    #[test]
    fn absolute_value_negative() {
        assert_eq!(absolute_value(-42), 42);
    }

    #[test]
    fn absolute_value_zero() {
        assert_eq!(absolute_value(0), 0);
    }

    // --- is_palindrome tests ---

    #[test]
    fn palindrome_simple() {
        assert!(is_palindrome("racecar"));
    }

    #[test]
    fn palindrome_case_insensitive() {
        assert!(is_palindrome("Racecar"));
    }

    #[test]
    fn palindrome_with_spaces() {
        assert!(is_palindrome("A man a plan a canal Panama"));
    }

    #[test]
    fn not_a_palindrome() {
        assert!(!is_palindrome("hello world"));
    }

    #[test]
    fn palindrome_empty_string() {
        assert!(is_palindrome(""));
    }

    // --- factorial tests ---

    #[test]
    fn factorial_zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_one() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn factorial_five() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn factorial_boundary() {
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
    }

    #[test]
    #[should_panic(expected = "Input too large")]
    fn factorial_overflow_panics() {
        factorial(21);
    }

    // --- find_max tests ---

    #[test]
    fn find_max_normal() {
        assert_eq!(find_max(&[1, 5, 3, 9, 2]), Some(9));
    }

    #[test]
    fn find_max_single_element() {
        assert_eq!(find_max(&[42]), Some(42));
    }

    #[test]
    fn find_max_empty() {
        assert_eq!(find_max(&[]), None);
    }

    #[test]
    fn find_max_negative_numbers() {
        assert_eq!(find_max(&[-5, -1, -10, -3]), Some(-1));
    }

    // --- clamp tests ---

    #[test]
    fn clamp_within_range() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
    }

    #[test]
    fn clamp_below_min() {
        assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
    }

    #[test]
    fn clamp_above_max() {
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    #[should_panic(expected = "must not be greater than max")]
    fn clamp_invalid_range() {
        clamp(5.0, 10.0, 0.0);
    }

    // --- word_count tests ---

    #[test]
    fn word_count_normal_sentence() {
        assert_eq!(word_count("hello world foo bar"), 4);
    }

    #[test]
    fn word_count_empty() {
        assert_eq!(word_count(""), 0);
    }

    #[test]
    fn word_count_only_spaces() {
        assert_eq!(word_count("    "), 0);
    }

    #[test]
    fn word_count_single_word() {
        assert_eq!(word_count("hello"), 1);
    }
}
