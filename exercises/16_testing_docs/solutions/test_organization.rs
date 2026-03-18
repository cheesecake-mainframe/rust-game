// Solution: Test Organization
// ============================

/// A simple calculator that performs basic operations.
struct Calculator {
    history: Vec<f64>,
}

impl Calculator {
    fn new() -> Self {
        Calculator { history: Vec::new() }
    }

    fn add(&mut self, a: f64, b: f64) -> f64 {
        let result = a + b;
        self.history.push(result);
        result
    }

    fn subtract(&mut self, a: f64, b: f64) -> f64 {
        let result = a - b;
        self.history.push(result);
        result
    }

    fn multiply(&mut self, a: f64, b: f64) -> f64 {
        let result = a * b;
        self.history.push(result);
        result
    }

    fn divide(&mut self, a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Division by zero".to_string())
        } else {
            let result = a / b;
            self.history.push(result);
            Ok(result)
        }
    }

    fn last_result(&self) -> Option<f64> {
        self.history.last().copied()
    }

    fn clear_history(&mut self) {
        self.history.clear();
    }
}

/// Validates that a username meets requirements:
/// - At least 3 characters
/// - At most 20 characters
/// - Only alphanumeric and underscores
fn validate_username(username: &str) -> Result<(), String> {
    if username.len() < 3 {
        return Err("Username too short (minimum 3 characters)".to_string());
    }
    if username.len() > 20 {
        return Err("Username too long (maximum 20 characters)".to_string());
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Username contains invalid characters".to_string());
    }
    Ok(())
}

fn main() {}

// FIX 1: Changed #[cfg(testing)] to #[cfg(test)]
#[cfg(test)]
mod tests {
    // FIX 2: Changed `use super::tests` to `use super::*`
    use super::*;

    // FIX 3: Removed #[test] from helper function -- it's not a test,
    // it's a utility used by other tests.
    fn create_calculator_with_history(values: &[f64]) -> Calculator {
        let mut calc = Calculator::new();
        for &v in values {
            calc.add(v, 0.0);
        }
        calc
    }

    // FIX 4: Added missing #[test] attribute
    #[test]
    fn test_add() {
        let mut calc = Calculator::new();
        assert_eq!(calc.add(2.0, 3.0), 5.0);
        assert_eq!(calc.add(-1.0, 1.0), 0.0);
    }

    #[test]
    fn test_subtract() {
        let mut calc = Calculator::new();
        assert_eq!(calc.subtract(10.0, 4.0), 6.0);
        assert_eq!(calc.subtract(0.0, 5.0), -5.0);
    }

    #[test]
    fn test_multiply() {
        let mut calc = Calculator::new();
        assert_eq!(calc.multiply(3.0, 4.0), 12.0);
        assert_eq!(calc.multiply(-2.0, 3.0), -6.0);
        assert_eq!(calc.multiply(0.0, 100.0), 0.0);
    }

    #[test]
    fn test_divide_success() {
        let mut calc = Calculator::new();
        assert_eq!(calc.divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        let mut calc = Calculator::new();
        assert!(calc.divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_history() {
        let calc = create_calculator_with_history(&[1.0, 2.0, 3.0]);
        assert_eq!(calc.last_result(), Some(3.0));
    }

    #[test]
    fn test_clear_history() {
        let mut calc = create_calculator_with_history(&[1.0, 2.0]);
        calc.clear_history();
        assert_eq!(calc.last_result(), None);
    }

    // FIX 6: Changed #[tset] to #[test]
    #[test]
    fn test_validate_username_valid() {
        assert!(validate_username("alice").is_ok());
        assert!(validate_username("bob_42").is_ok());
        assert!(validate_username("abc").is_ok());
    }

    #[test]
    fn test_validate_username_too_short() {
        assert!(validate_username("ab").is_err());
        assert!(validate_username("").is_err());
    }

    #[test]
    fn test_validate_username_too_long() {
        let long_name = "a".repeat(21);
        assert!(validate_username(&long_name).is_err());
    }

    #[test]
    fn test_validate_username_invalid_chars() {
        assert!(validate_username("hello world").is_err());
        assert!(validate_username("user@name").is_err());
    }
}
