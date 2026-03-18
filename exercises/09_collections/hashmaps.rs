// Exercise: HashMaps
// Type: implement_from_scratch
// Difficulty: intermediate
//
// HashMap<K, V> stores key-value pairs, like Python's dict or
// JavaScript's Map. Keys must implement Hash and Eq.
//
// Useful methods:
//   - insert(key, value) — insert or overwrite
//   - get(&key) -> Option<&V> — look up a value
//   - entry(key).or_insert(default) — insert only if key doesn't exist
//   - contains_key(&key) -> bool
//
// Implement the functions below to make the tests pass.

use std::collections::HashMap;

/// Count how many times each word appears in the text.
/// Words are split by whitespace. Convert to lowercase for counting.
/// Example: "the cat and the dog" => {"the": 2, "cat": 1, "and": 1, "dog": 1}
fn word_frequency(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Given a list of scores, group them by the first letter of the name.
/// Example: [("Alice", 90), ("Bob", 85), ("Anna", 92)]
///   => {'A': [("Alice", 90), ("Anna", 92)], 'B': [("Bob", 85)]}
fn group_by_initial(entries: &[(&str, i32)]) -> HashMap<char, Vec<(String, i32)>> {
    todo!()
}

/// Merge two hashmaps. If a key exists in both, sum the values.
fn merge_maps(a: &HashMap<String, i32>, b: &HashMap<String, i32>) -> HashMap<String, i32> {
    todo!()
}

/// Find the most frequent element in a slice.
/// Returns None if the slice is empty.
/// If there's a tie, return any one of the most frequent elements.
fn most_frequent<T: std::hash::Hash + Eq + Clone>(items: &[T]) -> Option<T> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_frequency() {
        let freq = word_frequency("the cat and the dog");
        assert_eq!(freq.get("the"), Some(&2));
        assert_eq!(freq.get("cat"), Some(&1));
        assert_eq!(freq.get("and"), Some(&1));
        assert_eq!(freq.get("dog"), Some(&1));
    }

    #[test]
    fn test_word_frequency_case_insensitive() {
        let freq = word_frequency("Hello hello HELLO");
        assert_eq!(freq.get("hello"), Some(&3));
        assert_eq!(freq.len(), 1);
    }

    #[test]
    fn test_word_frequency_empty() {
        let freq = word_frequency("");
        assert!(freq.is_empty());
    }

    #[test]
    fn test_group_by_initial() {
        let entries = vec![("Alice", 90), ("Bob", 85), ("Anna", 92)];
        let grouped = group_by_initial(&entries);

        let a_group = grouped.get(&'A').unwrap();
        assert_eq!(a_group.len(), 2);
        assert!(a_group.iter().any(|(name, _)| name == "Alice"));
        assert!(a_group.iter().any(|(name, _)| name == "Anna"));

        let b_group = grouped.get(&'B').unwrap();
        assert_eq!(b_group.len(), 1);
        assert_eq!(b_group[0].0, "Bob");
    }

    #[test]
    fn test_group_by_initial_empty() {
        let entries: Vec<(&str, i32)> = vec![];
        let grouped = group_by_initial(&entries);
        assert!(grouped.is_empty());
    }

    #[test]
    fn test_merge_maps() {
        let mut a = HashMap::new();
        a.insert("x".to_string(), 1);
        a.insert("y".to_string(), 2);

        let mut b = HashMap::new();
        b.insert("y".to_string(), 3);
        b.insert("z".to_string(), 4);

        let merged = merge_maps(&a, &b);
        assert_eq!(merged.get("x"), Some(&1));
        assert_eq!(merged.get("y"), Some(&5)); // 2 + 3
        assert_eq!(merged.get("z"), Some(&4));
    }

    #[test]
    fn test_merge_maps_no_overlap() {
        let mut a = HashMap::new();
        a.insert("a".to_string(), 10);

        let mut b = HashMap::new();
        b.insert("b".to_string(), 20);

        let merged = merge_maps(&a, &b);
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn test_most_frequent() {
        assert_eq!(most_frequent(&[1, 2, 3, 2, 2, 1]), Some(2));
    }

    #[test]
    fn test_most_frequent_strings() {
        let words = vec!["apple", "banana", "apple", "cherry", "apple"];
        assert_eq!(most_frequent(&words), Some("apple"));
    }

    #[test]
    fn test_most_frequent_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(most_frequent(&empty), None);
    }

    #[test]
    fn test_most_frequent_single() {
        assert_eq!(most_frequent(&[42]), Some(42));
    }
}
