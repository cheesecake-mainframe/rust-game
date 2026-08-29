# The Concurrency Arena

`thread::spawn(|| println!("{data}"))` fails before you run anything: the closure may outlive
the current function. Reach for `Rc` to share state and you're refused again —
`Rc<Mutex<i32>> cannot be sent between threads safely`. Both refusals come from one place,
and neither is arbitrary.

## Coming from Python/JS

Python has real threads, but the GIL serializes bytecode, so two threads never run Python at
the same instant. That buys memory safety for free — no torn objects — at the cost of no CPU
parallelism, and it does *not* save you from logical races: `counter += 1` from two threads
still loses updates. JS avoids the question with one thread plus message-copying workers.

Rust gives you real OS threads, real parallelism, and no global lock. Safety comes from the
type system instead, via two marker traits the compiler derives automatically:

- `Send` — this type is safe to *move* to another thread.
- `Sync` — this type is safe to *share by reference* across threads.

`thread::spawn` requires its closure to be `Send + 'static`. That one bound is what rejects
`Rc<T>`: its count is a plain integer, so two threads incrementing it can lose a count and
free live data. `Arc<T>` is the same type with an atomic counter, so it is `Send`. Likewise
`RefCell<T>` is not `Sync` (its borrow flag isn't atomic) while `Mutex<T>` is. No new rules
here — ownership and borrowing do the work, and concurrency adds two markers on top.

That is the "fearless" claim, and it is precise: the compiler statically rules out data
races. It does not rule out deadlocks, lost wakeups, or plain wrong logic.

## The rule

1. `thread::spawn(closure)` requires `Send + 'static`: the closure may not borrow from the
   spawning stack frame, because that frame might end first. Use `move`, and give each
   thread its own owned value or its own `Arc` clone.
2. `handle.join()` blocks until the thread finishes and returns a `Result` — `Ok(value)`
   holding whatever the closure returned, `Err` if it panicked. Without `join`, `main` can
   exit while your threads are still running.
3. Message passing: `mpsc::channel()` gives `(tx, rx)`. `tx.send(v)` moves `v`. Clone `tx`
   once per producer. `for msg in rx` ends only when **every** sender has been dropped —
   including the original you cloned from, which is why you often `drop(tx)`.
4. Shared state: `Arc<Mutex<T>>`. `Arc::clone(&a)` before each `move`. `.lock()` returns
   `Result<MutexGuard<T>>`; the guard derefs to `T` and unlocks when it drops. Hold it across
   as few lines as you can.
5. `Arc` is the thread-safe `Rc`; `Mutex` is the thread-safe `RefCell`. Same two axes as
   module 14 — shared ownership, interior mutability — with atomic machinery.

## Worked example

```rust
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared state: Arc for shared ownership, Mutex for exclusive access.
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for worker in 0..4 {
        let log = Arc::clone(&log); // one owner per thread
        handles.push(thread::spawn(move || {
            log.lock().unwrap().push(worker); // guard unlocks when dropped
        }));
    }
    for h in handles {
        h.join().unwrap(); // wait, and surface any panic from the thread
    }
    let mut ids = log.lock().unwrap().clone();
    ids.sort();
    assert_eq!(ids, vec![0, 1, 2, 3]);

    // Message passing: `send` moves the value out of this thread.
    let (tx, rx) = mpsc::channel();
    for worker in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || tx.send(worker * 10).unwrap());
    }
    drop(tx); // drop the original, or the loop below never ends

    let mut received: Vec<i32> = rx.iter().collect();
    received.sort();
    assert_eq!(received, vec![0, 10, 20]);
}
```

## The error you will hit

Using `Rc` where the threads need `Arc`:

```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter = Rc::clone(&counter);
        handles.push(thread::spawn(move || {
            *counter.lock().unwrap() += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}
```

```
error[E0277]: `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
  --> src/main.rs:11:36
   |
11 |           handles.push(thread::spawn(move || {
   |                        ------------- ^------
   |                        |             |
   |  ______________________|_____________within this `{closure@src/main.rs:11:36: 11:43}`
   | |                      |
   | |                      required by a bound introduced by this call
12 | |             *counter.lock().unwrap() += 1;
13 | |         }));
   | |_________^ `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
   |
   = help: within `{closure@src/main.rs:11:36: 11:43}`, the trait `Send` is not implemented for `Rc<std::sync::Mutex<i32>>`
```

Skip the ASCII art and read the `= help:` line. It names the type and the missing trait, and
that is the whole diagnosis: swap `Rc` for `Arc`. The compiler caught a data race in a
program that had not run yet.

## What's next

`thread_basics` is rules 1 and 2 — `move`, owned copies per thread, joining for results.
`channels` is rule 3, dropped-sender trap included. `shared_state` is the error above, four
times. `boss_concurrency` combines everything into a parallel word counter over
`Arc<Mutex<HashMap<..>>>`.

Source: https://doc.rust-lang.org/book/ch16-03-shared-state.html
