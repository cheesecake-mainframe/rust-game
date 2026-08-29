# The Flow Chamber

In Rust a trailing semicolon changes what your code *means*. `n * 2` returns a number;
`n * 2;` returns nothing. That single character is the difference between a working
function and `error[E0308]: mismatched types`, and it is the most common beginner error
in the language.

## Coming from Python/JS

Python and JavaScript draw a hard line between statements and expressions, and `return`
is the only way a function hands back a value. A block `{ ... }` in JS is just grouping;
it has no value.

Rust makes almost everything an expression. A block has a value: the value of its last
expression, provided that expression has no semicolon. `if` has a value. `match` has a
value. `loop` has a value. A function body is just a block, so the last expression in it
is the return value — `return` exists but is reserved for early exits.

```rust
fn main() {
    // A bare block is an expression. Its value is the last line, with no semicolon.
    let shipping = {
        let base = 5;
        let surcharge = 2;
        base + surcharge
    };
    println!("shipping = {}", shipping);
}
```

Why design it this way? Because it collapses several constructs into one. Rust needs no
ternary operator: `if` already produces a value. It needs no `switch` statement separate
from a `switch` expression. And it makes "compute a value across several steps, then
bind it once" expressible without a mutable temporary — which keeps bindings immutable,
which is what the borrow checker wants.

The cost is that the semicolon is now semantically significant. Adding one turns an
expression into a statement, and a statement evaluates to `()` — the *unit type*, Rust's
"no meaningful value". `()` is a real type, which is why the mismatch is caught at
compile time.

## The rule

1. A block evaluates to its final expression, if that expression has no trailing
   semicolon.
2. Adding a semicolon turns an expression into a statement, whose value is `()`.
3. Every arm of an `if`/`else` or a `match` used as a value must have the same type.
4. `if` requires an actual `bool`. There is no truthiness: `if 3 {}` is an error, and so
   is `if name {}`. Write the comparison out.
5. `match` must be exhaustive. Cover every case or add a `_` catch-all; the compiler
   will refuse to compile a match with a hole in it.
6. Ranges: `1..5` is exclusive and yields 1, 2, 3, 4. `1..=5` is inclusive and yields 1
   through 5. Off-by-one bugs live here.
7. `break` can carry a value out of a `loop`. `while` and `for` cannot.

## Worked example

```rust
fn describe(temp: i32) -> &'static str {
    // `match` is an expression, and it must be exhaustive.
    // Every arm has to produce the same type.
    match temp {
        i32::MIN..=-1 => "below freezing",
        0 => "freezing",
        1..=15 => "cold",
        16..=25 => "mild",
        _ => "hot",
    }
}

fn main() {
    // `if` is an expression — this is Rust's ternary.
    let n = 7;
    let parity = if n % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", n, parity);

    // `..=` is inclusive; a bare `..` would stop at 3.
    let mut total = 0;
    for i in 1..=4 {
        total += i;
    }
    println!("1+2+3+4 = {}", total);

    // `loop` can hand a value back out through `break`.
    let mut attempts = 0;
    let needed = loop {
        attempts += 1;
        if attempts * attempts >= 20 {
            break attempts;
        }
    };
    println!("needed {} attempts", needed);

    println!("20C is {}", describe(20));
}
```

`describe` has no `return` and no semicolon after the `match` — the match *is* the
return value. Note the arms are checked in order, so `16..=25` only sees temperatures
that failed every earlier arm.

## The error you will hit

```rust
fn tax_cents(amount: i32) -> i32 {
    amount * 8 / 100;
}
```

```
error[E0308]: mismatched types
 --> src/main.rs:1:30
  |
1 | fn tax_cents(amount: i32) -> i32 {
  |    ---------                 ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     amount * 8 / 100;
  |                     - help: remove this semicolon to return this value
```

`found ()` is the tell. Any time an error says a function returned `()` when you
expected a real type, look for a stray semicolon on the last line. The compiler even
underlines the exact character. The same error appears when the arms of an `if` used as
a value end in semicolons — those semicolons discard the value you wanted back.

## What's next

`expressions` is the semicolon problem in three flavours, including a `match` used as a
value. `loops` compiles but computes wrong answers — check every loop bound against rule
6, and check the declared types of the counters while you are there. `match_basics` asks
you to write three matches from scratch; let exhaustiveness checking guide you. The
Book's control-flow chapter has the full loop tour:
https://doc.rust-lang.org/book/ch03-05-control-flow.html

Source: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
