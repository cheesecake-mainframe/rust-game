// ========================================
// Exercise: Python to Rust (CodeTransformation)
// ========================================
// Difficulty: Intermediate
// Module: 13 - Closures & Iterators
//
// CONCEPT:
// Many Python idioms using list comprehensions and built-in functions
// translate directly to Rust iterator chains. This exercise helps you
// think in iterators by converting familiar Python patterns.
//
// YOUR TASK:
// Each function shows the equivalent Python code in comments.
// Write the idiomatic Rust equivalent using iterator chains.
// Do NOT use for loops -- use map, filter, collect, etc.
// ========================================

/// Python:
///   def double_all(nums):
///       return [x * 2 for x in nums]
fn double_all(nums: &[i32]) -> Vec<i32> {
    todo!()
}

/// Python:
///   def evens_only(nums):
///       return [x for x in nums if x % 2 == 0]
fn evens_only(nums: &[i32]) -> Vec<i32> {
    todo!()
}

/// Python:
///   def word_lengths(words):
///       return {word: len(word) for word in words}
///
/// Return a Vec of tuples instead of a HashMap for simplicity.
fn word_lengths(words: &[&str]) -> Vec<(String, usize)> {
    todo!()
}

/// Python:
///   def flatten(nested):
///       return [item for sublist in nested for item in sublist]
fn flatten(nested: &[Vec<i32>]) -> Vec<i32> {
    todo!()
}

/// Python:
///   def first_n_squares(n):
///       return [i * i for i in range(1, n + 1)]
fn first_n_squares(n: usize) -> Vec<usize> {
    todo!()
}

/// Python:
///   def sum_of_positives(nums):
///       return sum(x for x in nums if x > 0)
fn sum_of_positives(nums: &[i32]) -> i32 {
    todo!()
}

/// Python:
///   def any_negative(nums):
///       return any(x < 0 for x in nums)
fn any_negative(nums: &[i32]) -> bool {
    todo!()
}

/// Python:
///   def all_positive(nums):
///       return all(x > 0 for x in nums)
fn all_positive(nums: &[i32]) -> bool {
    todo!()
}

/// Python:
///   def enumerate_items(items):
///       return [(i, item.upper()) for i, item in enumerate(items)]
fn enumerate_upper(items: &[&str]) -> Vec<(usize, String)> {
    todo!()
}

/// Python:
///   def running_total(nums):
///       totals = []
///       acc = 0
///       for n in nums:
///           acc += n
///           totals.append(acc)
///       return totals
///
/// Hint: Use .scan() or a creative approach with fold/map.
fn running_total(nums: &[i32]) -> Vec<i32> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_all() {
        assert_eq!(double_all(&[1, 2, 3]), vec![2, 4, 6]);
        assert_eq!(double_all(&[]), Vec::<i32>::new());
        assert_eq!(double_all(&[-1, 0, 1]), vec![-2, 0, 2]);
    }

    #[test]
    fn test_evens_only() {
        assert_eq!(evens_only(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(evens_only(&[1, 3, 5]), Vec::<i32>::new());
        assert_eq!(evens_only(&[0, 2]), vec![0, 2]);
    }

    #[test]
    fn test_word_lengths() {
        assert_eq!(
            word_lengths(&["hi", "hello", "hey"]),
            vec![
                (String::from("hi"), 2),
                (String::from("hello"), 5),
                (String::from("hey"), 3),
            ]
        );
        assert_eq!(word_lengths(&[]), Vec::<(String, usize)>::new());
    }

    #[test]
    fn test_flatten() {
        assert_eq!(
            flatten(&[vec![1, 2], vec![3, 4], vec![5]]),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(flatten(&[vec![], vec![1]]), vec![1]);
        assert_eq!(flatten(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_first_n_squares() {
        assert_eq!(first_n_squares(5), vec![1, 4, 9, 16, 25]);
        assert_eq!(first_n_squares(1), vec![1]);
        assert_eq!(first_n_squares(0), Vec::<usize>::new());
    }

    #[test]
    fn test_sum_of_positives() {
        assert_eq!(sum_of_positives(&[-1, 2, -3, 4, 5]), 11);
        assert_eq!(sum_of_positives(&[-1, -2, -3]), 0);
        assert_eq!(sum_of_positives(&[]), 0);
    }

    #[test]
    fn test_any_negative() {
        assert!(any_negative(&[1, 2, -3, 4]));
        assert!(!any_negative(&[1, 2, 3, 4]));
        assert!(!any_negative(&[]));
    }

    #[test]
    fn test_all_positive() {
        assert!(all_positive(&[1, 2, 3]));
        assert!(!all_positive(&[1, -2, 3]));
        assert!(all_positive(&[])); // vacuously true
    }

    #[test]
    fn test_enumerate_upper() {
        assert_eq!(
            enumerate_upper(&["hello", "world"]),
            vec![
                (0, String::from("HELLO")),
                (1, String::from("WORLD")),
            ]
        );
        assert_eq!(enumerate_upper(&[]), Vec::<(usize, String)>::new());
    }

    #[test]
    fn test_running_total() {
        assert_eq!(running_total(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(running_total(&[5]), vec![5]);
        assert_eq!(running_total(&[]), Vec::<i32>::new());
        assert_eq!(running_total(&[1, -1, 1, -1]), vec![1, 0, 1, 0]);
    }
}
