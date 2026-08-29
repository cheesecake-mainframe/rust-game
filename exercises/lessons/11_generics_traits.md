# The Trait Nexus

You write `fn largest<T>(items: &[T]) -> &T`, the body reads fine, and rustc refuses it
because `T` doesn't implement `PartialOrd`. That is not the compiler being pedantic. A
generic `T` in Rust starts with **zero** capabilities — it cannot be printed, compared,
copied, or added — and you hand them back one trait bound at a time.

## Coming from Python/JS

In Python you pass anything into a function and hope it has `.area()`. If it doesn't, you
find out at runtime, in production. TypeScript improves on that with *structural* typing:
any object whose shape matches `interface Shape` satisfies it, checked at compile time and
erased before the JS ever runs.

Rust does neither. Traits are **nominal** — a type implements `Scored` only because someone
wrote `impl Scored for Quiz`. Having a method with the right name and signature is not
enough. And generic code is **monomorphized**: rustc stamps out a separate, fully
specialized copy of your function for every concrete `T` you call it with. There is no
vtable, no boxing, no dynamic lookup.

Those two choices are exactly why bounds are mandatory. rustc type-checks the generic body
*once*, before it knows any concrete `T`, so it can only permit what the bounds promise.
What you buy is abstraction that costs nothing at runtime, plus no accidental conformance —
some unrelated type never satisfies your trait by coincidence of naming.

## The rule

1. A generic `T` can do nothing until a bound says otherwise. `{}` needs `Display`, `{:?}`
   needs `Debug`, `>` needs `PartialOrd`, `.clone()` needs `Clone`, `+` needs
   `Add<Output = T>`, `T::default()` needs `Default`.
2. Three equivalent spellings: `fn f<T: Display>(x: T)`, a `where T: Display` clause, or
   `fn f(x: impl Display)` in argument position. Stack bounds with `+`.
3. `trait` groups method signatures. A signature ending in `;` is required; a signature with
   a body is a default that implementors get free and may override. Default bodies may call
   the required methods.
4. Bounds on an `impl` block are separate from the struct's own. `impl<T: Display>
   Wrapper<T>` means "these methods exist only when `T` is `Display`" — one type can have
   several `impl` blocks with different bounds, each offering different methods.
5. Orphan rule: you may write `impl Trait for Type` only if you own the trait or the type.

## Worked example

```rust
trait Scored {
    fn score(&self) -> u32;

    // A default method. Implementors get it for free, and it may call
    // the required methods above it.
    fn passed(&self) -> bool {
        self.score() >= 60
    }
}

struct Quiz { correct: u32, total: u32 }
struct Essay { rubric_points: u32 }

impl Scored for Quiz {
    fn score(&self) -> u32 {
        self.correct * 100 / self.total
    }
}

impl Scored for Essay {
    fn score(&self) -> u32 {
        self.rubric_points
    }
    fn passed(&self) -> bool {
        self.rubric_points >= 70 // override the default
    }
}

// `T: Scored` is the promise. rustc generates one machine-code copy of
// `best` per concrete T you actually call it with.
fn best<T: Scored>(items: &[T]) -> Option<&T> {
    items.iter().max_by_key(|item| item.score())
}

// Identical meaning, `where`-clause form. Prefer it once bounds get long.
fn pass_rate<T>(items: &[T]) -> f64
where
    T: Scored,
{
    if items.is_empty() {
        return 0.0;
    }
    let passing = items.iter().filter(|i| i.passed()).count();
    passing as f64 / items.len() as f64
}

fn main() {
    let quizzes = vec![
        Quiz { correct: 9, total: 10 },
        Quiz { correct: 4, total: 10 },
    ];
    assert_eq!(best(&quizzes).unwrap().score(), 90);
    assert_eq!(pass_rate(&quizzes), 0.5);
    assert!(!Essay { rubric_points: 65 }.passed());
}
```

## The error you will hit

Using a `T` you never constrained:

```rust
fn describe<T>(value: T) -> String {
    format!("value = {value}")
}
```

```
error[E0277]: `T` doesn't implement `std::fmt::Display`
 --> src/main.rs:2:22
  |
2 |     format!("value = {value}")
  |                      ^^^^^^^ `T` cannot be formatted with the default formatter
  |
help: consider restricting type parameter `T` with trait `Display`
  |
1 | fn describe<T: std::fmt::Display>(value: T) -> String {
  |              +++++++++++++++++++
```

Read the `help:` block. On missing-bound errors rustc writes the exact fix, including the
`+++` marks showing where the characters go. This is the friendliest error class in Rust —
treat it as autocomplete, not as a rejection.

## What's next

`generic_functions` is that error six times over: read each body, name the capability it
uses, add the bound. `defining_traits` is rule 3 — a required method plus a default that
calls it. `trait_bounds` pushes bounds onto structs and `impl` blocks. `boss_traits` is
rule 4 end to end: one generic collection, several `impl` blocks, different bounds each.

Source: https://doc.rust-lang.org/book/ch10-02-traits.html
