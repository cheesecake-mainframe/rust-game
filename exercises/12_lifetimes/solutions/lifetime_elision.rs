// ========================================
// Solution: Lifetime Elision
// ========================================
// Key fixes:
// 1. longer_string: Both params share lifetime 'a, return the actual longer one.
// 2. content_or_default: Give `default` the same lifetime as the struct, return it when empty.
// 3. pick_greeting: Both params share lifetime 'a, return the correct one.
// 4. find_shortest: Return &str with proper lifetime instead of owned String.
// 5. safe_prefix: Return &str instead of owned String.

/// Both parameters share the same lifetime so either can be returned.
fn longer_string<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

struct TextBuffer<'a> {
    content: &'a str,
}

impl<'a> TextBuffer<'a> {
    fn new(content: &'a str) -> Self {
        TextBuffer { content }
    }

    /// Fix: Give `default` the same lifetime 'a as the struct's content,
    /// so the return value can be either self.content or default.
    fn content_or_default(&self, default: &'a str) -> &'a str {
        if self.content.is_empty() {
            default
        } else {
            self.content
        }
    }

    fn first_n_chars(&self, n: usize) -> &str {
        if n >= self.content.len() {
            self.content
        } else {
            &self.content[..n]
        }
    }
}

/// Fix: Both parameters share lifetime 'a so either can be returned.
fn pick_greeting<'a>(formal: &'a str, casual: &'a str, use_formal: bool) -> &'a str {
    if use_formal {
        formal
    } else {
        casual
    }
}

/// Fix: Return &str with proper lifetime instead of allocating a String.
/// Lifetime elision handles this automatically -- single input lifetime
/// is assigned to the output.
fn find_shortest<'a>(items: &[&'a str]) -> &'a str {
    let mut shortest = items[0];
    for &item in &items[1..] {
        if item.len() < shortest.len() {
            shortest = item;
        }
    }
    shortest
}

/// Fix: Return &str instead of String. Lifetime elision means the
/// return lifetime matches the input lifetime automatically.
fn safe_prefix(text: &str, len: usize) -> &str {
    if len >= text.len() {
        text
    } else {
        &text[..len]
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longer_string_first_longer() {
        let result = longer_string("hello world", "hi");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_longer_string_second_longer() {
        let result = longer_string("hi", "hello world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_longer_string_equal() {
        let result = longer_string("abc", "xyz");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_text_buffer_non_empty() {
        let buf = TextBuffer::new("hello");
        assert_eq!(buf.content_or_default("default"), "hello");
    }

    #[test]
    fn test_text_buffer_empty_returns_default() {
        let buf = TextBuffer::new("");
        assert_eq!(buf.content_or_default("fallback"), "fallback");
    }

    #[test]
    fn test_text_buffer_first_n() {
        let buf = TextBuffer::new("hello world");
        assert_eq!(buf.first_n_chars(5), "hello");
    }

    #[test]
    fn test_text_buffer_first_n_exceeds() {
        let buf = TextBuffer::new("hi");
        assert_eq!(buf.first_n_chars(100), "hi");
    }

    #[test]
    fn test_pick_formal_greeting() {
        let result = pick_greeting("Good morning", "hey", true);
        assert_eq!(result, "Good morning");
    }

    #[test]
    fn test_pick_casual_greeting() {
        let result = pick_greeting("Good morning", "hey", false);
        assert_eq!(result, "hey");
    }

    #[test]
    fn test_find_shortest() {
        let result = find_shortest(&["medium", "hi", "longer word"]);
        assert_eq!(result, "hi");
    }

    #[test]
    fn test_find_shortest_single() {
        let result = find_shortest(&["only"]);
        assert_eq!(result, "only");
    }

    #[test]
    fn test_safe_prefix_short() {
        let result = safe_prefix("hello world", 5);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_safe_prefix_exceeds() {
        let result = safe_prefix("hi", 100);
        assert_eq!(result, "hi");
    }
}
