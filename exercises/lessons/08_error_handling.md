# The Error Gauntlet

Rust has no exceptions. Nothing is thrown, nothing unwinds past you silently, and there
is no `except` block that catches a failure you never knew was possible. A function that
can fail says so in its return type, and the compiler makes you deal with it. The `?`
operator is the closest thing to `try`, and it only works where the signature admits
failure.

## Coming from Python/JS

In Python, failure is invisible in the signature:

```python
def parse_port(text):
    return int(text)   # raises ValueError; nothing here says so
```

You discover `ValueError` from the docs, or from a traceback in production. JS is the
same story with `throw` and rejected promises. The Rust version puts the failure in the
type:

```rust
fn parse_port(text: &str) -> Result<u16, std::num::ParseIntError> {
    text.trim().parse()
}
```

`Result` is not magic — it is an enum exactly like the ones in the last module:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Two things follow. Reading a signature tells you what can go wrong, so you never again
wrap a call in `try` "just in case". And error handling becomes ordinary control flow —
values returned and matched — rather than a second, invisible execution path.

Rust's `panic!` covers the other half of what exceptions do: unrecoverable bugs, like
indexing past the end of a vector. A panic unwinds the thread and is not meant to be
caught. `unwrap()` and `expect()` are the bridge — they turn an `Err` into a panic. In a
test or a five-line script that is fine. In code someone else calls, it is a landmine.

## The rules

1. **`Result<T, E>` is `Ok(value)` or `Err(error)`**, and it is `#[must_use]`: ignoring a
   returned `Result` is a warning, not silence.
2. **`?` unwraps `Ok` or returns the `Err` immediately.** It is only legal in a function
   whose return type can carry the failure:

```rust
use std::num::ParseIntError;

fn parse_port(text: &str) -> Result<u16, ParseIntError> {
    let port = text.trim().parse::<u16>()?;
    Ok(port)
}

// What `?` expands to, roughly:
fn parse_port_longhand(text: &str) -> Result<u16, ParseIntError> {
    let port = match text.trim().parse::<u16>() {
        Ok(value) => value,
        Err(e) => return Err(From::from(e)),
    };
    Ok(port)
}
```

3. **That `From::from` is the hook for your own error type.** Write
   `impl From<ParseIntError> for MyError` and `?` converts automatically. Without it,
   convert by hand with `.map_err(|e| ...)`.
4. **A custom error type is three pieces**: an enum of the ways you can fail,
   `#[derive(Debug)]` (what `unwrap` and failing tests print), and `impl Display` (the
   message a human reads).
5. **`unwrap` and `expect` panic.** Prefer `match`, `?`, `unwrap_or(default)`, or
   `unwrap_or_else(|e| ...)` when the caller can keep going.

## Worked example

```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum HexError {
    WrongLength(usize),
    BadDigit(ParseIntError),
}

impl fmt::Display for HexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HexError::WrongLength(n) => write!(f, "expected 6 hex digits, got {n}"),
            HexError::BadDigit(e) => write!(f, "bad hex digit: {e}"),
        }
    }
}

impl From<ParseIntError> for HexError {
    fn from(e: ParseIntError) -> HexError {
        HexError::BadDigit(e)
    }
}

fn parse_color(hex: &str) -> Result<(u8, u8, u8), HexError> {
    let body = hex.strip_prefix('#').unwrap_or(hex);
    if body.len() != 6 {
        return Err(HexError::WrongLength(body.len()));
    }
    // `?` unwraps Ok, or returns early — converting ParseIntError
    // into HexError through the From impl above.
    let r = u8::from_str_radix(&body[0..2], 16)?;
    let g = u8::from_str_radix(&body[2..4], 16)?;
    let b = u8::from_str_radix(&body[4..6], 16)?;
    Ok((r, g, b))
}

fn main() {
    match parse_color("#1e90ff") {
        Ok((r, g, b)) => println!("rgb({r}, {g}, {b})"),
        Err(e) => println!("error: {e}"),
    }
    println!("error: {}", parse_color("#12").unwrap_err());
}
```

## The error you will hit

Reaching for `?` in a function that does not return a `Result`:

```rust
fn double(input: &str) -> u32 {
    let n: u32 = input.parse()?;
    n * 2
}
```

```
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:2:31
  |
1 | fn double(input: &str) -> u32 {
  | ----------------------------- this function should return `Result` or `Option` to accept `?`
2 |     let n: u32 = input.parse()?;
  |                               ^ cannot use the `?` operator in a function that returns `u32`
```

`?` does not handle the error; it *hands it to your caller*. A function returning `u32`
has nowhere to put it. Either change the signature to
`Result<u32, ParseIntError>` (and wrap the success in `Ok`), or decide the failure here
with `match` or `unwrap_or(0)`. That choice — propagate or handle — is the whole of Rust
error handling, and the type system makes you make it out loud.

## What's next

`result_type` (replace `unwrap` with real handling), `question_mark`, `custom_errors`
(enum plus `Display`), and the `boss_errors` battle — a CSV parser with five failure
modes.

Source: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
