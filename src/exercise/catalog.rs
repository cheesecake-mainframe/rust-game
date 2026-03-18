use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;

use super::loader;
use super::types::*;

/// The exercise catalog: holds all modules and exercises, provides
/// querying, ordering, and module-exercise grouping.
pub struct ExerciseCatalog {
    modules: Vec<Module>,
    exercises: Vec<Exercise>,
    /// Module ID → sorted exercise IDs
    exercises_by_module: HashMap<String, Vec<String>>,
}

impl ExerciseCatalog {
    /// Load the catalog from the exercises directory.
    pub fn load(exercises_dir: &Path) -> Result<Self> {
        let (modules, mut exercises) = loader::load_manifest(exercises_dir)?;

        // Sort exercises by module order, then exercise order
        let module_order: HashMap<&str, u32> =
            modules.iter().map(|m| (m.id.as_str(), m.order)).collect();

        exercises.sort_by(|a, b| {
            let ma = module_order.get(a.module_id.as_str()).unwrap_or(&0);
            let mb = module_order.get(b.module_id.as_str()).unwrap_or(&0);
            ma.cmp(mb).then(a.order.cmp(&b.order))
        });

        // Build module → exercises index
        let mut exercises_by_module: HashMap<String, Vec<String>> = HashMap::new();
        for e in &exercises {
            exercises_by_module
                .entry(e.module_id.clone())
                .or_default()
                .push(e.id.clone());
        }

        Ok(Self {
            modules,
            exercises,
            exercises_by_module,
        })
    }

    pub fn modules(&self) -> &[Module] {
        &self.modules
    }

    pub fn exercises(&self) -> &[Exercise] {
        &self.exercises
    }

    pub fn total_exercises(&self) -> usize {
        self.exercises.len()
    }

    /// Get exercises for a specific module, sorted by order.
    pub fn exercises_for_module(&self, module_id: &str) -> Vec<&Exercise> {
        self.exercises_by_module
            .get(module_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.get_exercise(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get a single exercise by ID.
    pub fn get_exercise(&self, id: &str) -> Option<&Exercise> {
        self.exercises.iter().find(|e| e.id == id)
    }

    /// Get a single module by ID.
    pub fn get_module(&self, id: &str) -> Option<&Module> {
        self.modules.iter().find(|m| m.id == id)
    }

    /// Get exercises tagged for CI verification.
    pub fn ci_exercises(&self) -> Vec<&Exercise> {
        self.exercises.iter().filter(|e| e.ci).collect()
    }

    /// Get the next exercise after the given one (by global order).
    pub fn next_exercise_after(&self, current_id: &str) -> Option<&Exercise> {
        let pos = self.exercises.iter().position(|e| e.id == current_id)?;
        self.exercises.get(pos + 1)
    }

    /// Get the first exercise in the catalog.
    pub fn first_exercise(&self) -> Option<&Exercise> {
        self.exercises.first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_test_catalog(dir: &Path) {
        let exercises_dir = dir.join("exercises");
        fs::create_dir_all(exercises_dir.join("01_mod/solutions")).unwrap();
        fs::create_dir_all(exercises_dir.join("02_mod/solutions")).unwrap();

        fs::write(exercises_dir.join("01_mod/ex1.rs"), "fn main() {}").unwrap();
        fs::write(exercises_dir.join("01_mod/solutions/ex1.rs"), "fn main() {}").unwrap();
        fs::write(exercises_dir.join("01_mod/ex2.rs"), "fn main() {}").unwrap();
        fs::write(exercises_dir.join("01_mod/solutions/ex2.rs"), "fn main() {}").unwrap();
        fs::write(exercises_dir.join("02_mod/ex1.rs"), "fn main() {}").unwrap();
        fs::write(exercises_dir.join("02_mod/solutions/ex1.rs"), "fn main() {}").unwrap();

        let toml = r#"
[[modules]]
id = "01_mod"
name = "Module 1"
theme_name = "Theme 1"
flavor_text = "Flavor 1"
tier = "foundation"
order = 1
prerequisites = []

[[modules]]
id = "02_mod"
name = "Module 2"
theme_name = "Theme 2"
flavor_text = "Flavor 2"
tier = "foundation"
order = 2
prerequisites = ["01_mod"]

[[exercises]]
id = "01_mod/ex1"
name = "Exercise 1"
module = "01_mod"
type = "fix_compiler_error"
difficulty = "beginner"
base_xp = 10
file = "01_mod/ex1.rs"
solution = "01_mod/solutions/ex1.rs"
order = 1
description = "First exercise"
ci = true

[[exercises]]
id = "01_mod/ex2"
name = "Exercise 2"
module = "01_mod"
type = "debug_logic_bug"
difficulty = "beginner"
base_xp = 15
file = "01_mod/ex2.rs"
solution = "01_mod/solutions/ex2.rs"
order = 2
description = "Second exercise"

[[exercises]]
id = "02_mod/ex1"
name = "Exercise 3"
module = "02_mod"
type = "implement_from_scratch"
difficulty = "intermediate"
base_xp = 20
file = "02_mod/ex1.rs"
solution = "02_mod/solutions/ex1.rs"
order = 1
description = "Third exercise"
"#;
        fs::write(exercises_dir.join("info.toml"), toml).unwrap();
    }

    #[test]
    fn test_catalog_loads_and_sorts() {
        let tmp = tempfile::tempdir().unwrap();
        setup_test_catalog(tmp.path());
        let catalog = ExerciseCatalog::load(&tmp.path().join("exercises")).unwrap();

        assert_eq!(catalog.modules().len(), 2);
        assert_eq!(catalog.total_exercises(), 3);

        // Exercises should be sorted: module 1 first, then module 2
        let ids: Vec<&str> = catalog.exercises().iter().map(|e| e.id.as_str()).collect();
        assert_eq!(ids, vec!["01_mod/ex1", "01_mod/ex2", "02_mod/ex1"]);
    }

    #[test]
    fn test_exercises_for_module() {
        let tmp = tempfile::tempdir().unwrap();
        setup_test_catalog(tmp.path());
        let catalog = ExerciseCatalog::load(&tmp.path().join("exercises")).unwrap();

        let mod1_exercises = catalog.exercises_for_module("01_mod");
        assert_eq!(mod1_exercises.len(), 2);
        assert_eq!(mod1_exercises[0].id, "01_mod/ex1");
        assert_eq!(mod1_exercises[1].id, "01_mod/ex2");

        let mod2_exercises = catalog.exercises_for_module("02_mod");
        assert_eq!(mod2_exercises.len(), 1);
    }

    #[test]
    fn test_get_exercise_and_module() {
        let tmp = tempfile::tempdir().unwrap();
        setup_test_catalog(tmp.path());
        let catalog = ExerciseCatalog::load(&tmp.path().join("exercises")).unwrap();

        assert!(catalog.get_exercise("01_mod/ex1").is_some());
        assert!(catalog.get_exercise("nonexistent").is_none());
        assert!(catalog.get_module("01_mod").is_some());
        assert!(catalog.get_module("nonexistent").is_none());
    }

    #[test]
    fn test_next_exercise() {
        let tmp = tempfile::tempdir().unwrap();
        setup_test_catalog(tmp.path());
        let catalog = ExerciseCatalog::load(&tmp.path().join("exercises")).unwrap();

        let next = catalog.next_exercise_after("01_mod/ex1").unwrap();
        assert_eq!(next.id, "01_mod/ex2");

        let next = catalog.next_exercise_after("01_mod/ex2").unwrap();
        assert_eq!(next.id, "02_mod/ex1");

        assert!(catalog.next_exercise_after("02_mod/ex1").is_none());
    }

    #[test]
    fn test_ci_exercises() {
        let tmp = tempfile::tempdir().unwrap();
        setup_test_catalog(tmp.path());
        let catalog = ExerciseCatalog::load(&tmp.path().join("exercises")).unwrap();

        let ci = catalog.ci_exercises();
        assert_eq!(ci.len(), 1);
        assert_eq!(ci[0].id, "01_mod/ex1");
    }
}
