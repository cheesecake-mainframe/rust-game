//! Regression tests for verification correctness.
//!
//! Each of these fails against the code as it stood before the pipe-drain and
//! environment-scrubbing fixes. They are slower than the unit tests because
//! they invoke real cargo builds, but each one guards a bug that made the tool
//! report something false about the student's code.

use std::path::{Path, PathBuf};
use std::time::Duration;

use rust_game::runner::cargo_command::run_cargo;
use rust_game::runner::sandbox::Sandbox;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("rust-game-hermeticity").join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    dir
}

fn write_crate(dir: &Path, manifest_extra: &str, main_rs: &str) {
    std::fs::write(
        dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"probe\"\nversion = \"0.1.0\"\nedition = \"2021\"\n{}",
            manifest_extra
        ),
    )
    .unwrap();
    std::fs::write(dir.join("src/main.rs"), main_rs).unwrap();
}

/// The headline bug: pipes were only read *after* the child exited, so a child
/// that filled the 64 KiB pipe buffer blocked forever and the timeout fired.
/// The student was told their finite loop was infinite.
#[test]
fn large_output_does_not_false_timeout() {
    let dir = scratch("large_output");
    let mut src = String::from("fn main() {\n");
    for i in 0..2000 {
        src.push_str(&format!(
            "    let v{i}: i32 = \"deliberately the wrong type\";\n"
        ));
    }
    src.push_str("}\n");
    write_crate(&dir, "[workspace]\n", &src);

    let out = run_cargo(&dir, &["build"], Duration::from_secs(60)).unwrap();

    assert!(
        !out.timed_out,
        "a build producing >64KB of output must not be reported as a timeout"
    );
    assert!(
        out.stderr.len() > 64 * 1024,
        "expected the full compiler output, got {} bytes",
        out.stderr.len()
    );
}

/// A genuine timeout must still show what the run produced. Discarding it is
/// what made the pipe deadlock indistinguishable from a real infinite loop.
#[test]
fn real_timeout_keeps_captured_output() {
    let dir = scratch("real_timeout");
    write_crate(
        &dir,
        "[workspace]\n",
        "fn main() {\n    loop {\n        println!(\"still going\");\n    }\n}\n",
    );

    // Build first so the timeout lands on the run, not on compilation.
    let _ = run_cargo(&dir, &["build"], Duration::from_secs(120));
    let out = run_cargo(&dir, &["run"], Duration::from_secs(3)).unwrap();

    assert!(out.timed_out, "an infinite loop should time out");
    assert!(
        out.stderr.contains("Output captured before the timeout"),
        "a timeout must say it kept the output"
    );
    assert!(
        out.stdout.contains("still going"),
        "a timeout must actually retain the output, not just claim to"
    );
}

/// Verification must not depend on the shell that launched the game. With
/// `RUSTFLAGS=-D warnings` exported, an unused function used to fail a
/// perfectly correct solution.
#[test]
fn ambient_rustflags_do_not_leak_into_verification() {
    let dir = scratch("rustflags");
    write_crate(
        &dir,
        "[workspace]\n",
        "fn unused_helper() {}\nfn main() { println!(\"ok\"); }\n",
    );

    // SAFETY: single-threaded by virtue of being the only test in this binary
    // that touches the environment; `set_var` is process-global.
    std::env::set_var("RUSTFLAGS", "-D warnings");
    let out = run_cargo(&dir, &["build"], Duration::from_secs(60)).unwrap();
    std::env::remove_var("RUSTFLAGS");

    assert!(
        out.success,
        "ambient RUSTFLAGS must not fail a correct solution; stderr:\n{}",
        out.stderr
    );
}

/// Sandboxes live inside the repo, so an ancestor manifest with a `[workspace]`
/// table used to break every verification. The sandbox declares its own.
#[test]
fn sandbox_builds_under_an_ancestor_workspace() {
    let root = scratch("ancestor_workspace");
    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    )
    .unwrap();

    // Build the nested project through Sandbox so the manifest under test is
    // the real one. Writing our own `[workspace]` here would test the test.
    let nested = root.join("cache").join("exercise");
    let sandbox = Sandbox::in_dir(nested.clone()).expect("sandbox should be creatable");
    assert!(
        std::fs::read_to_string(sandbox.dir().join("Cargo.toml"))
            .unwrap()
            .contains("[workspace]"),
        "the sandbox manifest must declare its own workspace"
    );

    let out = run_cargo(&nested, &["build"], Duration::from_secs(60)).unwrap();
    assert!(
        out.success,
        "a sandbox must build beneath an ancestor workspace; stderr:\n{}",
        out.stderr
    );
}
