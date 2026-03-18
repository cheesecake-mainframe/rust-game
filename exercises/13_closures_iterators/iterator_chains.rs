// ========================================
// Exercise: Iterator Chains (ImplementFromScratch)
// ========================================
// Difficulty: Intermediate
// Module: 13 - Closures & Iterators
//
// CONCEPT:
// Rust iterators are lazy and composable. You can chain adaptor methods
// to build powerful data pipelines:
//   - map(f): transform each element
//   - filter(p): keep elements matching a predicate
//   - fold(init, f): accumulate into a single value
//   - collect(): consume the iterator into a collection
//   - enumerate(): add index to each element
//   - zip(): combine two iterators element-wise
//   - take(n) / skip(n): limit or skip elements
//   - flat_map(f): map + flatten
//   - chain(): concatenate two iterators
//
// YOUR TASK:
// Implement each function using iterator chains. Do NOT use for loops
// or manual indexing -- use iterator adaptors only!
// ========================================

/// Returns the sum of squares of all even numbers in the input.
/// Example: [1, 2, 3, 4, 5] -> 2*2 + 4*4 = 4 + 16 = 20
fn sum_of_even_squares(numbers: &[i32]) -> i32 {
    todo!()
}

/// Returns a vector of strings where each string is "INDEX: VALUE"
/// for each element in the input.
/// Example: ["a", "b", "c"] -> ["0: a", "1: b", "2: c"]
fn indexed_labels(items: &[&str]) -> Vec<String> {
    todo!()
}

/// Counts how many words in the input have more than `min_length` characters.
/// Example: (["hi", "hello", "hey", "wonderful"], 3) -> 2 ("hello", "wonderful")
fn count_long_words(words: &[&str], min_length: usize) -> usize {
    todo!()
}

/// Returns a single string joining all words that start with an uppercase letter,
/// separated by ", ".
/// Example: ["hello", "World", "foo", "Bar"] -> "World, Bar"
fn join_capitalized(words: &[&str]) -> String {
    todo!()
}

/// Given two slices of the same length, returns a vector of tuples pairing
/// corresponding elements.
/// Example: ([1, 2, 3], ["a", "b", "c"]) -> [(1, "a"), (2, "b"), (3, "c")]
fn pair_up(nums: &[i32], labels: &[&str]) -> Vec<(i32, String)> {
    todo!()
}

/// Flattens a slice of string slices by splitting each on whitespace,
/// then collects all words into a single vector.
/// Example: ["hello world", "foo bar baz"] -> ["hello", "world", "foo", "bar", "baz"]
fn flatten_words(sentences: &[&str]) -> Vec<String> {
    todo!()
}

/// Returns the product of the first `n` positive integers in the list.
/// Skip non-positive numbers, then take `n` positive ones and multiply.
/// Example: ([-1, 2, 0, 3, 5, 7], 3) -> 2 * 3 * 5 = 30
fn product_of_first_n_positive(numbers: &[i32], n: usize) -> i32 {
    todo!()
}

/// Returns a string containing only the unique characters from the input
/// (in order of first appearance), all lowercased.
/// Example: "Hello World" -> "helo wrd"
fn unique_chars_lower(input: &str) -> String {
    todo!()
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
