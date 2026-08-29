use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{bail, Context, Result};
use serde::Deserialize;

use super::types::*;

// ─── TOML deserialization types (private) ────────────────────
// These map directly to the info.toml structure.
// They are converted to runtime types (Exercise, Module) after parsing.

#[derive(Deserialize)]
struct TomlManifest {
    #[serde(default)]
    modules: Vec<TomlModule>,
    #[serde(default)]
    exercises: Vec<TomlExercise>,
}

#[derive(Deserialize)]
struct TomlModule {
    id: String,
    name: String,
    theme_name: String,
    flavor_text: String,
    tier: Tier,
    order: u32,
    #[serde(default)]
    prerequisites: Vec<String>,
    #[serde(default)]
    lesson: Option<String>,
    #[serde(default)]
    book_url: Option<String>,
    #[serde(default)]
    concepts: Vec<String>,
}

#[derive(Deserialize)]
struct TomlExercise {
    id: String,
    name: String,
    module: String,
    #[serde(rename = "type")]
    exercise_type: ExerciseType,
    difficulty: Difficulty,
    base_xp: u32,
    file: String,
    solution: String,
    order: u32,
    description: String,
    #[serde(default)]
    flavor_text: Option<String>,
    #[serde(default)]
    hints: Vec<String>,
    #[serde(default)]
    time_limit_secs: Option<u32>,
    #[serde(default)]
    extra_files: Vec<String>,
    #[serde(default)]
    multiple_choice: Vec<MCOption>,
    #[serde(default)]
    custom_checks: Vec<CustomCheck>,
    #[serde(default)]
    ci: bool,
}

// ─── Loading ─────────────────────────────────────────────────

/// Load and validate exercises from info.toml.
/// `exercises_dir` is the path to the `exercises/` directory.
pub fn load_manifest(exercises_dir: &Path) -> Result<(Vec<Module>, Vec<Exercise>)> {
    let toml_path = exercises_dir.join("info.toml");
    let content = std::fs::read_to_string(&toml_path)
        .with_context(|| format!("Failed to read {}", toml_path.display()))?;

    let manifest: TomlManifest = toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", toml_path.display()))?;

    let modules = convert_modules(manifest.modules, exercises_dir);
    let exercises = convert_exercises(manifest.exercises, exercises_dir);

    validate(&modules, &exercises)?;

    Ok((modules, exercises))
}

fn convert_modules(raw: Vec<TomlModule>, exercises_dir: &Path) -> Vec<Module> {
    raw.into_iter()
        .map(|m| Module {
            id: m.id,
            name: m.name,
            theme_name: m.theme_name,
            flavor_text: m.flavor_text,
            tier: m.tier,
            order: m.order,
            prerequisites: m.prerequisites,
            lesson: m.lesson.map(|l| exercises_dir.join(l)),
            book_url: m.book_url,
            concepts: m.concepts,
        })
        .collect()
}

fn convert_exercises(raw: Vec<TomlExercise>, exercises_dir: &Path) -> Vec<Exercise> {
    raw.into_iter()
        .map(|e| Exercise {
            id: e.id,
            name: e.name,
            module_id: e.module,
            exercise_type: e.exercise_type,
            difficulty: e.difficulty,
            base_xp: e.base_xp,
            file_path: exercises_dir.join(&e.file),
            solution_path: exercises_dir.join(&e.solution),
            hints: e.hints,
            description: e.description,
            flavor_text: e.flavor_text,
            time_limit_secs: e.time_limit_secs,
            custom_checks: e.custom_checks,
            multiple_choice_options: e.multiple_choice,
            extra_files: e.extra_files.iter().map(|f| exercises_dir.join(f)).collect(),
            order: e.order,
            ci: e.ci,
        })
        .collect()
}

// ─── Validation ──────────────────────────────────────────────

fn validate(modules: &[Module], exercises: &[Exercise]) -> Result<()> {
    let mut errors: Vec<String> = Vec::new();

    // Collect module IDs
    let module_ids: HashSet<&str> = modules.iter().map(|m| m.id.as_str()).collect();

    // Check for duplicate module IDs
    let mut seen_module_ids: HashSet<&str> = HashSet::new();
    for m in modules {
        if !seen_module_ids.insert(&m.id) {
            errors.push(format!("Duplicate module ID: '{}'", m.id));
        }
    }

    // Check for duplicate exercise IDs
    let mut seen_exercise_ids: HashSet<&str> = HashSet::new();
    for e in exercises {
        if !seen_exercise_ids.insert(&e.id) {
            errors.push(format!("Duplicate exercise ID: '{}'", e.id));
        }
    }

    // Check exercise module references
    for e in exercises {
        if !module_ids.contains(e.module_id.as_str()) {
            errors.push(format!(
                "Exercise '{}' references unknown module '{}'",
                e.id, e.module_id
            ));
        }
    }

    // Check prerequisite references
    for m in modules {
        for prereq in &m.prerequisites {
            if !module_ids.contains(prereq.as_str()) {
                errors.push(format!(
                    "Module '{}' has unknown prerequisite '{}'",
                    m.id, prereq
                ));
            }
        }
    }

    // Check for prerequisite cycles (topological sort)
    if let Err(cycle) = check_prerequisite_cycles(modules) {
        errors.push(format!("Prerequisite cycle detected: {}", cycle));
    }

    // Check for order collisions within modules
    let mut orders_by_module: HashMap<&str, HashSet<u32>> = HashMap::new();
    for e in exercises {
        let orders = orders_by_module.entry(&e.module_id).or_default();
        if !orders.insert(e.order) {
            errors.push(format!(
                "Duplicate order {} in module '{}' (exercise '{}')",
                e.order, e.module_id, e.id
            ));
        }
    }

    // Check declared lesson files exist. A module with no `lesson` key is fine;
    // one that names a missing file is a content bug worth failing loudly on.
    for m in modules {
        if let Some(lesson) = &m.lesson {
            if !lesson.exists() {
                errors.push(format!(
                    "Module '{}': lesson file not found: {}",
                    m.id,
                    lesson.display()
                ));
            }
        }
    }

    // Check exercise file paths exist
    for e in exercises {
        if !e.file_path.exists() {
            errors.push(format!(
                "Exercise '{}': file not found: {}",
                e.id,
                e.file_path.display()
            ));
        }
        if !e.solution_path.exists() {
            errors.push(format!(
                "Exercise '{}': solution not found: {}",
                e.id,
                e.solution_path.display()
            ));
        }
        for extra in &e.extra_files {
            if !extra.exists() {
                errors.push(format!(
                    "Exercise '{}': extra file not found: {}",
                    e.id,
                    extra.display()
                ));
            }
        }
    }

    // Check MCQ exercises have valid options
    for e in exercises {
        if e.exercise_type == ExerciseType::ReverseEngineeringMultipleChoice {
            if e.multiple_choice_options.is_empty() {
                errors.push(format!(
                    "Exercise '{}' is MCQ type but has no multiple_choice options",
                    e.id
                ));
            } else {
                let correct_count = e
                    .multiple_choice_options
                    .iter()
                    .filter(|o| o.correct)
                    .count();
                if correct_count != 1 {
                    errors.push(format!(
                        "Exercise '{}': MCQ must have exactly 1 correct answer, found {}",
                        e.id, correct_count
                    ));
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        bail!(
            "Content validation failed with {} error(s):\n  - {}",
            errors.len(),
            errors.join("\n  - ")
        );
    }
}

/// Check for prerequisite cycles using topological sort (Kahn's algorithm).
/// Returns Err with the cycle description if a cycle is found.
fn check_prerequisite_cycles(modules: &[Module]) -> std::result::Result<(), String> {
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();

    for m in modules {
        in_degree.entry(&m.id).or_insert(0);
        adjacency.entry(&m.id).or_default();
        for prereq in &m.prerequisites {
            adjacency.entry(prereq.as_str()).or_default().push(&m.id);
            *in_degree.entry(&m.id).or_insert(0) += 1;
        }
    }

    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&id, _)| id)
        .collect();

    let mut visited = 0;

    while let Some(node) = queue.pop() {
        visited += 1;
        if let Some(neighbors) = adjacency.get(node) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(neighbor);
                    }
                }
            }
        }
    }

    if visited == modules.len() {
        Ok(())
    } else {
        let in_cycle: Vec<&str> = in_degree
            .iter()
            .filter(|(_, &deg)| deg > 0)
            .map(|(&id, _)| id)
            .collect();
        Err(format!(
            "modules involved in cycle: {}",
            in_cycle.join(", ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_manifest(dir: &Path) {
        let exercises_dir = dir.join("exercises");
        fs::create_dir_all(exercises_dir.join("01_test/solutions")).unwrap();

        // Create exercise files
        fs::write(
            exercises_dir.join("01_test/hello.rs"),
            "fn main() { println!(\"hello\"); }",
        )
        .unwrap();
        fs::write(
            exercises_dir.join("01_test/solutions/hello.rs"),
            "fn main() { println!(\"Hello, world!\"); }",
        )
        .unwrap();

        let toml = r#"
[[modules]]
id = "01_test"
name = "Test Module"
theme_name = "The Test"
flavor_text = "A test module."
tier = "foundation"
order = 1
prerequisites = []

[[exercises]]
id = "01_test/hello"
name = "Hello"
module = "01_test"
type = "fix_compiler_error"
difficulty = "beginner"
base_xp = 10
file = "01_test/hello.rs"
solution = "01_test/solutions/hello.rs"
order = 1
description = "Fix hello world."
hints = ["Check the println! macro."]
"#;
        fs::write(exercises_dir.join("info.toml"), toml).unwrap();
    }

    #[test]
    fn test_load_valid_manifest() {
        let tmp = tempfile::tempdir().unwrap();
        create_test_manifest(tmp.path());
        let (modules, exercises) =
            load_manifest(&tmp.path().join("exercises")).unwrap();
        assert_eq!(modules.len(), 1);
        assert_eq!(exercises.len(), 1);
        assert_eq!(exercises[0].id, "01_test/hello");
        assert_eq!(exercises[0].exercise_type, ExerciseType::FixCompilerError);
        assert_eq!(exercises[0].hints.len(), 1);
    }

    #[test]
    fn test_duplicate_module_id() {
        let tmp = tempfile::tempdir().unwrap();
        let exercises_dir = tmp.path().join("exercises");
        fs::create_dir_all(&exercises_dir).unwrap();

        let toml = r#"
[[modules]]
id = "mod1"
name = "M1"
theme_name = "T"
flavor_text = "F"
tier = "foundation"
order = 1
prerequisites = []

[[modules]]
id = "mod1"
name = "M1 Dup"
theme_name = "T"
flavor_text = "F"
tier = "foundation"
order = 2
prerequisites = []
"#;
        fs::write(exercises_dir.join("info.toml"), toml).unwrap();
        let err = load_manifest(&exercises_dir).unwrap_err();
        assert!(
            err.to_string().contains("Duplicate module ID"),
            "Expected duplicate module error, got: {}",
            err
        );
    }

    #[test]
    fn test_unknown_prerequisite() {
        let tmp = tempfile::tempdir().unwrap();
        let exercises_dir = tmp.path().join("exercises");
        fs::create_dir_all(&exercises_dir).unwrap();

        let toml = r#"
[[modules]]
id = "mod1"
name = "M1"
theme_name = "T"
flavor_text = "F"
tier = "foundation"
order = 1
prerequisites = ["nonexistent"]
"#;
        fs::write(exercises_dir.join("info.toml"), toml).unwrap();
        let err = load_manifest(&exercises_dir).unwrap_err();
        assert!(
            err.to_string().contains("unknown prerequisite"),
            "Expected unknown prerequisite error, got: {}",
            err
        );
    }

    #[test]
    fn test_prerequisite_cycle() {
        let tmp = tempfile::tempdir().unwrap();
        let exercises_dir = tmp.path().join("exercises");
        fs::create_dir_all(&exercises_dir).unwrap();

        let toml = r#"
[[modules]]
id = "a"
name = "A"
theme_name = "T"
flavor_text = "F"
tier = "foundation"
order = 1
prerequisites = ["c"]

[[modules]]
id = "b"
name = "B"
theme_name = "T"
flavor_text = "F"
tier = "foundation"
order = 2
prerequisites = ["a"]

[[modules]]
id = "c"
name = "C"
theme_name = "T"
flavor_text = "F"
tier = "foundation"
order = 3
prerequisites = ["b"]
"#;
        fs::write(exercises_dir.join("info.toml"), toml).unwrap();
        let err = load_manifest(&exercises_dir).unwrap_err();
        assert!(
            err.to_string().contains("cycle"),
            "Expected cycle error, got: {}",
            err
        );
    }

    #[test]
    fn test_invalid_exercise_type_gives_clear_error() {
        let tmp = tempfile::tempdir().unwrap();
        let exercises_dir = tmp.path().join("exercises");
        fs::create_dir_all(&exercises_dir).unwrap();

        let toml = r#"
[[modules]]
id = "mod1"
name = "M1"
theme_name = "T"
flavor_text = "F"
tier = "foundation"
order = 1
prerequisites = []

[[exercises]]
id = "mod1/ex1"
name = "Ex1"
module = "mod1"
type = "fix_compier_error"
difficulty = "beginner"
base_xp = 10
file = "mod1/ex1.rs"
solution = "mod1/solutions/ex1.rs"
order = 1
description = "Test"
"#;
        fs::write(exercises_dir.join("info.toml"), toml).unwrap();
        let err = load_manifest(&exercises_dir).unwrap_err();
        // anyhow chains errors — check the full chain
        let msg = format!("{:#}", err);
        assert!(
            msg.contains("unknown variant") || msg.contains("Failed to parse"),
            "Expected clear parse error for typo, got: {}",
            msg
        );
    }
}
