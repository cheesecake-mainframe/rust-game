use std::collections::HashMap;

use crate::exercise::types::{ExerciseStatus, Module};

/// Check which modules are unlocked based on completion state.
///
/// A module is unlocked if ALL of its prerequisites are completed.
/// Modules with no prerequisites are always unlocked.
pub fn compute_unlocked_modules(
    modules: &[Module],
    module_completion: &HashMap<String, bool>,
) -> HashMap<String, bool> {
    let mut unlocked = HashMap::new();

    for module in modules {
        if module.prerequisites.is_empty() {
            unlocked.insert(module.id.clone(), true);
            continue;
        }

        let all_prereqs_complete = module.prerequisites.iter().all(|prereq_id| {
            module_completion.get(prereq_id).copied().unwrap_or(false)
        });

        unlocked.insert(module.id.clone(), all_prereqs_complete);
    }

    unlocked
}

/// Check if a module is complete (all exercises in it are completed).
pub fn is_module_complete(
    _module_id: &str,
    exercise_ids: &[String],
    exercise_statuses: &HashMap<String, ExerciseStatus>,
) -> bool {
    if exercise_ids.is_empty() {
        return false;
    }
    exercise_ids.iter().all(|id| {
        exercise_statuses
            .get(id)
            .map(|s| *s == ExerciseStatus::Completed)
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercise::types::Tier;

    fn make_module(id: &str, prereqs: Vec<&str>) -> Module {
        Module {
            id: id.to_string(),
            name: id.to_string(),
            theme_name: "Theme".into(),
            flavor_text: "Flavor".into(),
            tier: Tier::Foundation,
            order: 1,
            prerequisites: prereqs.into_iter().map(String::from).collect(),
            lesson: None,
            book_url: None,
            concepts: vec![],
        }
    }

    #[test]
    fn test_no_prerequisites_always_unlocked() {
        let modules = vec![make_module("mod1", vec![])];
        let completion = HashMap::new();
        let unlocked = compute_unlocked_modules(&modules, &completion);
        assert_eq!(unlocked.get("mod1"), Some(&true));
    }

    #[test]
    fn test_prerequisite_not_met() {
        let modules = vec![
            make_module("mod1", vec![]),
            make_module("mod2", vec!["mod1"]),
        ];
        let completion = HashMap::from([("mod1".to_string(), false)]);
        let unlocked = compute_unlocked_modules(&modules, &completion);
        assert_eq!(unlocked.get("mod2"), Some(&false));
    }

    #[test]
    fn test_prerequisite_met() {
        let modules = vec![
            make_module("mod1", vec![]),
            make_module("mod2", vec!["mod1"]),
        ];
        let completion = HashMap::from([("mod1".to_string(), true)]);
        let unlocked = compute_unlocked_modules(&modules, &completion);
        assert_eq!(unlocked.get("mod2"), Some(&true));
    }

    #[test]
    fn test_multiple_prerequisites_all_needed() {
        let modules = vec![
            make_module("a", vec![]),
            make_module("b", vec![]),
            make_module("c", vec!["a", "b"]),
        ];
        // Only 'a' is complete
        let completion = HashMap::from([
            ("a".to_string(), true),
            ("b".to_string(), false),
        ]);
        let unlocked = compute_unlocked_modules(&modules, &completion);
        assert_eq!(unlocked.get("c"), Some(&false));

        // Now both complete
        let completion = HashMap::from([
            ("a".to_string(), true),
            ("b".to_string(), true),
        ]);
        let unlocked = compute_unlocked_modules(&modules, &completion);
        assert_eq!(unlocked.get("c"), Some(&true));
    }

    #[test]
    fn test_linear_chain() {
        let modules = vec![
            make_module("01", vec![]),
            make_module("02", vec!["01"]),
            make_module("03", vec!["02"]),
        ];
        let completion = HashMap::from([
            ("01".to_string(), true),
            ("02".to_string(), false),
        ]);
        let unlocked = compute_unlocked_modules(&modules, &completion);
        assert_eq!(unlocked.get("01"), Some(&true));
        assert_eq!(unlocked.get("02"), Some(&true)); // prereq 01 is done
        assert_eq!(unlocked.get("03"), Some(&false)); // prereq 02 is not done
    }

    #[test]
    fn test_module_complete() {
        let exercises = vec!["ex1".to_string(), "ex2".to_string()];
        let statuses = HashMap::from([
            ("ex1".to_string(), ExerciseStatus::Completed),
            ("ex2".to_string(), ExerciseStatus::Completed),
        ]);
        assert!(is_module_complete("mod", &exercises, &statuses));
    }

    #[test]
    fn test_module_incomplete() {
        let exercises = vec!["ex1".to_string(), "ex2".to_string()];
        let statuses = HashMap::from([
            ("ex1".to_string(), ExerciseStatus::Completed),
            ("ex2".to_string(), ExerciseStatus::InProgress),
        ]);
        assert!(!is_module_complete("mod", &exercises, &statuses));
    }

    #[test]
    fn test_empty_module_not_complete() {
        let exercises: Vec<String> = vec![];
        let statuses = HashMap::new();
        assert!(!is_module_complete("mod", &exercises, &statuses));
    }
}
