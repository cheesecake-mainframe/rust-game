// ========================================
// Solution: Boss Battle - Parallel Word Counter
// ========================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// Counts word frequencies in a single string. Words are split on whitespace
/// and converted to lowercase for case-insensitive counting.
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

/// Merges word counts from `source` into `target`.
fn merge_counts(target: &mut HashMap<String, usize>, source: &HashMap<String, usize>) {
    for (word, &count) in source {
        *target.entry(word.clone()).or_insert(0) += count;
    }
}

/// Splits text into `num_chunks` roughly equal pieces on word boundaries.
fn split_into_chunks(text: &str, num_chunks: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() || num_chunks == 0 {
        return vec![String::new()];
    }

    let chunk_size = (words.len() + num_chunks - 1) / num_chunks;
    let mut chunks = Vec::new();

    for chunk_words in words.chunks(chunk_size) {
        chunks.push(chunk_words.join(" "));
    }

    chunks
}

/// The main parallel word counter.
fn parallel_word_count(text: &str, num_threads: usize) -> HashMap<String, usize> {
    if text.trim().is_empty() {
        return HashMap::new();
    }

    let chunks = split_into_chunks(text, num_threads);
    let shared_counts = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for chunk in chunks {
        if chunk.is_empty() {
            continue;
        }
        let counts_clone = Arc::clone(&shared_counts);
        let handle = thread::spawn(move || {
            let local_counts = count_words(&chunk);
            let mut shared = counts_clone.lock().unwrap();
            merge_counts(&mut shared, &local_counts);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Extract the HashMap from the Arc<Mutex<...>>
    let lock = shared_counts.lock().unwrap();
    lock.clone()
}

/// Returns the top N most frequent words, sorted by frequency descending,
/// then alphabetically for ties.
fn top_n_words(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut entries: Vec<(String, usize)> = counts
        .iter()
        .map(|(word, &count)| (word.clone(), count))
        .collect();

    // Sort by count descending, then by word ascending for ties
    entries.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });

    entries.truncate(n);
    entries
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_split_into_chunks_even() {
        let chunks = split_into_chunks("a b c d e f", 3);
        assert_eq!(chunks.len(), 3);
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
        let total_words: usize = chunks.iter().map(|c| c.split_whitespace().count()).sum();
        assert_eq!(total_words, 2);
    }

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
        let text = "the quick brown fox jumps over the lazy dog the fox the";
        let counts = parallel_word_count(text, 3);

        assert_eq!(counts.get("the"), Some(&4));
        assert_eq!(counts.get("fox"), Some(&2));
        assert_eq!(counts.get("quick"), Some(&1));
    }

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
