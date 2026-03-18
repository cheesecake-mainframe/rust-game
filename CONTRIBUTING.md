# Contributing to rust-game

Thanks for your interest in improving rust-game! Here's how to contribute.

## Quick Setup

```bash
git clone https://github.com/cheesecakeMafia/rust-game.git
cd rust-game
cargo build
cargo test
```

All 83+ tests should pass. The 4 `#[ignore]` tests (solution verification + performance benchmarks) can be run with `cargo test -- --ignored`.

## Adding an Exercise

Each exercise consists of three things:

### 1. The exercise file

Create `exercises/<module>/<exercise_name>.rs`:

```rust
// ========================================
// Exercise: <Name>
// ========================================
// Type: <fix_compiler_error | debug_logic_bug | implement_from_scratch | ...>
// Difficulty: <beginner | intermediate | advanced | expert>
// Module: <NN> - <Module Name>
//
// <Concept explanation>
//
// YOUR TASK:
// <What the student needs to do>
// ========================================

// Exercise code here...
// For fix_compiler_error: include the broken code
// For implement_from_scratch: include TODO markers and tests
// For debug_logic_bug: include compiling code with a bug + failing tests

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    // Tests that validate the student's solution
}
```

### 2. The solution file

Create `exercises/<module>/solutions/<exercise_name>.rs` with the working solution. This file must pass the verification pipeline for its exercise type.

### 3. The info.toml entry

Add an `[[exercises]]` block to `exercises/info.toml`:

```toml
[[exercises]]
id = "<module>/<exercise_name>"
name = "Human Readable Name"
module = "<module_id>"
type = "<exercise_type>"
difficulty = "<difficulty>"
base_xp = 10                    # 10 fix, 15 debug, 20 implement, 25 optimization, 50 boss
file = "<module>/<exercise_name>.rs"
solution = "<module>/solutions/<exercise_name>.rs"
order = 4                       # Position within the module
description = "One-line description."
hints = [
    "First hint — gentle nudge.",
    "Second hint — more specific.",
]
```

**Optional fields:**
- `flavor_text = "Thematic text"` — displayed in the TUI
- `time_limit_secs = 120` — makes it a time trial (+15 XP bonus)
- `ci = true` — include in CI verification set (one per exercise type)
- `custom_checks = [{ check_type = "no_clone", message = "..." }]` — for optimization exercises
- `multiple_choice = [{ label = "A", text = "...", correct = true }, ...]` — for MCQ exercises

## Exercise Types

| Type | info.toml value | Verification | When to use |
|------|----------------|-------------|-------------|
| Fix Compiler Error | `fix_compiler_error` | `cargo build` | Teach syntax, types, ownership |
| Debug Logic Bug | `debug_logic_bug` | `cargo build` + `cargo test` | Teach common mistakes |
| Implement from Scratch | `implement_from_scratch` | `cargo build` + `cargo test` | Teach API design, problem solving |
| Code Transformation | `code_transformation` | `cargo build` + `cargo test` + `cargo clippy` | Python-to-Rust, refactoring |
| Boss Battle | `boss_battle` | `cargo build` + `cargo test` | Multi-concept challenges |
| Reverse Engineering (Fill-in) | `reverse_engineering_fill_in` | `cargo build` + `cargo test` | Predict output |
| Reverse Engineering (MCQ) | `reverse_engineering_multiple_choice` | TUI selection | Predict output (multiple choice) |
| Optimization | `optimization` | `cargo build` + `cargo test` + custom checks | Remove `.clone()`, allocations |

## Exercise Quality Checklist

Before submitting an exercise:

- [ ] **Solution passes the pipeline**: Run `cargo run -- verify <exercise_id>` with the solution file
- [ ] **Broken state fails clearly**: The exercise file (before solving) should produce a clear compiler error or test failure
- [ ] **Hints are progressive**: First hint is a gentle nudge, last hint is nearly the answer
- [ ] **Description is one line**: Save details for comments in the `.rs` file
- [ ] **XP matches the type**: 10 (fix), 15 (debug/RE), 20 (implement/transform), 25 (optimization), 50 (boss)
- [ ] **Tests cover edge cases**: Don't just test the happy path

## Running Tests

```bash
# Fast tests (unit + integration)
cargo test

# Solution verification (compiles all 65 solutions — slow)
cargo test -- --ignored test_all_solutions

# Performance benchmarks
cargo test -- --ignored performance

# Single test
cargo test test_name
```

## Code Structure

```
src/
  exercise/     Data layer — types, TOML loader, catalog
  game/         Game mechanics — XP, levels, streaks, progression
  state/        Persistence — save/load JSON state
  runner/       Verification — sandbox, cargo commands, pipeline
  tui/          Terminal UI — screens, widgets, event loop
  ai/           AI tutor context formatting
  app.rs        Top-level orchestration
  cli.rs        Clap CLI definitions
```

## Pull Request Guidelines

- One exercise per PR (unless they're tightly related)
- Include the exercise file, solution file, and info.toml entry
- Run `cargo test` before submitting
- Run the solution through the pipeline: `cargo run -- verify <id>`
- Update ATTRIBUTION.md if the exercise is adapted from another source
