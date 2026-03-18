use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};

use crate::exercise::types::Exercise;

const SANDBOX_CARGO_TOML: &str = r#"[package]
name = "exercise_check"
version = "0.1.0"
edition = "2021"
"#;

/// A persistent sandbox for compiling exercises.
///
/// Each exercise gets its own sandbox directory that persists across
/// verifications, enabling incremental compilation (2-3s first time, <1s after).
///
/// The sandbox detects Rust toolchain changes via a `.rustc-version` file
/// and rebuilds when the toolchain is updated.
pub struct Sandbox {
    dir: PathBuf,
}

impl Sandbox {
    /// Get or create a sandbox for the given exercise.
    ///
    /// `cache_root` is the `.rust-game-cache/` directory.
    pub fn for_exercise(cache_root: &Path, exercise: &Exercise) -> Result<Self> {
        let dir = cache_root.join(&exercise.id);
        let sandbox = Self { dir };
        sandbox.ensure_project()?;
        sandbox.check_toolchain()?;
        Ok(sandbox)
    }

    /// Create a sandbox in a specific directory (for testing).
    pub fn in_dir(dir: PathBuf) -> Result<Self> {
        let sandbox = Self { dir };
        sandbox.ensure_project()?;
        Ok(sandbox)
    }

    /// The sandbox directory path.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Copy the exercise file into the sandbox for verification.
    ///
    /// If the exercise file doesn't contain `fn main`, appends a dummy
    /// `fn main() {}` so it compiles as a binary crate (needed for cargo test).
    pub fn prepare_exercise(&self, exercise: &Exercise) -> Result<()> {
        let content = fs::read_to_string(&exercise.file_path).with_context(|| {
            format!(
                "Exercise file not found: {}",
                exercise.file_path.display()
            )
        })?;

        let src_dir = self.dir.join("src");

        // Write main exercise file
        let final_content = if content.contains("fn main") {
            content
        } else {
            format!("{}\n\nfn main() {{}}\n", content)
        };
        fs::write(src_dir.join("main.rs"), final_content)
            .context("Failed to write exercise to sandbox")?;

        // Copy extra files to src/
        for extra in &exercise.extra_files {
            let filename = extra
                .file_name()
                .context("Extra file has no filename")?;
            let dest = src_dir.join(filename);
            fs::copy(extra, &dest).with_context(|| {
                format!("Failed to copy extra file: {}", extra.display())
            })?;
        }

        Ok(())
    }

    /// Clean up the sandbox (delete the directory).
    pub fn clean(&self) -> Result<()> {
        if self.dir.exists() {
            fs::remove_dir_all(&self.dir)
                .with_context(|| format!("Failed to clean sandbox: {}", self.dir.display()))?;
        }
        Ok(())
    }

    /// Ensure the Cargo project structure exists.
    fn ensure_project(&self) -> Result<()> {
        let src_dir = self.dir.join("src");
        let cargo_toml = self.dir.join("Cargo.toml");

        if !src_dir.exists() {
            fs::create_dir_all(&src_dir)
                .with_context(|| format!("Failed to create sandbox: {}", self.dir.display()))?;
        }

        if !cargo_toml.exists() {
            fs::write(&cargo_toml, SANDBOX_CARGO_TOML)
                .context("Failed to write sandbox Cargo.toml")?;
        }

        // Write a placeholder main.rs if none exists yet
        let main_rs = src_dir.join("main.rs");
        if !main_rs.exists() {
            fs::write(&main_rs, "fn main() {}\n")
                .context("Failed to write placeholder main.rs")?;
        }

        Ok(())
    }

    /// Check if the Rust toolchain has changed since the sandbox was created.
    /// If so, delete the target dir to force a clean rebuild.
    fn check_toolchain(&self) -> Result<()> {
        let version_file = self.dir.join(".rustc-version");
        let current_version = get_rustc_version()?;

        if version_file.exists() {
            let stored_version = fs::read_to_string(&version_file).unwrap_or_default();
            if stored_version.trim() != current_version.trim() {
                // Toolchain changed — nuke the target dir for clean rebuild
                let target_dir = self.dir.join("target");
                if target_dir.exists() {
                    let _ = fs::remove_dir_all(&target_dir);
                }
            }
        }

        fs::write(&version_file, &current_version)
            .context("Failed to write .rustc-version")?;

        Ok(())
    }
}

/// Clean all sandbox directories under the cache root.
pub fn clean_all_sandboxes(cache_root: &Path) -> Result<()> {
    if cache_root.exists() {
        fs::remove_dir_all(cache_root)
            .with_context(|| format!("Failed to clean cache: {}", cache_root.display()))?;
    }
    Ok(())
}

fn get_rustc_version() -> Result<String> {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .context("Failed to run rustc --version")?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercise::types::*;

    fn make_test_exercise(dir: &Path) -> Exercise {
        let file_path = dir.join("test_ex.rs");
        let solution_path = dir.join("test_solution.rs");
        fs::write(&file_path, "fn main() { println!(\"hello\"); }").unwrap();
        fs::write(&solution_path, "fn main() { println!(\"hello\"); }").unwrap();

        Exercise {
            id: "test_module/test_ex".into(),
            name: "Test".into(),
            module_id: "test_module".into(),
            exercise_type: ExerciseType::FixCompilerError,
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
    fn test_sandbox_creation() {
        let tmp = tempfile::tempdir().unwrap();
        let cache_root = tmp.path().join("cache");
        let exercise = make_test_exercise(tmp.path());

        let sandbox = Sandbox::for_exercise(&cache_root, &exercise).unwrap();
        assert!(sandbox.dir().join("Cargo.toml").exists());
        assert!(sandbox.dir().join("src").exists());
        assert!(sandbox.dir().join(".rustc-version").exists());
    }

    #[test]
    fn test_prepare_exercise_with_main() {
        let tmp = tempfile::tempdir().unwrap();
        let cache_root = tmp.path().join("cache");
        let exercise = make_test_exercise(tmp.path());

        let sandbox = Sandbox::for_exercise(&cache_root, &exercise).unwrap();
        sandbox.prepare_exercise(&exercise).unwrap();

        let main_content = fs::read_to_string(sandbox.dir().join("src/main.rs")).unwrap();
        assert!(main_content.contains("fn main()"));
        // Should NOT have a duplicate main
        assert_eq!(
            main_content.matches("fn main").count(),
            1,
            "Should have exactly one fn main"
        );
    }

    #[test]
    fn test_prepare_exercise_without_main() {
        let tmp = tempfile::tempdir().unwrap();
        let cache_root = tmp.path().join("cache");
        let exercise = make_test_exercise(tmp.path());

        // Write exercise without fn main
        fs::write(
            &exercise.file_path,
            r#"
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() { assert_eq!(add(2, 3), 5); }
}
"#,
        )
        .unwrap();

        let sandbox = Sandbox::for_exercise(&cache_root, &exercise).unwrap();
        sandbox.prepare_exercise(&exercise).unwrap();

        let main_content = fs::read_to_string(sandbox.dir().join("src/main.rs")).unwrap();
        assert!(
            main_content.contains("fn main() {}"),
            "Should append dummy main for exercises without one"
        );
    }

    #[test]
    fn test_sandbox_clean() {
        let tmp = tempfile::tempdir().unwrap();
        let cache_root = tmp.path().join("cache");
        let exercise = make_test_exercise(tmp.path());

        let sandbox = Sandbox::for_exercise(&cache_root, &exercise).unwrap();
        assert!(sandbox.dir().exists());
        sandbox.clean().unwrap();
        assert!(!sandbox.dir().exists());
    }

    #[test]
    fn test_clean_all() {
        let tmp = tempfile::tempdir().unwrap();
        let cache_root = tmp.path().join("cache");
        let exercise = make_test_exercise(tmp.path());

        let _sandbox = Sandbox::for_exercise(&cache_root, &exercise).unwrap();
        assert!(cache_root.exists());
        clean_all_sandboxes(&cache_root).unwrap();
        assert!(!cache_root.exists());
    }
}
