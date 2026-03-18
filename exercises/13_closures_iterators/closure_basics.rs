// ========================================
// Exercise: Closure Basics (FixCompilerError)
// ========================================
// Difficulty: Intermediate
// Module: 13 - Closures & Iterators
//
// CONCEPT:
// Closures in Rust capture variables from their environment. There are
// three ways they can capture:
//   - By reference (&T) -- the default when possible (implements Fn)
//   - By mutable reference (&mut T) -- when the closure mutates (implements FnMut)
//   - By value (T) -- when the closure takes ownership (implements FnOnce)
//
// The `move` keyword forces a closure to take ownership of captured variables.
// This is especially important when closures need to outlive the scope
// where they were created (e.g., when passed to threads).
//
// YOUR TASK:
// Fix each closure so the code compiles. Common issues:
// - Missing `move` keyword
// - Borrowing issues (mutable + immutable at the same time)
// - FnOnce vs Fn trait mismatches
// - Closure outliving borrowed data
// ========================================

// FIX: The closure captures `name` by reference, but `name` is dropped
// before the closure is called.
fn create_greeter() -> Box<dyn Fn() -> String> {
    let name = String::from("Rustacean");
    let greeter = Box::new(|| {
        format!("Hello, {}!", name)
    });
    greeter
}

// FIX: The closure tries to mutate `count`, but it's passed as Fn (not FnMut).
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

// FIX: The closure consumes the value (FnOnce), but we try to call it twice.
fn reusable_formatter(prefix: String) -> Box<dyn Fn(String) -> String> {
    Box::new(|msg| {
        // This consumes `prefix` because of the String concatenation
        let result = prefix + ": " + &msg;
        result
    })
}

// FIX: Two closures both try to mutably borrow `data` at the same time.
fn split_operations() -> (Vec<i32>, i32) {
    let mut data = vec![1, 2, 3, 4, 5];

    let mut add_item = |item| {
        data.push(item);
    };

    let sum = || -> i32 {
        data.iter().sum()
    };

    add_item(6);
    let total = sum();

    (data, total)
}

// FIX: The closure needs to own the vector because it's returned from
// the function.
fn make_counter_from(start: i32) -> Box<dyn FnMut() -> i32> {
    let mut current = start;
    Box::new(|| {
        let val = current;
        current += 1;
        val
    })
}

// FIX: The `process` function expects an Fn closure, but we're passing
// one that mutates its environment.
fn process<F: Fn(i32) -> i32>(f: F, values: &[i32]) -> Vec<i32> {
    values.iter().map(|&v| f(v)).collect()
}

fn apply_processing() -> Vec<i32> {
    let mut call_count = 0;
    let processor = |x: i32| -> i32 {
        call_count += 1;  // This makes it FnMut, not Fn
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
    assert_eq!(greeter(), "Hello, Rustacean!"); // Must work twice

    // Test count_calls
    assert_eq!(count_calls(), 3);

    // Test reusable_formatter
    let fmt = reusable_formatter(String::from("INFO"));
    assert_eq!(fmt(String::from("started")), "INFO: started");
    assert_eq!(fmt(String::from("finished")), "INFO: finished"); // Must work twice

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
