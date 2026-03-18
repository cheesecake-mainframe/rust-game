// Exercise: Custom Errors
// Type: implement_from_scratch
// Difficulty: intermediate
//
// In real applications, you define your own error types to represent
// the different ways your code can fail. A custom error enum lets
// callers match on specific error variants and handle them differently.
//
// Implement the custom error enum and the functions that use it.

use std::fmt;
use std::num::ParseIntError;

// TODO: Define a `TemperatureError` enum with these variants:
//   - BelowAbsoluteZero(f64)    — temperature below -273.15 C
//   - ParseFailed(ParseIntError) — couldn't parse the input string
//   - UnknownScale(char)         — unrecognized temperature scale letter

// TODO: Implement std::fmt::Display for TemperatureError.
// Use these messages:
//   BelowAbsoluteZero(t) => "Temperature {t}°C is below absolute zero"
//   ParseFailed(e) => "Failed to parse temperature: {e}"
//   UnknownScale(c) => "Unknown temperature scale: '{c}'"

// TODO: Implement From<ParseIntError> for TemperatureError
// so the ? operator can auto-convert parse errors.

/// Parse a temperature string like "100C" or "212F" into Celsius.
/// The last character is the scale (C or F), the rest is the number.
fn parse_temperature(input: &str) -> Result<f64, TemperatureError> {
    todo!()
    // Steps:
    // 1. Split input into number part and scale character
    //    (hint: use input[..input.len()-1] and input.chars().last())
    // 2. Parse the number part as i32 using ? (From<ParseIntError> converts it)
    // 3. Match on scale:
    //    'C' | 'c' => use as-is
    //    'F' | 'f' => convert: (fahrenheit - 32) * 5 / 9
    //    other => return Err(TemperatureError::UnknownScale(other))
    // 4. Check that result >= -273.15, otherwise BelowAbsoluteZero
}

/// Check if a temperature string represents "comfortable" range (18-26°C).
fn is_comfortable(input: &str) -> Result<bool, TemperatureError> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_celsius() {
        let result = parse_temperature("100C");
        assert_eq!(result.unwrap() as i32, 100);
    }

    #[test]
    fn test_parse_fahrenheit() {
        let result = parse_temperature("32F");
        assert_eq!(result.unwrap() as i32, 0); // 32F = 0C
    }

    #[test]
    fn test_parse_fahrenheit_212() {
        let result = parse_temperature("212F");
        assert_eq!(result.unwrap() as i32, 100); // 212F = 100C
    }

    #[test]
    fn test_unknown_scale() {
        let result = parse_temperature("100X");
        assert!(result.is_err());
        if let Err(TemperatureError::UnknownScale(c)) = result {
            assert_eq!(c, 'X');
        } else {
            panic!("Expected UnknownScale error");
        }
    }

    #[test]
    fn test_parse_error() {
        let result = parse_temperature("abcC");
        assert!(result.is_err());
        assert!(matches!(result, Err(TemperatureError::ParseFailed(_))));
    }

    #[test]
    fn test_below_absolute_zero() {
        let result = parse_temperature("-300C");
        assert!(result.is_err());
        assert!(matches!(result, Err(TemperatureError::BelowAbsoluteZero(_))));
    }

    #[test]
    fn test_is_comfortable_yes() {
        assert_eq!(is_comfortable("22C").unwrap(), true);
        assert_eq!(is_comfortable("68F").unwrap(), true); // 68F = 20C
    }

    #[test]
    fn test_is_comfortable_no() {
        assert_eq!(is_comfortable("5C").unwrap(), false);
        assert_eq!(is_comfortable("100F").unwrap(), false); // 100F = ~37.7C
    }

    #[test]
    fn test_display_below_absolute_zero() {
        let err = TemperatureError::BelowAbsoluteZero(-300.0);
        let msg = format!("{}", err);
        assert!(msg.contains("-300"));
        assert!(msg.contains("absolute zero"));
    }

    #[test]
    fn test_display_unknown_scale() {
        let err = TemperatureError::UnknownScale('X');
        let msg = format!("{}", err);
        assert!(msg.contains("'X'"));
    }
}
