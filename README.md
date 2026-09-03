# rust-game

**A gamified CLI tool for learning Rust through exercises, XP, streaks, boss battles, and a rich terminal UI.**

Learn Rust by actually writing Rust. Write your solution in the built-in editor — or in your own, if you prefer — save, and get instant feedback: the tool compiles your code, runs tests, awards XP, and tracks your streak. Designed for developers who already know Python, JavaScript, or TypeScript and want to learn Rust's unique concepts: ownership, borrowing, lifetimes, traits, pattern matching, and the borrow checker.

This is not a textbook. This is a game where the exercises ARE the game.

---

## What You Get

- **65 exercises** across 18 modules, from "Hello World" to async Rust
- **7 exercise types** — fix compiler errors, debug logic bugs, implement from scratch, code transformations (Python to Rust), boss battles, reverse engineering (predict output), and optimization challenges
- **XP system** with first-try bonuses, time trial bonuses, and streak multipliers
- **Leveling** (1-20) with level-up celebrations
- **Streak tracking** — builds on consecutive completions, resets after 24 hours without one. Never broken by a failed attempt — experiment freely
- **Rich terminal UI** — dashboard, module map, exercise view, watch mode with live verification, stats breakdown
- **Watch mode** — save your file, verification runs automatically, XP awarded instantly
- **Boss battles** — multi-concept challenges that combine several topics
- **Time trials** — bonus XP if you solve exercises under the time limit
- **Lessons before exercises** — one authored lesson per module, readable in the TUI or taught conversationally by Claude Code, Codex, or Gemini via `/lesson`
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
3. **Read the lesson** — it opens by itself the first time you enter a module. `Esc` skips it for this session, `m` marks it read, and `l` reopens it any time
4. **Select an exercise** — read the instructions and the starter code
5. **Press `w` to edit** — the exercise opens in a split: your code above, compiler output below
6. **Write and save** — `Ctrl+S` saves and verifies. If tests pass, you earn XP
7. **Compare** — stuck, or curious how the reference did it? `Ctrl+D` shows your code beside the solution with the differences highlighted. No XP penalty — it is simply recorded, so your stats keep meaning something
8. **Level up** — progress through modules, unlock new tiers, maintain your streak

### Where your code lives

`exercises/` holds pristine templates that stay tracked in git. The first time you open
an exercise, your personal copy is created under `workspace/`, which is gitignored — so
`git status` stays clean no matter how much you write, and `rust-game reset --exercise
<id>` restores the original with a file copy.

The built-in editor writes to that path for you. If you would rather use your own
editor, press `o` for the path and open it there — the file watcher picks up external
saves exactly as before, and the two can even be used on the same exercise: a change
made outside is reloaded into the editor automatically.

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

A streak resets after 24 hours without a completion, but **never on failure** — keep experimenting without penalty.

---

## AI Tutor Support

rust-game is designed to work with AI coding assistants as your Rust tutor. The repo includes pre-configured context files:

| Tool | Context File | Setup |
|------|-------------|-------|
| [Claude Code](https://claude.ai/claude-code) | `CLAUDE.md` | Works automatically |
| [Codex CLI](https://github.com/openai/codex) | `AGENTS.md` | Works automatically |
| [Gemini CLI](https://github.com/google-gemini/gemini-cli) | `GEMINI.md` | Works automatically |

Each tool also ships a project-scoped `/lesson` skill (`.claude/skills/`, `.codex/skills/`,
`.gemini/skills/`). Run `/lesson 04_ownership_moves` and the agent teaches that module's
lesson conversationally, answers your follow-up questions, and hands you the first
exercise. The shared teaching protocol lives in the context files above.

Use `rust-game hint-ai <exercise>` to generate a formatted context block with your current code, compiler errors, hints, and progress — paste it into any AI chat for targeted help.

No AI tool is required. Exercises include progressive hints you can reveal with `h`.

---

## Prerequisites

- **Rust** (1.88.0 or newer) — install via [rustup.rs](https://rustup.rs)
- **git** — to clone this repo
- **A C compiler** (`cc`, `gcc`, or `clang`) — the editor's syntax highlighter
  builds a small C library. Already present on most systems.
- A terminal. A separate text editor is optional — there is one built in.

That's it. No IDE plugins, no Docker, no accounts to create.

---

## Quick Start

```bash
# 1. Clone this repo
git clone https://github.com/cheesecake-mainframe/rust-game.git
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
rust-game lesson MOD   Print a module's lesson (--mark-read to mark it read)
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
| `l` | Read the module's lesson |
| `m` | Mark a lesson as read |
| `w` | Open the exercise in the editor |
| `v` | Verify exercise |
| `h` | Reveal next hint |
| `a` | Show AI context |
| `o` | Show the file path (for editing elsewhere) |
| `n` | Next exercise |
| `s` | Stats screen |
| `Esc` | Go back |
| `q` | Quit |

### Key Bindings (editor)

While editing, the editor owns ordinary keys — they type. Every game command is on
`Ctrl`, and editing is modeless: no insert mode to enter, no mode to escape from.
`Alt` belongs entirely to the editor's own word motions.

| Key | Action |
|-----|--------|
| `Ctrl+S` | Save and verify |
| `Ctrl+F` | Search within the file (press again for the next match) |
| `Ctrl+U` / `Ctrl+R` | Undo / redo |
| `Ctrl+D` | Compare your code against the reference solution |
| `Ctrl+J` | Switch between side-by-side and stacked panes |
| `Ctrl+E` | Show the full compiler output full-screen |
| `Ctrl+↑` / `Ctrl+↓` | Scroll the compiler output |
| `Ctrl+G` | Reveal next hint |
| `Ctrl+L` | Read the module's lesson |
| `Ctrl+A` | Show AI context |
| `Ctrl+N` | Next exercise |
| `Ctrl+X` | Reset this exercise (press twice) |
| `Esc` | Cancel a search, or leave the editor (your work is saved either way) |

Quit with `Ctrl+C` — it saves your buffer first. Everything else, including
every `Alt` combination, goes to the editor.

Some keys are deliberately left alone. `Ctrl+I` and `Ctrl+M` arrive as Tab and
Enter, so no application can tell them apart, and `Ctrl+H` is how some terminals
send Backspace. `Ctrl+Q`, `Ctrl+T`, `Ctrl+W` and `Ctrl+Z` are commonly claimed by
the terminal emulator itself — a terminal binding is consumed before the
application ever sees it, so the game stays off them.

Copy and paste: paste with your terminal's usual shortcut (bracketed paste is
supported, so indentation survives). To copy *out*, select with the mouse as you
normally would — the editor deliberately does not capture the mouse, so your
terminal's own selection keeps working.

---

## Disk Usage

Each exercise gets its own persistent cargo sandbox under `.rust-game-cache/`
so re-verification is incremental. That costs roughly 8 MB per exercise you
attempt — a few hundred MB if you work through all 65. `make clean` removes it;
nothing is lost but compile time.

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
