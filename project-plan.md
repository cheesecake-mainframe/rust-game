# rust-game: Gamified Rust Learning CLI — Implementation Plan

## Context

**Problem:** Learning Rust is challenging for developers coming from Python/JS/TS due to unique concepts like ownership, borrowing, lifetimes, and the borrow checker. Existing tools (Rustlings) teach well but lack gamification. We want to build a tool where exercises ARE the game.

**Inspiration:** The [system-design-game](/home/cheesecake/Downloads/programming/system-design-game/) project demonstrates how to create a gamified learning experience with thematic naming, progressive difficulty, AI tutor integration, and external dependencies cloned via setup scripts.

**Outcome:** A Rust CLI tool with rich TUI that teaches Rust through a staged curriculum. The long-term plan is ~60 gamified exercises (expandable to ~100) across 18 modules, with XP scoring, leveling, streak combos, time trials, boss battles, and 7 distinct exercise types. The first release should prove the loop with a smaller slice.

---

## Design Decisions (from discussion)

| Decision | Choice |
|---|---|
| Format | Rust CLI tool with watch mode (like Rustlings but gamified) |
| Game element | Exercises ARE the game — scoring, time trials, boss battles |
| Scope | Long-term: 18 modules, ~60 exercises (expandable to ~100). V1 ships a smaller slice first. |
| AI tutor | Optional — `/hint-ai` command formats context for AI chat |
| Scoring | XP + leveling + streak/combo system |
| Exercise types | 7 types: fix compiler, debug bug, implement, code transform, boss battle, reverse engineering, optimization. Time trials are a modifier (any exercise with `time_limit_secs`) |
| Theme | Light — thematic module names and flavor text |
| Technical | Written in Rust, ratatui TUI, offline-first, no leaderboards |
| Content | Mix of adapted (Rust by Example, Rustlings-inspired) + original |
| Progression | Semi-open — Foundation linear, then flexible within tiers |
| Hints | Completely free, no penalty |
| Name | rust-game |

---

## MVP / Release Scope

### V1 Goal
Ship a **small but complete learning loop**, not the full curriculum. V1 should prove that the combination of authored Rust exercises, fast verification, watch mode, progression, and a usable terminal interface is compelling before the project invests in the full 18-module roadmap.

### V1 In Scope
- Foundation-only curriculum: modules 01-05
- ~15-20 exercises total, including at least:
  - fix compiler error
  - debug logic bug
  - implement from scratch
  - one boss battle
- CLI commands required to complete the loop: `list`, `verify`, `hint`, `stats`, `next`, `reset`
- Minimal TUI: dashboard, module view, exercise view, watch mode, stats
- XP, levels, streaks, unlock logic, local state persistence
- Persistent sandbox compilation cache and watch-mode verification
- `info.toml` loading and content validation tooling

### V1 Explicitly Out of Scope
- Modules 06-18
- Async exercises and per-exercise external crate dependencies beyond what is needed to prove the sandbox model
- Multiple choice reverse-engineering screen
- AI context screen inside the TUI
- Full ~60 exercise catalog
- Broad polish work that does not improve the core verify-learn-progress loop

### V1 Implementation Boundaries
- **ExerciseType enum:** All 7 variants are defined in the enum (for forward-compatible serialization), but V1 pipeline paths only implement `FixCompilerError`, `DebugLogicBug`, `ImplementFromScratch`, and `BossBattle`. Other variants return a clear "not yet supported" error if encountered.
- **Prerequisite graph:** V1 modules are strictly linear (01→02→03→04→05), so the unlock logic is a simple "are all prereqs completed?" check. Full graph-based cycle detection is deferred to content validation (post-V1), not runtime.

### Post-V1
- Expand the catalog from ~15-20 exercises to ~60
- Add the remaining exercise types where they materially improve the learning loop
- Add modules 06-18 in tier order
- Add optional AI context export in both CLI and TUI once the core loop is stable

### Platform Support Gate
The project still targets Linux, macOS, and Windows. However, **cross-platform support is gated by an early platform spike**:
- If process cleanup, file watching, and user-data persistence work on all three platforms, keep the broad target.
- If Windows-specific behavior is materially less reliable, V1 narrows to Linux + macOS and Windows becomes a follow-up milestone with explicit unblockers.

---

## Project Directory Structure

```
rust-game/
├── .gitignore
├── Cargo.toml / Cargo.lock
├── Makefile
├── setup.sh / setup.ps1
├── CLAUDE.md / AGENTS.md / GEMINI.md    # AI tutor context (identical)
├── LICENSE / ATTRIBUTION.md
│
├── src/
│   ├── main.rs                           # Entry point + clap dispatch
│   ├── app.rs                            # App state machine (AppMode enum)
│   ├── cli.rs                            # Clap subcommands
│   │
│   ├── exercise/
│   │   ├── types.rs                      # Exercise, ExerciseType, Module, Tier, etc.
│   │   ├── loader.rs                     # Parse info.toml
│   │   └── catalog.rs                    # Ordering, filtering, module grouping
│   │
│   ├── runner/
│   │   ├── sandbox.rs                    # Persistent sandbox per exercise (incremental compilation)
│   │   ├── cargo_command.rs              # Shared cargo invocation: spawn, capture output, timeout, process group kill
│   │   ├── custom_checks.rs             # Source-level checks (no_clone, etc.)
│   │   └── pipeline.rs                   # Orchestrate verification per exercise type
│   │
│   ├── watcher/
│   │   └── file_watcher.rs              # notify + debouncer (300ms)
│   │
│   ├── game/
│   │   ├── xp.rs                         # XP calculation (base + bonuses + streak)
│   │   ├── level.rs                      # Level thresholds: 25*(level-1)^1.8
│   │   ├── streak.rs                     # Streak tracking (breaks on quit/skip, NOT failure)
│   │   └── progress.rs                   # Module unlock logic
│   │
│   ├── state/
│   │   ├── game_state.rs                 # GameState struct (serde)
│   │   └── persistence.rs                # Atomic JSON save/load
│   │
│   ├── tui/
│   │   ├── terminal.rs                   # Setup, teardown, panic handler
│   │   ├── event.rs                      # Unified event loop (keys + watcher + ticks)
│   │   ├── screens/
│   │   │   ├── home.rs                   # Dashboard: module map, XP, level, streak
│   │   │   ├── module_view.rs            # Exercise list for a module
│   │   │   ├── exercise_view.rs          # Instructions, status, hints, timer
│   │   │   ├── stats.rs                  # Detailed stats
│   │   │   ├── multiple_choice.rs        # MCQ interaction screen
│   │   │   └── ai_context.rs             # /hint-ai formatted output
│   │   └── widgets/
│   │       ├── xp_bar.rs, streak_display.rs, module_card.rs
│   │       ├── exercise_list.rs, hint_panel.rs, timer.rs
│   │
│   └── ai/
│       └── context_formatter.rs          # Format exercise context for AI
│
├── exercises/
│   ├── info.toml                         # Master manifest (modules + exercises)
│   ├── 01_getting_started/               # Exercise .rs files
│   │   ├── hello_world.rs
│   │   └── solutions/hello_world.rs
│   ├── 02_variables_types/
│   ├── ... (18 module directories)
│   └── 18_async_rust/
│
├── deps/                                 # Gitignored, cloned by setup.sh
│   ├── rust-by-example/
│   └── rustlings/                        # Optional reference
│
└── tests/
    ├── exercise_loading_test.rs
    ├── xp_calculation_test.rs
    ├── level_system_test.rs
    ├── streak_test.rs
    ├── pipeline_test.rs
    ├── progress_test.rs
    ├── state_persistence_test.rs
    ├── catalog_test.rs
    ├── toml_validation_test.rs          # Malformed info.toml error messages
    ├── tui_smoke_test.rs               # TUI screens render without panic
    └── fixtures/                        # Test exercise files
        ├── pass/                        # Known-good exercises
        └── fail/                        # Known-bad exercises (compiler errors, test failures)
```

---

## Cargo.toml Dependencies

```toml
[dependencies]
clap = { version = "4.6", features = ["derive"] }
ratatui = "0.30"
crossterm = "0.29"
notify = "8.0"
notify-debouncer-full = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "1.0"
anyhow = "1.0"
thiserror = "2.0"
chrono = { version = "0.4", features = ["serde"] }
tempfile = "3.27"

# Platform-specific user data directory for state persistence
dirs = "6.0"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.1"
tempfile = "3.27"
```

---

## Core Types (`src/exercise/types.rs`)

```rust
pub enum ExerciseType {
    FixCompilerError,              // (A) Classic Rustlings — fix so it compiles
    DebugLogicBug,                 // (B) Compiles but tests fail — find the bug
    ImplementFromScratch,          // (C) Write code to pass tests
    CodeTransformation,            // (D) Python→Rust or unidiomatic→idiomatic
    BossBattle,                    // (E) Multi-concept challenge
    ReverseEngineeringFillIn,      // (F1) Predict output, fill ANSWER const
    ReverseEngineeringMultipleChoice, // (F2) Predict output, TUI selection
    Optimization,                  // (G) No .clone(), avoid allocations
}
// NOTE: TimeTrial is NOT a type — it's a modifier. Any exercise with
// `time_limit_secs: Some(N)` becomes a time trial regardless of its type.

// VerificationMethod is DERIVED from ExerciseType, not stored independently.
// This eliminates the risk of type/verification mismatch in info.toml.
impl ExerciseType {
    pub fn verification_method(&self) -> VerificationMethod {
        match self {
            Self::FixCompilerError => VerificationMethod::CompileOnly,
            Self::DebugLogicBug | Self::ImplementFromScratch
                | Self::BossBattle | Self::ReverseEngineeringFillIn
                => VerificationMethod::CompileAndTest,
            Self::CodeTransformation => VerificationMethod::CompileTestClippy,
            Self::Optimization => VerificationMethod::CompileTestCustom,
            Self::ReverseEngineeringMultipleChoice => VerificationMethod::MultipleChoice,
        }
    }
}

pub enum VerificationMethod {
    CompileOnly, CompileAndTest, CompileTestClippy,
    CompileTestCustom, MultipleChoice,
}

pub enum Tier { Foundation, Intermediate, Advanced, Expert }
pub enum ExerciseStatus { Locked, Available, InProgress, Completed }
pub enum Difficulty { Beginner, Intermediate, Advanced, Expert }

pub struct Exercise {
    pub id: String,                    // "01_getting_started/hello_world"
    pub name: String,                  // "Hello, World!"
    pub module_id: String,
    pub exercise_type: ExerciseType,
    pub difficulty: Difficulty,
    pub base_xp: u32,
    pub file_path: PathBuf,
    pub solution_path: PathBuf,
    pub hints: Vec<String>,
    pub description: String,
    pub flavor_text: Option<String>,
    pub time_limit_secs: Option<u32>,  // Any exercise with Some(N) is a time trial
    pub custom_checks: Vec<CustomCheck>,
    pub multiple_choice_options: Option<Vec<MCOption>>,
    pub extra_files: Vec<PathBuf>,     // Additional files needed (modules, boss battles)
    pub order: u32,
}

pub struct Module {
    pub id: String,                    // "01_getting_started"
    pub name: String, pub theme_name: String, pub flavor_text: String,
    pub tier: Tier, pub order: u32,
    pub prerequisites: Vec<String>,    // Module IDs
    // NOTE: exercises list is COMPUTED from exercises with matching module_id,
    // not stored. Eliminates bidirectional reference sync issues.
}

pub enum CustomCheckType {
    NoClone,        // Catches .clone(), Clone::clone(), .to_owned(), .to_vec()
    NoUnwrap,       // Catches .unwrap(), .expect()
    NoCollect,      // Must not collect into intermediate Vec
    NoBoxDyn,       // Must use generics, not Box<dyn>
    MaxLines(u32),  // Solution must be under N non-empty, non-comment lines
}
```

---

## Game Mechanics

### XP System (`src/game/xp.rs`)

| Exercise Type | Base XP |
|---|---|
| fix_compiler_error | 10 |
| debug_logic_bug | 15 |
| implement_from_scratch | 20 |
| code_transformation | 20 |
| optimization | 25 |
| reverse_engineering (both) | 15 |
| boss_battle | 50 |

Any exercise with `time_limit_secs` set gets the time trial bonus (+15 XP if solved under the limit) on top of its base type XP.

**Bonuses:**
- First-try bonus: +50% of base XP (solved on attempt 1)
- Time trial bonus: +15 XP if solved under time limit
- Streak multiplier: 2 consecutive=1.25x, 3-4=1.5x, 5-9=2.0x, 10+=3.0x

**Formula:** `total = (base + first_try_bonus + time_trial_bonus) * streak_multiplier`

### Level Thresholds (`src/game/level.rs`)

Formula: `XP = 25 * (level-1)^1.8`

| Level | XP Required |
|---|---|
| 1 | 0 |
| 2 | 25 |
| 5 | 293 |
| 10 | 1177 |
| 15 | 2374 |
| 20 | 3943 |

Perfect run yields ~2500-3500 XP, reaching Level 17-19. Level 20 requires most first-try bonuses + good streaks.

### Streak System (`src/game/streak.rs`)

- Increments on each exercise completion
- Breaks on quit/skip, **NOT on failure** (encourage experimentation)
- 24-hour grace period: streak preserved if `last_exercise_at` is within 24 hours of the current time (not calendar-day based — avoids timezone complexity). Checked on app startup.

---

## Module Curriculum (18 modules, ~100 exercises)

### Foundation Tier (linear, must complete in order)
| # | Module | Theme Name | Exercises | Key Concepts |
|---|---|---|---|---|
| 01 | Getting Started | The First Steps | 3-4 | Hello World, Cargo, project structure |
| 02 | Variables & Types | The Binding Forge | 5-6 | let, mut, shadowing, type annotations, scalars |
| 03 | Functions & Control Flow | The Flow Chamber | 4-5 | fn, if/else, loops, match, expressions |
| 04 | Ownership & Moves | The Ownership Trials | 6-7 + boss | Stack/heap, move semantics, Copy, Clone |
| 05 | Borrowing & References | The Borrow Sanctum | 6-7 + boss | &T, &mut T, rules, slices, dangling refs |

### Intermediate Tier (needs Foundation, flexible within tier)
| # | Module | Theme Name | Exercises | Prerequisites |
|---|---|---|---|---|
| 06 | Structs & Methods | The Builders' Guild | 5-6 | 05 |
| 07 | Enums & Pattern Matching | The Pattern Archives | 5-6 + transform | 05 |
| 08 | Error Handling | The Error Gauntlet | 5-6 + boss | 07 |
| 09 | Collections | The Data Vault | 5-6 | 05 |
| 10 | Modules & Packages | The Crate Workshop | 4-5 | 05 |

### Advanced Tier (needs relevant intermediate modules)
| # | Module | Theme Name | Exercises | Prerequisites |
|---|---|---|---|---|
| 11 | Generics & Traits | The Trait Nexus | 6-7 + boss | 06, 07 |
| 12 | Lifetimes | The Lifetime Crucible | 5-6 | 11 |
| 13 | Closures & Iterators | The Iterator Engine | 5-6 + transform + optimization | 11 |
| 14 | Smart Pointers | The Pointer Maze | 5-6 | 11 |
| 15 | Fearless Concurrency | The Concurrency Arena | 5-6 + boss | 14 |

### Expert Tier (needs relevant advanced modules)
| # | Module | Theme Name | Exercises | Prerequisites |
|---|---|---|---|---|
| 16 | Testing & Documentation | The Quality Gate | 4-5 | 13 |
| 17 | Macros | The Macro Forge | 4-5 | 11 |
| 18 | Async Rust | The Async Dimension | 5-6 + boss | 15, 13 |

### Prerequisite Graph
```
Foundation (linear): 01 -> 02 -> 03 -> 04 -> 05
                                              |
Intermediate:  05 -> 06, 07, 09, 10 (any order)    07 -> 08
                                              |
Advanced:      06+07 -> 11 -> 12, 13, 14 (any order)    14 -> 15
                                              |
Expert:        13 -> 16    11 -> 17    15+13 -> 18
```

---

## Exercise Types — File Format Examples

### Type A: Fix Compiler Error
```rust
// Exercise: Immutable by Default
// Fix the compiler error below.
fn main() {
    let x = 5;
    x = 10; // TODO: Fix this line
    println!("x is now: {}", x);
}
```

### Type B: Debug Logic Bug
```rust
// Code compiles but tests fail. Find the bug.
fn count_evens(numbers: &[i32]) -> usize {
    let mut count = 0;
    for i in 0..numbers.len() {
        if numbers[i] % 2 == 1 { count += 1; } // BUG: should check == 0
    }
    count
}
#[cfg(test)] mod tests { /* assert_eq!(count_evens(&[1,2,3,4,5,6]), 3); */ }
```

### Type C: Implement from Scratch
```rust
// Define a Rectangle struct. Implement area() and can_hold() methods.
// TODO: Define struct and impl here
#[cfg(test)] mod tests { /* tests that use Rectangle */ }
```

### Time Trials (modifier, not a type)
Any exercise can be a time trial by setting `time_limit_secs` in info.toml. The TUI shows a countdown timer. Bonus +15 XP if solved under the limit. The file format is identical to the base exercise type.

### Type D: Code Transformation (two variants)
- **Python to Rust:** Python in comments, student writes Rust. Verified by tests.
- **Unidiomatic to Idiomatic:** Working but ugly Rust. Refactor. Verified by tests + Clippy.

### Type E: Boss Battle
Larger multi-concept challenge. Tests cover multiple concepts. Uses `extra_files` in info.toml for multi-file setups (also used by Module 10 exercises that teach module organization).

### Type F: Reverse Engineering (two variants)
- **Fill-in:** `const ANSWER: &str = r#"???"#;` — supports multi-line outputs via raw strings. Verified by test comparing to function output.
- **Multiple Choice:** Options in info.toml, TUI presents selection.

### Type G: Optimization
Working code with unnecessary cloning. Student refactors. Custom source-level checks catch `.clone()`, `Clone::clone()`, `.to_owned()`, `.to_vec()`, `.to_string()` (when converting from `&str`), and `.expect()`/`.unwrap()` where specified.

---

## Verification Pipeline (`src/runner/pipeline.rs`)

| Exercise Type | Step 1 | Step 2 | Step 3 | Step 4 |
|---|---|---|---|---|
| fix_compiler_error | cargo build | — | — | — |
| debug_logic_bug | cargo build | cargo test | — | — |
| implement_from_scratch | cargo build | cargo test | — | — |
| code_transformation | cargo build | cargo test | cargo clippy | — |
| boss_battle | cargo build | cargo test | — | — |
| reverse_engineering_fill_in | cargo build | cargo test | — | — |
| reverse_engineering_mcq | TUI selection | — | — | — |
| optimization | cargo build | cargo test | custom checks | — |

Verification method is derived from `ExerciseType` (not stored separately). Time trials add a time check on top of the base type's pipeline.

### Sandbox Mechanism (Persistent, Incremental)
Uses a **persistent sandbox directory per exercise** for incremental compilation:
1. Sandbox lives at `.rust-game-cache/<exercise_id>/` (gitignored). Exercise IDs contain slashes (e.g., `01_getting_started/hello_world`), which creates nested directories grouped by module — this is intentional and keeps the cache organized.
2. On first run: create Cargo project structure with `Cargo.toml` (pin `edition = "2021"`, add tokio for async exercises). Write a `.rustc-version` file containing the output of `rustc --version`.
3. On each verification: check `.rustc-version` against current `rustc --version`. If they differ, delete the sandbox and recreate it (toolchain update invalidates incremental cache). Then copy the exercise `.rs` file into the sandbox `src/`.
4. Copy `extra_files` into sandbox preserving filenames: each extra file is placed at `src/<filename>` (e.g., `extra_files = ["04_ownership/helper.rs"]` becomes `src/helper.rs` in the sandbox). This supports `mod helper;` declarations in exercises.
5. Run cargo commands — benefits from **incremental compilation cache**.
6. On timeout: kill process group (Unix: `setsid` + `killpg`; Windows: job objects) to prevent orphaned `rustc` processes.
7. `rust-game clean-cache` command wipes all sandbox directories.

This avoids the cold-build penalty on every save. First compilation for an exercise takes 2-3s; subsequent ones take <1s.

### Timeouts
| Type | Timeout |
|---|---|
| fix_compiler_error | 10s |
| debug/implement/transform/optimization | 15s |
| boss_battle | 30s |
| async exercises | 60s |

### Edge Cases
- **Infinite loops:** killed after timeout via process group kill (prevents orphaned rustc), "Check for infinite loops" message
- **Concurrent saves:** in-progress verification cancelled (process group killed), new one starts
- **Broken test code:** normal compilation error reported
- **Multi-file exercises:** `extra_files` in info.toml lists additional `.rs` files copied into sandbox (for module organization and boss battle exercises)
- **Custom check bypass:** pattern matching catches `.clone()`, `Clone::clone()`, `.to_owned()`, `.to_vec()`, `.to_string()`, `.unwrap()`, `.expect()` — documented as "best effort" in exercise instructions
- **Exercise file deleted during watch mode:** detect file-not-found before sandbox copy, show "Exercise file not found — press `x` to reset it to original state" instead of passing a missing file to cargo and displaying a confusing error

### Watch-Mode Consistency Rules
Watch mode needs explicit correctness rules so fast save cycles do not corrupt state:
1. Each verification run gets a monotonically increasing `verification_run_id`.
2. UI updates from a completed run are applied only if that run matches the latest active ID for the exercise view.
3. Cancelled or stale runs may log diagnostics, but they must not award XP, mark exercises complete, update streaks, or write persistent state.
4. XP and completion rewards are **idempotent**: the same exercise completion transition is applied once, even if multiple successful verifications arrive close together.
5. The pipeline must distinguish between:
   - `cancelled_by_new_save`
   - `timeout`
   - `verification_failed`
   - `verification_passed`
6. If a newer run starts while an older success result is being processed, the older result is discarded unless its state write has already committed.
7. **"Committed" is defined as:** the `persist()` call on the `NamedTempFile` has returned `Ok`. Before that point, any result is discardable. The receive side must check `run_id == latest_run_id` **before** any state mutation (XP award, status change, persist). This eliminates the TOCTOU window between channel receive and file write.
8. **Process kill race:** When a process group is killed, the verification thread may still send a result on the channel before the kill takes effect. The `run_id` check at the receive side catches this — a stale `run_id` is silently dropped.

---

## TUI Architecture

### Screens
1. **Dashboard** — Module map, XP bar, level, streak, recent activity
2. **Module View** — Exercise list with status icons, type, XP
3. **Exercise View** — Instructions, file path, hints, timer
4. **Watch Mode** — Live verification output, auto-reruns on save
5. **Stats View** — Detailed progression data
6. **Multiple Choice** — MCQ interaction for reverse engineering (post-V1)
7. **AI Context** — Formatted output for pasting to AI (post-V1)
8. **Level Up Animation** — Brief overlay on level-up (auto-dismiss 3s)

### Key Bindings
- **Global:** `q`/`Ctrl+C` quit, `?` help, `Esc` back
- **Dashboard:** `Enter` select module, `j/k` navigate, `s` stats, `n` next exercise
- **Exercise:** `w` watch mode, `v` verify, `h` hint, `o` open in `$EDITOR`, `a` AI context (post-V1), `S` solution (post-V1)
- **Watch:** `h` hint, `n` next (if complete), `Esc` exit watch, `a` AI (post-V1)
- **MCQ:** `1-4`/`a-d` select, `Enter` confirm (post-V1)

### TUI Layout Examples

```
Dashboard:
+--------------------------------------------------+
|  RUST-GAME                    Level 5 ####. XP    |
+----------------------------+---------------------+
|                            |  Player Stats       |
|  Module Map                |  XP: 450/600        |
|  [x] 01 First Steps       |  Level: 5           |
|  [x] 02 Binding Forge     |  Streak: 3          |
|  [>] 03 Flow Chamber      |  Best: 7            |
|  [ ] 04 Ownership         |  Completed: 23/100  |
|  [ ] 05 Borrow Sanctum    |                     |
|                            |  Recent Activity    |
|                            |  + shadowing +10XP  |
+----------------------------+---------------------+
|  [Enter] Select  [s] Stats  [n] Next  [q] Quit   |
+--------------------------------------------------+

Watch Mode:
+--------------------------------------------------+
|  Exercise: Move Semantics        Timer: 01:45     |
+--------------------------------------------------+
|  Status: COMPILE ERROR                            |
|                                                   |
|  error[E0382]: borrow of moved value: `s1`        |
|    --> exercises/04_.../move_semantics.rs:5:20     |
|  4 |     let s2 = s1;                             |
|    |              -- value moved here             |
|  5 |     println!("{}", s1);                      |
|    |                    ^^ value borrowed after   |
|    |                       move                   |
|                                                   |
|  File: exercises/04_ownership_moves/move_se...    |
+--------------------------------------------------+
|  Hints (1/3 shown):                               |
|  > When you assign s1 to s2, s1 is moved.         |
+--------------------------------------------------+
|  [h] More hints  [a] AI help  [n] Next  [Esc]    |
+--------------------------------------------------+
```

### Concurrency Model
```
Main thread:    TUI render loop + event dispatch
Thread 2:       Event handler (crossterm + file watcher + ticks)
Thread 3:       Verification runner (spawned per verification)
                +-- Child process: cargo build/test/clippy
```
All communication via `std::sync::mpsc`. No shared mutable state.

---

## CLI Subcommands (`src/cli.rs`)

**V1 commands:**
```
rust-game              # Default: launch TUI dashboard
rust-game tui          # Explicit TUI
rust-game watch [ID]   # Watch mode for exercise
rust-game verify ID    # One-shot verification
rust-game list [-m]    # List exercises, optionally filter by module
rust-game hint ID      # Show hints
rust-game stats        # Show progress
rust-game next         # Jump to next available exercise
rust-game reset [--exercise ID | --all]  # Reset progress
rust-game clean-cache                    # Wipe sandbox compilation caches
```

**Post-V1 commands (not implemented in V1):**
```
rust-game hint-ai ID   # Format AI context for pasting to external AI
rust-game solution ID  # Show reference solution
```

---

## info.toml Schema

```toml
[[modules]]
id = "01_getting_started"
name = "Getting Started"
theme_name = "The First Steps"
flavor_text = "Every great Rustacean started here."
tier = "foundation"
order = 1
prerequisites = []
# NOTE: No exercises list here — computed from exercises with matching module_id

[[exercises]]
id = "01_getting_started/hello_world"
name = "Hello, World!"
module = "01_getting_started"
type = "fix_compiler_error"        # ExerciseType enum value
difficulty = "beginner"
base_xp = 10
file = "01_getting_started/hello_world.rs"
solution = "01_getting_started/solutions/hello_world.rs"
order = 1
# NOTE: No verification field — derived from exercise type automatically
description = "Fix the classic Hello World program."
flavor_text = "Every journey begins with a single println!."
# Hints as INLINE ARRAY (safe for reordering exercises, unlike [[exercises.hints]])
hints = [
    "Look at the syntax of the println! macro. Is something missing?",
    "In Rust, println! is a macro (note the !). Make sure the string is in double quotes.",
]
# Optional fields:
# ci = true                       # Include in CI exercise verification set (one per exercise type)
# time_limit_secs = 120           # Makes this a time trial (+15 XP bonus)
# extra_files = ["01_getting_started/helper.rs"]  # Additional files for sandbox (placed at src/<filename>)
# [exercises.sandbox_deps]        # For async exercises needing external crates
# tokio = { version = "1", features = ["full"] }

# Multiple choice as inline array of tables (only for MCQ type):
# multiple_choice = [
#     { label = "A", text = "Option text", correct = false },
#     { label = "B", text = "Correct option", correct = true },
# ]

# Custom checks as inline array of tables (only for optimization type):
# custom_checks = [
#     { check_type = "no_clone", message = "Your solution still uses .clone()" },
# ]
```

### `sandbox_deps` Policy
Per-exercise dependencies are allowed only when the exercise cannot reasonably be taught with the standard sandbox template.

Rules:
- Default sandbox has **no external crates** beyond the common template.
- `sandbox_deps` is allowlisted, not open-ended.
- V1 allowlist should be minimal; start with `tokio` only if async content is actually in scope.
- Dependency versions are pinned centrally by the app when generating sandbox `Cargo.toml`; exercise authors do not get arbitrary semver ranges.
- If multiple exercises in a module need the same dependency set, prefer a shared module-level template over repeated per-exercise customization.
- Content validation must reject unknown crates, unsupported features, or dependency declarations that do not match the allowlist.

## Content Validation & CI Policy

The exercise catalog needs its own validation layer in addition to TOML parsing. Before content is accepted, automated validation should check:
- duplicate module IDs, exercise IDs, or `order` collisions within a module
- missing `file`, `solution`, or `extra_files` paths
- invalid prerequisite references or prerequisite cycles (use topological sort — if the module graph cannot be topologically sorted, report the cycle with the involved module IDs)
- invalid `multiple_choice` definitions (wrong number of options, zero/multiple correct answers, labels not matching UI assumptions)
- invalid `custom_checks` payloads or unsupported check names
- `sandbox_deps` entries outside the allowlist
- mismatches between the declared module, file path, and exercise ID naming conventions

CI should run:
- Rust tests
- content validation
- a **CI exercise set**: one exercise per exercise type present in the catalog, tagged with `ci = true` in `info.toml`. This keeps CI fast (~5-10 compilations) while covering every pipeline path.
- the full slow solution-verification suite on demand or before release branches are cut

---

## State Persistence

### State File Location (platform-specific, via `dirs` crate)
State is stored in the **user's data directory**, not the project directory. This ensures progress survives repo reinstalls/reclones.
- Linux: `~/.local/share/rust-game/state.json`
- macOS: `~/Library/Application Support/rust-game/state.json`
- Windows: `%APPDATA%/rust-game/state.json`
- **Fallback:** If `dirs::data_dir()` returns `None` (containers, CI, unusual environments), fall back to `.rust-game-state.json` in the current working directory and print a warning: "Could not determine user data directory; storing state in current directory."

### State File Format (`state.json`)
```json
{
  "version": 1,
  "player": {
    "xp": 450, "level": 5,
    "current_streak": 3, "best_streak": 7,
    "total_time_played_secs": 3600,
    "last_exercise_at": "2026-03-17T10:30:00Z"
  },
  "exercises": {
    "01_getting_started/hello_world": {
      "status": "completed", "xp_earned": 15,
      "attempts": 1, "completed_at": "...", "time_taken_secs": 45
    }
  },
  "modules": {
    "01_getting_started": { "unlocked": true, "completed": true }
  }
}
```

**Key design decisions:**
- `exercises_completed` and `total_exercises` are **computed at load time** from the exercises HashMap and catalog, not stored (avoids sync issues)
- Atomic writes via `tempfile::NamedTempFile` + `persist()` (rename is atomic on Unix)
- **Migration strategy:** On load, check `version`. If version < current: auto-migrate and back up old file as `state.json.bak`. If version > current: refuse to run with "Please update rust-game" error.
- **Windows note:** `tempfile::NamedTempFile::persist()` can fail on Windows if another process has the state file open. This is unlikely in normal use (no other process reads the state file). If `persist()` fails, retry once after 100ms. If it still fails, log a warning and skip the write (state will be saved on next successful persist).

---

## AI Integration

### CLAUDE.md / AGENTS.md / GEMINI.md
All three files identical (following system-design-game pattern). Content includes:
- Role definition (Rust tutor for Python/JS/TS developers)
- Teaching principles (hints not answers, compare to Python/JS equivalents)
- Exercise type explanations
- Module overview with concepts per module
- /hint-ai context format specification
- Directory layout reference

### /hint-ai Command (`src/ai/context_formatter.rs`)
Formats a complete context block containing:
1. Exercise metadata (name, type, difficulty, module)
2. Current exercise code (with student's edits)
3. Compiler/test error output (if any)
4. Hints already shown
5. Module concepts
6. Student's current level and progress

Output is displayed in TUI with copy instructions.

---

## External Content & Licensing

The project uses Rust By Example and Rustlings as **authoring references**, not as runtime dependencies of the learner experience.

Rules:
- `deps/rust-by-example` and `deps/rustlings` are cloned only to support exercise authoring, comparison, and attribution.
- Setup scripts must pin those repositories to known commits or tags so the authoring environment is reproducible.
- `ATTRIBUTION.md` must record which exercises are adapted, from which source, and under what license terms.
- Adapted exercises must be rewritten enough to fit the game's progression, hints, and formatting rather than copied wholesale.
- Content review must verify that attribution remains accurate when exercises are edited or expanded.

---

## Setup Scripts

### setup.sh
```bash
#!/usr/bin/env bash
set -euo pipefail
# 1. Check prerequisites: git, rustc (>= 1.80.0), cargo
# 2. Create deps/ directory
# 3. Clone rust-by-example (--depth 1)
# 4. Optionally clone rustlings (--depth 1, interactive prompt)
# 5. cargo build --release (with live output — do NOT suppress stdout/stderr)
#    Print: "Building rust-game (first build takes 2-5 minutes, subsequent builds are fast)..."
# 6. Print next steps
```

### setup.ps1
Windows equivalent with PowerShell idioms.

### Makefile
```makefile
setup:    ./setup.sh
build:    cargo build --release
install:  cargo install --path .
test:     cargo test
clean:    cargo clean
```

---

## Build Sequence (Implementation Order)

### Phase 0: Project Skeleton
1. `cargo init --name rust-game`
2. Create full directory structure
3. Write `Cargo.toml`, `.gitignore`, `setup.sh`, `setup.ps1`, `Makefile`
4. Write `CLAUDE.md` / `AGENTS.md` / `GEMINI.md`
5. Write content-validation scaffolding and `ATTRIBUTION.md`
6. `git init` + initial commit
7. **Verify:** `cargo build` succeeds, `cargo test` runs, and content-validation scaffolding can execute against an empty/minimal catalog

### Phase 0.5: Platform Spike
Write small standalone test programs (not full integration tests yet) that prove each concern:
1. **Process cleanup on timeout:** Spawn a child process in a process group that loops forever, kill the group after 2s, then check `ps aux` (Unix) or `tasklist` (Windows) for orphaned processes. Automated as a script that exits non-zero if orphans remain.
2. **File watching with rapid save bursts:** Write a file 10 times in 500ms via a script, verify the notify watcher fires a debounced event and does not panic or miss the final state.
3. **State-file creation in the platform data directory:** Call `dirs::data_dir()`, create the rust-game subdirectory, write a JSON file, read it back, delete it. Verify on each platform. Document fallback behavior if `data_dir()` returns `None`.
4. **Sandbox lifecycle:** Create a persistent sandbox, compile a trivial `.rs` file, modify the file, recompile (should use incremental cache and be faster), then delete the sandbox directory. Verify no leftover files.
5. **Verify:** Record results in a `PLATFORM_COMPAT.md` file. If a platform fails any spike, narrow V1 scope before Phase 1 continues.

### Phase 1: Exercise Loading
1. `src/exercise/types.rs` — all core types
2. `src/exercise/loader.rs` — parse info.toml
3. `src/exercise/catalog.rs` — ordering, filtering
4. Implement content validation (IDs, paths, prerequisites, MCQ shape, custom checks, `sandbox_deps`)
5. Write `exercises/info.toml` with first 5-10 exercises + their .rs files + solutions
6. `tests/exercise_loading_test.rs`, `tests/catalog_test.rs`, validation tests
7. **Verify:** loader tests pass, invalid content produces actionable errors, and the catalog can list the initial exercises without manual fixes

### Phase 2: Game Engine (parallel with Phase 1)
1. `src/game/xp.rs`, `level.rs`, `streak.rs`, `progress.rs`
2. `src/state/game_state.rs`, `persistence.rs`
3. `tests/xp_calculation_test.rs`, `level_system_test.rs`, `streak_test.rs`, `progress_test.rs`, `state_persistence_test.rs`
4. Add idempotency coverage for "award XP once per completion transition"
5. **Verify:** all game-logic tests pass, save/load round-trips are stable, and repeated completion events do not double-award XP

### Phase 3: Verification Pipeline (depends on Phase 1)
1. `src/runner/sandbox.rs` — persistent sandbox project creation with rustc version tracking
2. `src/runner/cargo_command.rs` — shared cargo invocation (spawn, capture output, timeout, process group kill)
3. `src/runner/custom_checks.rs` — source-level pattern checks
4. `src/runner/pipeline.rs` — orchestration with explicit result states and run IDs
5. `tests/pipeline_test.rs` — test with known-good and known-bad files from `tests/fixtures/`
6. Add timeout/cancellation tests
7. Add watch-mode consistency unit tests: simulate channel messages with different `run_id` values, verify that only the latest ID triggers state mutations. These are deterministic unit tests (no timing dependencies), not integration tests with real file saves.
8. **Verify:** exercises are correctly identified as pass/fail, timeout cleanup works, stale/cancelled runs cannot mutate state, and rustc version change triggers sandbox rebuild

### Phase 4: CLI + Non-TUI Commands (depends on 1, 2, 3)
1. `src/cli.rs` — clap definitions
2. `src/main.rs` — entry point
3. Implement V1 commands: `list`, `verify`, `hint`, `stats`, `next`, `reset`
4. `src/app.rs` — app state machine
5. **Verify:** `rust-game list`, `rust-game verify <id>`, and `rust-game next` work against the seed catalog without TUI dependencies

### Phase 5: TUI Framework (depends on Phase 4)
1. `src/tui/terminal.rs` — setup/teardown/panic handler
2. `src/tui/event.rs` — event loop
3. V1 screens: `home.rs` -> `module_view.rs` -> `exercise_view.rs` -> `stats.rs`
4. Widgets: `xp_bar.rs`, `module_card.rs`, `streak_display.rs`, `exercise_list.rs`, `hint_panel.rs`, `timer.rs`
5. **Verify:** the V1 screens render without panic, keyboard navigation works end-to-end, and the UI remains responsive under repeated event-loop ticks

### Phase 6: Watch Mode (depends on 3, 5)
1. `src/watcher/file_watcher.rs` — notify + debouncer
2. Integrate into TUI event loop
3. Wire verification result -> XP award -> state persistence -> level-up detection using the watch-mode consistency rules above
4. **Verify:** edit exercise, save, see live verification in TUI, and repeated rapid saves never display stale status or double-apply rewards

### Phase 7: V1 Exercise Content (depends on 3, can parallel with 5/6)
Write the initial **~15-20 V1 exercises**, module by module:
1. Foundation only: modules 01-05
2. Prioritize ownership, borrowing, and compiler-feedback exercises because they validate the core learning value proposition
3. Include at least one boss battle to prove the larger multi-file format
4. For each: write `.rs` file, write solution, add to `info.toml`, run validation, verify via pipeline
5. **Verify:** every V1 exercise has a passing solution, at least one intentional failing case, and clear hints that match the intended concept

### Phase 8: Extended Curriculum (post-V1)
1. Expand from ~15-20 exercises to ~60 total
2. Add modules 06-18 in tier order
3. Introduce additional exercise types only after the simpler loop is stable
4. Add post-V1 screens such as `multiple_choice.rs` only when the catalog needs them
5. **Verify:** expansion does not regress V1 performance or overwhelm navigation/discoverability in the UI

### Phase 9: AI Integration (depends on Phase 4, post-V1 by default)
1. `src/ai/context_formatter.rs`
2. Write CLAUDE.md / AGENTS.md / GEMINI.md content
3. Add `hint-ai` and `solution` CLI flows if they still fit the product direction
4. Add the `ai_context.rs` screen only if it improves the workflow rather than cluttering the UI

### Phase 10: Polish & Testing
1. Edge case handling, error messages, UX polish
2. State file migration (version field)
3. End-to-end testing
4. Performance profiling (compilation speed, TUI responsiveness)
5. **Verify:** repeat verification is consistently fast on the supported platforms, the state file survives restarts and repo reinstalls, and release criteria are met for the declared V1 scope

---

## Verification Plan

### Automated Tests
- `cargo test` — runs all unit + integration tests (fast tests only)
- Content validation runs on every test/CI pass
- Pipeline tests use `tests/fixtures/pass/` and `tests/fixtures/fail/` for known-good/bad exercises
- info.toml validation tests verify helpful error messages for malformed TOML (typos in type names, missing required fields)
- TUI smoke tests using ratatui `TestBackend` — each screen renders without panic
- State persistence roundtrip tests (save/load, corrupt file recovery, version migration)
- Watch-mode consistency tests: **deterministic unit tests** that simulate channel messages with varying `run_id` values. Verify that only the latest `run_id` triggers state mutations, that stale IDs are silently dropped, and that double-completion of the same exercise does not double-award XP. No timing-dependent integration tests (those are flaky on CI).

### Slow Tests (`#[ignore]`, run with `cargo test -- --ignored`)
- `test_all_solutions_pass_verification` — compiles every solution through the pipeline (initially V1 scope, later the full catalog)
- Timeout tests with infinite loop exercises

### Manual Testing
- TUI navigation across all screens
- Watch mode with real file editing
- Level-up animation triggers
- Module unlock when prerequisites complete
- Time trial countdown and bonus XP
- Multiple choice interaction
- AI context formatting output
- Streak increment, break, and multiplier application
- Edge cases: infinite loop timeout (verify process group cleanup), concurrent saves, corrupt state file
- Persistent sandbox: verify incremental compilation works (second save compiles faster than first)
- Platform checklist: file watcher stability, user-data location, cache cleanup, and process cleanup on each supported OS

### Release Criteria
- V1 ships only after the Foundation-only scope is complete and stable
- The supported-platform matrix is explicit and matches the Phase 0.5 spike results
- Seed content passes validation with no manual exceptions
- All V1 exercises pass the solution-verification suite
- Repeat verification latency is acceptably fast on supported hardware
- No known stale-run or double-award bugs remain in watch mode

### Exercise Validation
```rust
#[test]
#[ignore] // Slow: compiles all exercises
fn test_all_solutions_pass_verification() {
    // For each exercise, run the solution through the pipeline
    // Ensures all exercises are solvable
}
```

---

## Review-Driven Changes Log

The following changes were made after a structured external review. Changes are grouped by severity.

### Critical Fixes
1. **Persistent sandbox for incremental compilation** — Changed from creating a fresh temp Cargo project per verification to a persistent sandbox directory per exercise (`.rust-game-cache/<exercise_id>/`). Only the `.rs` file is copied on each save; the Cargo project structure and compilation cache persist. Reduces verify-on-save from 2-5s to <1s after first compilation.
2. **Process group cleanup on timeout** — Changed from `child.kill()` (which orphans rustc subprocesses) to process group killing (`setsid` + `killpg` on Unix, job objects on Windows). Prevents zombie processes when students write infinite loops.
3. **Removed `TimeTrial` from `ExerciseType`** — Time trial is now a modifier: any exercise with `time_limit_secs: Some(N)` is a time trial. Eliminated the dispatch ambiguity where the pipeline needed to know the "underlying type" of a time trial.
4. **State file moved to user data directory** — Changed from `.rust-game-state.json` in project directory to platform-specific user data dir (`~/.local/share/rust-game/` etc.). Progress now survives repo reinstalls. Added `dirs = "6.0"` dependency.

### Major Fixes
5. **`VerificationMethod` derived from `ExerciseType`** — Removed `verification` field from info.toml schema. Verification method is now computed via `ExerciseType::verification_method()`, eliminating type/verification mismatch risk.
6. **Module exercises list computed, not stored** — Removed `exercises: Vec<String>` from `Module`. The exercise list is computed at load time by filtering exercises on `module_id`, eliminating bidirectional reference sync issues.
7. **`exercises_completed` computed, not stored** — Removed from state file. Computed at load time from the exercises HashMap.
8. **TOML hints switched to inline arrays** — Changed from `[[exercises.hints]]` (nested array-of-tables, prone to reordering bugs) to `hints = ["hint1", "hint2"]` (inline array, safe to reorder). Same for `multiple_choice` and `custom_checks`.
9. **Exercise count reduced to ~60 initially** — Changed from ~100 (unrealistic pace) to ~60 (3-4 per module), with documented expansion path to ~100 post-release.
10. **Custom check patterns expanded** — `NoClone` now also catches `Clone::clone()`, `.to_owned()`, `.to_vec()`, `.to_string()`. `NoUnwrap` also catches `.expect()`.
11. **`extra_files` generalized for any exercise type** — Changed from boss-battle-only `// EXTRA_FILE:` directive to `extra_files` field in info.toml, available for any exercise (needed by Module 10: Modules & Packages).
12. **Multi-line reverse engineering answers** — Changed from `const ANSWER: &str = "???";` to raw string format `const ANSWER: &str = r#"???"#;` to support multi-line outputs.
13. **State file migration strategy specified** — Added: check version on load, auto-migrate with `.bak` backup, refuse to run if version > current.

### Minor Fixes
14. **Removed `colored` dependency** — Minimal non-TUI colored output doesn't justify an extra crate. Use ratatui's `Stylize` or raw ANSI codes.
15. **Pinned Rust edition in sandbox Cargo.toml** — Explicitly set `edition = "2021"` to avoid surprises from system default.
16. **Added `tests/fixtures/` directory** — `pass/` and `fail/` subdirectories for pipeline test exercises.
17. **Added TUI smoke tests** — Basic `TestBackend` tests to verify each screen renders without panicking.
18. **Added info.toml validation tests** — Tests with intentionally malformed TOML verify helpful error messages.
19. **Solution verification test marked `#[ignore]`** — Prevents slow ~60-compilation test from blocking routine `cargo test`.
20. **Added `clean-cache` CLI command** — Wipes persistent sandbox directories.

---

## Second Review Changes Log

Changes from the second review pass, focused on the V1 scoping additions, watch-mode consistency rules, and operational edge cases.

### Major Fixes
21. **V1 implementation boundaries documented** — ExerciseType enum defines all 7 variants for forward-compatible serialization, but V1 only implements pipeline paths for 4 types (FixCompilerError, DebugLogicBug, ImplementFromScratch, BossBattle). Other variants return "not yet supported" errors. Prerequisite graph logic deferred to simple "are all prereqs completed?" for V1's linear foundation.
22. **Watch-mode race condition closed** — Clarified that "committed" means `persist()` returned `Ok`. Added explicit rule that receive side checks `run_id == latest_run_id` before any state mutation. Stale results from killed process groups are caught by the run ID check.
23. **Sandbox rustc version tracking** — Added `.rustc-version` file to persistent sandbox. On each verification, compare against current `rustc --version`. If they differ, rebuild the sandbox. Prevents stale incremental cache after toolchain updates.
24. **extra_files path mapping specified** — Extra files are placed at `src/<filename>` in the sandbox (e.g., `helper.rs` → `src/helper.rs`). This supports `mod helper;` declarations in exercises.
25. **Consolidated cargo runner** — Replaced separate `compiler.rs`, `tester.rs`, `clippy.rs` with a single `cargo_command.rs` module. All three follow the same pattern (spawn in sandbox, capture output, timeout, process group kill).
26. **CI exercise set defined** — "Representative subset" for CI is now explicit: one exercise per exercise type present in the catalog, tagged with `ci = true` in `info.toml`. Keeps CI fast (~5-10 compilations).
27. **Cycle detection algorithm specified** — Content validation uses topological sort on the module prerequisite graph. If the graph cannot be sorted, report the cycle with involved module IDs.

### Minor Fixes
28. **24-hour streak clarified** — Uses "24 hours from `last_exercise_at`" (elapsed time), not calendar-day boundaries. Avoids timezone complexity entirely.
29. **File deletion during watch mode** — Added edge case: detect missing exercise file before sandbox copy, show "file not found — press `x` to reset" instead of a confusing cargo error.
30. **`dirs::data_dir()` fallback** — If `None` (containers, CI), fall back to `.rust-game-state.json` in the current directory with a warning.
31. **Windows atomic rename retry** — `persist()` can fail on Windows if the file is open. Added: retry once after 100ms, then log warning and skip write.
32. **CLI commands split into V1 and post-V1** — `hint-ai` and `solution` moved to a separate "Post-V1 commands" subsection to avoid confusion during V1 implementation.
33. **Setup script build feedback** — Added live cargo output and upfront time estimate: "First build takes 2-5 minutes, subsequent builds are fast."
34. **Phase 0.5 spike defined concretely** — Each proof is now a standalone test script with specific pass/fail criteria, not a vague "prove it works." Results recorded in `PLATFORM_COMPAT.md`.
35. **Watch-mode consistency tests specified** — Deterministic unit tests simulating channel messages with varying `run_id` values. No timing-dependent integration tests (flaky on CI).
