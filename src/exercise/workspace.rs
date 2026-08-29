use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::types::Exercise;

/// Maps a tracked exercise template to the student's personal working copy.
///
/// Templates live under `exercises/` and are committed to the repository.
/// The student's edits live under a gitignored `workspace/` directory and are
/// materialized on first use. This type owns the entire mapping — nothing else
/// in the codebase should construct a workspace path.
///
/// Three methods, three distinct jobs:
/// - [`Workspace::working_path`] — where the copy *would* live (pure)
/// - [`Workspace::source_path`] — what to read or display *right now* (pure)
/// - [`Workspace::ensure_materialized`] — create the copy if absent (writes)
///
/// The rule for callers: **display or read uses `source_path`; the student is
/// about to work on it uses `ensure_materialized`.**
pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// The workspace root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Where the student's copy lives: `<root>/<module_id>/<template file name>`.
    /// Pure — creates nothing.
    pub fn working_path(&self, exercise: &Exercise) -> PathBuf {
        self.root
            .join(&exercise.module_id)
            .join(template_file_name(exercise))
    }

    /// Has the student started this exercise?
    pub fn is_materialized(&self, exercise: &Exercise) -> bool {
        self.working_path(exercise).exists()
    }

    /// What to read or display right now: the working copy if it exists,
    /// otherwise the pristine template. Pure — creates nothing.
    ///
    /// The template fallback is what lets the exercise view, the AI context
    /// screen, and the MCQ code preview show the starter code before the
    /// student has opened an exercise.
    pub fn source_path(&self, exercise: &Exercise) -> PathBuf {
        let working = self.working_path(exercise);
        if working.exists() {
            working
        } else {
            exercise.file_path.clone()
        }
    }

    /// Return the working path, copying the template in if it is absent.
    ///
    /// Never overwrites an existing working copy — the student's edits are
    /// preserved across every call.
    pub fn ensure_materialized(&self, exercise: &Exercise) -> Result<PathBuf> {
        let working = self.working_path(exercise);
        if working.exists() {
            return Ok(working);
        }
        copy_template(exercise, &working)?;
        Ok(working)
    }

    /// Overwrite the working copy with the pristine template.
    ///
    /// Destructive: any edits the student made are lost. Callers must say so
    /// before invoking this.
    pub fn reset(&self, exercise: &Exercise) -> Result<PathBuf> {
        let working = self.working_path(exercise);
        // Keep one level of undo. The workspace is gitignored, so a sibling
        // `.bak` costs nothing and is the only recovery path a reset has.
        if working.exists() {
            let backup = working.with_extension("rs.bak");
            // Propagated, not ignored: the caller tells the student their work
            // was backed up, and reset is otherwise unrecoverable.
            fs::copy(&working, &backup).with_context(|| {
                format!("Failed to back up {} before resetting it", working.display())
            })?;
        }
        copy_template(exercise, &working)?;
        Ok(working)
    }
}

/// The template's file name, falling back to the exercise ID's last segment
/// when the template path has no final component.
fn template_file_name(exercise: &Exercise) -> PathBuf {
    exercise
        .file_path
        .file_name()
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let stem = exercise.id.rsplit('/').next().unwrap_or(&exercise.id);
            PathBuf::from(format!("{}.rs", stem))
        })
}

fn copy_template(exercise: &Exercise, dest: &Path) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!("Failed to create workspace directory: {}", parent.display())
        })?;
    }
    fs::copy(&exercise.file_path, dest).with_context(|| {
        format!(
            "Failed to copy exercise template {} into {}",
            exercise.file_path.display(),
            dest.display()
        )
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercise::types::*;

    const TEMPLATE: &str = "fn main() {\n    // TODO: fix me\n}\n";

    fn make_exercise(dir: &Path) -> Exercise {
        let module_dir = dir.join("exercises").join("01_getting_started");
        fs::create_dir_all(&module_dir).unwrap();
        let file_path = module_dir.join("hello_world.rs");
        fs::write(&file_path, TEMPLATE).unwrap();

        Exercise {
            id: "01_getting_started/hello_world".into(),
            name: "Hello, World!".into(),
            module_id: "01_getting_started".into(),
            exercise_type: ExerciseType::FixCompilerError,
            difficulty: Difficulty::Beginner,
            base_xp: 10,
            file_path,
            solution_path: module_dir.join("solutions").join("hello_world.rs"),
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
    fn test_materialize_creates_file_and_parent_dirs() {
        let tmp = tempfile::tempdir().unwrap();
        let ex = make_exercise(tmp.path());
        let ws = Workspace::new(tmp.path().join("workspace"));

        assert!(!ws.is_materialized(&ex));
        let path = ws.ensure_materialized(&ex).unwrap();

        assert!(path.exists(), "working copy should exist after materialize");
        assert!(ws.is_materialized(&ex));
        assert_eq!(fs::read_to_string(&path).unwrap(), TEMPLATE);
        assert_eq!(
            path,
            tmp.path()
                .join("workspace")
                .join("01_getting_started")
                .join("hello_world.rs")
        );
    }

    #[test]
    fn test_materialize_never_overwrites_student_edits() {
        let tmp = tempfile::tempdir().unwrap();
        let ex = make_exercise(tmp.path());
        let ws = Workspace::new(tmp.path().join("workspace"));

        let path = ws.ensure_materialized(&ex).unwrap();
        fs::write(&path, "fn main() { println!(\"my solution\"); }").unwrap();

        // Second call must leave the student's work alone.
        let again = ws.ensure_materialized(&ex).unwrap();
        assert_eq!(path, again);
        assert_eq!(
            fs::read_to_string(&again).unwrap(),
            "fn main() { println!(\"my solution\"); }"
        );
    }

    #[test]
    fn test_source_path_falls_back_to_template_then_tracks_working_copy() {
        let tmp = tempfile::tempdir().unwrap();
        let ex = make_exercise(tmp.path());
        let ws = Workspace::new(tmp.path().join("workspace"));

        // Before materialization: the pristine template.
        assert_eq!(ws.source_path(&ex), ex.file_path);

        // After: the student's copy.
        let working = ws.ensure_materialized(&ex).unwrap();
        assert_eq!(ws.source_path(&ex), working);
    }

    #[test]
    fn test_reset_restores_the_template_over_edits() {
        let tmp = tempfile::tempdir().unwrap();
        let ex = make_exercise(tmp.path());
        let ws = Workspace::new(tmp.path().join("workspace"));

        let path = ws.ensure_materialized(&ex).unwrap();
        fs::write(&path, "garbage that does not compile").unwrap();

        let restored = ws.reset(&ex).unwrap();
        assert_eq!(restored, path);
        assert_eq!(fs::read_to_string(&restored).unwrap(), TEMPLATE);
    }

    #[test]
    fn test_reset_materializes_when_not_yet_started() {
        let tmp = tempfile::tempdir().unwrap();
        let ex = make_exercise(tmp.path());
        let ws = Workspace::new(tmp.path().join("workspace"));

        assert!(!ws.is_materialized(&ex));
        let path = ws.reset(&ex).unwrap();
        assert!(path.exists());
        assert_eq!(fs::read_to_string(&path).unwrap(), TEMPLATE);
    }
}
