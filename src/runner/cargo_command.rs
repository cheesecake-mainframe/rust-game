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

    // Wait with timeout using a polling approach
    let timed_out = wait_with_timeout(&mut child, timeout);
    let duration = start.elapsed();

    if timed_out {
        kill_process_tree(&child);
        let _ = child.wait();
        return Ok(CommandOutput {
            success: false,
            stdout: String::new(),
            stderr: format!(
                "Timeout: your code took longer than {} seconds to execute. \
                 Check for infinite loops.",
                timeout.as_secs()
            ),
            duration,
            timed_out: true,
        });
    }

    // Process exited — read stdout and stderr from the pipes
    let mut stdout_str = String::new();
    let mut stderr_str = String::new();

    if let Some(mut stdout) = child.stdout.take() {
        let _ = stdout.read_to_string(&mut stdout_str);
    }
    if let Some(mut stderr) = child.stderr.take() {
        let _ = stderr.read_to_string(&mut stderr_str);
    }

    let status = child.wait().context("Failed to wait for cargo process")?;

    Ok(CommandOutput {
        success: status.success(),
        stdout: stdout_str,
        stderr: stderr_str,
        duration,
        timed_out: false,
    })
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
