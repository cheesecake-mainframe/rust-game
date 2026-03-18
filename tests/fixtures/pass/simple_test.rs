fn double(x: i32) -> i32 {
    x * 2
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        assert_eq!(double(5), 10);
    }

    #[test]
    fn test_double_zero() {
        assert_eq!(double(0), 0);
    }
}
