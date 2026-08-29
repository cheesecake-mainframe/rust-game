# The Iterator Engine

Two things bite here. First, a closure you return from a function gets rejected with "closure
may outlive the current function" — because in Rust a closure is a value with captured
fields, not a pointer into a garbage-collected scope. Second, `.iter()` hands your closure a
`&T`, the types don't line up, and `.clone()` looks like the obvious way out. It usually
isn't.

## Coming from Python/JS

A JS closure captures the enclosing scope and the GC keeps that scope alive as long as the
closure is reachable. Python captures by cell, same story. You never decide *how* something
is captured, because there is only one way.

In Rust a closure is an anonymous struct whose fields are its captures. Whether a capture is
stored as `&T`, `&mut T`, or an owned `T` is inferred from what the body does — and it's part
of the type. Nothing else keeps that data alive, so a closure that outlives the scope it was
written in has to *own* what it captured. That's what `move` is for.

Iterators diverge just as sharply. `[x * 2 for x in xs]` in Python and `xs.map(f).filter(g)`
in JS both build real intermediate arrays. Rust's adaptors are lazy: `.map()` returns a `Map`
struct wrapping the previous iterator and does no work. The chain collapses into a single
loop with zero intermediate allocation — the "zero-cost abstraction" claim is literal here;
a chain and a hand-written `for` loop emit the same machine code. Laziness costs you this:
you must say what drives the chain, and what you're collecting *into*.

## The rule

1. Capture mode is inferred from the body: reads → `&T`, mutates → `&mut T`, consumes → `T`.
   `move` forces by-value capture of everything.
2. Three closure traits, nested. `FnOnce` — callable at least once, may consume its captures.
   `FnMut` — callable repeatedly, may mutate them. `Fn` — callable repeatedly, only reads.
   Every closure is `FnOnce`; most are also `FnMut`; only non-mutating ones are `Fn`. Pass a
   mutating closure to a parameter bounded `F: Fn` and it will not fit.
3. `.iter()` yields `&T`. `.iter_mut()` yields `&mut T`. `.into_iter()` yields `T` and
   consumes the collection. Choosing correctly is most of the gap between a clone-heavy
   version and a zero-copy one.
4. Nothing runs until a consumer pulls: `collect`, `sum`, `count`, `fold`, `any`, `all`,
   `max_by_key`, `for_each`, or a `for` loop. An unconsumed chain is dead code.
5. `collect()` cannot guess its target. Annotate the binding (`let v: Vec<_> = ...`) or use
   the turbofish (`.collect::<Vec<_>>()`).

The Python translations are mostly one-to-one: comprehension → `.map().collect()`, filtered
comprehension → `.filter().map().collect()`, `any(...)`/`all(...)` → `.any()`/`.all()`,
`sum(...)` → `.sum()`, nested comprehension → `.flat_map()`, and a running accumulator →
`.scan()`.

## Worked example

```rust
struct Track {
    title: String,
    seconds: u32,
}

// Borrow instead of clone: each &str points into a Track's own title.
fn titles_over(tracks: &[Track], min_secs: u32) -> Vec<&str> {
    tracks
        .iter()                            // yields &Track
        .filter(|t| t.seconds >= min_secs) // lazy: nothing has run yet
        .map(|t| t.title.as_str())         // &String -> &str, no allocation
        .collect()                         // this is what drives the chain
}

fn main() {
    let tracks = vec![
        Track { title: String::from("Aurora"), seconds: 240 },
        Track { title: String::from("Ember"), seconds: 95 },
    ];

    assert_eq!(titles_over(&tracks, 120), vec!["Aurora"]);

    // `longest` is captured by &mut, which makes this closure FnMut.
    let mut longest = 0;
    tracks.iter().for_each(|t| longest = longest.max(t.seconds));
    assert_eq!(longest, 240);

    let total: u32 = tracks.iter().map(|t| t.seconds).sum();
    assert_eq!(total, 335);
}
```

Note the elided lifetime on `titles_over`: one input lifetime, so the returned `&str`s tie to
`tracks`. No annotation needed, no allocation performed.

## The error you will hit

A closure that borrows a local, then escapes the function:

```rust
fn make_greeter() -> Box<dyn Fn() -> String> {
    let name = String::from("Ada");
    Box::new(|| format!("Hello, {name}!"))
}
```

```
error[E0373]: closure may outlive the current function, but it borrows `name`, which is owned by the current function
 --> src/main.rs:3:14
  |
3 |     Box::new(|| format!("Hello, {name}!"))
  |              ^^                  ---- `name` is borrowed here
  |              |
  |              may outlive borrowed value `name`
  |
note: closure is returned here
help: to force the closure to take ownership of `name` (and any other referenced variables), use the `move` keyword
  |
3 |     Box::new(move || format!("Hello, {name}!"))
  |              ++++
```

`move` is not a performance switch and not a hint — it changes the closure's captured fields
from references to owned values, which is the only way the returned closure can be valid.

## What's next

`closure_basics` is rules 1 and 2: each broken closure is capturing the wrong way or claiming
the wrong `Fn` trait. `iterator_chains` and `python_to_rust` are rules 3-5 — reach for the
adaptor, never a `for` loop. `zero_copy` is rule 3 alone: every `.clone()` in it exists
because someone returned an owned type where a borrowed one would do.

Source: https://doc.rust-lang.org/book/ch13-01-closures.html
