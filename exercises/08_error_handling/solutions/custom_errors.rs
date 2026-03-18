use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum TemperatureError {
    BelowAbsoluteZero(f64),
    ParseFailed(ParseIntError),
    UnknownScale(char),
}

impl fmt::Display for TemperatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemperatureError::BelowAbsoluteZero(t) => {
                write!(f, "Temperature {}°C is below absolute zero", t)
            }
            TemperatureError::ParseFailed(e) => {
                write!(f, "Failed to parse temperature: {}", e)
            }
            TemperatureError::UnknownScale(c) => {
                write!(f, "Unknown temperature scale: '{}'", c)
            }
        }
    }
}

impl From<ParseIntError> for TemperatureError {
    fn from(e: ParseIntError) -> Self {
        TemperatureError::ParseFailed(e)
    }
}

fn parse_temperature(input: &str) -> Result<f64, TemperatureError> {
    let scale = input.chars().last().unwrap_or(' ');
    let number_part = &input[..input.len() - 1];
    let value: i32 = number_part.parse()?;

    let celsius = match scale {
        'C' | 'c' => value as f64,
        'F' | 'f' => (value as f64 - 32.0) * 5.0 / 9.0,
        other => return Err(TemperatureError::UnknownScale(other)),
    };

    if celsius < -273.15 {
        return Err(TemperatureError::BelowAbsoluteZero(celsius));
    }

    Ok(celsius)
}

fn is_comfortable(input: &str) -> Result<bool, TemperatureError> {
    let celsius = parse_temperature(input)?;
    Ok(celsius >= 18.0 && celsius <= 26.0)
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
        assert_eq!(result.unwrap() as i32, 0);
    }

    #[test]
    fn test_parse_fahrenheit_212() {
        let result = parse_temperature("212F");
        assert_eq!(result.unwrap() as i32, 100);
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
        assert_eq!(is_comfortable("68F").unwrap(), true);
    }

    #[test]
    fn test_is_comfortable_no() {
        assert_eq!(is_comfortable("5C").unwrap(), false);
        assert_eq!(is_comfortable("100F").unwrap(), false);
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
