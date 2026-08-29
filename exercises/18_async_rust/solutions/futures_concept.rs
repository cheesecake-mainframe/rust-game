// Solution: Futures Concept
// ==========================

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

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
// ReadyFuture — immediately ready with a value
// =================================================================
struct ReadyFuture<T> {
    value: Option<T>,
}

impl<T> ReadyFuture<T> {
    fn new(value: T) -> Self {
        ReadyFuture {
            value: Some(value),
        }
    }
}

impl<T> Unpin for ReadyFuture<T> {}

impl<T> Future for ReadyFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        match this.value.take() {
            Some(v) => Poll::Ready(v),
            None => panic!("polled after completion"),
        }
    }
}

// =================================================================
// CountdownFuture — counts down to zero, then is ready
// =================================================================
struct CountdownFuture {
    count: u32,
}

impl CountdownFuture {
    fn new(count: u32) -> Self {
        CountdownFuture { count }
    }
}

impl Unpin for CountdownFuture {}

impl Future for CountdownFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.count == 0 {
            Poll::Ready("done!")
        } else {
            this.count -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// =================================================================
// MapFuture — transforms the output of another future
// =================================================================
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
        MapFuture {
            future,
            f: Some(f),
        }
    }
}

impl<F, M, T> Unpin for MapFuture<F, M, T>
where
    F: Future + Unpin,
    M: FnOnce(F::Output) -> T,
{
}

impl<F, M, T> Future for MapFuture<F, M, T>
where
    F: Future + Unpin,
    M: FnOnce(F::Output) -> T,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        match Pin::new(&mut this.future).poll(cx) {
            Poll::Ready(value) => {
                let f = this.f.take().expect("MapFuture polled after completion");
                Poll::Ready(f(value))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

// =================================================================
// async_add — async function that adds two numbers
// =================================================================
async fn async_add(a: i32, b: i32) -> i32 {
    a + b
}

// =================================================================
// chain_futures — awaits two futures and sums their results
// =================================================================
async fn chain_futures(a: ReadyFuture<i32>, b: ReadyFuture<i32>) -> i32 {
    let val_a = a.await;
    let val_b = b.await;
    val_a + val_b
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

    #[test]
    fn test_countdown_zero_is_ready() {
        let mut future = CountdownFuture::new(0);
        let result = poll_once(&mut future);
        assert_eq!(result, Poll::Ready("done!"));
    }

    #[test]
    fn test_countdown_three() {
        let mut future = CountdownFuture::new(3);
        assert_eq!(poll_once(&mut future), Poll::Pending);
        assert_eq!(poll_once(&mut future), Poll::Pending);
        assert_eq!(poll_once(&mut future), Poll::Pending);
        assert_eq!(poll_once(&mut future), Poll::Ready("done!"));
    }

    #[test]
    fn test_countdown_one() {
        let mut future = CountdownFuture::new(1);
        assert_eq!(poll_once(&mut future), Poll::Pending);
        assert_eq!(poll_once(&mut future), Poll::Ready("done!"));
    }

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

    #[test]
    fn test_async_add() {
        let mut future = Box::pin(async_add(3, 4));
        let result = poll_once(&mut future);
        assert_eq!(result, Poll::Ready(7));
    }

    #[test]
    fn test_chain_futures() {
        let a = ReadyFuture::new(10);
        let b = ReadyFuture::new(20);
        let mut future = Box::pin(chain_futures(a, b));
        let result = poll_once(&mut future);
        assert_eq!(result, Poll::Ready(30));
    }
}
