// Exercise: Async Patterns (FixCompilerError)
// =============================================
//
// This exercise covers common mistakes when working with async Rust:
//   - Forgetting to .await async function calls
//   - Trying to use .await outside async contexts
//   - Async closures and async blocks
//   - Send bounds for futures used across threads
//   - Returning futures from non-async functions
//
// Fix all the compilation errors!

use std::future::Future;
use std::pin::Pin;

// ---------------------------------------------------------------
// FIX 1: This function calls an async function but doesn't await it.
// The function itself needs to be async too!
// ---------------------------------------------------------------
fn fetch_data() -> String {
    let data = get_data();
    data
}

async fn get_data() -> String {
    String::from("important data")
}

// ---------------------------------------------------------------
// FIX 2: This async function uses a closure that tries to .await,
// but regular closures can't be async. Use an async block instead.
// Hint: Instead of a closure, just do the work directly or use
// an async block.
// ---------------------------------------------------------------
async fn transform_items(items: Vec<i32>) -> Vec<String> {
    let mut results = Vec::new();
    for item in items {
        let transformer = |x: i32| -> String {
            let doubled = double_async(x).await;
            format!("item: {}", doubled)
        };
        results.push(transformer(item));
    }
    results
}

async fn double_async(x: i32) -> i32 {
    x * 2
}

// ---------------------------------------------------------------
// FIX 3: This function returns a future but the return type is wrong.
// An `async {}` block returns an `impl Future`, not the value directly.
// ---------------------------------------------------------------
fn create_greeting(name: String) -> String {
    async move {
        format!("Welcome, {}!", name)
    }
}

// ---------------------------------------------------------------
// FIX 4: This function should take a generic future, but the bounds
// are incomplete. The future needs to be Send (for use across threads)
// and its Output needs to implement Debug.
// ---------------------------------------------------------------
async fn log_future_result<F>(future: F)
where
    F: Future,
{
    let result = future.await;
    println!("Future completed with: {:?}", result);
}

// ---------------------------------------------------------------
// FIX 5: This struct wraps an async operation. The method tries to
// call .await but isn't marked async.
// ---------------------------------------------------------------
struct AsyncProcessor {
    prefix: String,
}

impl AsyncProcessor {
    fn new(prefix: &str) -> Self {
        AsyncProcessor {
            prefix: prefix.to_string(),
        }
    }

    fn process(&self, input: &str) -> String {
        let formatted = self.format_input(input).await;
        formatted
    }

    async fn format_input(&self, input: &str) -> String {
        format!("{}: {}", self.prefix, input)
    }
}

// ---------------------------------------------------------------
// FIX 6: This function should return a boxed future (useful for
// trait objects), but the Pin/Box syntax is wrong.
// ---------------------------------------------------------------
fn boxed_future(x: i32) -> Box<Future<Output = i32>> {
    Box::new(async move { x + 1 })
}

// ---------------------------------------------------------------
// FIX 7: This async function has multiple .await points but one
// is missing, causing a type mismatch.
// ---------------------------------------------------------------
async fn multi_step_process(input: &str) -> String {
    let step1: String = async { input.to_uppercase() }.await;
    let step2: String = async { format!("[{}]", step1) };
    let step3: String = async { format!("Result: {}", step2) }.await;
    step3
}

fn main() {
    // Verify everything compiles by creating futures (not running them)
    let _f = fetch_data();
    println!("fetch_data compiles");

    let _f = transform_items(vec![1, 2, 3]);
    println!("transform_items compiles");

    let _f = create_greeting("Alice".to_string());
    println!("create_greeting compiles");

    let processor = AsyncProcessor::new("LOG");
    let _f = processor.process("test");
    println!("AsyncProcessor::process compiles");

    let _f = multi_step_process("hello");
    println!("multi_step_process compiles");

    println!("All async patterns compile correctly!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;

    fn assert_is_future<F: Future>(_f: &F) {}

    #[test]
    fn test_fetch_data_is_future() {
        let f = fetch_data();
        assert_is_future(&f);
    }

    #[test]
    fn test_transform_items_is_future() {
        let f = transform_items(vec![1, 2]);
        assert_is_future(&f);
    }

    #[test]
    fn test_create_greeting_is_future() {
        let f = create_greeting("Bob".to_string());
        assert_is_future(&f);
    }

    #[test]
    fn test_processor_is_future() {
        let p = AsyncProcessor::new("TEST");
        let f = p.process("data");
        assert_is_future(&f);
    }

    #[test]
    fn test_multi_step_is_future() {
        let f = multi_step_process("input");
        assert_is_future(&f);
    }

    #[test]
    fn test_boxed_future_returns_pin_box() {
        let f = boxed_future(10);
        // Just verify it compiles and returns the right type
        let _: Pin<Box<dyn Future<Output = i32>>> = f;
    }
}
