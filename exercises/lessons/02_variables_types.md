# The Binding Forge

`let x = 5;` followed by `x = 6;` does not compile. In Rust a `let` binding is immutable
by default, and reassigning it is an error you have to opt out of. The escape hatches
are two different things — `mut` and shadowing — and picking the wrong one is the second
surprise.

## Coming from Python/JS

In Python every name is rebindable and retypeable: `x = 5` then `x = "five"` is legal.
In JavaScript, `let` is rebindable and `const` is not, but `const` only freezes the
binding — `const a = []; a.push(1)` works fine.

Rust inverts the default. `let` gives you an immutable binding, and you write `let mut`
when you want to reassign. That is not moralising about functional purity; it is
load-bearing. Immutability is what lets the compiler prove that a value nobody can write
to is safe to share, which is the entire foundation of the borrow checker in modules 04
and 05. `mut` is also documentation: a reader sees at the declaration site which
variables change.

The type story is different too. Rust is statically typed but infers aggressively, so
you rarely annotate — you write `let x = 5` and get an `i32`. You annotate when
inference has more than one valid answer, most commonly with `.parse()`, which can
produce any numeric type and needs to be told which.

For values that are constant program-wide, `const` is a third thing: always immutable,
type annotation mandatory, usable at any scope including the top level.

```rust
const MAX_PLAYERS: u32 = 4;

fn main() {
    println!("up to {} players", MAX_PLAYERS);
}
```

## The rule

1. `let` bindings are immutable. Add `mut` to allow reassignment.
2. `mut` lets you change the *value*, never the *type*. The type is fixed at
   declaration.
3. Shadowing — writing `let` again with the same name — creates a **new** binding.
   Because it is a new binding, it may have a different type. Use it for
   transformations.
4. Types are inferred where possible. Annotate with `name: Type` when the compiler says
   it cannot pick.
5. Integer arithmetic on integers stays integral. `7 / 2` is `3`, not `3.5`. Division
   truncates toward zero, and the truncation happens at every step of an expression.

Rule 5 is the one that bites hardest, because it produces wrong answers instead of
errors.

## Worked example

```rust
fn main() {
    // Immutable by default — this binding will never change.
    let max_health = 100;

    // Opt in to mutation with `mut`.
    let mut health = max_health;
    health -= 30;
    println!("{}/{} HP", health, max_health);

    // Shadowing: a brand-new binding that reuses the name, so the type may change.
    let raw = "42";
    let raw: u32 = raw.parse().expect("not a number");
    println!("parsed and incremented: {}", raw + 1);

    // Annotate when inference has no single answer.
    let exact: f64 = 7.0 / 2.0; // 3.5
    let whole: i32 = 7 / 2;     // 3 — integer division truncates toward zero
    println!("{} vs {}", exact, whole);
}
```

The `raw` lines are the key contrast. `raw` goes from `&str` to `u32` while keeping its
name, and it stays immutable the whole time — you never had a mutable variable, you had
two immutable ones in sequence. Doing the same with `mut` instead of a second `let`
fails, because `mut` cannot change a type.

## The error you will hit

```rust
fn main() {
    let attempts = 3;
    println!("attempts: {}", attempts);
    attempts = 4;
    println!("attempts: {}", attempts);
}
```

```
error[E0384]: cannot assign twice to immutable variable `attempts`
 --> src/main.rs:4:5
  |
2 |     let attempts = 3;
  |         -------- first assignment to `attempts`
3 |     println!("attempts: {}", attempts);
4 |     attempts = 4;
  |     ^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut attempts = 3;
  |         +++
```

Note that the compiler points at *two* lines: the caret marks the illegal assignment,
and the dashes upstream mark the declaration that made it illegal. Rust errors almost
always show you both the crime and the cause. The `help:` here suggests `mut` — correct
when you want to change the value, but if what you actually want is a new type, reach
for shadowing instead.

## What's next

`immutable_by_default` is exactly the E0384 above. `shadowing` asks you to change a
binding's type mid-function — remember that the annotation must describe the *new*
value, not the old one. `type_annotations` compiles cleanly and still returns wrong
numbers; rule 5 is why.

Source: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
