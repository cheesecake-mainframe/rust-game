//! Performance Profiling Tests
//!
//! Measures compilation speed, incremental rebuild performance, and sandbox
//! disk usage. Marked #[ignore] because they involve real compilation.
//! Run with: cargo test -- --ignored performance

use std::path::PathBuf;
use std::time::Instant;

fn exercises_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("exercises")
}

fn perf_cache_dir(name: &str) -> PathBuf {
    let tmp = std::env::temp_dir().join(format!("rust-game-perf-{}", name));
    let _ = std::fs::create_dir_all(&tmp);
    tmp
}

/// Measure first-compile vs incremental-compile time for a representative exercise.
#[test]
#[ignore]
fn performance_first_vs_incremental_compile() {
    use rust_game::exercise::loader;
    use rust_game::runner::pipeline;

    let exercises_root = exercises_dir();
    let (_modules, exercises) =
        loader::load_manifest(&exercises_root).expect("Failed to load exercises");

    let cache = perf_cache_dir("compile");

    // Clean cache to ensure cold start
    let _ = std::fs::remove_dir_all(&cache);
    let _ = std::fs::create_dir_all(&cache);

    // Pick a representative exercise (match_basics: ImplementFromScratch, compile+test)
    let exercise = exercises
        .iter()
        .find(|e| e.id == "03_functions_control_flow/match_basics")
        .expect("match_basics exercise not found");

    // Use the solution file for a passing verification
    let mut solution_exercise = exercise.clone();
    solution_exercise.file_path = exercises_root.join(&exercise.solution_path);

    // First compile (cold cache)
    let start = Instant::now();
    let run_id = pipeline::next_run_id();
    let result = pipeline::verify_exercise(&solution_exercise, &cache, run_id)
        .expect("Verification failed");
    let first_compile_ms = start.elapsed().as_millis();
    assert!(result.passed(), "Solution should pass");

    // Incremental compile (warm cache, same file)
    let start = Instant::now();
    let run_id = pipeline::next_run_id();
    let result = pipeline::verify_exercise(&solution_exercise, &cache, run_id)
        .expect("Verification failed");
    let incremental_ms = start.elapsed().as_millis();
    assert!(result.passed(), "Solution should pass on re-verify");

    // Report
    println!("\n=== Performance Report: Compile Speed ===");
    println!("Exercise:            {}", exercise.id);
    println!("First compile:       {}ms", first_compile_ms);
    println!("Incremental compile: {}ms", incremental_ms);
    println!(
        "Speedup:             {:.1}x",
        first_compile_ms as f64 / incremental_ms.max(1) as f64
    );
    println!("=========================================\n");

    // Incremental should not be significantly slower than first compile.
    // Allow 20% variance for measurement noise on small exercises.
    let threshold = (first_compile_ms as f64 * 1.2) as u128;
    assert!(
        incremental_ms <= threshold,
        "Incremental compile ({}ms) should not be >20% slower than first compile ({}ms)",
        incremental_ms,
        first_compile_ms
    );

    // Clean up
    let _ = std::fs::remove_dir_all(&cache);
}

/// Measure sandbox disk usage for a single exercise.
#[test]
#[ignore]
fn performance_sandbox_disk_usage() {
    use rust_game::exercise::loader;
    use rust_game::runner::pipeline;

    let exercises_root = exercises_dir();
    let (_modules, exercises) =
        loader::load_manifest(&exercises_root).expect("Failed to load exercises");

    let cache = perf_cache_dir("disk");
    let _ = std::fs::remove_dir_all(&cache);
    let _ = std::fs::create_dir_all(&cache);

    // Compile a single exercise
    let exercise = exercises
        .iter()
        .find(|e| e.id == "01_getting_started/hello_world")
        .expect("hello_world exercise not found");

    let mut solution_exercise = exercise.clone();
    solution_exercise.file_path = exercises_root.join(&exercise.solution_path);

    let run_id = pipeline::next_run_id();
    pipeline::verify_exercise(&solution_exercise, &cache, run_id)
        .expect("Verification failed");

    // Measure disk usage
    let size = dir_size(&cache);
    let size_mb = size as f64 / (1024.0 * 1024.0);

    println!("\n=== Performance Report: Disk Usage ===");
    println!("Single exercise sandbox: {:.1} MB", size_mb);
    println!(
        "Projected for 65 exercises: {:.0} MB",
        size_mb * 65.0
    );
    println!("======================================\n");

    // Clean up
    let _ = std::fs::remove_dir_all(&cache);
}

/// Measure compilation time across multiple exercise types.
#[test]
#[ignore]
fn performance_multi_exercise_benchmark() {
    use rust_game::exercise::loader;
    use rust_game::runner::pipeline;

    let exercises_root = exercises_dir();
    let (_modules, exercises) =
        loader::load_manifest(&exercises_root).expect("Failed to load exercises");

    let cache = perf_cache_dir("bench");
    let _ = std::fs::remove_dir_all(&cache);
    let _ = std::fs::create_dir_all(&cache);

    // Pick one exercise of each type
    let ci_exercises: Vec<_> = exercises.iter().filter(|e| e.ci).collect();

    println!("\n=== Performance Report: Multi-Exercise Benchmark ===");
    println!("{:<45} {:>8} {:>8}", "Exercise", "First", "Incr.");
    println!("{}", "-".repeat(65));

    let mut total_first = 0u128;
    let mut total_incr = 0u128;

    for exercise in &ci_exercises {
        // Skip MCQ exercises (no compilation)
        if exercise.exercise_type
            == rust_game::exercise::types::ExerciseType::ReverseEngineeringMultipleChoice
        {
            continue;
        }

        let mut solution_exercise = (*exercise).clone();
        solution_exercise.file_path = exercises_root.join(&exercise.solution_path);

        if !solution_exercise.file_path.exists() {
            println!("{:<45} MISSING SOLUTION", exercise.id);
            continue;
        }

        // First compile
        let start = Instant::now();
        let run_id = pipeline::next_run_id();
        let result = pipeline::verify_exercise(&solution_exercise, &cache, run_id);
        let first_ms = start.elapsed().as_millis();

        // Incremental compile
        let start = Instant::now();
        let run_id = pipeline::next_run_id();
        let _ = pipeline::verify_exercise(&solution_exercise, &cache, run_id);
        let incr_ms = start.elapsed().as_millis();

        let status = match result {
            Ok(r) if r.passed() => "OK",
            Ok(_) => "FAIL",
            Err(_) => "ERR",
        };

        println!(
            "{:<45} {:>6}ms {:>6}ms  [{}]",
            exercise.id, first_ms, incr_ms, status
        );

        total_first += first_ms;
        total_incr += incr_ms;
    }

    println!("{}", "-".repeat(65));
    println!(
        "{:<45} {:>6}ms {:>6}ms",
        "TOTAL", total_first, total_incr
    );
    println!("=====================================================\n");

    // Clean up
    let _ = std::fs::remove_dir_all(&cache);
}

/// Recursively measure directory size in bytes.
fn dir_size(path: &std::path::Path) -> u64 {
    let mut total = 0;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let meta = entry.metadata();
            if let Ok(m) = meta {
                if m.is_file() {
                    total += m.len();
                } else if m.is_dir() {
                    total += dir_size(&entry.path());
                }
            }
        }
    }
    total
}
