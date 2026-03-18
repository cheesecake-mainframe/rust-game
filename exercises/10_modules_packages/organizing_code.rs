// Exercise: Organizing Code
// Type: implement_from_scratch
// Difficulty: intermediate
//
// Good module structure is key to maintainable Rust code. Modules
// group related functionality, control visibility, and create
// clear APIs for the rest of your code.
//
// Implement the modules and types described below to make the tests pass.
// Pay attention to what needs to be `pub` and what should stay private.

// TODO: Create a `math` module containing:
//
//   A public function `gcd(a: u64, b: u64) -> u64`
//       Compute greatest common divisor using Euclid's algorithm.
//
//   A public function `lcm(a: u64, b: u64) -> u64`
//       Compute least common multiple: a * b / gcd(a, b).
//       Handle the case where both are 0 (return 0).
//
//   A sub-module `stats` containing:
//       A public function `mean(data: &[f64]) -> Option<f64>`
//           Return the arithmetic mean, or None if the slice is empty.
//
//       A public function `median(data: &[f64]) -> Option<f64>`
//           Return the median value. For even-length data, average the
//           two middle values. Return None if empty.
//           Hint: sort a clone of the data.

// TODO: Create a `validation` module containing:
//
//   A public struct `Validator` with no public fields.
//       It should have a private field `rules: Vec<Rule>`.
//
//   A private struct `Rule` with fields:
//       name: String
//       check: Box<dyn Fn(&str) -> bool>
//
//   Implement for Validator:
//       pub fn new() -> Validator
//           Creates an empty validator.
//
//       pub fn add_rule(&mut self, name: &str, check: impl Fn(&str) -> bool + 'static)
//           Adds a validation rule.
//
//       pub fn validate(&self, input: &str) -> Vec<String>
//           Returns a list of rule names that the input FAILED.
//           Empty vec means all rules passed.

fn main() {}

#[cfg(test)]
mod tests {
    use super::math;
    use super::math::stats;
    use super::validation::Validator;

    #[test]
    fn test_gcd() {
        assert_eq!(math::gcd(12, 8), 4);
        assert_eq!(math::gcd(7, 13), 1);
        assert_eq!(math::gcd(0, 5), 5);
        assert_eq!(math::gcd(5, 0), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(math::lcm(4, 6), 12);
        assert_eq!(math::lcm(3, 7), 21);
        assert_eq!(math::lcm(0, 0), 0);
    }

    #[test]
    fn test_mean() {
        assert_eq!(stats::mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
        assert_eq!(stats::mean(&[10.0]), Some(10.0));
        assert_eq!(stats::mean(&[]), None);
    }

    #[test]
    fn test_median_odd() {
        assert_eq!(stats::median(&[3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(stats::median(&[5.0, 1.0, 9.0, 3.0, 7.0]), Some(5.0));
    }

    #[test]
    fn test_median_even() {
        assert_eq!(stats::median(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
    }

    #[test]
    fn test_median_empty() {
        assert_eq!(stats::median(&[]), None);
    }

    #[test]
    fn test_validator_all_pass() {
        let mut v = Validator::new();
        v.add_rule("not_empty", |s| !s.is_empty());
        v.add_rule("short_enough", |s| s.len() <= 100);

        let failures = v.validate("hello");
        assert!(failures.is_empty());
    }

    #[test]
    fn test_validator_some_fail() {
        let mut v = Validator::new();
        v.add_rule("not_empty", |s| !s.is_empty());
        v.add_rule("has_uppercase", |s| s.chars().any(|c| c.is_uppercase()));
        v.add_rule("has_digit", |s| s.chars().any(|c| c.is_ascii_digit()));

        let failures = v.validate("hello");
        assert_eq!(failures.len(), 2);
        assert!(failures.contains(&"has_uppercase".to_string()));
        assert!(failures.contains(&"has_digit".to_string()));
    }

    #[test]
    fn test_validator_empty_input() {
        let mut v = Validator::new();
        v.add_rule("not_empty", |s| !s.is_empty());

        let failures = v.validate("");
        assert_eq!(failures, vec!["not_empty".to_string()]);
    }
}
