struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> TextAnalyzer<'a> {
        TextAnalyzer { text }
    }

    fn word_count(&self) -> usize {
        if self.text.is_empty() {
            0
        } else {
            self.text.split_whitespace().count()
        }
    }

    fn char_count(&self) -> usize {
        self.text.chars().filter(|c| !c.is_whitespace()).count()
    }

    fn longest_word(&self) -> &str {
        self.text
            .split_whitespace()
            .max_by_key(|w| w.len())
            .unwrap_or("")
    }

    fn contains_word(&self, word: &str) -> bool {
        self.text.split_whitespace().any(|w| w == word)
    }

    fn nth_word(&self, n: usize) -> Option<&str> {
        self.text.split_whitespace().nth(n)
    }
}

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
        assert_eq!(analyzer.char_count(), 10);
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
        assert!(!analyzer.contains_word("quic"));
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
        assert_eq!(text, "borrow me");
    }
}
