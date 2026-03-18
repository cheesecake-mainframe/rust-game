use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        let lower = word.to_lowercase();
        *freq.entry(lower).or_insert(0) += 1;
    }
    freq
}

fn group_by_initial(entries: &[(&str, i32)]) -> HashMap<char, Vec<(String, i32)>> {
    let mut groups: HashMap<char, Vec<(String, i32)>> = HashMap::new();
    for &(name, score) in entries {
        if let Some(initial) = name.chars().next() {
            groups
                .entry(initial)
                .or_insert_with(Vec::new)
                .push((name.to_string(), score));
        }
    }
    groups
}

fn merge_maps(a: &HashMap<String, i32>, b: &HashMap<String, i32>) -> HashMap<String, i32> {
    let mut result = a.clone();
    for (key, value) in b {
        *result.entry(key.clone()).or_insert(0) += value;
    }
    result
}

fn most_frequent<T: std::hash::Hash + Eq + Clone>(items: &[T]) -> Option<T> {
    if items.is_empty() {
        return None;
    }
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(item, _)| item.clone())
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
        assert_eq!(merged.get("y"), Some(&5));
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
