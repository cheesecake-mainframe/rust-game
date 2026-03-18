// ========================================
// Exercise: Lifetime Elision (DebugLogicBug)
// ========================================
// Difficulty: Intermediate
// Module: 12 - Lifetimes
//
// CONCEPT:
// Rust has "lifetime elision rules" that let you omit lifetime annotations
// in common cases. The three rules are:
//   1. Each reference parameter gets its own lifetime parameter.
//   2. If there's exactly one input lifetime, it's assigned to all output
//      lifetimes.
//   3. If one of the parameters is &self or &mut self, its lifetime is
//      assigned to all output lifetimes.
//
// Sometimes elision works perfectly. Other times, you MUST be explicit
// because the compiler can't infer the correct relationships.
//
// YOUR TASK:
// This code COMPILES but has logic bugs caused by incorrect lifetime
// thinking. The tests will fail. Fix the functions (both signatures
// and logic where needed) so all tests pass.
//
// NOTE: The code compiles as-is! But the tests fail because the functions
// produce wrong results due to the developer giving up on proper lifetime
// management and taking shortcuts.
// ========================================

/// Returns the longer of two strings.
/// BUG: The developer gave _b a separate lifetime so they couldn't return it.
/// Then they "simplified" the logic to always return `a` since that's the only
/// thing that has lifetime 'a. Fix it to actually return the longer string.
fn longer_string<'a>(a: &'a str, _b: &str) -> &'a str {
    // Developer thought: "The return must have lifetime 'a, so I can only return a"
    // This is the bug -- both parameters should share lifetime 'a so either can be returned.
    a
}

/// A text buffer that holds a reference to some text.
struct TextBuffer<'a> {
    content: &'a str,
}

impl<'a> TextBuffer<'a> {
    fn new(content: &'a str) -> Self {
        TextBuffer { content }
    }

    /// Returns the content or a default if content is empty.
    /// BUG: This always returns self.content, ignoring the empty case.
    /// The developer was confused about which lifetime to return and
    /// gave up on the fallback logic. Fix it so that when content is empty,
    /// the default string is returned instead.
    fn content_or_default(&self, _default: &str) -> &str {
        // Should return `default` when content is empty, but the developer
        // couldn't figure out the lifetimes so they just always return content.
        self.content
    }

    /// Returns the first n characters of the content.
    fn first_n_chars(&self, n: usize) -> &str {
        if n >= self.content.len() {
            self.content
        } else {
            &self.content[..n]
        }
    }
}

/// Picks the best greeting based on formality.
/// BUG: The developer hardcoded a return to avoid lifetime issues.
/// Both `formal` and `casual` have different lifetimes, so the developer
/// could only return `formal`. Fix the lifetimes and logic so it returns
/// the correct greeting.
fn pick_greeting<'a>(formal: &'a str, _casual: &str, use_formal: bool) -> &'a str {
    // Should return casual when use_formal is false, but dev couldn't
    // figure out the lifetimes with two different reference sources.
    if use_formal {
        formal
    } else {
        formal // BUG: should return casual
    }
}

/// Finds the shortest string in a slice.
/// BUG: The developer returns a cloned String instead of a reference because
/// they couldn't figure out the lifetime for the return value. The function
/// compiles but returns owned data unnecessarily, and the test expects a &str.
/// Fix to return a reference with the correct lifetime.
fn find_shortest(items: &[&str]) -> String {
    let mut shortest = items[0];
    for &item in &items[1..] {
        if item.len() < shortest.len() {
            shortest = item;
        }
    }
    // BUG: Returns an owned String instead of a reference.
    // The developer couldn't figure out the lifetime so they cloned.
    shortest.to_string()
}

/// Extracts a prefix from `text` of length `len`, or all of `text` if shorter.
/// BUG: Returns an owned String because the developer didn't realize
/// simple lifetime elision handles this case automatically.
fn safe_prefix(text: &str, len: usize) -> String {
    if len >= text.len() {
        text.to_string() // BUG: unnecessary allocation
    } else {
        text[..len].to_string() // BUG: unnecessary allocation
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
        assert_eq!(result, "hello world"); // FAILS: currently returns "hi"
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
        assert_eq!(buf.content_or_default("fallback"), "fallback"); // FAILS: returns ""
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
        assert_eq!(result, "hey"); // FAILS: currently returns "Good morning"
    }

    #[test]
    fn test_find_shortest() {
        let result = find_shortest(&["medium", "hi", "longer word"]);
        assert_eq!(result, "hi");
        // The test passes on value equality, but the solution should return &str
        // to avoid unnecessary allocation. The exercise returns String which works
        // but is wasteful. Fix find_shortest to return &str.
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
        // Works but returns owned String unnecessarily.
        // Fix to return &str.
    }

    #[test]
    fn test_safe_prefix_exceeds() {
        let result = safe_prefix("hi", 100);
        assert_eq!(result, "hi");
    }
}
