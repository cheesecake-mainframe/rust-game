fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}

fn last_n_chars(s: &str, n: usize) -> &str {
    if n >= s.len() {
        s
    } else {
        &s[s.len() - n..]
    }
}

fn starts_with(s: &str, prefix: &str) -> bool {
    if prefix.len() > s.len() {
        return false;
    }
    &s[..prefix.len()] == prefix
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
