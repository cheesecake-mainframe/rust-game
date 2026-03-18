fn first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn get_grade(name: &str) -> Option<char> {
    match name {
        "Alice" => Some('A'),
        "Bob" => Some('B'),
        "Charlie" => Some('C'),
        _ => None,
    }
}

fn format_greeting(title: Option<&str>, name: &str) -> String {
    match title {
        Some(t) => format!("Hello, {} {}!", t, name),
        None => format!("Hello, {}!", name),
    }
}

fn longer_option<'a>(a: Option<&'a str>, b: Option<&'a str>) -> Option<&'a str> {
    match (a, b) {
        (Some(sa), Some(sb)) => {
            if sa.len() >= sb.len() {
                Some(sa)
            } else {
                Some(sb)
            }
        }
        (Some(s), None) => Some(s),
        (None, Some(s)) => Some(s),
        (None, None) => None,
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
