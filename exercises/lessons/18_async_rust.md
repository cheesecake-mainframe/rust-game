# The Async Dimension

There is no runtime here. No tokio, no async-std, no `#[tokio::main]` — these exercises
build against `std` alone, and you will call `Future::poll` with your own hands. Reach for
`#[tokio::main]` and you will be stuck, because the sandbox has no dependencies at all.
The first thing to unlearn: in Rust, creating a future runs exactly none of its code.

## Coming from Python/JS

In JavaScript, `const p = fetch(url)` has already sent the request. A promise is a handle
to work that is *in flight*; `await` merely subscribes to its completion. Python's asyncio
is halfway there — a bare coroutine object is inert, but `asyncio.create_task(coro)` hands
it to an ambient event loop the language ships with.

Rust has no ambient loop. `let fut = handshake();` builds a value — a state machine the
compiler generated from your `async fn` body — and runs nothing. `.await` compiles to
roughly "poll the inner future; if it is `Pending`, return `Pending` to whoever is polling
*me*". Something above you must drive that: a runtime, or a `loop` you write. If nobody
polls, nothing ever happens, forever, silently.

Rust chose this because the alternative costs an allocation and a scheduler you cannot opt
out of. An inert future is a plain value: it lives on the stack, in a `struct`, in an
array. That is what lets tokio, embassy on a microcontroller, and your own polling loop all
consume the same `async fn`. The runtime is a library, not a keyword.

## The rule

1. `async fn f(..) -> T` desugars to `fn f(..) -> impl Future<Output = T>`. The return
   type in the signature is the *output*, not the future.
2. `.await` is legal only inside an `async fn` or an `async { }` block. A normal closure
   is not an async context — use an async block instead.
3. The trait is `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`,
   returning `Poll::Pending` or `Poll::Ready(value)`.
4. If you return `Pending`, you owe the caller a wakeup: call `cx.waker().wake_by_ref()`
   (or stash the waker) or you will never be polled again.
5. Most futures must not be polled after returning `Ready`. Take the value out with
   `Option::take` and panic on a second poll.
6. To store or return a future by name, box it: `Pin<Box<dyn Future<Output = T>>>`, built
   with `Box::pin`.

## Worked example

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// A future that reports Pending once, then completes.
struct Handshake {
    greeted: bool,
}

impl Future for Handshake {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.greeted {
            Poll::Ready("handshake complete")
        } else {
            self.greeted = true;
            // Tell the driver this future is worth polling again.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    let mut fut = Handshake { greeted: false };
    let mut cx = Context::from_waker(Waker::noop());

    println!("{:?}", Pin::new(&mut fut).poll(&mut cx)); // Pending
    println!("{:?}", Pin::new(&mut fut).poll(&mut cx)); // Ready("handshake complete")
}
```

`Pin::new` is available here because `Handshake` is a plain struct with no self-references,
so it is `Unpin` and safe to move. `Waker::noop()` is std's do-nothing waker; the exercises
hand-roll an equivalent from `RawWaker` so you can see what one is made of.

## The error you will hit

Write the same driver, then point it at an `async fn` instead of your own struct:

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

async fn handshake() -> &'static str {
    "handshake complete"
}

fn poll_once<F: Future + Unpin>(fut: &mut F) -> Poll<F::Output> {
    let mut cx = Context::from_waker(Waker::noop());
    Pin::new(fut).poll(&mut cx)
}

fn main() {
    let mut fut = handshake();
    println!("{:?}", poll_once(&mut fut));
}
```

```text
error[E0277]: `{async fn body of handshake()}` cannot be unpinned
  --> src/main.rs:16:32
   |
 5 | async fn handshake() -> &'static str {
   |                         ------------ within this `impl Future<Output = &'static str>`
...
16 |     println!("{:?}", poll_once(&mut fut));
   |                      --------- ^^^^^^^^ within `impl Future<Output = &'static str>`, the trait `Unpin` is not implemented for `{async fn body of handshake()}`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
note: required by a bound in `poll_once`
  --> src/main.rs:9:26
   |
 9 | fn poll_once<F: Future + Unpin>(fut: &mut F) -> Poll<F::Output> {
   |                          ^^^^^ required by this bound in `poll_once`
```

Here is why. The compiler turns an `async fn` body into a state machine, one variant per
stretch of code between `.await` points. If your body runs
`let s = build(); other(&s).await;` then across that await the machine must hold both `s`
and a reference to `s` — a struct
containing a pointer into itself. Move that struct after it has been polled and the pointer
still aims at the old address, which is now free for reuse. So the compiler marks every
`async fn` body `!Unpin`: not safe to move once pinned. `Pin<&mut T>` is the wrapper that
promises "this will not move again", and `Pin::new` only builds one for `Unpin` types.

The fix is `Box::pin(handshake())`, which parks the state machine on the heap where it will
stay put. The resulting `Pin<Box<...>>` is itself `Unpin` — the box can move; the future
inside it cannot — so it satisfies the bound. `std::pin::pin!` does the same on the stack.
Awaiting a future directly pins it for you, which is why you have never needed this before.

## What's next

**async_basics** fixes syntax and return types (`async fn` versus `impl Future`, missing
`.await`, `Future<Output = T>` bounds). **futures_concept** has you implement `Future` by
hand — a ready future, a countdown that returns `Pending` three times, and a `map`
combinator. **async_patterns** covers async blocks, `Send` bounds, and boxed futures.
**boss_async** is a task coordinator that polls a `Vec` of futures to completion: your own
miniature runtime.

Source: https://doc.rust-lang.org/book/ch17-05-traits-for-async.html
