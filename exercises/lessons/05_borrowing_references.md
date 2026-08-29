# The Borrow Sanctum

Module 04 left you with a problem: if passing a `String` to a function consumes it,
every function call needs the value handed back. The fix is `&` — borrowing. The new
surprise is that you cannot hold a `&` and a `&mut` to the same value at the same time,
and the compiler means it.

## Coming from Python/JS

In Python and JavaScript you are already passing references everywhere, implicitly and
without limit. Ten names can point at one list, and any of them can mutate it. That is
convenient right up to the moment something changes a list you were iterating over, or
two threads write the same dict, and you spend an afternoon on it.

Rust gives you the same reference, but attaches a rule: **aliasing or mutation, never
both.** Any number of readers, or exactly one writer. Nothing in between.

This is not caution for its own sake. It is exactly the condition that makes data races
impossible — a data race needs two pointers to one value with at least one of them
writing, and rule 1 below forbids that arrangement by construction. It is also what
makes iterator invalidation impossible: you cannot push to a `Vec` while a reference
into it is alive, because pushing may reallocate and leave your reference dangling.
Languages that allow this detect it at runtime, or not at all. Rust rejects it at
compile time. That is why module 15 can call itself "fearless concurrency" — the
guarantee was bought here, in module 05.

## The rule

1. At any given time you may have **either** one mutable reference **or** any number of
   immutable references to a value. Not both.
2. References must always be valid. A reference can never outlive the value it points
   to, so Rust rejects returning a reference to a local.
3. References are immutable by default, like bindings. `&T` reads; `&mut T` reads and
   writes.
4. To take `&mut x`, the owner `x` must itself be declared `mut`.
5. A borrow's lifetime runs from where it is created to its **last use**, not to the end
   of the enclosing block. This is why some code that looks like it violates rule 1
   compiles.
6. A slice — `&str`, `&[T]` — is a borrowed view into part of a collection. Its contents
   are not copied; it is a pointer plus a length. Prefer `&str` over `&String` in
   parameters: it accepts both, plus string literals.

Rule 5 is what makes the rules livable in practice:

```rust
fn main() {
    let mut log = String::from("boot");
    let snapshot = &log;
    println!("{}", snapshot); // last use of the immutable borrow
    log.push_str(" ok");      // borrow is over, so mutating is now allowed
    println!("{}", log);
}
```

## Worked example

```rust
fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn shout(text: &mut String) {
    text.push_str(" !!!");
}

fn main() {
    let mut banner = String::from("welcome aboard");

    // Any number of immutable borrows may coexist.
    let a = &banner;
    let b = &banner;
    println!("{} words: {} / {}", word_count(a), a, b);
    // `a` and `b` are never used again, so their borrows end right here.

    // Now a single mutable borrow is fine. Note the owner must be `mut` too.
    shout(&mut banner);
    println!("{}", banner);

    // A slice is a borrowed VIEW into the bytes — no allocation, no copy.
    let head: &str = &banner[..7];
    println!("head = {}", head);

    // `banner` owned its bytes the entire time; nothing was ever moved.
    println!("owner still has: {}", banner);
}
```

That prints `welcome aboard !!!` and `head = welcome`. `banner` is never moved, never
cloned, and never copied — the two functions and the slice all work on borrowed views of
one allocation. Also note `word_count(a)`: `a` is a `&String` and the parameter is
`&str`, and Rust converts automatically. That is why rule 6 says to write `&str`.

## The error you will hit

```rust
fn main() {
    let mut log = String::from("boot");
    let snapshot = &log;
    log.push_str(" ok");
    println!("{}", snapshot);
}
```

```
error[E0502]: cannot borrow `log` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let snapshot = &log;
  |                    ---- immutable borrow occurs here
4 |     log.push_str(" ok");
  |     ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("{}", snapshot);
  |                    -------- immutable borrow later used here
```

The third annotation is the important one, and it is the one people skim past. The
mutation on line 4 is only illegal *because* line 5 uses `snapshot` afterwards — that
use is what extends the borrow across line 4. Delete line 5, or move it above line 4,
and the same code compiles. Whenever you hit E0502, look at "immutable borrow later used
here" first: shrinking the borrow's live range is usually a smaller fix than
restructuring, and `.clone()` is almost never the right answer here.

You will also meet `E0596` — cannot borrow as mutable, as it is behind a `&` reference,
meaning you wrote `&T` where you needed `&mut T`. And `E0106` — missing lifetime
specifier, meaning you tried to return a reference to a local, breaking rule 2. Both
name the fix in the `help:` block.

## What's next

`immutable_references` converts an owning function to a borrowing one — change the
signature, then the call site. `mutable_references` is the `&`-to-`&mut` fix, and
remember rule 4: the owner needs `mut` as well. `reference_rules` is rules 1 and 2 back
to back; for the second, ask yourself what the reference would point at once the block
ends. `string_slices` has you build `&str` views by hand with range syntax. Then
`boss_borrowing`: a `TextAnalyzer` holding `&'a str` that analyses text it never owns,
returning slices that borrow from it.

Source: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
