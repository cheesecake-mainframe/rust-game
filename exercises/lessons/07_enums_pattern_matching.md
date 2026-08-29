# The Pattern Archives

Rust has no `null` and no `undefined`. A missing value is a *different type* —
`Option<T>` — and the compiler will not let you read the inside of one without first
proving it is there. The same machinery makes a `match` that forgets a case a compile
error, not a bug you find in production.

## Coming from Python/JS

The closest thing you know is a TypeScript discriminated union:

```ts
type Event =
  | { kind: "tick" }
  | { kind: "key"; ch: string }
  | { kind: "resize"; width: number; height: number };
```

You then `switch (e.kind)` and hope you covered every case. In Python you would reach for
a class hierarchy or a dict with a `"type"` key. Nothing stops you reading `e.ch` on a
resize event; you get `undefined` or an `AttributeError` at runtime.

A Rust `enum` is that union, enforced. Each variant carries its own data, the tag is part
of the value, and the only way to reach the data is to `match` — which proves which
variant you are in before it hands you the fields. Two things fall out of that. Illegal
states stop being representable: there is no `Event` with a `ch` and no `kind`. And when
you add a variant a year from now, every `match` that must change turns into a compile
error instead of a silent fallthrough.

`Option<T>` is not a special language feature; it is just this mechanism applied to
absence:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

That is why Rust needs no null checks. A `String` is always a string. Only an
`Option<String>` can be absent, and the type says so at every boundary it crosses — the
opposite of Python, where any name might be `None` and only the docs (or a crash) tell
you.

## The rules

1. **Variants come in three shapes**: unit (`Tick`), tuple (`Key(char)`), and struct-like
   (`Resize { width, height }`). They are namespaced under the enum: `Event::Tick`.
2. **`match` must be exhaustive.** Arms are tried top to bottom; `_` (or a plain name,
   which binds the value) catches everything remaining and must come last.
3. **`match` is an expression**, so every arm must produce the same type:

```rust
fn level_name(level: u8) -> String {
    // `match` is an expression: every arm must produce the same type.
    let name = match level {
        0 => "off",
        1 | 2 => "low",       // or-pattern
        3..=5 => "high",      // inclusive range
        _ => "unknown",       // catch-all, must come last
    };
    name.to_string()
}
```

4. **Matching a reference gives you references.** Inside `match self { ... }` the bindings
   are `&f64`, `&String`, and so on — you read the data without moving it out.
5. **`if let` handles exactly one variant**, and `let PATTERN = expr else { ... };` binds
   into the surrounding scope, with an `else` block that must diverge (`return`, `break`,
   `continue`, or `panic!`).

## Worked example

```rust
enum Event {
    Tick,
    Key(char),
    Resize { width: u32, height: u32 },
}

impl Event {
    fn describe(&self) -> String {
        match self {
            Event::Tick => "clock ticked".to_string(),
            Event::Key(c) => format!("key {c} pressed"),
            Event::Resize { width, height } => format!("resized to {width}x{height}"),
        }
    }
}

fn first_digit(text: &str) -> Option<char> {
    for c in text.chars() {
        if c.is_ascii_digit() {
            return Some(c);
        }
    }
    None
}

fn main() {
    let events = [Event::Tick, Event::Key('q'), Event::Resize { width: 80, height: 24 }];
    for e in &events {
        println!("{}", e.describe());
    }

    if let Some(d) = first_digit("port 8080") {
        println!("first digit: {d}");
    }

    let Some(d) = first_digit("no digits here") else {
        println!("nothing to parse");
        return;
    };
    println!("never reached with this input: {d}");
}
```

## The error you will hit

Leaving a variant out of a `match`:

```
error[E0004]: non-exhaustive patterns: `&Signal::Yellow` not covered
  --> src/main.rs:8:11
   |
 8 |     match signal {
   |           ^^^^^^ pattern `&Signal::Yellow` not covered
   |
note: `Signal` defined here
  --> src/main.rs:1:6
   |
 1 | enum Signal {
   |      ^^^^^^
 2 |     Red,
 3 |     Yellow,
   |     ------ not covered
   = note: the matched value is of type `&Signal`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
```

Read it as a checklist: rustc names the exact variant you skipped. Add the arm, or add
`_ => ...` if the rest genuinely share one behavior — but prefer naming variants, because
`_` is also what silently swallows the variant you add next year. Note `the matched value
is of type &Signal`: matching through a reference is normal, and the bindings you get are
references too.

## What's next

`enums_with_data` (build a `Shape` enum with a `match`-driven `area`), `if_let`,
`option_type` (find the logic bugs in code that fakes null with `Some`), and
`predict_match` (trace the arms by hand).

Source: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
