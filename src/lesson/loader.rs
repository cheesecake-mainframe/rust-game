use std::fs;

use anyhow::{Context, Result};

use super::types::Lesson;
use crate::exercise::types::Module;

/// Load a module's lesson, if it has one.
///
/// Returns `Ok(None)` when the module declares no lesson, or declares one whose
/// file is missing. Absence is a supported state, not an error: the TUI says so
/// and an AI tutor falls back to teaching from the Book chapter plus the
/// exercise files. Content validation in `exercise::loader` is what catches a
/// declared-but-missing lesson at load time.
pub fn load(module: &Module) -> Result<Option<Lesson>> {
    let path = match &module.lesson {
        Some(p) => p,
        None => return Ok(None),
    };

    if !path.exists() {
        return Ok(None);
    }

    let body = fs::read_to_string(path)
        .with_context(|| format!("Failed to read lesson: {}", path.display()))?;

    let title = extract_title(&body).unwrap_or_else(|| module.theme_name.clone());

    Ok(Some(Lesson {
        module_id: module.id.clone(),
        title,
        body,
        book_url: module.book_url.clone(),
        concepts: module.concepts.clone(),
    }))
}

/// Pull the first `# ` heading out of the markdown body.
fn extract_title(body: &str) -> Option<String> {
    body.lines()
        .find_map(|line| line.strip_prefix("# "))
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercise::types::Tier;
    use std::path::PathBuf;

    fn make_module(lesson: Option<PathBuf>) -> Module {
        Module {
            id: "04_ownership_moves".into(),
            name: "Ownership & Moves".into(),
            theme_name: "The Ownership Trials".into(),
            flavor_text: "flavor".into(),
            tier: Tier::Foundation,
            order: 4,
            prerequisites: vec![],
            lesson,
            book_url: Some("https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html".into()),
            concepts: vec!["move semantics".into(), "Copy".into()],
        }
    }

    #[test]
    fn test_loads_lesson_and_extracts_title() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("04_ownership_moves.md");
        fs::write(&path, "# The Ownership Trials\n\nSome prose here.\n").unwrap();

        let module = make_module(Some(path));
        let lesson = load(&module).unwrap().expect("lesson should load");

        assert_eq!(lesson.title, "The Ownership Trials");
        assert_eq!(lesson.module_id, "04_ownership_moves");
        assert!(lesson.body.contains("Some prose here."));
        assert_eq!(lesson.concepts_line(), "move semantics, Copy");
    }

    #[test]
    fn test_title_falls_back_to_theme_name() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("no_heading.md");
        fs::write(&path, "Just prose, no heading at all.\n").unwrap();

        let module = make_module(Some(path));
        let lesson = load(&module).unwrap().unwrap();

        assert_eq!(lesson.title, "The Ownership Trials");
    }

    #[test]
    fn test_module_without_lesson_returns_none() {
        let module = make_module(None);
        assert!(load(&module).unwrap().is_none());
    }

    #[test]
    fn test_declared_but_missing_file_returns_none() {
        let module = make_module(Some(PathBuf::from("/nonexistent/lesson.md")));
        assert!(load(&module).unwrap().is_none());
    }
}
