---
name: lesson
description: Teach a rust-game module lesson, then answer the student's follow-up questions. Use when the student invokes /lesson, asks to be taught or to learn a module or Rust concept, says they are starting a new module, or is stuck on an exercise because they were never taught the underlying idea.
---

# /lesson — teach a rust-game module

Follow the **Lesson Protocol** in `AGENTS.md` at the root of this repository. It is the
single source of truth and applies verbatim; this file only routes you to it.

**Argument:** a module ID (`04_ownership_moves`), a bare number (`4`), or any exercise
ID inside the module (`04_ownership_moves/move_semantics`). With no argument, ask which
module — or infer it from what the student is currently working on.

In short:

1. Resolve the argument to a module via `exercises/info.toml`.
2. Read `exercises/lessons/<module_id>.md`.
3. Teach it conversationally, comparing to Python/JS/TS. Do not paste the file.
4. Ask one comprehension question, then answer whatever they ask back.
5. Run `rust-game lesson <module_id> --mark-read` when you are done teaching.
6. Give a 2-3 line primer on what is new in the first exercise.
7. Hand them the `workspace/` path to open — never the `exercises/` template.

Never write their solution.
