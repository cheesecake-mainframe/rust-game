// Exercise: String Slices
// Type: implement_from_scratch
// Difficulty: intermediate
//
// A string slice (&str) is a reference to a portion of a String.
// It's the most common way to pass strings around without taking ownership.
//
// Implement the functions below using string slices.

/// Return the first word of a string (up to the first space).
/// If there's no space, return the entire string.
fn first_word(s: &str) -> &str {
    todo!()
}

/// Return the last n characters of a string.
/// If n >= s.len(), return the entire string.
fn last_n_chars(s: &str, n: usize) -> &str {
    todo!()
}

/// Check if a string starts with a given prefix.
/// (Implement without using the built-in starts_with method!)
fn starts_with(s: &str, prefix: &str) -> bool {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("multiple words here"), "multiple");
    }

    #[test]
    fn test_last_n_chars() {
        assert_eq!(last_n_chars("hello", 3), "llo");
        assert_eq!(last_n_chars("hello", 5), "hello");
        assert_eq!(last_n_chars("hello", 10), "hello");
        assert_eq!(last_n_chars("hello", 0), "");
    }

    #[test]
    fn test_starts_with() {
        assert!(starts_with("hello world", "hello"));
        assert!(starts_with("hello", "hello"));
        assert!(!starts_with("hello", "world"));
        assert!(starts_with("anything", ""));
        assert!(!starts_with("", "something"));
    }
}
