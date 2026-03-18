// ========================================
// Solution: Closure Basics
// ========================================

// Fix: Add `move` so the closure takes ownership of `name`.
fn create_greeter() -> Box<dyn Fn() -> String> {
    let name = String::from("Rustacean");
    let greeter = Box::new(move || {
        format!("Hello, {}!", name)
    });
    greeter
}

// This one actually compiles fine in the exercise. The `mut` on the binding
// and the closure being FnMut is correct. count is Copy so it works.
fn count_calls() -> i32 {
    let mut count = 0;
    let mut increment = || {
        count += 1;
    };

    increment();
    increment();
    increment();

    count
}

// Fix: Use `format!` instead of `+` to avoid consuming `prefix`.
// Also add `move` to own `prefix`.
fn reusable_formatter(prefix: String) -> Box<dyn Fn(String) -> String> {
    Box::new(move |msg| {
        format!("{}: {}", prefix, msg)
    })
}

// Fix: Don't create both closures simultaneously. Use sequential operations
// so there's no overlapping mutable/immutable borrows.
fn split_operations() -> (Vec<i32>, i32) {
    let mut data = vec![1, 2, 3, 4, 5];

    data.push(6);

    let total: i32 = data.iter().sum();

    (data, total)
}

// Fix: Add `move` to take ownership of `current`.
fn make_counter_from(start: i32) -> Box<dyn FnMut() -> i32> {
    let mut current = start;
    Box::new(move || {
        let val = current;
        current += 1;
        val
    })
}

// Fix: Change `process` to accept FnMut instead of Fn.
fn process<F: FnMut(i32) -> i32>(mut f: F, values: &[i32]) -> Vec<i32> {
    values.iter().map(|&v| f(v)).collect()
}

fn apply_processing() -> Vec<i32> {
    let mut call_count = 0;
    let processor = |x: i32| -> i32 {
        call_count += 1;
        x * 2
    };

    let result = process(processor, &[1, 2, 3, 4, 5]);
    println!("Processor was called {} times", call_count);
    result
}

fn main() {
    // Test create_greeter
    let greeter = create_greeter();
    assert_eq!(greeter(), "Hello, Rustacean!");
    assert_eq!(greeter(), "Hello, Rustacean!");

    // Test count_calls
    assert_eq!(count_calls(), 3);

    // Test reusable_formatter
    let fmt = reusable_formatter(String::from("INFO"));
    assert_eq!(fmt(String::from("started")), "INFO: started");
    assert_eq!(fmt(String::from("finished")), "INFO: finished");

    // Test split_operations
    let (data, total) = split_operations();
    assert_eq!(data, vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(total, 21);

    // Test make_counter_from
    let mut counter = make_counter_from(10);
    assert_eq!(counter(), 10);
    assert_eq!(counter(), 11);
    assert_eq!(counter(), 12);

    // Test apply_processing
    let result = apply_processing();
    assert_eq!(result, vec![2, 4, 6, 8, 10]);

    println!("All closure tests passed!");
}
