# The Lifetime Crucible

You write `fn longest(a: &str, b: &str) -> &str`, and rustc answers `missing lifetime
specifier`. You did not make a mistake. The compiler is asking a question your signature
never answered: when the caller holds onto that returned `&str`, which of the two inputs
has to still be alive for it to be valid?

## Coming from Python/JS

In Python and JS an object lives as long as anything can reach it. A refcount or a tracing
GC tracks that for you, forever, at runtime, and you never think about it.

Rust has no GC and no default refcount. A `&str` is a pointer plus a length, and nothing
about it keeps the buffer it points into alive. So the compiler proves, before your program
runs, that every reference dies before its data does. Inside one function body it can see
every scope and infers everything. Across a function boundary it sees only the signature —
so when a returned reference could have come from more than one input, the signature has to
say which.

That's the whole design. `'a` is not a runtime value and not a GC you program by hand. It is
a parameter in a signature, exactly like `T`, erased before code generation.

## The rule

1. Every reference has a lifetime. Almost always it is inferred and invisible.
2. `'a` **does not change how long anything lives.** It states a constraint: this returned
   reference is valid for at most as long as the shortest input tied to `'a`.
3. You must annotate only when the compiler cannot tell which input a returned reference
   borrows from.
4. A struct holding a reference must carry a lifetime parameter — `struct Excerpt<'a> { text:
   &'a str }` — and its `impl` block declares it too: `impl<'a> Excerpt<'a>`. It means the
   struct may not outlive the data it points at. Forget it and you get the same E0106.
5. **Elision.** The compiler fills in three patterns so you don't have to:
   a. Each reference parameter gets its own lifetime.
   b. If there is exactly one input lifetime, it is assigned to every output lifetime.
   c. If one parameter is `&self` or `&mut self`, *its* lifetime goes to every output.
   If those three leave an output lifetime unassigned, you get E0106. That is the only
   situation that forces you to write one by hand.

And if you ever "fix" a lifetime error by returning `String` instead of `&str`, you bought a
heap allocation to avoid four keystrokes. Notice when you're doing it.

## Worked example

```rust
struct Header<'a> {
    name: &'a str,
    value: &'a str,
}

impl<'a> Header<'a> {
    // The struct borrows from `line`, so `line` must outlive the Header.
    fn parse(line: &'a str) -> Option<Header<'a>> {
        let (name, value) = line.split_once(": ")?;
        Some(Header { name, value })
    }

    // Elision rule (c): `&self` donates its lifetime to the return type,
    // so you write no annotation here at all.
    fn tld(&self) -> &str {
        self.value.rsplit('.').next().unwrap_or("")
    }
}

// Two input references, one returned reference. Elision gives up, so you
// state which input the result borrows from: `line`, never `key`.
fn value_of<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let (name, value) = line.split_once(": ")?;
    if name.eq_ignore_ascii_case(key) {
        Some(value)
    } else {
        None
    }
}

fn main() {
    let raw = String::from("Host: api.example.com");
    let header = Header::parse(&raw).unwrap();
    assert_eq!(header.name, "Host");
    assert_eq!(header.tld(), "com");
    assert_eq!(value_of(&raw, "host"), Some("api.example.com"));
    assert_eq!(value_of(&raw, "accept"), None);
}
```

## The error you will hit

Two reference parameters, a returned reference, and no annotation:

```rust
fn value_of(line: &str, key: &str) -> Option<&str> {
    let (name, value) = line.split_once(": ")?;
    if name == key { Some(value) } else { None }
}
```

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:46
  |
1 | fn value_of(line: &str, key: &str) -> Option<&str> {
  |                   ----       ----            ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `line` or `key`
help: consider introducing a named lifetime parameter
  |
1 | fn value_of<'a>(line: &'a str, key: &'a str) -> Option<&'a str> {
  |            ++++        ++            ++                 ++
```

The `help:` sentence names the exact ambiguity. But rustc's suggested fix ties *both* inputs
to `'a` — the safe default, not always the right one. Here the result only comes from `line`,
so tying `key` to `'a` would force callers to keep their key alive for nothing. Decide which
inputs the return value really borrows from, and annotate only those.

## What's next

`lifetime_basics` is E0106 six times: ask where each returned reference comes from, then tie
only those inputs. `struct_lifetimes` is rule 4 — the same error on fields and `impl` blocks.
`lifetime_elision` already compiles; its bugs are what a developer wrote *after* giving up on
rule 2, including two needless `String` allocations.

Source: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
