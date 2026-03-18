use crate::exercise::types::{CustomCheck, CustomCheckType};

/// Result of a single custom check.
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub passed: bool,
    pub message: String,
}

/// Run all custom checks against an exercise's source code.
pub fn run_custom_checks(source: &str, checks: &[CustomCheck]) -> Vec<CheckResult> {
    let stripped = strip_comments_and_strings(source);

    checks
        .iter()
        .map(|check| run_single_check(&stripped, check))
        .collect()
}

fn run_single_check(stripped_source: &str, check: &CustomCheck) -> CheckResult {
    let found = match check.check_type {
        CustomCheckType::NoClone => {
            has_pattern(stripped_source, ".clone()")
                || has_pattern(stripped_source, "Clone::clone(")
                || has_pattern(stripped_source, ".to_owned()")
                || has_pattern(stripped_source, ".to_vec()")
        }
        CustomCheckType::NoUnwrap => {
            has_pattern(stripped_source, ".unwrap()")
                || has_pattern(stripped_source, ".expect(")
        }
        CustomCheckType::NoCollect => has_pattern(stripped_source, ".collect()"),
        CustomCheckType::NoBoxDyn => has_pattern(stripped_source, "Box<dyn"),
        CustomCheckType::MaxLines => {
            // MaxLines is handled specially — we check line count
            // The actual max is encoded in the message (TODO: add max_lines field)
            false
        }
    };

    if found {
        CheckResult {
            passed: false,
            message: check.message.clone(),
        }
    } else {
        CheckResult {
            passed: true,
            message: "Check passed.".into(),
        }
    }
}

fn has_pattern(source: &str, pattern: &str) -> bool {
    source.contains(pattern)
}

/// Strip line comments (//) and string literals ("...") from source code.
///
/// This is a best-effort approach — it handles the common cases without
/// needing a full Rust parser. It won't handle raw strings (r#"..."#)
/// or nested block comments perfectly, but it's good enough for detecting
/// .clone(), .unwrap(), etc.
fn strip_comments_and_strings(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    let mut in_block_comment = false;

    while let Some(ch) = chars.next() {
        if in_block_comment {
            if ch == '*' && chars.peek() == Some(&'/') {
                chars.next(); // consume '/'
                in_block_comment = false;
            }
            continue;
        }

        match ch {
            '/' => {
                match chars.peek() {
                    Some(&'/') => {
                        // Line comment — skip to end of line
                        for c in chars.by_ref() {
                            if c == '\n' {
                                result.push('\n');
                                break;
                            }
                        }
                    }
                    Some(&'*') => {
                        chars.next(); // consume '*'
                        in_block_comment = true;
                    }
                    _ => result.push(ch),
                }
            }
            '"' => {
                // String literal — skip until closing quote
                // (doesn't handle escaped quotes perfectly but good enough)
                let mut prev = '"';
                for c in chars.by_ref() {
                    if c == '"' && prev != '\\' {
                        break;
                    }
                    prev = c;
                }
                result.push_str("\"\""); // placeholder
            }
            _ => result.push(ch),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_check(check_type: CustomCheckType) -> CustomCheck {
        CustomCheck {
            check_type,
            message: format!("Found {:?}", check_type),
        }
    }

    #[test]
    fn test_no_clone_catches_clone() {
        let source = r#"
fn process(data: &Vec<i32>) -> Vec<i32> {
    data.clone()
}
fn main() {}
"#;
        let results = run_custom_checks(source, &[make_check(CustomCheckType::NoClone)]);
        assert!(!results[0].passed);
    }

    #[test]
    fn test_no_clone_catches_to_owned() {
        let source = r#"
fn process(s: &str) -> String {
    s.to_owned()
}
fn main() {}
"#;
        let results = run_custom_checks(source, &[make_check(CustomCheckType::NoClone)]);
        assert!(!results[0].passed);
    }

    #[test]
    fn test_no_clone_passes_clean_code() {
        let source = r#"
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
fn main() {}
"#;
        let results = run_custom_checks(source, &[make_check(CustomCheckType::NoClone)]);
        assert!(results[0].passed);
    }

    #[test]
    fn test_no_clone_ignores_comments() {
        let source = r#"
// You could use .clone() here but don't
fn process(s: &str) -> &str {
    s
}
fn main() {}
"#;
        let results = run_custom_checks(source, &[make_check(CustomCheckType::NoClone)]);
        assert!(results[0].passed, "Should ignore .clone() in comments");
    }

    #[test]
    fn test_no_clone_ignores_strings() {
        let source = r#"
fn main() {
    println!("Don't use .clone() here");
}
"#;
        let results = run_custom_checks(source, &[make_check(CustomCheckType::NoClone)]);
        assert!(results[0].passed, "Should ignore .clone() in strings");
    }

    #[test]
    fn test_no_unwrap_catches_unwrap() {
        let source = r#"
fn main() {
    let x: Option<i32> = Some(5);
    let y = x.unwrap();
}
"#;
        let results = run_custom_checks(source, &[make_check(CustomCheckType::NoUnwrap)]);
        assert!(!results[0].passed);
    }

    #[test]
    fn test_no_unwrap_catches_expect() {
        let source = r#"
fn main() {
    let x: Option<i32> = Some(5);
    let y = x.expect("should exist");
}
"#;
        let results = run_custom_checks(source, &[make_check(CustomCheckType::NoUnwrap)]);
        assert!(!results[0].passed);
    }

    #[test]
    fn test_multiple_checks() {
        let source = r#"
fn main() {
    let v = vec![1, 2, 3];
    let v2 = v.clone();
    let x = v2.first().unwrap();
}
"#;
        let checks = vec![
            make_check(CustomCheckType::NoClone),
            make_check(CustomCheckType::NoUnwrap),
        ];
        let results = run_custom_checks(source, &checks);
        assert!(!results[0].passed, "Should catch .clone()");
        assert!(!results[1].passed, "Should catch .unwrap()");
    }

    #[test]
    fn test_strip_block_comments() {
        let source = r#"
/* this has .clone() in it */
fn main() {}
"#;
        let results = run_custom_checks(source, &[make_check(CustomCheckType::NoClone)]);
        assert!(results[0].passed, "Should ignore .clone() in block comments");
    }
}
