// Solution: Async Basics
// =======================

use std::future::Future;
use std::pin::Pin;

// FIX 1: Changed `asnc` to `async`.
async fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// FIX 2: Added `.await` when calling the async function `greet`.
async fn greet_loudly(name: &str) -> String {
    let greeting = greet(name).await;
    greeting.to_uppercase()
}

// FIX 3: Changed return type to `impl Future<Output = u32>` since
// the body returns an async block, not a plain u32.
fn make_number() -> impl Future<Output = u32> {
    async { 42 }
}

// FIX 4: Changed `Future<String>` to `Future<Output = String>`.
// Future is a trait with an associated type Output, not a generic parameter.
async fn process_future<F>(fut: F) -> String
where
    F: Future<Output = String>,
{
    let value = fut.await;
    format!("Processed: {}", value)
}

// FIX 5: Made the function `async` so we can use `.await` inside it.
async fn compute_sum(a: i32, b: i32) -> i32 {
    let result = async { a + b }.await;
    result
}

// FIX 6: This function was already correct — async fn with &str works fine
// because the compiler desugars the lifetimes properly.
async fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

struct AsyncResult<T> {
    value: T,
}

impl<T> AsyncResult<T> {
    // FIX 7: Made this method `async` so .await can be used inside.
    async fn get_value(self) -> T {
        let v = async { self.value }.await;
        v
    }
}

fn main() {
    let _future = greet("world");
    println!("greet() created a future successfully");

    let _future = greet_loudly("world");
    println!("greet_loudly() created a future successfully");

    let _future = compute_sum(3, 4);
    println!("compute_sum() created a future successfully");

    let _future = first_word("hello world");
    println!("first_word() created a future successfully");

    println!("All async basics compile correctly!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;

    #[test]
    fn test_greet_returns_future() {
        let future = greet("test");
        fn assert_future<F: Future<Output = String>>(_f: F) {}
        assert_future(future);
    }

    #[test]
    fn test_greet_loudly_returns_future() {
        let future = greet_loudly("test");
        fn assert_future<F: Future<Output = String>>(_f: F) {}
        assert_future(future);
    }

    #[test]
    fn test_compute_sum_returns_future() {
        let future = compute_sum(1, 2);
        fn assert_future<F: Future<Output = i32>>(_f: F) {}
        assert_future(future);
    }

    #[test]
    fn test_first_word_returns_future() {
        let text = String::from("hello world");
        let future = first_word(&text);
        fn assert_future<'a, F: Future<Output = &'a str>>(_f: F) {}
        assert_future(future);
    }

    #[test]
    fn test_async_result_compiles() {
        let result = AsyncResult { value: 42 };
        let future = result.get_value();
        fn assert_future<F: Future<Output = i32>>(_f: F) {}
        assert_future(future);
    }
}
