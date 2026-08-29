# The Ownership Trials

Assign a `String` to a second variable, then use the first one, and your program will
not compile: `error[E0382]: borrow of moved value`. Nothing was mutated, nothing was
freed, and in every language you have used this was fine. This is Rust's central idea,
and it is worth slowing down for.

## Coming from Python/JS

In Python and JavaScript, `b = a` copies a *reference*. Both names now point at one
object, and the garbage collector frees it once nothing points at it any more. You never
think about it. The price is a runtime that has to keep tracking liveness while your
program runs, and pauses when it decides to collect.

C and C++ take the other path: you `malloc` and you `free`, with no supervision. Free
too early and you have a use-after-free; free twice and you have heap corruption; never
free and you leak. Decades of security vulnerabilities live in that gap.

Rust picks a third option. Every heap value has exactly one owning variable, and when
that variable goes out of scope Rust automatically frees the value. No GC, no manual
`free`, and no runtime cost — the whole scheme is checked at compile time and erased
before your program runs.

```rust
fn main() {
    let outer = String::from("lives to the end");
    {
        let inner = String::from("freed at the closing brace");
        println!("{}", inner);
    } // `inner` goes out of scope here: Rust calls drop, heap memory is freed.
    println!("{}", outer);
}
```

Now the catch. If `let s2 = s1;` gave you two owners of one heap allocation, both would
try to free it at scope end — a double free. Rust's answer is that assignment *moves*
ownership: `s2` becomes the owner and `s1` is marked invalid from that line onward.
Nothing is copied, nothing is slow. `s1` just stops existing as far as the compiler is
concerned.

## The rule

1. Each value has exactly one owner.
2. There can be only one owner at a time.
3. When the owner goes out of scope, the value is dropped and its memory freed.

And the consequences that actually generate errors:

4. Assigning to a new variable, passing to a function, or returning from a function all
   **move** ownership — unless the type is `Copy`.
5. `Copy` types are small, stack-only values that are trivially duplicated instead of
   moved: all integers, `f32`/`f64`, `bool`, `char`, and tuples made only of `Copy`
   types. `String`, `Vec`, and anything owning heap memory are not `Copy`.
6. `.clone()` makes a real, deep, independent copy. It always works, and it is always
   visible in the source — which is the point. Rust never deep-copies silently.

## Worked example

```rust
fn count(v: Vec<i32>) -> usize {
    v.len()
} // `v` is dropped here — the caller's Vec is gone for good.

fn hand_back(mut v: Vec<i32>) -> Vec<i32> {
    v.push(0);
    v // ownership moves out to the caller
}

fn main() {
    // i32 is Copy: `total` is duplicated, not moved. Both stay usable.
    let total = 99;
    let mirror = total;
    println!("{} and {}", total, mirror);

    // Vec is not Copy: passing it MOVES it.
    let scores = vec![10, 20, 30];
    println!("counted {}", count(scores));
    // `scores` is unusable from here on.

    // Want it back? Return it.
    let padded = hand_back(vec![1, 2]);
    println!("{:?}", padded);

    // Or clone, and pay for the deep copy where everyone can see it.
    let names = vec![String::from("ana")];
    let duplicate = names.clone();
    println!("{:?} {:?}", names, duplicate);
} // `padded`, `names`, `duplicate` are dropped here. No GC, no free() call.
```

Read `count` and `hand_back` as contracts. `count(v: Vec<i32>)` says "give me your
vector, I am keeping it". `hand_back(v) -> Vec<i32>` says "give it to me and I will give
it back". A signature in Rust tells you what happens to your data — which is more than
most languages' signatures tell you.

## The error you will hit

```rust
fn archive(log: String) {
    println!("archived {} bytes", log.len());
}

fn main() {
    let log = String::from("boot sequence complete");
    archive(log);
    println!("still here: {}", log);
}
```

```
error[E0382]: borrow of moved value: `log`
 --> src/main.rs:8:32
  |
6 |     let log = String::from("boot sequence complete");
  |         --- move occurs because `log` has type `String`, which does not implement the `Copy` trait
7 |     archive(log);
  |             --- value moved here
8 |     println!("still here: {}", log);
  |                                ^^^ value borrowed here after move
  |
note: consider changing this parameter type in function `archive` to borrow instead if owning the value isn't necessary
 --> src/main.rs:1:17
  |
1 | fn archive(log: String) {
  |    -------      ^^^^^^ this parameter takes ownership of the value
help: consider cloning the value if the performance cost is acceptable
  |
7 |     archive(log.clone());
  |                ++++++++
```

This error tells you the whole story in four annotations: where the value was created,
why it moves (`String` is not `Copy`), where it moved, and where you illegally touched
it after. Then it offers both real fixes. `.clone()` is the blunt one and costs an
allocation. The `note:` names the better one — have `archive` take a borrow instead of
ownership, since it only reads. That is module 05, and it is where ownership stops
feeling restrictive.

## What's next

Six exercises. `what_is_ownership` and `move_semantics` are the E0382 above, by
assignment and by function call. `clone_and_copy` sorts types into the two categories.
`ownership_functions` has you write signatures that take and return ownership.
`predict_output` asks you to trace moves by hand — do it on paper before compiling. Then
`boss_ownership`: a `Playlist` that owns its `Song`s, adds by value, hands one back with
`Option<Song>`, and lends out `&Song` references for searching. All five ideas at once.

Source: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
