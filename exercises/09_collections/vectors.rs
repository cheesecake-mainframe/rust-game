// Exercise: Vectors
// Type: debug_logic_bug
// Difficulty: intermediate
//
// Vec<T> is Rust's growable array type — like Python's list or
// JavaScript's Array. Elements are stored contiguously in memory.
//
// The code below compiles but has logic bugs. Fix all bugs so
// the tests pass.

/// Return the sum of all elements in the vector.
fn sum_vec(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;
    // BUG: off-by-one — skips the first element
    for i in 1..numbers.len() {
        total += numbers[i];
    }
    total
}

/// Return a new vector with only the even numbers, in order.
fn filter_even(numbers: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for &n in numbers {
        // BUG: wrong condition — this keeps odd numbers
        if n % 2 != 0 {
            result.push(n);
        }
    }
    result
}

/// Return the second-largest element, or None if fewer than 2 elements.
fn second_largest(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.len() < 2 {
        return None;
    }
    let mut sorted = numbers.clone();
    sorted.sort();
    // BUG: returns the second element (second smallest), not second largest
    Some(sorted[1])
}

/// Remove all duplicates from the vector, keeping first occurrence order.
fn remove_duplicates(numbers: &Vec<i32>) -> Vec<i32> {
    let mut seen = Vec::new();
    let mut result = Vec::new();
    for &n in numbers {
        // BUG: pushes to seen but not to result, and vice versa
        if !seen.contains(&n) {
            seen.push(n);
            // Missing: result.push(n)
        } else {
            result.push(n);
        }
    }
    result
}

/// Rotate a vector left by `n` positions.
/// e.g., rotate_left([1,2,3,4,5], 2) => [3,4,5,1,2]
fn rotate_left(numbers: &Vec<i32>, n: usize) -> Vec<i32> {
    if numbers.is_empty() {
        return vec![];
    }
    // BUG: doesn't handle n larger than vec length
    let mut result = Vec::new();
    for i in n..numbers.len() {
        result.push(numbers[i]);
    }
    for i in 0..n {
        result.push(numbers[i]);
    }
    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_vec() {
        assert_eq!(sum_vec(&vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_sum_vec_single() {
        assert_eq!(sum_vec(&vec![42]), 42);
    }

    #[test]
    fn test_sum_vec_empty() {
        assert_eq!(sum_vec(&vec![]), 0);
    }

    #[test]
    fn test_filter_even() {
        assert_eq!(filter_even(&vec![1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_even_none() {
        assert_eq!(filter_even(&vec![1, 3, 5]), Vec::<i32>::new());
    }

    #[test]
    fn test_second_largest() {
        assert_eq!(second_largest(&vec![3, 1, 4, 1, 5]), Some(4));
    }

    #[test]
    fn test_second_largest_with_duplicates() {
        assert_eq!(second_largest(&vec![5, 5, 3]), Some(5));
    }

    #[test]
    fn test_second_largest_too_short() {
        assert_eq!(second_largest(&vec![42]), None);
    }

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(remove_duplicates(&vec![1, 2, 3, 2, 1, 4]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_duplicates_no_dups() {
        assert_eq!(remove_duplicates(&vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_rotate_left() {
        assert_eq!(rotate_left(&vec![1, 2, 3, 4, 5], 2), vec![3, 4, 5, 1, 2]);
    }

    #[test]
    fn test_rotate_left_full() {
        assert_eq!(rotate_left(&vec![1, 2, 3], 3), vec![1, 2, 3]);
    }

    #[test]
    fn test_rotate_left_more_than_len() {
        assert_eq!(rotate_left(&vec![1, 2, 3], 5), vec![3, 1, 2]);
    }

    #[test]
    fn test_rotate_left_empty() {
        assert_eq!(rotate_left(&vec![], 3), vec![]);
    }
}
