use crate::exercise::catalog::ExerciseCatalog;
use crate::exercise::types::Exercise;
use crate::runner::pipeline::VerificationResult;
use crate::state::game_state::GameState;

/// Format exercise context for pasting into an AI chat.
///
/// Produces a self-contained block with all the information an AI tutor
/// needs to help the student without seeing the full codebase.
pub fn format_ai_context(
    exercise: &Exercise,
    exercise_source: &str,
    verification_result: Option<&VerificationResult>,
    catalog: &ExerciseCatalog,
    state: &GameState,
) -> String {
    let module = catalog.get_module(&exercise.module_id);
    let module_name = module.map(|m| m.name.as_str()).unwrap_or("Unknown");
    let module_theme = module.map(|m| m.theme_name.as_str()).unwrap_or("");
    let module_concepts = module
        .map(|m| m.concepts.join(", "))
        .filter(|c| !c.is_empty())
        .unwrap_or_else(|| "(not listed)".to_string());

    let output_section = match verification_result {
        Some(result) => {
            let status = if result.passed() { "PASSED" } else { "FAILED" };
            let details = result
                .steps
                .iter()
                .map(|s| {
                    let icon = if s.success { "✓" } else { "✗" };
                    format!("{} {}: {}", icon, s.step_name, s.output.trim())
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!("Status: {}\n{}", status, details)
        }
        None => "No verification run yet.".to_string(),
    };

    let hints_section = if exercise.hints.is_empty() {
        "No hints available.".to_string()
    } else {
        exercise
            .hints
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{}. {}", i + 1, h))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        r#"## rust-game /hint-ai Context

**Exercise:** {} (`{}`)
**Type:** {:?}
**Difficulty:** {:?}
**Module:** {} — {}
**Module concepts:** {}

### Description
{}

### Current Exercise Code
```rust
{}
```

### Compiler/Test Output
```
{}
```

### Hints
{}

### Student Progress
Level {} | {}/{} exercises completed
"#,
        exercise.name,
        exercise.id,
        exercise.exercise_type,
        exercise.difficulty,
        module_name,
        module_theme,
        module_concepts,
        exercise.description,
        exercise_source,
        output_section,
        hints_section,
        state.player.level,
        state.exercises_completed(),
        catalog.total_exercises(),
    )
}
