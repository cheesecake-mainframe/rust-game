// Exercise: Option Type
// Type: debug_logic_bug
// Difficulty: intermediate
//
// Rust has no null values. Instead, `Option<T>` represents a value
// that might or might not exist:
//   - Some(value) — the value is present
//   - None        — no value
//
// The code below compiles but has logic bugs in how it handles
// Option values. Fix the bugs so all tests pass.

/// Find the first even number in a slice.
fn first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    // BUG: returning Some(0) when no even number is found
    Some(0)
}

/// Safely divide two numbers, returning None if divisor is zero.
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    // BUG: this will panic on division by zero instead of returning None
    Some(a / b)
}

/// Look up a student's grade. Returns None if student not found.
fn get_grade(name: &str) -> Option<char> {
    match name {
        "Alice" => Some('A'),
        "Bob" => Some('B'),
        "Charlie" => Some('C'),
        _ => Some('F'),  // BUG: unknown students should return None, not 'F'
    }
}

/// Format a greeting using an optional title.
/// "Hello, Dr. Smith!" or "Hello, Smith!" if no title.
fn format_greeting(title: Option<&str>, name: &str) -> String {
    // BUG: unwrap() will panic if title is None
    format!("Hello, {} {}!", title.unwrap(), name)
}

/// Get the longer of two optional strings.
/// Returns None if both are None.
fn longer_option(a: Option<&str>, b: Option<&str>) -> Option<&str> {
    // BUG: wrong logic — should compare lengths, not just return a
    match (a, b) {
        (Some(s), _) => Some(s),
        (_, Some(s)) => Some(s),
        _ => None,
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_even_found() {
        assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
    }

    #[test]
    fn test_first_even_not_found() {
        assert_eq!(first_even(&[1, 3, 5, 7]), None);
    }

    #[test]
    fn test_first_even_empty() {
        assert_eq!(first_even(&[]), None);
    }

    #[test]
    fn test_safe_divide_normal() {
        assert_eq!(safe_divide(10.0, 3.0), Some(10.0 / 3.0));
    }

    #[test]
    fn test_safe_divide_by_zero() {
        assert_eq!(safe_divide(10.0, 0.0), None);
    }

    #[test]
    fn test_grade_known_student() {
        assert_eq!(get_grade("Alice"), Some('A'));
        assert_eq!(get_grade("Bob"), Some('B'));
    }

    #[test]
    fn test_grade_unknown_student() {
        assert_eq!(get_grade("Unknown"), None);
    }

    #[test]
    fn test_greeting_with_title() {
        assert_eq!(format_greeting(Some("Dr."), "Smith"), "Hello, Dr. Smith!");
    }

    #[test]
    fn test_greeting_without_title() {
        assert_eq!(format_greeting(None, "Smith"), "Hello, Smith!");
    }

    #[test]
    fn test_longer_both_some() {
        assert_eq!(longer_option(Some("hi"), Some("hello")), Some("hello"));
    }

    #[test]
    fn test_longer_one_none() {
        assert_eq!(longer_option(Some("hi"), None), Some("hi"));
        assert_eq!(longer_option(None, Some("hello")), Some("hello"));
    }

    #[test]
    fn test_longer_both_none() {
        let result: Option<&str> = longer_option(None, None);
        assert_eq!(result, None);
    }
}
