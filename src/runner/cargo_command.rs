use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};

/// Result of running a cargo command.
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub duration: Duration,
    pub timed_out: bool,
}

/// Run a cargo command in the given directory with a timeout.
///
/// On Unix, spawns the child in its own process group so that on timeout
/// the entire group (including rustc subprocesses) is killed cleanly.
pub fn run_cargo(
    working_dir: &Path,
    args: &[&str],
    timeout: Duration,
) -> Result<CommandOutput> {
    let start = Instant::now();

    let mut cmd = Command::new("cargo");
    cmd.args(args)
        .current_dir(working_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        // Suppress cargo's incremental compilation status bar noise
        .env("CARGO_TERM_PROGRESS_WHEN", "never");

    // Verification must not depend on the shell that launched the game.
    // `RUSTFLAGS="-D warnings"` in a profile would fail a correct solution over
    // an unused function, and a shared `CARGO_TARGET_DIR` lets one exercise's
    // artifacts satisfy another's build — which can report success for code
    // that does not compile.
    //
    // Deliberately not `env_clear()`: that would drop PATH, HOME, and the
    // RUSTUP_* shims this runs behind, trading a rare bug for a common one.
    for var in ENV_TO_SCRUB {
        cmd.env_remove(var);
    }

    // On Unix, create a new process group so we can kill the whole group
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            cmd.pre_exec(|| {
                libc::setsid();
                Ok(())
            });
        }
    }

    let mut child = cmd.spawn().context("Failed to spawn cargo process")?;

    // Drain both pipes on their own threads *while* we wait.
    //
    // A pipe is a fixed-size kernel buffer (64 KiB on Linux). If we waited for
    // the child before reading, a child that produced more than that would
    // block forever in write() — and we would block forever waiting for it to
    // exit. The timeout would then fire and report an infinite loop, which is
    // exactly wrong: print-debugging inside a finite loop triggers it.
    let out_handle = spawn_reader(child.stdout.take());
    let err_handle = spawn_reader(child.stderr.take());

    let timed_out = wait_with_timeout(&mut child, timeout);
    let duration = start.elapsed();

    if timed_out {
        // Killing the process group closes every write end — including any
        // rustc grandchildren, which share the group via setsid — so the
        // readers below reach EOF and their joins cannot hang.
        kill_process_tree(&child);
    }

    // Reap the child exactly once, on both paths. A failure to reap is not
    // worth discarding the output we captured, so this is deliberately not `?`.
    let status = child.wait().ok();

    let stdout_str = join_reader(out_handle);
    let captured_stderr = join_reader(err_handle);

    let stderr_str = if timed_out {
        // Keep what the run produced. A real timeout with thousands of lines of
        // println! visible is itself the diagnosis; discarding it is what made
        // the pipe deadlock indistinguishable from a genuine infinite loop.
        format!(
            "Timeout: your code took longer than {} seconds to execute. \
             Check for infinite loops.\n\nOutput captured before the timeout:\n{}",
            timeout.as_secs(),
            captured_stderr
        )
    } else {
        captured_stderr
    };

    Ok(CommandOutput {
        success: !timed_out && status.map(|s| s.success()).unwrap_or(false),
        stdout: stdout_str,
        stderr: stderr_str,
        duration,
        timed_out,
    })
}

/// Cargo/rustc environment variables that would otherwise leak in from the
/// user's shell and change what verification reports.
const ENV_TO_SCRUB: [&str; 10] = [
    "RUSTFLAGS",
    "CARGO_ENCODED_RUSTFLAGS",
    "CARGO_BUILD_RUSTFLAGS",
    "CARGO_TARGET_DIR",
    "CARGO_BUILD_TARGET_DIR",
    "RUSTC_WRAPPER",
    "RUSTC_WORKSPACE_WRAPPER",
    "CARGO_INCREMENTAL",
    "CARGO_BUILD_TARGET",
    "RUSTC",
];

/// Move a child pipe onto a thread that reads it to end-of-file.
fn spawn_reader<R: Read + Send + 'static>(
    pipe: Option<R>,
) -> Option<std::thread::JoinHandle<String>> {
    pipe.map(|mut r| {
        std::thread::spawn(move || {
            let mut buf = String::new();
            let _ = r.read_to_string(&mut buf);
            buf
        })
    })
}

/// Collect a reader thread's output. A panicking reader yields an empty string
/// rather than poisoning the whole verification.
fn join_reader(handle: Option<std::thread::JoinHandle<String>>) -> String {
    handle
        .map(|h| h.join().unwrap_or_default())
        .unwrap_or_default()
}

/// Poll the child process until it exits or the timeout is reached.
/// Returns true if the timeout was reached.
fn wait_with_timeout(child: &mut std::process::Child, timeout: Duration) -> bool {
    let start = Instant::now();
    let poll_interval = Duration::from_millis(50);

    loop {
        match child.try_wait() {
            Ok(Some(_)) => return false, // Process exited
            Ok(None) => {
                if start.elapsed() >= timeout {
                    return true; // Timed out
                }
                std::thread::sleep(poll_interval);
            }
            Err(_) => return false, // Error — treat as exited
        }
    }
}

/// Kill a process and all its children.
///
/// On Unix: kills the process group (all children spawned by this process).
/// On Windows: kills just the process (job objects would be better but are complex).
fn kill_process_tree(child: &std::process::Child) {
    let pid = child.id();

    #[cfg(unix)]
    {
        // Kill the entire process group. The negative PID means "kill the group".
        unsafe {
            libc::kill(-(pid as i32), libc::SIGKILL);
        }
    }

    #[cfg(not(unix))]
    {
        // Fallback: just kill the main process
        // On Windows, this may leave orphaned rustc subprocesses
        let _ = Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .output();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_simple_cargo_project(dir: &Path) {
        fs::create_dir_all(dir.join("src")).unwrap();
        fs::write(
            dir.join("Cargo.toml"),
            r#"
[package]
name = "test_exercise"
version = "0.1.0"
edition = "2021"
"#,
        )
        .unwrap();
        fs::write(dir.join("src/main.rs"), "fn main() { println!(\"ok\"); }")
            .unwrap();
    }

    #[test]
    fn test_cargo_build_success() {
        let tmp = tempfile::tempdir().unwrap();
        create_simple_cargo_project(tmp.path());

        let result = run_cargo(tmp.path(), &["build"], Duration::from_secs(30)).unwrap();
        assert!(result.success, "Build failed: {}", result.stderr);
        assert!(!result.timed_out);
    }

    #[test]
    fn test_cargo_build_failure() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(
            tmp.path().join("Cargo.toml"),
            r#"
[package]
name = "test_exercise"
version = "0.1.0"
edition = "2021"
"#,
        )
        .unwrap();
        // Invalid Rust code
        fs::write(tmp.path().join("src/main.rs"), "fn main() { let x: = 5; }")
            .unwrap();

        let result = run_cargo(tmp.path(), &["build"], Duration::from_secs(30)).unwrap();
        assert!(!result.success);
        assert!(!result.timed_out);
        assert!(!result.stderr.is_empty());
    }

    #[test]
    fn test_cargo_test_success() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(
            tmp.path().join("Cargo.toml"),
            r#"
[package]
name = "test_exercise"
version = "0.1.0"
edition = "2021"
"#,
        )
        .unwrap();
        fs::write(
            tmp.path().join("src/main.rs"),
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
        )
        .unwrap();

        let result = run_cargo(tmp.path(), &["test"], Duration::from_secs(30)).unwrap();
        assert!(result.success, "Tests failed: {}", result.stderr);
    }

    #[test]
    fn test_timeout_kills_process() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(
            tmp.path().join("Cargo.toml"),
            r#"
[package]
name = "test_exercise"
version = "0.1.0"
edition = "2021"
"#,
        )
        .unwrap();
        fs::write(
            tmp.path().join("src/main.rs"),
            "fn main() { loop {} }",
        )
        .unwrap();

        // Build first (so the run is fast), then run with short timeout
        let build = run_cargo(tmp.path(), &["build"], Duration::from_secs(30)).unwrap();
        if !build.success {
            // Build failed — that's fine, skip the timeout test
            return;
        }

        let result = run_cargo(tmp.path(), &["run"], Duration::from_secs(2)).unwrap();
        assert!(!result.success);
        assert!(result.timed_out);
        assert!(result.stderr.contains("Timeout"));
    }
}
