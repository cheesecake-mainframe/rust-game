# The First Steps

The first Rust program you write will probably fail to compile, and the reason will be a
single character: `println` is not a function, it is a macro, and macros are invoked
with a trailing `!`. Get that one fact in place now and the classic "hello, world" stops
being a trap.

## Coming from Python/JS

In Python you call `print("hi")`. In JavaScript you call `console.log("hi")`. Both are
ordinary functions: they take any number of arguments, of any type, and figure out what
to do at runtime. That flexibility is free because both languages carry a runtime that
can inspect a value and ask it how to stringify itself.

Rust has no such runtime. Every call is type-checked and resolved before your program
exists, and a plain Rust function has a fixed number of parameters with fixed types. You
cannot express "takes a format string plus any number of arguments of any type, and
verifies at compile time that the placeholders match" as a normal function signature.

So Rust does it with a macro instead. `println!` runs at compile time, reads your format
string, counts the `{}` placeholders, checks each argument can be displayed, and
generates the formatting code. Pass too few arguments and you get a compile error, not a
runtime surprise. The `!` is not decoration — it marks the places where code is
generating code.

`format!` is the same machinery, but it hands you a `String` rather than writing to
stdout:

```rust
fn main() {
    let name = "Ferris";
    // Same macro family: `format!` builds a String instead of printing it.
    let msg = format!("Hello, {}!", name);
    println!("{}", msg);
}
```

## The rule

1. A name followed by `!` is a macro, not a function. `println!`, `format!`, `vec!`,
   `assert_eq!` are all macros. Forgetting the `!` is a compile error, never a silent
   bug.
2. Execution starts at `fn main()`. It takes no parameters and returns nothing by
   default.
3. Statements end with a semicolon. Most lines of Rust do.
4. Rust compiles ahead of time. `rustc main.rs` produces a standalone binary.
   `cargo build` does the same while managing dependencies. `cargo run` builds, then runs.
5. Cargo expects your source under `src/`, with `Cargo.toml` at the project root
   declaring the package name, version, and edition.

## Worked example

```rust
fn main() {
    let language = "Rust";
    let year = 2010;
    println!("{} first appeared in {}.", language, year);
    println!("Twelve doubled is {}.", double(12));
}

fn double(n: i32) -> i32 {
    n * 2
}
```

Two things worth noticing. `double` is declared *after* `main` and that is fine — Rust
does not care about declaration order at module scope, unlike a Python script read top
to bottom. And `double`'s body is `n * 2` with no semicolon and no `return`; the last
expression in a function body is its return value. That idea gets a full lesson in
module 03.

## The error you will hit

Drop the `!` and the compiler is unusually direct about it:

```rust
fn main() {
    println("Hello from Rust!");
}
```

```
error[E0423]: expected function, found macro `println`
 --> src/main.rs:2:5
  |
2 |     println("Hello from Rust!");
  |     ^^^^^^^ not a function
  |
help: use `!` to invoke the macro
  |
2 |     println!("Hello from Rust!");
  |            +
```

Read that from the bottom up, which is how you should read most Rust errors. The `help:`
block shows the exact fix and the `+` under the caret line marks the character to
insert. The `error[E0423]` code is a lookup key: `rustc --explain E0423` prints a full
page on it. Rust's compiler is verbose on purpose — it is trying to be a code reviewer,
not a gatekeeper.

## What's next

Three exercises. `hello_world` is the missing-`!` error you just read about.
`cargo_basics` is about statement termination and Cargo's project layout.
`first_function` asks you to write a function that takes a `&str` and returns a `String`
— reach for `format!`.

Source: https://doc.rust-lang.org/book/ch01-02-hello-world.html
