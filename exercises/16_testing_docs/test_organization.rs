// Exercise: Test Organization (FixCompilerError)
// ================================================
//
// Rust has a specific way to organize tests:
//   - Unit tests go in a `#[cfg(test)] mod tests { ... }` block
//   - `use super::*;` imports items from the parent module
//   - Test helper functions can be defined inside the test module
//   - `#[test]` marks a function as a test
//
// This file has several organizational issues that prevent compilation.
// Fix them all so the tests compile and pass!
//
// HINTS:
// - The test module needs the right attribute to only compile during testing
// - Tests need access to the parent module's functions
// - The `#[test]` attribute must be on the right functions
// - Helper functions in the test module should NOT have #[test]
// - Look for typos in attribute names

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

// FIX 1: This module needs the attribute that tells Rust to only compile
// it when running tests. The attribute below has a typo.
#[cfg(testing)]
mod tests {
    // FIX 2: We need to import everything from the parent module.
    // The syntax below is wrong.
    use super::tests;

    // FIX 3: This is a helper function used by tests -- it should NOT
    // have the #[test] attribute! Remove it.
    #[test]
    fn create_calculator_with_history(values: &[f64]) -> Calculator {
        let mut calc = Calculator::new();
        for &v in values {
            calc.add(v, 0.0); // adds each value to history
        }
        calc
    }

    // FIX 4: This test is missing the #[test] attribute.
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
        // FIX 5: This uses the helper function -- make sure the helper
        // compiles correctly (it shouldn't be a #[test]).
        let calc = create_calculator_with_history(&[1.0, 2.0, 3.0]);
        assert_eq!(calc.last_result(), Some(3.0));
    }

    #[test]
    fn test_clear_history() {
        let mut calc = create_calculator_with_history(&[1.0, 2.0]);
        calc.clear_history();
        assert_eq!(calc.last_result(), None);
    }

    // FIX 6: This test function has #[test] spelled wrong.
    #[tset]
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
