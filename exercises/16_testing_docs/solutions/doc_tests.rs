// Solution: Doc Tests
// ====================

/// Adds two numbers together and returns the result.
///
/// # Examples
///
/// ```
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
/// // FIX: Function name corrected to `repeat_str` and comparison uses String.
/// let result = repeat_str("ha", 3);
/// assert_eq!(result, "hahaha".to_string());
/// ```
pub fn repeat_str(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// Divides two numbers, returning an error message for division by zero.
///
/// # Examples
///
/// ```
/// // FIX: Removed extra argument, unwrap the Result, use approximate comparison.
/// let result = safe_divide(10.0, 3.0).unwrap();
/// assert!((result - 3.3333).abs() < 0.001);
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
/// // FIX: Added variable binding for the return value.
/// let greeting = greet("Alice");
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
/// let numbers = vec![10, 20, 30];
/// let first = first_element(&numbers);
/// assert_eq!(first.unwrap_or_default(), 10);
///
/// let empty: Vec<i32> = vec![];
/// let first = first_element(&empty);
/// // FIX: Option doesn't have a .value() method -- just compare directly.
/// assert_eq!(first, None);
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
/// // FIX: Corrected the expected boolean values.
/// assert_eq!(is_leap_year(2000), true);   // divisible by 400
/// assert_eq!(is_leap_year(1900), false);  // divisible by 100 but not 400
/// assert_eq!(is_leap_year(2024), true);   // divisible by 4 but not 100
/// assert_eq!(is_leap_year(2023), false);  // not divisible by 4
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
