// Exercise: Unit Tests (ImplementFromScratch)
// =============================================
//
// Testing is a fundamental part of Rust development. Rust has first-class
// support for unit testing built right into the language and tooling.
//
// Below are several functions that perform various operations. Your task is
// to write comprehensive unit tests for each function. You should:
//
// - Use `assert_eq!` to check expected return values
// - Use `assert!` to check boolean conditions
// - Use `assert_ne!` to verify values differ
// - Use `#[should_panic]` for functions that should panic on bad input
// - Test edge cases (empty inputs, zero, negative numbers, boundaries)
//
// The functions are complete and correct -- you just need to test them!

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

// TODO: Write comprehensive unit tests for ALL the functions above.
//
// Your test module should include at minimum:
//
// 1. Tests for `absolute_value`:
//    - Positive number stays positive
//    - Negative number becomes positive
//    - Zero remains zero
//
// 2. Tests for `is_palindrome`:
//    - A simple palindrome like "racecar"
//    - Case-insensitive: "Racecar" is still a palindrome
//    - With spaces: "A man a plan a canal Panama"
//    - Non-palindrome returns false
//    - Empty string is a palindrome
//
// 3. Tests for `factorial`:
//    - factorial(0) == 1
//    - factorial(1) == 1
//    - factorial(5) == 120
//    - factorial(20) should work (boundary)
//    - factorial(21) should panic (use #[should_panic])
//
// 4. Tests for `find_max`:
//    - Normal slice with multiple values
//    - Single element slice
//    - Empty slice returns None
//    - Slice with negative numbers
//
// 5. Tests for `clamp`:
//    - Value within range stays the same
//    - Value below min gets clamped to min
//    - Value above max gets clamped to max
//    - min > max should panic (use #[should_panic])
//
// 6. Tests for `word_count`:
//    - Normal sentence
//    - Empty string returns 0
//    - String with only spaces returns 0
//    - Single word

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Write all the tests described above!
    // Each test should be a function annotated with #[test].
    // For panic tests, use #[test] and #[should_panic].
}
