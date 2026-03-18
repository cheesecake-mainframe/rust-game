pub mod math {
    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    pub fn lcm(a: u64, b: u64) -> u64 {
        if a == 0 && b == 0 {
            return 0;
        }
        a / gcd(a, b) * b
    }

    pub mod stats {
        pub fn mean(data: &[f64]) -> Option<f64> {
            if data.is_empty() {
                return None;
            }
            let sum: f64 = data.iter().sum();
            Some(sum / data.len() as f64)
        }

        pub fn median(data: &[f64]) -> Option<f64> {
            if data.is_empty() {
                return None;
            }
            let mut sorted = data.to_vec();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let len = sorted.len();
            if len % 2 == 1 {
                Some(sorted[len / 2])
            } else {
                Some((sorted[len / 2 - 1] + sorted[len / 2]) / 2.0)
            }
        }
    }
}

pub mod validation {
    struct Rule {
        name: String,
        check: Box<dyn Fn(&str) -> bool>,
    }

    pub struct Validator {
        rules: Vec<Rule>,
    }

    impl Validator {
        pub fn new() -> Validator {
            Validator { rules: Vec::new() }
        }

        pub fn add_rule(&mut self, name: &str, check: impl Fn(&str) -> bool + 'static) {
            self.rules.push(Rule {
                name: name.to_string(),
                check: Box::new(check),
            });
        }

        pub fn validate(&self, input: &str) -> Vec<String> {
            self.rules
                .iter()
                .filter(|rule| !(rule.check)(input))
                .map(|rule| rule.name.clone())
                .collect()
        }
    }
}

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
