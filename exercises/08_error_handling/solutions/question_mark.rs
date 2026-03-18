use std::num::ParseIntError;

fn parse_and_add_ten(input: &str) -> Result<u32, ParseIntError> {
    let n = input.parse::<u32>()?;
    Ok(n + 10)
}

fn average_of_two(a: &str, b: &str) -> Result<f64, std::num::ParseFloatError> {
    let x = a.parse::<f64>()?;
    let y = b.parse::<f64>()?;
    Ok((x + y) / 2.0)
}

fn parse_config_value(entry: &str) -> Result<i32, String> {
    let (_key, value) = entry
        .split_once('=')
        .ok_or_else(|| "Missing '=' delimiter".to_string())?;
    let parsed = value.parse::<i32>().map_err(|e| e.to_string())?;
    Ok(parsed)
}

fn parse_positive_sqrt(input: &str) -> Result<u32, String> {
    let n = input.parse::<i32>().map_err(|e: ParseIntError| e.to_string())?;
    if n <= 0 {
        return Err(format!("Expected positive number, got {}", n));
    }
    let root = (n as f64).sqrt();
    Ok(root as u32)
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
        assert_eq!(parse_positive_sqrt("2"), Ok(1));
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
