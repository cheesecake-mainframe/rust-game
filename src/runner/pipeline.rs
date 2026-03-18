use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use anyhow::Result;

use crate::exercise::types::{Exercise, ExerciseType, VerificationMethod};
use super::cargo_command;
use super::custom_checks;
use super::sandbox::Sandbox;

/// Global counter for verification run IDs.
/// Used by watch-mode consistency rules to discard stale results.
static NEXT_RUN_ID: AtomicU64 = AtomicU64::new(1);

/// Generate a new unique verification run ID.
pub fn next_run_id() -> u64 {
    NEXT_RUN_ID.fetch_add(1, Ordering::SeqCst)
}

/// Status of a verification run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationStatus {
    Passed,
    Failed,
    Timeout,
    Cancelled,
}

/// Result of a single verification step.
#[derive(Debug, Clone)]
pub struct StepResult {
    pub step_name: String,
    pub success: bool,
    pub output: String,
    pub duration: Duration,
}

/// Complete result of verifying an exercise.
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub run_id: u64,
    pub exercise_id: String,
    pub status: VerificationStatus,
    pub steps: Vec<StepResult>,
    pub total_duration: Duration,
}

impl VerificationResult {
    pub fn passed(&self) -> bool {
        self.status == VerificationStatus::Passed
    }

    /// Get the first failing step's output (for display in TUI).
    pub fn first_error(&self) -> Option<&str> {
        self.steps
            .iter()
            .find(|s| !s.success)
            .map(|s| s.output.as_str())
    }
}

/// Default timeouts by exercise type.
fn default_timeout(exercise: &Exercise) -> Duration {
    match exercise.exercise_type {
        ExerciseType::FixCompilerError => Duration::from_secs(10),
        ExerciseType::BossBattle => Duration::from_secs(30),
        _ => Duration::from_secs(15),
    }
}

/// Verify an exercise by running the appropriate pipeline steps.
///
/// This is the main entry point for exercise verification.
/// `cache_root` is the `.rust-game-cache/` directory.
pub fn verify_exercise(
    exercise: &Exercise,
    cache_root: &Path,
    run_id: u64,
) -> Result<VerificationResult> {
    let start = Instant::now();
    let timeout = default_timeout(exercise);
    let mut steps: Vec<StepResult> = Vec::new();

    // Set up sandbox
    let sandbox = Sandbox::for_exercise(cache_root, exercise)?;
    sandbox.prepare_exercise(exercise)?;

    let method = exercise.exercise_type.verification_method();

    // Step 1: Compile (all types except MCQ)
    if method != VerificationMethod::MultipleChoice {
        let compile_result = cargo_command::run_cargo(
            sandbox.dir(),
            &["build"],
            timeout,
        )?;

        steps.push(StepResult {
            step_name: "compile".into(),
            success: compile_result.success,
            output: compile_result.stderr.clone(),
            duration: compile_result.duration,
        });

        if !compile_result.success {
            return Ok(VerificationResult {
                run_id,
                exercise_id: exercise.id.clone(),
                status: if compile_result.timed_out {
                    VerificationStatus::Timeout
                } else {
                    VerificationStatus::Failed
                },
                steps,
                total_duration: start.elapsed(),
            });
        }
    }

    // Step 2: Test (for types that need it)
    if matches!(
        method,
        VerificationMethod::CompileAndTest
            | VerificationMethod::CompileTestClippy
            | VerificationMethod::CompileTestCustom
    ) {
        let test_result = cargo_command::run_cargo(
            sandbox.dir(),
            &["test"],
            timeout,
        )?;

        steps.push(StepResult {
            step_name: "test".into(),
            success: test_result.success,
            output: format!("{}\n{}", test_result.stdout, test_result.stderr),
            duration: test_result.duration,
        });

        if !test_result.success {
            return Ok(VerificationResult {
                run_id,
                exercise_id: exercise.id.clone(),
                status: if test_result.timed_out {
                    VerificationStatus::Timeout
                } else {
                    VerificationStatus::Failed
                },
                steps,
                total_duration: start.elapsed(),
            });
        }
    }

    // Step 3: Clippy (for code transformation exercises)
    // --tests includes test targets so functions only used in #[cfg(test)] aren't flagged as dead code
    if method == VerificationMethod::CompileTestClippy {
        let clippy_result = cargo_command::run_cargo(
            sandbox.dir(),
            &["clippy", "--tests", "--", "-D", "warnings"],
            timeout,
        )?;

        steps.push(StepResult {
            step_name: "clippy".into(),
            success: clippy_result.success,
            output: clippy_result.stderr.clone(),
            duration: clippy_result.duration,
        });

        if !clippy_result.success {
            return Ok(VerificationResult {
                run_id,
                exercise_id: exercise.id.clone(),
                status: VerificationStatus::Failed,
                steps,
                total_duration: start.elapsed(),
            });
        }
    }

    // Step 4: Custom source-level checks (for optimization exercises)
    if method == VerificationMethod::CompileTestCustom && !exercise.custom_checks.is_empty() {
        let source = fs::read_to_string(&exercise.file_path)?;
        let check_results = custom_checks::run_custom_checks(&source, &exercise.custom_checks);
        let step_start = Instant::now();

        let all_passed = check_results.iter().all(|r| r.passed);
        let output = check_results
            .iter()
            .filter(|r| !r.passed)
            .map(|r| r.message.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        steps.push(StepResult {
            step_name: "custom_checks".into(),
            success: all_passed,
            output: if all_passed {
                "All custom checks passed.".into()
            } else {
                output
            },
            duration: step_start.elapsed(),
        });

        if !all_passed {
            return Ok(VerificationResult {
                run_id,
                exercise_id: exercise.id.clone(),
                status: VerificationStatus::Failed,
                steps,
                total_duration: start.elapsed(),
            });
        }
    }

    // All steps passed
    Ok(VerificationResult {
        run_id,
        exercise_id: exercise.id.clone(),
        status: VerificationStatus::Passed,
        steps,
        total_duration: start.elapsed(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercise::types::*;
    fn make_exercise_in(dir: &Path, id: &str, code: &str, ex_type: ExerciseType) -> Exercise {
        let module_dir = dir.join("exercises").join("test_module");
        let solutions_dir = module_dir.join("solutions");
        fs::create_dir_all(&solutions_dir).unwrap();

        let file_path = module_dir.join(format!("{}.rs", id));
        let solution_path = solutions_dir.join(format!("{}.rs", id));
        fs::write(&file_path, code).unwrap();
        fs::write(&solution_path, code).unwrap();

        Exercise {
            id: format!("test_module/{}", id),
            name: id.into(),
            module_id: "test_module".into(),
            exercise_type: ex_type,
            difficulty: Difficulty::Beginner,
            base_xp: 10,
            file_path,
            solution_path,
            hints: vec![],
            description: "test".into(),
            flavor_text: None,
            time_limit_secs: None,
            custom_checks: vec![],
            multiple_choice_options: vec![],
            extra_files: vec![],
            order: 1,
            ci: false,
        }
    }

    #[test]
    fn test_fix_compiler_error_pass() {
        let tmp = tempfile::tempdir().unwrap();
        let cache = tmp.path().join("cache");
        let ex = make_exercise_in(
            tmp.path(),
            "good_compile",
            "fn main() { println!(\"hello\"); }",
            ExerciseType::FixCompilerError,
        );

        let result = verify_exercise(&ex, &cache, 1).unwrap();
        assert!(result.passed(), "Expected pass, got: {:?}", result.steps);
        assert_eq!(result.steps.len(), 1); // compile only
        assert_eq!(result.steps[0].step_name, "compile");
    }

    #[test]
    fn test_fix_compiler_error_fail() {
        let tmp = tempfile::tempdir().unwrap();
        let cache = tmp.path().join("cache");
        let ex = make_exercise_in(
            tmp.path(),
            "bad_compile",
            "fn main() { let x: = 5; }",
            ExerciseType::FixCompilerError,
        );

        let result = verify_exercise(&ex, &cache, 1).unwrap();
        assert!(!result.passed());
        assert_eq!(result.status, VerificationStatus::Failed);
    }

    #[test]
    fn test_debug_logic_bug_pass() {
        let tmp = tempfile::tempdir().unwrap();
        let cache = tmp.path().join("cache");
        let ex = make_exercise_in(
            tmp.path(),
            "good_test",
            r#"
fn add(a: i32, b: i32) -> i32 { a + b }
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() { assert_eq!(add(2, 3), 5); }
}
"#,
            ExerciseType::DebugLogicBug,
        );

        let result = verify_exercise(&ex, &cache, 1).unwrap();
        assert!(result.passed(), "Expected pass, got: {:?}", result.steps);
        assert_eq!(result.steps.len(), 2); // compile + test
    }

    #[test]
    fn test_debug_logic_bug_test_fails() {
        let tmp = tempfile::tempdir().unwrap();
        let cache = tmp.path().join("cache");
        let ex = make_exercise_in(
            tmp.path(),
            "bad_test",
            r#"
fn add(a: i32, b: i32) -> i32 { a - b }
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() { assert_eq!(add(2, 3), 5); }
}
"#,
            ExerciseType::DebugLogicBug,
        );

        let result = verify_exercise(&ex, &cache, 1).unwrap();
        assert!(!result.passed());
        assert_eq!(result.steps.len(), 2); // compile passed, test failed
        assert!(result.steps[0].success); // compile ok
        assert!(!result.steps[1].success); // test failed
    }

    #[test]
    fn test_run_id_increments() {
        let id1 = next_run_id();
        let id2 = next_run_id();
        assert!(id2 > id1);
    }

    #[test]
    fn test_verification_result_first_error() {
        let result = VerificationResult {
            run_id: 1,
            exercise_id: "test".into(),
            status: VerificationStatus::Failed,
            steps: vec![
                StepResult {
                    step_name: "compile".into(),
                    success: true,
                    output: "ok".into(),
                    duration: Duration::from_millis(100),
                },
                StepResult {
                    step_name: "test".into(),
                    success: false,
                    output: "assertion failed".into(),
                    duration: Duration::from_millis(50),
                },
            ],
            total_duration: Duration::from_millis(150),
        };

        assert_eq!(result.first_error(), Some("assertion failed"));
    }
}
