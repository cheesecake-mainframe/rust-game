// ========================================
// Solution: Python to Rust
// ========================================

fn double_all(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|&x| x * 2).collect()
}

fn evens_only(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|&&x| x % 2 == 0).copied().collect()
}

fn word_lengths(words: &[&str]) -> Vec<(String, usize)> {
    words
        .iter()
        .map(|&w| (String::from(w), w.len()))
        .collect()
}

fn flatten(nested: &[Vec<i32>]) -> Vec<i32> {
    nested.iter().flat_map(|v| v.iter()).copied().collect()
}

fn first_n_squares(n: usize) -> Vec<usize> {
    (1..=n).map(|i| i * i).collect()
}

fn sum_of_positives(nums: &[i32]) -> i32 {
    nums.iter().filter(|&&x| x > 0).sum()
}

fn any_negative(nums: &[i32]) -> bool {
    nums.iter().any(|&x| x < 0)
}

fn all_positive(nums: &[i32]) -> bool {
    nums.iter().all(|&x| x > 0)
}

fn enumerate_upper(items: &[&str]) -> Vec<(usize, String)> {
    items
        .iter()
        .enumerate()
        .map(|(i, &item)| (i, item.to_uppercase()))
        .collect()
}

fn running_total(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
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
        assert!(all_positive(&[]));
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
