# The Macro Forge

`macro_rules!` is not a function and it is not a decorator. It is a pattern matcher that
runs over the *tokens* you typed, splicing new syntax into your program before the
compiler has decided what any of it means. That is why `stringify!(x + 1)` can hand you
back the string `"x + 1"`, and why a macro can emit an entire `impl` block — something no
function will ever do.

## Coming from Python/JS

A Python decorator is a runtime construct: by the time `@memoize` runs, `def slow(n)` has
already been parsed, compiled to bytecode, and turned into a function object. The
decorator receives that object and returns another one. Same for `getattr`, metaclasses,
and `exec` — everything happens after parsing, on live values. JS is the same story;
Babel and TypeScript do transform real source, but they are a separate build step, not
something you write inline in the program they transform.

Rust macros run *during* compilation, after parsing but before type checking, and they
operate on token trees. Three consequences fall straight out of that:

- A macro can be variadic. `println!("hi")` and `println!("{} {}", a, b)` are the same
  macro. No function signature can express that.
- A macro can emit items — `struct`, `enum`, `fn`, `impl`. A function is called at
  runtime, and by then the type system is long since finished.
- A macro cannot inspect a type, because types do not exist yet. `$t:ty` captures the
  tokens `Vec<u8>`; it does not know that `Vec<u8>` has a `len` method.

Rust chose the pattern-matching design over "arbitrary code that mangles source" (the C
preprocessor, or Lisp's unrestricted macros) because tokens are already structured. The
macro cannot produce nonsense the parser accepts, and — critically — the expansion is
*hygienic* for local variables: a `let total = 0;` created inside a macro is invisible at
the call site, so a macro can never silently capture or clobber your bindings. C's
`#define` cannot promise that.

## The rule

1. Arms work like `match`: `(pattern) => { expansion };`, matched top to bottom, first
   match wins. Arms are separated by semicolons.
2. Captures are written `$name:fragment`. The fragment specifier tells the parser how much
   to consume: `expr`, `ident`, `ty`, `literal`, `pat`, `stmt`, `block`, `path`, `tt`,
   `item`, `lifetime`, `vis`, `meta`. `value` and `identifier` are not among them.
3. Repetition is `$( ... )` followed by an optional separator token and then `*` (zero or
   more), `+` (one or more), or `?` (zero or one). `$( $e:expr ),*` means
   comma-separated.
4. **Every repetition in the pattern needs a matching repetition in the expansion, at the
   same nesting depth.** This is the rule people break.
5. Trailing commas are opt-in: end the pattern with `$(,)?` to tolerate one.
6. A macro must be defined textually *above* its first use in the file. Unlike functions,
   order matters.

## Worked example

```rust
macro_rules! show {
    () => {
        println!("(nothing to show)");
    };
    ($( $e:expr ),+ $(,)?) => {
        $(
            println!("{} = {:?}", stringify!($e), $e);
        )+
    };
}

fn main() {
    let name = "ferris";
    let scores = vec![3, 1, 4];

    show!();
    show!(name, name.len(), scores.iter().sum::<i32>(),);
}
```

```text
(nothing to show)
name = "ferris"
name.len() = 6
scores.iter().sum::<i32>() = 8
```

Look at what the second arm did: it printed each argument's *source text* alongside its
value. A function receives `6`; it can never learn that you wrote `name.len()`. That is
the whole point of operating on syntax.

## The error you will hit

Drop the repetition from the expansion — keep `$( ... ),+` in the pattern but write the
body once — and you get an error that says nothing about macros:

```rust
macro_rules! show {
    ($( $e:expr ),+) => {
        println!("{} = {:?}", stringify!($e), $e);
    };
}

fn main() {
    let name = "ferris";
    show!(name, name.len());
}
```

```text
error: variable `e` is still repeating at this depth
 --> src/main.rs:3:42
  |
3 |         println!("{} = {:?}", stringify!($e), $e);
  |                                          ^^
```

"At this depth" is the key phrase. The pattern bound `$e` *inside* one level of
repetition, so `$e` is not a single token tree — it is a list of them. Using it outside a
`$( ... )` in the expansion asks the macro engine to splice a list into a slot that holds
one item, and it refuses. Wrap the body: `$( println!(...); )+`. The depths must match on
both sides.

## What's next

**macro_basics** is six broken `macro_rules!` definitions: invalid fragment specifiers,
mismatched capture names, missing semicolons between arms, and exactly the
repetition-depth bug above. **custom_vec** has you rebuild `vec!` — three arms, including
the `[val; count]` form. **macro_patterns** pushes into `ident`, `ty`, and `literal`
captures, generating structs and impls, and counting expressions with a repetition trick.

Source: https://doc.rust-lang.org/book/ch20-05-macros.html
