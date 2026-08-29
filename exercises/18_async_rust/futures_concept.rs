// Exercise: Futures Concept (ImplementFromScratch)
// =================================================
//
// This exercise explores the Future trait and related concepts without
// needing an async runtime. You'll implement functions that demonstrate
// understanding of how Futures work under the hood.
//
// Key concepts:
//   - `Future` trait: has a `poll` method that returns `Poll<Self::Output>`
//   - `Poll::Ready(value)` — the future has completed
//   - `Poll::Pending` — the future is not yet complete
//   - `Pin<&mut Self>` — futures must be pinned in memory
//   - `Context` / `Waker` — the runtime uses these to know when to re-poll
//
// In this exercise we create simple futures that can be polled manually.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

// A helper to create a no-op waker for manual polling in tests.
fn noop_waker() -> Waker {
    fn noop(_: *const ()) {}
    fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    unsafe { Waker::from_raw(raw) }
}

// =================================================================
// TODO 1: Implement `ReadyFuture`
// =================================================================
// A future that is immediately ready with a value.
// When polled, it should always return `Poll::Ready(self.value)`.
//
// - Store the value as `Option<T>` so it can be taken on first poll
// - Return `Poll::Ready(value)` on first poll
// - Panic with "polled after completion" if polled again (value is None)

struct ReadyFuture<T> {
    value: Option<T>,
}

impl<T> ReadyFuture<T> {
    fn new(value: T) -> Self {
        todo!()
    }
}

impl<T> Future for ReadyFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

// =================================================================
// TODO 2: Implement `CountdownFuture`
// =================================================================
// A future that counts down from a given number to zero.
// Each time it's polled, it decrements the counter.
// It returns `Poll::Pending` until the counter reaches 0,
// then returns `Poll::Ready("done!")`.
//
// Fields:
//   count: u32

struct CountdownFuture {
    count: u32,
}

impl CountdownFuture {
    fn new(count: u32) -> Self {
        todo!()
    }
}

impl Future for CountdownFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
        // Remember: when returning Pending, you should call cx.waker().wake_by_ref()
        // to signal that the future should be polled again (in a real runtime).
    }
}

// =================================================================
// TODO 3: Implement `MapFuture`
// =================================================================
// A future combinator that transforms the output of another future.
// This is like `.map()` on iterators, but for futures.
//
// It wraps a future `F` and a function `Fn(F::Output) -> T`.
// When the inner future completes, apply the function to the result.
//
// Fields:
//   future: F       (the inner future, wrapped in Option for Pin safety)
//   f: Option<M>    (the mapping function)
//
// For simplicity, we'll use Unpin bound on F.

struct MapFuture<F, M, T>
where
    F: Future + Unpin,
    M: FnOnce(F::Output) -> T,
{
    future: F,
    f: Option<M>,
}

impl<F, M, T> MapFuture<F, M, T>
where
    F: Future + Unpin,
    M: FnOnce(F::Output) -> T,
{
    fn new(future: F, f: M) -> Self {
        todo!()
    }
}

impl<F, M, T> Future for MapFuture<F, M, T>
where
    F: Future + Unpin,
    M: FnOnce(F::Output) -> T,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
        // Hint: Get a &mut to self, poll the inner future with Pin::new(),
        // if Ready, take the function from self.f and apply it.
    }
}

// =================================================================
// TODO 4: Implement `async_add`
// =================================================================
// An async function that adds two numbers. Even though this is trivial,
// it demonstrates that async fn returns impl Future.
async fn async_add(a: i32, b: i32) -> i32 {
    todo!()
}

// =================================================================
// TODO 5: Implement `chain_futures`
// =================================================================
// An async function that takes two ReadyFutures and returns the sum
// of their values (both i32).
async fn chain_futures(a: ReadyFuture<i32>, b: ReadyFuture<i32>) -> i32 {
    todo!()
}

fn main() {
    println!("Futures Concept exercise!");
    println!("This exercise tests your understanding of the Future trait.");
    println!("Run the tests to verify your implementation.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    fn poll_once<F: Future + Unpin>(future: &mut F) -> Poll<F::Output> {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        Pin::new(future).poll(&mut cx)
    }

    // --- ReadyFuture tests ---

    #[test]
    fn test_ready_future_is_immediately_ready() {
        let mut future = ReadyFuture::new(42);
        let result = poll_once(&mut future);
        assert_eq!(result, Poll::Ready(42));
    }

    #[test]
    fn test_ready_future_string() {
        let mut future = ReadyFuture::new(String::from("hello"));
        let result = poll_once(&mut future);
        assert_eq!(result, Poll::Ready(String::from("hello")));
    }

    // --- CountdownFuture tests ---

    #[test]
    fn test_countdown_zero_is_ready() {
        let mut future = CountdownFuture::new(0);
        let result = poll_once(&mut future);
        assert_eq!(result, Poll::Ready("done!"));
    }

    #[test]
    fn test_countdown_three() {
        let mut future = CountdownFuture::new(3);

        // First three polls should be Pending
        assert_eq!(poll_once(&mut future), Poll::Pending);
        assert_eq!(poll_once(&mut future), Poll::Pending);
        assert_eq!(poll_once(&mut future), Poll::Pending);

        // Fourth poll should be Ready
        assert_eq!(poll_once(&mut future), Poll::Ready("done!"));
    }

    #[test]
    fn test_countdown_one() {
        let mut future = CountdownFuture::new(1);
        assert_eq!(poll_once(&mut future), Poll::Pending);
        assert_eq!(poll_once(&mut future), Poll::Ready("done!"));
    }

    // --- MapFuture tests ---

    #[test]
    fn test_map_future_transforms_output() {
        let inner = ReadyFuture::new(5);
        let mut mapped = MapFuture::new(inner, |x| x * 10);
        let result = poll_once(&mut mapped);
        assert_eq!(result, Poll::Ready(50));
    }

    #[test]
    fn test_map_future_type_change() {
        let inner = ReadyFuture::new(42);
        let mut mapped = MapFuture::new(inner, |x| format!("value: {}", x));
        let result = poll_once(&mut mapped);
        assert_eq!(result, Poll::Ready(String::from("value: 42")));
    }

    #[test]
    fn test_map_countdown() {
        let inner = CountdownFuture::new(2);
        let mut mapped = MapFuture::new(inner, |s| s.to_uppercase());

        assert_eq!(poll_once(&mut mapped), Poll::Pending);
        assert_eq!(poll_once(&mut mapped), Poll::Pending);
        assert_eq!(poll_once(&mut mapped), Poll::Ready(String::from("DONE!")));
    }

    // --- async_add tests ---

    #[test]
    fn test_async_add() {
        // `async fn` bodies are `!Unpin` (they may hold a reference across an
        // await point, so moving one after polling would dangle). `Box::pin`
        // pins it on the heap, and `Pin<Box<F>>` *is* `Unpin`.
        let mut future = Box::pin(async_add(3, 4));
        let result = poll_once(&mut future);
        assert_eq!(result, Poll::Ready(7));
    }

    // --- chain_futures tests ---

    #[test]
    fn test_chain_futures() {
        let a = ReadyFuture::new(10);
        let b = ReadyFuture::new(20);
        // `async fn` bodies are `!Unpin` (they may hold a reference across an
        // await point, so moving one after polling would dangle). `Box::pin`
        // pins it on the heap, and `Pin<Box<F>>` *is* `Unpin`.
        let mut future = Box::pin(chain_futures(a, b));
        let result = poll_once(&mut future);
        assert_eq!(result, Poll::Ready(30));
    }
}
