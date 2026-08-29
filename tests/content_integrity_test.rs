//! Content integrity checks over the real `exercises/` tree.
//!
//! These are fast — they only parse `info.toml` and stat files, no compilation.
//! They exist because the failure they guard against is *silent*: `TomlExercise`
//! and `TomlModule` both ignore unknown keys, so a `lesson` key that drifts out
//! of its module table binds to nothing, the module quietly loses its lesson,
//! and the content validation in `exercise::loader` never fires.

use std::collections::HashSet;
use std::path::PathBuf;

use rust_game::exercise::loader;

fn exercises_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("exercises")
}

/// Every lesson markdown file on disk must be claimed by some module.
///
/// An orphaned file is the visible symptom of a `lesson` key that has drifted
/// out of its `[[modules]]` table — the file still exists, but nothing points
/// at it, so the module silently shows no lesson.
#[test]
fn every_lesson_file_is_referenced_by_a_module() {
    let root = exercises_dir();
    let (modules, _) = loader::load_manifest(&root).expect("manifest should load");

    let referenced: HashSet<PathBuf> = modules
        .iter()
        .filter_map(|m| m.lesson.as_ref())
        .filter_map(|p| p.canonicalize().ok())
        .collect();

    let lessons_dir = root.join("lessons");
    if !lessons_dir.is_dir() {
        return; // No lessons authored yet — nothing to check.
    }

    let mut orphans = Vec::new();
    for entry in std::fs::read_dir(&lessons_dir).expect("lessons dir should be readable") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let canonical = path.canonicalize().expect("lesson path should canonicalize");
        if !referenced.contains(&canonical) {
            orphans.push(path.display().to_string());
        }
    }

    assert!(
        orphans.is_empty(),
        "These lesson files exist but no module references them. A `lesson` key has \
         most likely drifted out of its [[modules]] table in info.toml:\n  {}",
        orphans.join("\n  ")
    );
}

/// A module that declares a lesson must have all three lesson keys together.
///
/// Catches a partial drift — e.g. `lesson` still inside the table while
/// `book_url` and `concepts` slid out — which would leave the reader with no
/// Book citation and an empty "Covers:" line.
#[test]
fn lesson_metadata_keys_travel_together() {
    let (modules, _) = loader::load_manifest(&exercises_dir()).expect("manifest should load");

    let incomplete: Vec<String> = modules
        .iter()
        .filter(|m| m.lesson.is_some())
        .filter(|m| m.book_url.is_none() || m.concepts.is_empty())
        .map(|m| {
            format!(
                "{} (book_url: {}, concepts: {})",
                m.id,
                if m.book_url.is_some() { "yes" } else { "MISSING" },
                if m.concepts.is_empty() { "MISSING" } else { "yes" }
            )
        })
        .collect();

    assert!(
        incomplete.is_empty(),
        "Modules declaring a lesson but missing its companion metadata:\n  {}",
        incomplete.join("\n  ")
    );
}

/// The Lesson Protocol lives in three byte-identical files, one per agent.
/// Nothing enforced that until now, so a fix applied to one could silently
/// leave the other two teaching something different.
#[test]
fn agent_context_files_stay_byte_identical() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let claude = std::fs::read(root.join("CLAUDE.md")).expect("CLAUDE.md");
    let agents = std::fs::read(root.join("AGENTS.md")).expect("AGENTS.md");
    let gemini = std::fs::read(root.join("GEMINI.md")).expect("GEMINI.md");

    assert_eq!(claude, agents, "CLAUDE.md and AGENTS.md have drifted apart");
    assert_eq!(claude, gemini, "CLAUDE.md and GEMINI.md have drifted apart");
}
