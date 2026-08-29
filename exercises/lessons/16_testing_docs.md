# The Quality Gate

Your tests do not live in a `tests/` folder that a runner discovers by scanning filenames.
They live in the same file as the code they test, inside a module the compiler deletes
from release builds. And the code examples in your `///` documentation comments get
compiled and executed on every `cargo test` — documentation that lies is a build failure.

## Coming from Python/JS

pytest finds `test_*.py` by convention; jest finds `*.test.ts`. Discovery happens at
runtime, by scanning the filesystem, and your tests import the module under test the same
way any other consumer would — so they only see the public surface, and reaching a
`_private_helper` is a hack.

Rust does none of that. `#[test]` is an attribute the compiler reads, so the list of tests
is assembled at compile time: no scanning, no reflection, no import machinery. The tests
sit in a child module of the code they test, and in Rust a child module can see its
parent's private items. Testing a non-`pub` function is normal and sanctioned, not a
workaround.

The cost of compiling tests into the same crate is binary size and compile time, so Rust
pays it only when you ask. `#[cfg(test)]` is conditional compilation: that module is not
compiled *at all* unless the `test` cfg flag is set, which `cargo test` sets and
`cargo build` does not. This is why the exact spelling matters. `#[cfg(testing)]` is not
an error — `cfg` accepts arbitrary names — so the module never compiles, and you get:

```text
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored
```

A green bar that ran nothing. Read the count, not the color.

## The rule

1. Unit tests go in `#[cfg(test)] mod tests { ... }` in the same file as the code.
2. The test module starts empty. Pull in the parent's items with `use super::*;`.
3. `#[test]` marks a test function. It takes no arguments. A helper function inside the
   test module must **not** carry `#[test]` — helpers take arguments and return values,
   which a test function may not.
4. Assert with `assert!`, `assert_eq!`, `assert_ne!`. To assert a panic, use
   `#[should_panic(expected = "...")]`; the string is matched as a *substring* of the
   panic message.
5. Doc comments use `///` and their fenced blocks run as tests — but only for **library**
   targets. A `src/main.rs` binary's doc tests are never run by `cargo test`; run them
   with `rustdoc --test`. Tag a doc example's fence with `should_panic` when the example
   is supposed to panic.

## Worked example

```rust
/// Converts a Celsius temperature to Fahrenheit.
///
/// # Examples
///
/// ```
/// use quality_gate::to_fahrenheit;
///
/// assert_eq!(to_fahrenheit(100.0), 212.0);
/// ```
///
/// # Panics
///
/// Panics below absolute zero, which is not a physical temperature:
///
/// ```should_panic
/// use quality_gate::to_fahrenheit;
///
/// to_fahrenheit(-300.0);
/// ```
pub fn to_fahrenheit(celsius: f64) -> f64 {
    assert!(celsius >= -273.15, "below absolute zero: {celsius}");
    celsius * 9.0 / 5.0 + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freezing_point_is_32() {
        assert_eq!(to_fahrenheit(0.0), 32.0);
    }

    #[test]
    #[should_panic(expected = "below absolute zero")]
    fn rejects_impossible_temperature() {
        to_fahrenheit(-300.0);
    }
}
```

`cargo test` on that library runs four tests, not two:

```text
running 2 tests
test tests::freezing_point_is_32 ... ok
test tests::rejects_impossible_temperature - should panic ... ok

   Doc-tests quality_gate

running 2 tests
test src/lib.rs - to_fahrenheit (line 5) ... ok
test src/lib.rs - to_fahrenheit (line 15) - should panic ... ok
```

## The error you will hit

You write the test module in a single-file exercise, forget `use super::*;`, and the
function you are testing is suddenly invisible:

```rust
fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("{}", to_fahrenheit(0.0));
}

#[cfg(test)]
mod tests {
    #[test]
    fn freezing_point_is_32() {
        assert_eq!(to_fahrenheit(0.0), 32.0);
    }
}
```

```text
error[E0425]: cannot find function `to_fahrenheit` in this scope
  --> src/main.rs:13:20
   |
13 |         assert_eq!(to_fahrenheit(0.0), 32.0);
   |                    ^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function
   |
11 +     use crate::to_fahrenheit;
   |
```

`mod tests` is a real module with its own namespace, not a decorated block. It inherits
nothing. `use super::*;` says "everything from my parent" and is the idiomatic fix;
`use crate::to_fahrenheit;`, which the compiler suggests, works too but does not scale.
Note the shape of the mistake: `use super::tests;` imports the test module *into itself*,
which compiles but brings in nothing useful.

## What's next

**unit_tests** asks you to write a full test module from scratch, including
`#[should_panic]` cases for `factorial` and `clamp`. **test_organization** hands you a
test module with six broken pieces — a misspelled `cfg`, a bad import, a helper wearing
`#[test]`, a misspelled attribute. **doc_tests** makes you repair documentation examples
until they compile and pass.

Source: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
