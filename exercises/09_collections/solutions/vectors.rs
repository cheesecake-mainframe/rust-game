fn sum_vec(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;
    for i in 0..numbers.len() {
        total += numbers[i];
    }
    total
}

fn filter_even(numbers: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for &n in numbers {
        if n % 2 == 0 {
            result.push(n);
        }
    }
    result
}

fn second_largest(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.len() < 2 {
        return None;
    }
    let mut sorted = numbers.clone();
    sorted.sort();
    Some(sorted[sorted.len() - 2])
}

fn remove_duplicates(numbers: &Vec<i32>) -> Vec<i32> {
    let mut seen = Vec::new();
    let mut result = Vec::new();
    for &n in numbers {
        if !seen.contains(&n) {
            seen.push(n);
            result.push(n);
        }
    }
    result
}

fn rotate_left(numbers: &Vec<i32>, n: usize) -> Vec<i32> {
    if numbers.is_empty() {
        return vec![];
    }
    let n = n % numbers.len();
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
