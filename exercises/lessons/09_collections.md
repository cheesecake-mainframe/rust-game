# The Data Vault

`Vec<T>` is your list and `HashMap<K, V>` is your dict — right up until you look something
up. A missing index does not give you `undefined`, a missing key does not give you `None`
at runtime, and `"crab"[0]` on a `String` does not even compile. Every one of those
differences is UTF-8 or the borrow checker showing through.

## Coming from Python/JS

```python
jobs = ["build", "test"]
jobs.append("ship")
tally = {}
tally["red"] = tally.get("red", 0) + 1
first = "crab"[0]        # 'c'
```

The Rust equivalents diverge in four places.

**One type per collection.** A Python list holds anything; `Vec<T>` holds one `T`. That is
what lets the elements sit next to each other in memory with no per-element box, which is
most of why they are fast.

**Lookups hand back `Option`, and a reference at that.** `tally.get("pink")` is
`Option<&u32>`, never a bare number and never a crash. The indexing forms `jobs[7]` and
`tally["pink"]` do exist and *panic* on a miss, so the choice is explicit at every call
site: is a miss a bug (index) or an expected case (`get`)? Python has both spellings too,
but nothing there records which one you meant in the type.

**A `String` is UTF-8 bytes, not characters.** `s[0]` would have to mean "byte 0" — and
for `"🦀"`, byte 0 is a quarter of a crab. Rather than hand you a meaningless byte in O(1)
or scan the string in O(n) behind an innocent-looking `[]`, Rust refuses to compile the
indexing at all. You say what you meant: `.chars()`, `.bytes()`, or a slice that lands on
character boundaries.

**Collections own their contents.** `map.insert(key, value)` moves both in, and `get`
hands back a *reference* into the collection — so the borrowing rules from module 05 apply
to a `Vec` exactly as they applied to a `String`.

## The rules

1. **`Vec<T>` is homogeneous and needs `mut` to grow.** `v[i]` panics out of range;
   `v.get(i)` returns `Option<&T>`.
2. **A live reference into a collection freezes it.** You cannot `push` while a `&` to an
   element is still in use — `push` may reallocate and move every element, leaving that
   reference dangling.
3. **`String` owns, `&str` borrows.** `push(c)` takes one `char`, `push_str(s)` takes a
   `&str`, `a + &b` moves `a` and borrows `b`, and `format!("{a}{b}")` borrows both.
4. **No integer indexing into strings.** Use `.chars()`, `.chars().nth(n)`, `.bytes()`, or
   slice on a character boundary (`&s[0..4]`), which panics if it cuts one in half.
5. **`HashMap<K, V>`**: `insert` moves owned keys and values, `get(&k)` returns
   `Option<&V>`, and `entry(k).or_insert(0)` returns `&mut V` — so you write `*count += 1`.
   Keys must implement `Hash` and `Eq`, which is why `f64` cannot be a key.

## Worked example

```rust
use std::collections::HashMap;

fn main() {
    let mut queue: Vec<&str> = Vec::new();
    queue.push("build");
    queue.push("test");

    match queue.get(7) {
        Some(job) => println!("job 7 is {job}"),
        None => println!("no job at index 7"), // this one runs
    }

    let mut line = String::from("crab");
    line.push('s');           // one char
    line.push_str(" rule");   // a &str
    let shout = line + "!";   // `+` moves the left String, borrows the right
    println!("{shout} — {} bytes, {} chars", shout.len(), shout.chars().count());

    let mut tally: HashMap<String, u32> = HashMap::new();
    for word in "red blue red green red".split_whitespace() {
        let count = tally.entry(word.to_string()).or_insert(0);
        *count += 1; // or_insert hands back &mut u32
    }
    println!("red seen {:?} times", tally.get("red"));
    println!("pink seen {:?} times", tally.get("pink"));
}
```

## The error you will hit

Holding a reference to an element and then growing the collection:

```rust
fn main() {
    let mut scores = vec![10, 20, 30];
    let first = &scores[0];
    scores.push(40);
    println!("first score is {first}");
}
```

```
error[E0502]: cannot borrow `scores` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let first = &scores[0];
  |                  ------ immutable borrow occurs here
4 |     scores.push(40);
  |     ^^^^^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("first score is {first}");
  |                               ----- immutable borrow later used here
```

This looks like the compiler being pedantic — you only read element 0. It is not. `push`
can outgrow the allocation, copy every element to a new address, and free the old block;
`first` would then point at freed memory. Python survives this because every element is a
separately allocated object behind a reference count; C++ lets you do it and hands you
undefined behavior. Rust makes it a compile error. The fix is usually to take the value
instead of a reference to it:

```rust
fn main() {
    let mut scores = vec![10, 20, 30];
    let first = scores[0]; // i32 is Copy, so this is a value, not a borrow
    scores.push(40);
    println!("first is {first}, {} scores now", scores.len());
}
```

## What's next

`vectors` (hunt logic bugs in indexing and iteration), `strings` (a pile of `String`
versus `&str` compiler errors), and `hashmaps` (build word counts and groupings with the
`entry` API).

Source: https://doc.rust-lang.org/book/ch08-00-common-collections.html
