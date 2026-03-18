// Exercise: The ? Operator
// Type: implement_from_scratch
// Difficulty: intermediate
//
// The `?` operator is syntactic sugar for propagating errors.
// If the Result is Ok, it unwraps the value. If it's Err, it
// returns the error from the current function immediately.
//
//   let value = something_that_can_fail()?;
//
// is equivalent to:
//   let value = match something_that_can_fail() {
//       Ok(v) => v,
//       Err(e) => return Err(e.into()),
//   };
//
// Implement the functions below using the ? operator.

use std::num::ParseIntError;

/// Parse a string as a u32, add 10, and return the result.
/// Use ? to propagate the parse error.
fn parse_and_add_ten(input: &str) -> Result<u32, ParseIntError> {
    todo!()
}

/// Parse two strings as f64 values and return their average.
/// If either parse fails, return the error.
fn average_of_two(a: &str, b: &str) -> Result<f64, std::num::ParseFloatError> {
    todo!()
}

/// A config entry is a "key=value" string.
/// Parse the value portion as an i32.
/// Return an error message if the format is wrong or the value can't be parsed.
fn parse_config_value(entry: &str) -> Result<i32, String> {
    todo!()
    // Hint: use .split_once('=') which returns Option<(&str, &str)>
    // Convert None to Err("Missing '=' delimiter".to_string())
    // Convert ParseIntError to Err using .map_err(|e| e.to_string())
}

/// Chain multiple fallible operations:
/// 1. Parse `input` as i32
/// 2. Check that it's positive (return Err if not)
/// 3. Compute its square root as f64 (use (n as f64).sqrt())
/// 4. Return the integer part as u32
fn parse_positive_sqrt(input: &str) -> Result<u32, String> {
    todo!()
    // Hint: use .map_err(|e: ParseIntError| e.to_string()) to convert error types
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_add_ten() {
        assert_eq!(parse_and_add_ten("5"), Ok(15));
        assert_eq!(parse_and_add_ten("0"), Ok(10));
    }

    #[test]
    fn test_parse_and_add_ten_error() {
        assert!(parse_and_add_ten("abc").is_err());
        assert!(parse_and_add_ten("").is_err());
    }

    #[test]
    fn test_average_of_two() {
        assert_eq!(average_of_two("10.0", "20.0"), Ok(15.0));
        assert_eq!(average_of_two("3.0", "7.0"), Ok(5.0));
    }

    #[test]
    fn test_average_of_two_error() {
        assert!(average_of_two("abc", "5.0").is_err());
        assert!(average_of_two("5.0", "xyz").is_err());
    }

    #[test]
    fn test_parse_config_value() {
        assert_eq!(parse_config_value("port=8080"), Ok(8080));
        assert_eq!(parse_config_value("timeout=30"), Ok(30));
    }

    #[test]
    fn test_parse_config_value_missing_delimiter() {
        let result = parse_config_value("no_equals_sign");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_config_value_bad_number() {
        let result = parse_config_value("port=abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_positive_sqrt() {
        assert_eq!(parse_positive_sqrt("16"), Ok(4));
        assert_eq!(parse_positive_sqrt("100"), Ok(10));
        assert_eq!(parse_positive_sqrt("2"), Ok(1)); // sqrt(2) = 1.41... -> 1
    }

    #[test]
    fn test_parse_positive_sqrt_negative() {
        assert!(parse_positive_sqrt("-5").is_err());
    }

    #[test]
    fn test_parse_positive_sqrt_zero() {
        assert!(parse_positive_sqrt("0").is_err());
    }

    #[test]
    fn test_parse_positive_sqrt_bad_input() {
        assert!(parse_positive_sqrt("hello").is_err());
    }
}
