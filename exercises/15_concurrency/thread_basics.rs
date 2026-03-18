// ========================================
// Exercise: Thread Basics (FixCompilerError)
// ========================================
// Difficulty: Intermediate
// Module: 15 - Concurrency
//
// CONCEPT:
// Rust's standard library provides OS-level threads via std::thread::spawn.
// Key concepts:
//   - thread::spawn(closure) creates a new thread
//   - The closure must be 'static + Send (no borrowed data from the parent)
//   - Use `move` to transfer ownership of captured variables into the thread
//   - JoinHandle::join() waits for the thread to finish and returns its result
//   - Threads can return values from their closures
//
// Common pitfalls:
//   - Forgetting `move` when the closure captures variables
//   - Not joining threads (the main thread may exit before spawned threads finish)
//   - Trying to use a value after it has been moved into a thread
//
// YOUR TASK:
// Fix each function so the code compiles and the threads work correctly.
// ========================================

use std::thread;

// FIX: The closure captures `name` by reference, but threads require
// owned data (the thread might outlive the current scope).
// Add the `move` keyword to transfer ownership.
fn greet_from_thread() -> String {
    let name = String::from("Rustacean");

    let handle = thread::spawn(|| {
        format!("Hello from thread, {}!", name)
    });

    handle.join().unwrap()
}

// FIX: We try to use `numbers` after moving it into the thread.
// Clone the data before spawning the thread so both can use it.
fn sum_in_thread() -> (i32, Vec<i32>) {
    let numbers = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        numbers.iter().sum::<i32>()
    });

    let sum = handle.join().unwrap();
    (sum, numbers) // ERROR: numbers was moved into the thread
}

// FIX: Multiple threads each need their own copy of the data.
// Each thread captures `message` by move, but we spawn multiple threads
// with the same variable. Clone it for each thread.
fn parallel_greetings() -> Vec<String> {
    let message = String::from("Hello");
    let mut handles = Vec::new();

    for i in 0..3 {
        let handle = thread::spawn(move || {
            format!("{} from thread {}!", message, i)
        });
        handles.push(handle);
    }

    let mut results: Vec<String> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    results.sort(); // Sort for deterministic test comparison
    results
}

// FIX: The thread handle is never joined, so the result is lost.
// Also, the function signature is wrong -- it should return the thread's value.
fn compute_in_background() {
    let handle = thread::spawn(|| {
        let mut sum = 0u64;
        for i in 1..=100 {
            sum += i;
        }
        sum
    });

    // FIX: Join the thread and return its result.
    // Currently the handle is dropped without joining.
}

// FIX: This tries to share a mutable reference across threads, which
// is not allowed. Each thread needs its own data to work with.
// Split the work: give each thread its own chunk of the input,
// then combine results.
fn parallel_transform() -> Vec<i32> {
    let data = vec![1, 2, 3, 4, 5, 6];
    let mut results = Vec::new();

    let handle1 = thread::spawn(|| {
        data[0..3].iter().map(|x| x * 2).collect::<Vec<i32>>()
    });

    let handle2 = thread::spawn(|| {
        data[3..6].iter().map(|x| x * 2).collect::<Vec<i32>>()
    });

    results.extend(handle1.join().unwrap());
    results.extend(handle2.join().unwrap());
    results
}

fn main() {
    // Test greet_from_thread
    let greeting = greet_from_thread();
    assert_eq!(greeting, "Hello from thread, Rustacean!");

    // Test sum_in_thread
    let (sum, nums) = sum_in_thread();
    assert_eq!(sum, 15);
    assert_eq!(nums, vec![1, 2, 3, 4, 5]);

    // Test parallel_greetings
    let greetings = parallel_greetings();
    assert_eq!(greetings, vec![
        "Hello from thread 0!",
        "Hello from thread 1!",
        "Hello from thread 2!",
    ]);

    // Test compute_in_background
    let result = compute_in_background();
    assert_eq!(result, 5050);

    // Test parallel_transform
    let transformed = parallel_transform();
    assert_eq!(transformed, vec![2, 4, 6, 8, 10, 12]);

    println!("All thread basics tests passed!");
}
