fn day_name(day: u32) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "invalid",
    }
}

fn letter_grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_name() {
        assert_eq!(day_name(1), "Monday");
        assert_eq!(day_name(5), "Friday");
        assert_eq!(day_name(7), "Sunday");
        assert_eq!(day_name(0), "invalid");
        assert_eq!(day_name(8), "invalid");
    }

    #[test]
    fn test_letter_grade() {
        assert_eq!(letter_grade(95), 'A');
        assert_eq!(letter_grade(85), 'B');
        assert_eq!(letter_grade(75), 'C');
        assert_eq!(letter_grade(65), 'D');
        assert_eq!(letter_grade(55), 'F');
        assert_eq!(letter_grade(100), 'A');
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(7), "7");
    }
}
