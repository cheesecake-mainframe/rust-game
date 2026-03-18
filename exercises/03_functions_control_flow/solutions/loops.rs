fn countdown(start: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut n = start;
    while n >= 1 {
        result.push(n);
        n -= 1;
    }
    result
}

fn sum_to_n(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn first_power_of_2_above(threshold: u32) -> u32 {
    let mut power = 1;
    loop {
        if power > threshold {
            return power;
        }
        power *= 2;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countdown() {
        assert_eq!(countdown(5), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_countdown_one() {
        assert_eq!(countdown(1), vec![1]);
    }

    #[test]
    fn test_sum_to_n() {
        assert_eq!(sum_to_n(5), 15);
        assert_eq!(sum_to_n(10), 55);
    }

    #[test]
    fn test_sum_to_one() {
        assert_eq!(sum_to_n(1), 1);
    }

    #[test]
    fn test_first_power_above() {
        assert_eq!(first_power_of_2_above(10), 16);
        assert_eq!(first_power_of_2_above(1), 2);
        assert_eq!(first_power_of_2_above(100), 128);
    }
}
