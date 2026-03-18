# rust-game

**A gamified CLI tool for learning Rust through exercises, XP, streaks, boss battles, and a rich terminal UI.**

Learn Rust by actually writing Rust. Edit exercises in your editor, save, and get instant feedback — the tool compiles your code, runs tests, awards XP, and tracks your streak. Designed for developers who already know Python, JavaScript, or TypeScript and want to learn Rust's unique concepts: ownership, borrowing, lifetimes, traits, pattern matching, and the borrow checker.

This is not a textbook. This is a game where the exercises ARE the game.

---

## What You Get

- **65 exercises** across 18 modules, from "Hello World" to async Rust
- **7 exercise types** — fix compiler errors, debug logic bugs, implement from scratch, code transformations (Python to Rust), boss battles, reverse engineering (predict output), and optimization challenges
- **XP system** with first-try bonuses, time trial bonuses, and streak multipliers
- **Leveling** (1-20) with level-up celebrations
- **Streak tracking** — builds on consecutive completions, breaks on quit/skip (not failure — experimentation is encouraged)
- **Rich terminal UI** — dashboard, module map, exercise view, watch mode with live verification, stats breakdown
- **Watch mode** — save your file, verification runs automatically, XP awarded instantly
- **Boss battles** — multi-concept challenges that combine several topics
- **Time trials** — bonus XP if you solve exercises under the time limit
- **AI tutor integration** — `hint-ai` command formats your exercise context for pasting into any AI chat
- **Persistent progress** — your XP, level, and streak survive across sessions

---

## Curriculum

The curriculum follows a proven learning path inspired by [The Rust Programming Language](https://doc.rust-lang.org/book/). Modules unlock progressively — Foundation is linear, then you choose your path.

```
FOUNDATION (linear, complete in order)
  01 The First Steps           Hello World, Cargo, project structure
  02 The Binding Forge         let, mut, shadowing, type annotations
  03 The Flow Chamber          fn, if/else, loops, match, expressions
  04 The Ownership Trials      Stack/heap, move semantics, Copy, Clone
  05 The Borrow Sanctum        &T, &mut T, borrowing rules, slices

INTERMEDIATE (flexible order after Foundation)
  06 The Builders' Guild       Structs, methods, impl blocks
  07 The Pattern Archives      Enums with data, match, if let, Option
  08 The Error Gauntlet        Result, ?, custom errors          + Boss Battle
  09 The Data Vault            Vec, String, HashMap
  10 The Crate Workshop        Modules, visibility, use paths

ADVANCED (unlocks after relevant Intermediate modules)
  11 The Trait Nexus           Generics, trait bounds, derive     + Boss Battle
  12 The Lifetime Crucible     Lifetime annotations, elision, structs
  13 The Iterator Engine       Closures, iterator chains, Python-to-Rust
  14 The Pointer Maze          Box, Rc, RefCell
  15 The Concurrency Arena     Threads, channels, Arc, Mutex     + Boss Battle

EXPERT (unlocks after relevant Advanced modules)
  16 The Quality Gate          Unit tests, doc tests, test organization
  17 The Macro Forge           macro_rules!, custom vec, patterns
  18 The Async Dimension       async/await, futures, manual polling + Boss Battle
```

---

## Exercise Types

| Type | What You Do | Verification |
|------|-------------|-------------|
| Fix Compiler Error | Fix broken code so it compiles | `cargo build` |
| Debug Logic Bug | Code compiles but tests fail — find the bug | `cargo build` + `cargo test` |
| Implement from Scratch | Write code to make tests pass | `cargo build` + `cargo test` |
| Code Transformation | Convert Python to idiomatic Rust | `cargo build` + `cargo test` + `cargo clippy` |
| Boss Battle | Multi-concept challenge combining several topics | `cargo build` + `cargo test` |
| Reverse Engineering | Predict what code outputs (fill-in or multiple choice) | Test comparison or TUI selection |
| Optimization | Remove unnecessary `.clone()` calls using references | `cargo build` + `cargo test` + source checks |

Any exercise can also be a **time trial** — bonus +15 XP if solved under the time limit.

---

## How It Works

1. **Launch the TUI** — run `rust-game` to see the dashboard with your module map, XP, level, and streak
2. **Pick a module** — navigate with `j/k` and press `Enter`
3. **Select an exercise** — read the instructions, open the `.rs` file in your editor
4. **Enter watch mode** — press `w` and the tool watches your file for changes
5. **Edit and save** — verification runs automatically. If tests pass, you earn XP
6. **Level up** — progress through modules, unlock new tiers, maintain your streak

```
+--------------------------------------------------+
|  RUST-GAME          Level 5 | 450 XP | Streak: 3 |
+----------------------------+---------------------+
|                            |  Player Stats       |
|  Module Map                |  XP: 450/600        |
|  [x] 01 First Steps       |  Level: 5           |
|  [x] 02 Binding Forge     |  Streak: 3          |
|  [>] 03 Flow Chamber      |  Best: 7            |
|  [ ] 04 Ownership         |  Completed: 23/65   |
|  [ ] 05 Borrow Sanctum    |                     |
+----------------------------+---------------------+
|  [Enter] Select  [n] Next  [s] Stats  [q] Quit  |
+--------------------------------------------------+
```

---

## XP and Scoring

| Bonus | Amount |
|-------|--------|
| First-try bonus | +50% of base XP |
| Time trial bonus | +15 XP (if solved under time limit) |
| Streak 2 | 1.25x multiplier |
| Streak 3-4 | 1.5x multiplier |
| Streak 5-9 | 2.0x multiplier |
| Streak 10+ | 3.0x multiplier |

Streaks break on quit or skip, but **not on failure** — keep experimenting without penalty.

---

## AI Tutor Support

rust-game is designed to work with AI coding assistants as your Rust tutor. The repo includes pre-configured context files:

| Tool | Context File | Setup |
|------|-------------|-------|
| [Claude Code](https://claude.ai/claude-code) | `CLAUDE.md` | Works automatically |
| [Codex CLI](https://github.com/openai/codex) | `AGENTS.md` | Works automatically |
| [Gemini CLI](https://github.com/google-gemini/gemini-cli) | `GEMINI.md` | Works automatically |

Use `rust-game hint-ai <exercise>` to generate a formatted context block with your current code, compiler errors, hints, and progress — paste it into any AI chat for targeted help.

No AI tool is required. Exercises include progressive hints you can reveal with `h`.

---

## Prerequisites

- **Rust** (1.80.0 or newer) — install via [rustup.rs](https://rustup.rs)
- **git** — to clone this repo
- A terminal and a text editor

That's it. No IDE plugins, no Docker, no accounts to create.

---

## Quick Start

```bash
# 1. Clone this repo
git clone https://github.com/cheesecakeMafia/rust-game.git
cd rust-game

# 2. Run the setup script
./setup.sh
# Or: make setup
# Or on Windows: powershell -ExecutionPolicy Bypass -File setup.ps1

# 3. Launch the game
cargo run
# Or if installed: rust-game
```

### CLI Commands

```
rust-game              Launch the TUI dashboard
rust-game list         List all modules and exercises
rust-game verify ID    Verify a single exercise
rust-game hint ID      Show progressive hints
rust-game hint-ai ID   Format context for AI tutor
rust-game solution ID  Show the reference solution
rust-game stats        Show your progress
rust-game next         Jump to the next available exercise
rust-game reset --all  Reset all progress
rust-game clean-cache  Wipe compilation caches
```

### Key Bindings (TUI)

| Key | Action |
|-----|--------|
| `j/k` | Navigate up/down |
| `Enter` | Select module/exercise |
| `w` | Enter watch mode |
| `v` | Verify exercise |
| `h` | Reveal next hint |
| `a` | Show AI context |
| `n` | Next exercise |
| `s` | Stats screen |
| `Esc` | Go back |
| `q` | Quit |

---

## Platform Support

rust-game targets **Linux, macOS, and Windows**.

- Process cleanup uses `setsid`/`killpg` on Unix and `taskkill /T` on Windows
- State is stored in your platform's data directory (`~/.local/share/`, `~/Library/Application Support/`, `%APPDATA%`)
- File watching uses `notify` v8 with 300ms debouncing

---

## Building from Source

```bash
# Development build
cargo build

# Run tests (76 unit + 7 integration)
cargo test

# Run slow tests (compiles all 65 solutions)
cargo test -- --ignored

# Install system-wide
cargo install --path .
```

---

## Inspired By

This project draws inspiration from two excellent Rust learning resources:

- **[Rustlings](https://github.com/rust-lang/rustlings)** — The original "fix exercises to learn Rust" tool. rust-game borrows the watch-and-verify workflow but adds gamification, a TUI, and more exercise types.
- **[Rust by Example](https://github.com/rust-lang/rust-by-example)** — Exercise patterns and concept ordering are informed by its examples.

See [ATTRIBUTION.md](ATTRIBUTION.md) for full details.

---

## Contributing

Found a bug? Have a better exercise idea? Want to improve a hint? Contributions are welcome.

- Exercises live in `exercises/` — each is a standalone `.rs` file with tests
- Exercise metadata is in `exercises/info.toml`
- Solutions are in `exercises/<module>/solutions/`

---

## License

[MIT](LICENSE)
