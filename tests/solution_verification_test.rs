//! Solution Verification Test
//!
//! Compiles every exercise's solution through the verification pipeline
//! to ensure all exercises are solvable. Marked #[ignore] because it's slow
//! (~65 compilations). Run with: cargo test -- --ignored

use std::path::PathBuf;

fn exercises_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("exercises")
}

fn cache_dir() -> PathBuf {
    let tmp = std::env::temp_dir().join("rust-game-solution-test-cache");
    let _ = std::fs::create_dir_all(&tmp);
    tmp
}

/// Load the exercise catalog and verify each solution passes its pipeline.
#[test]
#[ignore]
fn test_all_solutions_pass_verification() {
    use rust_game::exercise::loader;
    use rust_game::exercise::types::ExerciseType;
    use rust_game::runner::pipeline;

    let exercises_root = exercises_dir();

    let (_modules, exercises) =
        loader::load_manifest(&exercises_root).expect("Failed to load exercises");

    let cache = cache_dir();
    let mut failures: Vec<String> = Vec::new();
    let mut skipped = 0u32;

    for exercise in &exercises {
        // Skip MCQ exercises — they don't use the cargo pipeline
        if exercise.exercise_type == ExerciseType::ReverseEngineeringMultipleChoice {
            skipped += 1;
            continue;
        }


        if !exercise.solution_path.exists() {
            failures.push(format!(
                "MISSING SOLUTION: {} ({})",
                exercise.id,
                exercise.solution_path.display()
            ));
            continue;
        }

        let run_id = pipeline::next_run_id();
        match pipeline::verify_exercise(exercise, &exercise.solution_path, &cache, run_id) {
            Ok(result) => {
                if !result.passed() {
                    let error = result.first_error().unwrap_or("unknown error");
                    let status_label = if result.status
                        == rust_game::runner::pipeline::VerificationStatus::Timeout
                    {
                        "TIMEOUT"
                    } else {
                        "FAIL"
                    };
                    failures.push(format!(
                        "FAILED: {} — {}: {}",
                        exercise.id,
                        status_label,
                        error.chars().take(200).collect::<String>()
                    ));
                }
            }
            Err(e) => {
                failures.push(format!("ERROR: {} — {:#}", exercise.id, e));
            }
        }
    }

    // Clean up cache
    let _ = std::fs::remove_dir_all(&cache);

    if !failures.is_empty() {
        panic!(
            "\n{} solution(s) failed verification:\n\n{}\n",
            failures.len(),
            failures.join("\n")
        );
    }

    let verified = exercises.len() as u32 - skipped;
    println!(
        "All {} solutions passed verification ({} MCQ skipped).",
        verified, skipped
    );
}
