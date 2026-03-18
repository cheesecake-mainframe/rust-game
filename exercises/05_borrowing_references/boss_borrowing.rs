// Exercise: The Borrow Sanctum (Boss Battle)
// Type: boss_battle
// Difficulty: intermediate
//
// === BOSS BATTLE ===
// Build a TextAnalyzer that borrows text and provides analysis
// without ever taking ownership. Every method returns borrowed data
// or computed values.

struct TextAnalyzer<'a> {
    text: &'a str,
}

// TODO: Implement these methods for TextAnalyzer:
//
// - new(text: &str) -> TextAnalyzer
//       Create a new analyzer that borrows the text.
//
// - word_count(&self) -> usize
//       Count the number of words (split by whitespace).
//
// - char_count(&self) -> usize
//       Count total characters (excluding whitespace).
//
// - longest_word(&self) -> &str
//       Return the longest word. If empty text, return "".
//       If tie, return the first longest.
//
// - contains_word(&self, word: &str) -> bool
//       Check if the text contains the given word (case-sensitive, whole word).
//
// - nth_word(&self, n: usize) -> Option<&str>
//       Return the nth word (0-indexed). None if out of bounds.

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let text = String::from("the quick brown fox jumps");
        let analyzer = TextAnalyzer::new(&text);
        assert_eq!(analyzer.word_count(), 5);
    }

    #[test]
    fn test_word_count_empty() {
        let analyzer = TextAnalyzer::new("");
        assert_eq!(analyzer.word_count(), 0);
    }

    #[test]
    fn test_char_count() {
        let analyzer = TextAnalyzer::new("hello world");
        assert_eq!(analyzer.char_count(), 10); // excludes the space
    }

    #[test]
    fn test_longest_word() {
        let text = String::from("I love programming");
        let analyzer = TextAnalyzer::new(&text);
        assert_eq!(analyzer.longest_word(), "programming");
    }

    #[test]
    fn test_longest_word_empty() {
        let analyzer = TextAnalyzer::new("");
        assert_eq!(analyzer.longest_word(), "");
    }

    #[test]
    fn test_contains_word() {
        let analyzer = TextAnalyzer::new("the quick brown fox");
        assert!(analyzer.contains_word("quick"));
        assert!(!analyzer.contains_word("slow"));
        assert!(!analyzer.contains_word("quic")); // partial match doesn't count
    }

    #[test]
    fn test_nth_word() {
        let analyzer = TextAnalyzer::new("one two three");
        assert_eq!(analyzer.nth_word(0), Some("one"));
        assert_eq!(analyzer.nth_word(2), Some("three"));
        assert_eq!(analyzer.nth_word(5), None);
    }

    #[test]
    fn test_text_not_consumed() {
        let text = String::from("borrow me");
        let analyzer = TextAnalyzer::new(&text);
        let _ = analyzer.word_count();
        // text is still usable — analyzer only borrows it
        assert_eq!(text, "borrow me");
    }
}
