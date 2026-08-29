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
            '\'' => {
                // A tick opens either a char literal ('a', '\n', '\'') or a
                // lifetime ('a, as in `&'a str`). Only the literal form may be
                // skipped: treating a lifetime as one swallows every character
                // to the next tick, which on a file with an odd number of them
                // means the rest of the source vanishes from the check.
                let mut look = chars.clone();
                let is_char_literal = match (look.next(), look.next()) {
                    (Some('\\'), _) => true,       // escaped: '\n', '\'', '\\'
                    (Some(_), Some('\'')) => true,  // simple:  'a'
                    _ => false,                     // otherwise a lifetime
                };

                if is_char_literal {
                    let mut prev = '\'';
                    for c in chars.by_ref() {
                        if c == '\'' && prev != '\\' {
                            break;
                        }
                        prev = if prev == '\\' && c == '\\' { ' ' } else { c };
                    }
                    result.push_str("''"); // placeholder
                } else {
                    result.push('\'');
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
    fn test_lifetime_annotations_do_not_swallow_source() {
        // A lifetime tick used to open char-literal consumption, hiding every
        // subsequent line from the check — so a `.clone()` below a lifetime
        // annotation passed silently and awarded XP for an unsolved constraint.
        let source = r#"
fn names<'a>(items: &'a [String]) -> Vec<&'a str> {
    items.iter().map(|s| s.as_str()).collect()
}

fn sneaky(v: &Vec<i32>) -> Vec<i32> {
    v.clone()
}
"#;
        let checks = vec![CustomCheck {
            check_type: CustomCheckType::NoClone,
            message: "no clone".into(),
        }];
        let results = run_custom_checks(source, &checks);
        assert!(
            !results[0].passed,
            "a .clone() after a lifetime annotation must still be caught"
        );
    }

    #[test]
    fn test_char_literals_do_not_open_string_mode() {
        // The original bug this arm exists for: `'"'` used to be read as a
        // string opener, swallowing source up to the next quote.
        let source = r#"
fn main() {
    let quote = '"';
    let tick = '\'';
    let slash = '\\';
    let v = vec![1];
    let w = v.clone();
    println!("{:?} {} {} {:?}", quote, tick, slash, w);
}
"#;
        let checks = vec![CustomCheck {
            check_type: CustomCheckType::NoClone,
            message: "no clone".into(),
        }];
        let results = run_custom_checks(source, &checks);
        assert!(
            !results[0].passed,
            "char literals must not hide the .clone() that follows them"
        );
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
