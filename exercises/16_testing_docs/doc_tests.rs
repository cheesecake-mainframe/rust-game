// Exercise: Doc Tests (FixCompilerError)
// ========================================
//
// Rust supports documentation comments (///) that can include code examples.
// These examples are actually compiled and run as tests when you run
// `cargo test` or `rustdoc --test`.
//
// Doc-test examples use markdown code blocks with ```rust (or just ```).
// The code inside must compile and run correctly.
//
// This file has several documentation comments with broken code examples.
// Fix each doc-test so it compiles correctly.
//
// HINTS:
// - Doc-test code examples must be valid Rust
// - Function calls must match the actual function signature
// - Types must match (e.g., don't compare &str to String without conversion)
// - If a function returns Result or Option, handle it in the doc-test
// - Make sure `assert_eq!` arguments have matching types

/// Adds two numbers together and returns the result.
///
/// # Examples
///
/// ```
/// // FIX: The function call doesn't match the signature (wrong argument types).
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Repeats a string `n` times and returns the result.
///
/// # Examples
///
/// ```
/// // FIX: The function returns String, but the assert compares with &str.
/// // Also the function name is wrong in the example.
/// let result = repeat_string("ha", 3);
/// assert_eq!(result, "hahaha");
/// ```
pub fn repeat_str(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// Divides two numbers, returning an error message for division by zero.
///
/// # Examples
///
/// ```
/// // FIX: The Result needs to be unwrapped or matched to get the inner value.
/// // Also, the function is called with the wrong number of arguments.
/// let result = safe_divide(10.0, 3.0, 1);
/// assert_eq!(result, 3.3333);
/// ```
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Creates a greeting message for the given name.
///
/// # Examples
///
/// ```
/// // FIX: Missing variable binding and wrong comparison type.
/// greet("Alice");
/// assert_eq!(greeting, "Hello, Alice!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Returns the first element of a slice, or None if empty.
///
/// # Examples
///
/// ```
/// // FIX: The doc-test uses a method that doesn't exist on Option.
/// let numbers = vec![10, 20, 30];
/// let first = first_element(&numbers);
/// assert_eq!(first.unwrap_or_default(), 10);
///
/// let empty: Vec<i32> = vec![];
/// let first = first_element(&empty);
/// assert_eq!(first.value(), None);
/// ```
pub fn first_element(slice: &[i32]) -> Option<i32> {
    slice.first().copied()
}

/// Checks whether a year is a leap year.
///
/// A year is a leap year if:
/// - It is divisible by 4 AND
/// - It is NOT divisible by 100, UNLESS it is also divisible by 400
///
/// # Examples
///
/// ```
/// // FIX: The assertions use the wrong expected values.
/// assert_eq!(is_leap_year(2000), false);
/// assert_eq!(is_leap_year(1900), true);
/// assert_eq!(is_leap_year(2024), false);
/// assert_eq!(is_leap_year(2023), true);
/// ```
pub fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn main() {
    println!("Fix the doc-tests so they compile correctly!");
    println!("Test with: rustdoc --test doc_tests.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_repeat_str() {
        assert_eq!(repeat_str("ab", 2), "abab");
    }

    #[test]
    fn test_safe_divide() {
        assert!(safe_divide(1.0, 0.0).is_err());
        assert!((safe_divide(10.0, 4.0).unwrap() - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn test_first_element() {
        assert_eq!(first_element(&[5, 10]), Some(5));
        assert_eq!(first_element(&[]), None);
    }

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
    }
}
