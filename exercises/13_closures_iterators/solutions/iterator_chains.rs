// ========================================
// Solution: Iterator Chains
// ========================================

fn sum_of_even_squares(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .sum()
}

fn indexed_labels(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(i, &item)| format!("{}: {}", i, item))
        .collect()
}

fn count_long_words(words: &[&str], min_length: usize) -> usize {
    words
        .iter()
        .filter(|&&w| w.len() > min_length)
        .count()
}

fn join_capitalized(words: &[&str]) -> String {
    words
        .iter()
        .filter(|&&w| {
            w.chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
        })
        .copied()
        .collect::<Vec<&str>>()
        .join(", ")
}

fn pair_up(nums: &[i32], labels: &[&str]) -> Vec<(i32, String)> {
    nums.iter()
        .zip(labels.iter())
        .map(|(&n, &l)| (n, String::from(l)))
        .collect()
}

fn flatten_words(sentences: &[&str]) -> Vec<String> {
    sentences
        .iter()
        .flat_map(|s| s.split_whitespace())
        .map(String::from)
        .collect()
}

fn product_of_first_n_positive(numbers: &[i32], n: usize) -> i32 {
    numbers
        .iter()
        .filter(|&&x| x > 0)
        .take(n)
        .product()
}

fn unique_chars_lower(input: &str) -> String {
    let mut seen = Vec::new();
    input
        .chars()
        .map(|c| c.to_lowercase().next().unwrap())
        .filter(|c| {
            if seen.contains(c) {
                false
            } else {
                seen.push(*c);
                true
            }
        })
        .collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_even_squares() {
        assert_eq!(sum_of_even_squares(&[1, 2, 3, 4, 5]), 20);
        assert_eq!(sum_of_even_squares(&[2, 4, 6]), 4 + 16 + 36);
        assert_eq!(sum_of_even_squares(&[1, 3, 5]), 0);
        assert_eq!(sum_of_even_squares(&[]), 0);
    }

    #[test]
    fn test_indexed_labels() {
        assert_eq!(
            indexed_labels(&["a", "b", "c"]),
            vec!["0: a", "1: b", "2: c"]
        );
        assert_eq!(indexed_labels(&[]), Vec::<String>::new());
        assert_eq!(indexed_labels(&["only"]), vec!["0: only"]);
    }

    #[test]
    fn test_count_long_words() {
        assert_eq!(count_long_words(&["hi", "hello", "hey", "wonderful"], 3), 2);
        assert_eq!(count_long_words(&["a", "bb", "ccc"], 3), 0);
        assert_eq!(count_long_words(&["long", "words", "here"], 2), 3);
    }

    #[test]
    fn test_join_capitalized() {
        assert_eq!(join_capitalized(&["hello", "World", "foo", "Bar"]), "World, Bar");
        assert_eq!(join_capitalized(&["all", "lower"]), "");
        assert_eq!(join_capitalized(&["Alpha", "Beta"]), "Alpha, Beta");
    }

    #[test]
    fn test_pair_up() {
        assert_eq!(
            pair_up(&[1, 2, 3], &["a", "b", "c"]),
            vec![
                (1, String::from("a")),
                (2, String::from("b")),
                (3, String::from("c"))
            ]
        );
        assert_eq!(pair_up(&[], &[]), Vec::<(i32, String)>::new());
    }

    #[test]
    fn test_flatten_words() {
        assert_eq!(
            flatten_words(&["hello world", "foo bar baz"]),
            vec!["hello", "world", "foo", "bar", "baz"]
        );
        assert_eq!(flatten_words(&["single"]), vec!["single"]);
        assert_eq!(flatten_words(&[]), Vec::<String>::new());
    }

    #[test]
    fn test_product_of_first_n_positive() {
        assert_eq!(product_of_first_n_positive(&[-1, 2, 0, 3, 5, 7], 3), 30);
        assert_eq!(product_of_first_n_positive(&[1, 2, 3, 4, 5], 2), 2);
        assert_eq!(product_of_first_n_positive(&[-1, -2, 1], 1), 1);
    }

    #[test]
    fn test_unique_chars_lower() {
        assert_eq!(unique_chars_lower("Hello World"), "helo wrd");
        assert_eq!(unique_chars_lower("aabbcc"), "abc");
        assert_eq!(unique_chars_lower("Rust"), "rust");
        assert_eq!(unique_chars_lower(""), "");
    }
}
