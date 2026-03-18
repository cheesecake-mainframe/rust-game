// Exercise: Type Annotations
// Type: debug_logic_bug
// Difficulty: beginner
//
// This code compiles but produces the wrong result.
// The temperature conversion is incorrect because of a type issue.
// Fix the bug so the tests pass.

fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    celsius * 9 / 5 + 32
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    // BUG: Integer division truncates! 5/9 == 0 in integer math.
    (fahrenheit - 32) * 5 / 9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boiling_point() {
        assert_eq!(celsius_to_fahrenheit(100), 212);
    }

    #[test]
    fn test_freezing_point() {
        assert_eq!(celsius_to_fahrenheit(0), 32);
    }

    #[test]
    fn test_body_temp() {
        assert_eq!(celsius_to_fahrenheit(37), 98);
    }

    #[test]
    fn test_reverse_boiling() {
        assert_eq!(fahrenheit_to_celsius(212), 100);
    }

    #[test]
    fn test_reverse_freezing() {
        assert_eq!(fahrenheit_to_celsius(32), 0);
    }
}
