// ========================================
// Exercise: Boss Battle - Parallel Word Counter (BossBattle)
// ========================================
// Difficulty: Hard
// Module: 15 - Concurrency
//
// CONCEPT:
// This boss battle combines threads, Arc, Mutex, and HashMap to build
// a parallel word frequency counter. The approach:
//   1. Split input text into chunks (one per thread)
//   2. Each thread counts word frequencies in its chunk
//   3. Threads write results into a shared Arc<Mutex<HashMap>>
//   4. After all threads finish, return the merged word counts
//
// This exercises multiple concurrency concepts:
//   - Spawning multiple threads
//   - Shared mutable state with Arc<Mutex<T>>
//   - String processing and HashMap manipulation
//   - Joining threads and collecting results
//
// YOUR TASK:
// Implement all functions marked with todo!(). The tests at the bottom
// will verify your implementation.
// ========================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// Counts word frequencies in a single string. Words are split on whitespace
/// and converted to lowercase for case-insensitive counting.
///
/// Example: "Hello hello World" -> {"hello": 2, "world": 1}
fn count_words(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Merges word counts from `source` into `target`. If a word exists in both,
/// the counts are added together.
///
/// Example: target={"a": 2}, source={"a": 3, "b": 1} -> target={"a": 5, "b": 1}
fn merge_counts(target: &mut HashMap<String, usize>, source: &HashMap<String, usize>) {
    todo!()
}

/// Splits text into `num_chunks` roughly equal pieces, splitting on whitespace
/// boundaries (never splitting a word in half).
///
/// Returns a Vec of owned Strings, each containing a portion of the words.
/// All words from the input should appear in exactly one chunk.
///
/// Example: split_into_chunks("a b c d e f", 3) -> ["a b", "c d", "e f"]
fn split_into_chunks(text: &str, num_chunks: usize) -> Vec<String> {
    todo!()
}

/// The main parallel word counter. Splits the text into `num_threads` chunks
/// and counts words in parallel using threads and Arc<Mutex<HashMap>>.
///
/// Steps:
/// 1. Split text into chunks using split_into_chunks
/// 2. Create an Arc<Mutex<HashMap<String, usize>>> for shared results
/// 3. Spawn a thread for each chunk that:
///    a. Counts words in its chunk (using count_words)
///    b. Merges results into the shared HashMap (using merge_counts)
/// 4. Join all threads
/// 5. Return the final HashMap
fn parallel_word_count(text: &str, num_threads: usize) -> HashMap<String, usize> {
    todo!()
}

/// Returns the top N most frequent words from a word count map,
/// sorted by frequency (highest first). Ties are broken alphabetically.
///
/// Returns a Vec of (word, count) tuples.
fn top_n_words(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // --- count_words tests ---

    #[test]
    fn test_count_words_basic() {
        let counts = count_words("hello world hello");
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
        assert_eq!(counts.len(), 2);
    }

    #[test]
    fn test_count_words_case_insensitive() {
        let counts = count_words("Hello HELLO hello");
        assert_eq!(counts.get("hello"), Some(&3));
        assert_eq!(counts.len(), 1);
    }

    #[test]
    fn test_count_words_empty() {
        let counts = count_words("");
        assert!(counts.is_empty());
    }

    #[test]
    fn test_count_words_whitespace_only() {
        let counts = count_words("   \t  \n  ");
        assert!(counts.is_empty());
    }

    // --- merge_counts tests ---

    #[test]
    fn test_merge_counts_basic() {
        let mut target: HashMap<String, usize> = HashMap::new();
        target.insert("a".to_string(), 2);

        let mut source: HashMap<String, usize> = HashMap::new();
        source.insert("a".to_string(), 3);
        source.insert("b".to_string(), 1);

        merge_counts(&mut target, &source);

        assert_eq!(target.get("a"), Some(&5));
        assert_eq!(target.get("b"), Some(&1));
    }

    #[test]
    fn test_merge_counts_empty_source() {
        let mut target: HashMap<String, usize> = HashMap::new();
        target.insert("x".to_string(), 5);

        merge_counts(&mut target, &HashMap::new());

        assert_eq!(target.get("x"), Some(&5));
        assert_eq!(target.len(), 1);
    }

    // --- split_into_chunks tests ---

    #[test]
    fn test_split_into_chunks_even() {
        let chunks = split_into_chunks("a b c d e f", 3);
        assert_eq!(chunks.len(), 3);
        // All words should be present across all chunks
        let all_words: Vec<String> = chunks
            .iter()
            .flat_map(|c| c.split_whitespace().map(String::from))
            .collect();
        assert_eq!(all_words.len(), 6);
        assert!(all_words.contains(&"a".to_string()));
        assert!(all_words.contains(&"f".to_string()));
    }

    #[test]
    fn test_split_into_chunks_single() {
        let chunks = split_into_chunks("hello world", 1);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].split_whitespace().count(), 2);
    }

    #[test]
    fn test_split_into_chunks_more_chunks_than_words() {
        let chunks = split_into_chunks("hello world", 5);
        // Should handle gracefully -- some chunks might be empty
        let total_words: usize = chunks.iter().map(|c| c.split_whitespace().count()).sum();
        assert_eq!(total_words, 2);
    }

    // --- parallel_word_count tests ---

    #[test]
    fn test_parallel_word_count_basic() {
        let text = "hello world hello rust world hello";
        let counts = parallel_word_count(text, 2);

        assert_eq!(counts.get("hello"), Some(&3));
        assert_eq!(counts.get("world"), Some(&2));
        assert_eq!(counts.get("rust"), Some(&1));
        assert_eq!(counts.len(), 3);
    }

    #[test]
    fn test_parallel_word_count_case_insensitive() {
        let text = "Rust rust RUST";
        let counts = parallel_word_count(text, 2);
        assert_eq!(counts.get("rust"), Some(&3));
    }

    #[test]
    fn test_parallel_word_count_single_thread() {
        let text = "one two three one two one";
        let counts = parallel_word_count(text, 1);

        assert_eq!(counts.get("one"), Some(&3));
        assert_eq!(counts.get("two"), Some(&2));
        assert_eq!(counts.get("three"), Some(&1));
    }

    #[test]
    fn test_parallel_word_count_many_threads() {
        let text = "a b c d e f g h i j";
        let counts = parallel_word_count(text, 4);

        assert_eq!(counts.len(), 10);
        for word in ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"] {
            assert_eq!(counts.get(word), Some(&1));
        }
    }

    #[test]
    fn test_parallel_word_count_empty() {
        let counts = parallel_word_count("", 3);
        assert!(counts.is_empty());
    }

    #[test]
    fn test_parallel_word_count_repeated_text() {
        // Build a larger text with known frequencies
        let text = "the quick brown fox jumps over the lazy dog the fox the";
        let counts = parallel_word_count(text, 3);

        assert_eq!(counts.get("the"), Some(&4));
        assert_eq!(counts.get("fox"), Some(&2));
        assert_eq!(counts.get("quick"), Some(&1));
    }

    // --- top_n_words tests ---

    #[test]
    fn test_top_n_words() {
        let mut counts = HashMap::new();
        counts.insert("hello".to_string(), 5);
        counts.insert("world".to_string(), 3);
        counts.insert("rust".to_string(), 7);
        counts.insert("code".to_string(), 1);

        let top = top_n_words(&counts, 2);
        assert_eq!(top, vec![
            ("rust".to_string(), 7),
            ("hello".to_string(), 5),
        ]);
    }

    #[test]
    fn test_top_n_words_with_ties() {
        let mut counts = HashMap::new();
        counts.insert("alpha".to_string(), 3);
        counts.insert("beta".to_string(), 3);
        counts.insert("gamma".to_string(), 1);

        let top = top_n_words(&counts, 2);
        // Same count -> alphabetical order
        assert_eq!(top, vec![
            ("alpha".to_string(), 3),
            ("beta".to_string(), 3),
        ]);
    }

    #[test]
    fn test_top_n_words_more_than_available() {
        let mut counts = HashMap::new();
        counts.insert("only".to_string(), 1);

        let top = top_n_words(&counts, 5);
        assert_eq!(top.len(), 1);
        assert_eq!(top[0], ("only".to_string(), 1));
    }

    // --- integration test ---

    #[test]
    fn test_full_pipeline() {
        let text = "to be or not to be that is the question \
                    to be or not to be whether tis nobler \
                    to suffer the slings and arrows";

        let counts = parallel_word_count(text, 3);
        let top = top_n_words(&counts, 3);

        assert_eq!(top[0].0, "to");
        assert_eq!(top[0].1, 5);
        assert_eq!(top[1].0, "be");
        assert_eq!(top[1].1, 4);
    }
}
