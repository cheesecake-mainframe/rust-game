// Exercise: Match Basics
// Type: implement_from_scratch
// Difficulty: beginner
//
// Implement the functions below using Rust's `match` expression.
// Match is like a super-powered switch statement — it must be EXHAUSTIVE
// (cover every possible case).
//
// Coming from Python/JS: think of it as pattern matching on steroids.
// The compiler will tell you if you miss a case!

/// Return the name of the day for a number (1=Monday, 7=Sunday).
/// Return "invalid" for any other number.
fn day_name(day: u32) -> &'static str {
    todo!()
}

/// Classify a grade into a letter (90-100=A, 80-89=B, 70-79=C, 60-69=D, below 60=F).
fn letter_grade(score: u32) -> char {
    todo!()
}

/// FizzBuzz using match: return "Fizz" for multiples of 3,
/// "Buzz" for multiples of 5, "FizzBuzz" for multiples of both,
/// or the number as a string otherwise.
fn fizzbuzz(n: u32) -> String {
    todo!()
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
