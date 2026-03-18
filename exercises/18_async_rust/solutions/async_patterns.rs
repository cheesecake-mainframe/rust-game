// Solution: Async Patterns
// =========================

use std::future::Future;
use std::pin::Pin;

// FIX 1: Made function async and added .await on the async call.
async fn fetch_data() -> String {
    let data = get_data().await;
    data
}

async fn get_data() -> String {
    String::from("important data")
}

// FIX 2: Removed the closure. Instead, do the async work directly
// in the loop body using .await.
async fn transform_items(items: Vec<i32>) -> Vec<String> {
    let mut results = Vec::new();
    for item in items {
        let doubled = double_async(item).await;
        results.push(format!("item: {}", doubled));
    }
    results
}

async fn double_async(x: i32) -> i32 {
    x * 2
}

// FIX 3: Changed return type to `impl Future<Output = String>` since
// the function returns an async block, not a plain String.
fn create_greeting(name: String) -> impl Future<Output = String> {
    async move {
        format!("Welcome, {}!", name)
    }
}

// FIX 4: Added `Send` bound on F and `std::fmt::Debug` bound on F::Output.
async fn log_future_result<F>(future: F)
where
    F: Future + Send,
    F::Output: std::fmt::Debug,
{
    let result = future.await;
    println!("Future completed with: {:?}", result);
}

// FIX 5: Made the `process` method async so .await can be used.
struct AsyncProcessor {
    prefix: String,
}

impl AsyncProcessor {
    fn new(prefix: &str) -> Self {
        AsyncProcessor {
            prefix: prefix.to_string(),
        }
    }

    async fn process(&self, input: &str) -> String {
        let formatted = self.format_input(input).await;
        formatted
    }

    async fn format_input(&self, input: &str) -> String {
        format!("{}: {}", self.prefix, input)
    }
}

// FIX 6: Changed return type to Pin<Box<dyn Future<Output = i32>>>
// and wrapped in Pin::from.
fn boxed_future(x: i32) -> Pin<Box<dyn Future<Output = i32>>> {
    Box::pin(async move { x + 1 })
}

// FIX 7: Added .await to step2's async block.
async fn multi_step_process(input: &str) -> String {
    let step1: String = async { input.to_uppercase() }.await;
    let step2: String = async { format!("[{}]", step1) }.await;
    let step3: String = async { format!("Result: {}", step2) }.await;
    step3
}

fn main() {
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
        let _: Pin<Box<dyn Future<Output = i32>>> = f;
    }
}
