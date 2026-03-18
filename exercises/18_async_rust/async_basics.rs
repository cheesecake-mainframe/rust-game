// Exercise: Async Basics (FixCompilerError)
// ==========================================
//
// Async Rust allows writing asynchronous code using `async` and `.await`.
//
// Key concepts:
//   - `async fn` declares an asynchronous function that returns a Future
//   - `.await` is used inside async contexts to wait for a Future to complete
//   - An `async fn foo() -> T` actually returns `impl Future<Output = T>`
//   - You cannot call `.await` outside of an async context
//
// This file has several async-related compilation errors. Fix them!
//
// NOTE: Since we don't have an async runtime (like tokio) in this sandbox,
// we test that the code compiles correctly and that types are right.
// We use `std::future::Future` and manual polling concepts.

use std::future::Future;
use std::pin::Pin;

// FIX 1: This function should be async, but the keyword is misspelled.
// An async fn that returns a greeting string.
asnc fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// FIX 2: This async function tries to call another async function
// but forgets to `.await` it.
async fn greet_loudly(name: &str) -> String {
    let greeting = greet(name);
    greeting.to_uppercase()
}

// FIX 3: This function signature says it returns a number, but
// async functions return impl Future<Output = T>, not T directly
// when used as a trait bound. Fix the return type annotation.
fn make_number() -> u32 {
    async { 42 }
}

// FIX 4: This function takes a future as an argument, but the trait
// bound is wrong. A Future needs an Output associated type.
async fn process_future<F>(fut: F) -> String
where
    F: Future<String>,
{
    let value = fut.await;
    format!("Processed: {}", value)
}

// FIX 5: This function tries to use .await in a non-async function.
// Fix it by making the function async.
fn compute_sum(a: i32, b: i32) -> i32 {
    let result = async { a + b }.await;
    result
}

// FIX 6: This async function has a lifetime issue. The return type
// needs to work with the async borrow. Fix the function.
async fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

// This struct wraps an async computation result.
struct AsyncResult<T> {
    value: T,
}

impl<T> AsyncResult<T> {
    // FIX 7: This method should be async but isn't declared as such.
    fn get_value(self) -> T {
        let v = async { self.value }.await;
        v
    }
}

fn main() {
    // We can't actually run async code without a runtime, but we can
    // verify that async functions compile and have the right return types.

    // Verify that greet returns a Future (async fn returns impl Future)
    let _future = greet("world");
    println!("greet() created a future successfully");

    // Verify greet_loudly compiles
    let _future = greet_loudly("world");
    println!("greet_loudly() created a future successfully");

    // Verify compute_sum compiles
    let _future = compute_sum(3, 4);
    println!("compute_sum() created a future successfully");

    // Verify first_word compiles
    let _future = first_word("hello world");
    println!("first_word() created a future successfully");

    println!("All async basics compile correctly!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;

    // We can test that async functions return the right types
    // by checking they implement Future with the correct Output.

    #[test]
    fn test_greet_returns_future() {
        let future = greet("test");
        // This confirms greet returns impl Future<Output = String>
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
