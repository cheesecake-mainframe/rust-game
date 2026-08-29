# The Pointer Maze

`Rc<RefCell<T>>` reads like line noise the first time you meet it, and most explanations make
it worse by treating it as one thing. It is two independent opt-ins stacked: `Rc` relaxes
"exactly one owner," `RefCell` relaxes "mutation is checked at compile time." You reach for
the pair only when you genuinely need both.

## Coming from Python/JS

Every CPython object is already heap-allocated behind a pointer, already reference-counted,
and already mutable through any alias. `a = b` gives you two names for one object and either
name can mutate it. JS is the same with a tracing collector instead of a count. You never
annotate any of this, because it is the only mode the language has — and you pay for it on
every object you ever create.

Rust's default is the exact opposite: values live inline in their owner, there is one owner,
and mutation requires exclusive access proven at compile time. Smart pointers sell those
capabilities back one at a time, and you pay only for what you buy:

- `Box<T>` — heap allocation, still exactly one owner. Cost: one pointer of indirection.
- `Rc<T>` — many owners, read-only, single-threaded. Cost: a plain counter, bumped on clone
  and on drop.
- `RefCell<T>` — one owner, but the borrow rules move from compile time to runtime. Cost: a
  borrow flag, and a panic instead of a compile error when you break the rules.

So `Rc<RefCell<T>>` is, near enough, "an ordinary Python object": shared and mutable. Python
hands you that for everything whether you need it or not. Rust makes you type it out, which
is irritating exactly once and then tells every future reader precisely what the sharing and
mutation model of that value is.

## The rule

1. `Box<T>` when the size isn't known at compile time. An enum variant containing itself has
   infinite size; a `Box` is always one pointer wide. rustc says so outright:
   `help: insert some indirection (e.g., a Box, Rc, or &) to break the cycle`. Also use
   `Box<dyn Trait>` to return one of several concrete types behind a single interface.
2. `Rc::new(v)` creates. `Rc::clone(&r)` adds an owner — it increments a counter and does
   **not** deep-copy. `Rc::strong_count(&r)` inspects it. Prefer `Rc::clone(&r)` over
   `r.clone()` so readers can see the operation is cheap. The value drops at count zero.
3. `Rc<T>` only ever hands out shared references. That is the E0596 below, and the entire
   reason `RefCell` exists.
4. `RefCell<T>`: `borrow()` gives a `Ref<T>`, `borrow_mut()` gives a `RefMut<T>`. Same rule
   as always — many shared XOR one exclusive — but enforced at runtime. A second
   `borrow_mut()` while one is live panics with `already borrowed: BorrowMutError`. Keep the
   guards short-lived; let them drop at the end of the statement.
5. The order is `Rc<RefCell<T>>`, never `RefCell<Rc<T>>`. Outer layer is sharing, inner layer
   is mutability.
6. Deref coercion means you call the inner type's methods straight through the pointer.

## Worked example

```rust
use std::cell::RefCell;
use std::rc::Rc;

// Box gives the recursive variant a known size: a Box is one pointer,
// however big the thing it points at turns out to be.
enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(left, right) => eval(left) + eval(right),
    }
}

struct Meter {
    readings: Vec<u32>,
}

fn main() {
    let e = Expr::Add(Box::new(Expr::Num(2)), Box::new(Expr::Num(40)));
    assert_eq!(eval(&e), 42);

    // Rc buys many owners. RefCell buys mutation through a shared reference.
    let meter = Rc::new(RefCell::new(Meter { readings: Vec::new() }));
    let sensor_a = Rc::clone(&meter);
    let sensor_b = Rc::clone(&meter);

    sensor_a.borrow_mut().readings.push(12);
    sensor_b.borrow_mut().readings.push(30);

    assert_eq!(Rc::strong_count(&meter), 3);
    assert_eq!(meter.borrow().readings, vec![12, 30]);
}
```

## The error you will hit

Reaching for `Rc` alone when you actually needed both halves:

```rust
use std::rc::Rc;

fn main() {
    let shared = Rc::new(vec![1, 2, 3]);
    let alias = Rc::clone(&shared);
    alias.push(4);
    println!("{shared:?}");
}
```

```
error[E0596]: cannot borrow data in an `Rc` as mutable
 --> src/main.rs:6:5
  |
6 |     alias.push(4);
  |     ^^^^^ cannot borrow as mutable
  |
  = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<Vec<i32>>`
```

`Rc` deliberately refuses `DerefMut`, because handing two owners a `&mut` to the same value
would break the aliasing rule the whole language is built on. Wrapping the inside in
`RefCell` is how you say "I'll take responsibility for that rule at runtime."

## What's next

`box_basics` is rule 1 — recursive enums and `Box<dyn Trait>`. `rc_shared` is rule 2, with a
diamond graph where one child has two parents and `strong_count` proves it. `refcell_interior`
is rules 3 and 4: a cache that mutates itself behind `&self`.

Source: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
