// ========================================
// Exercise: Shared State (FixCompilerError)
// ========================================
// Difficulty: Intermediate
// Module: 15 - Concurrency
//
// CONCEPT:
// When multiple threads need to read AND write shared data, use:
//   - Mutex<T>: Mutual exclusion lock. Only one thread can access the data at a time.
//     - mutex.lock() returns a MutexGuard that auto-unlocks when dropped.
//   - Arc<T>: Atomically Reference Counted. Thread-safe version of Rc<T>.
//     - Arc::clone(&arc) creates a new owner (increments atomic refcount).
//     - Required because Rc<T> is NOT Send (not thread-safe).
//
// The pattern: Arc<Mutex<T>>
//   1. Wrap your data in Mutex for safe interior mutability
//   2. Wrap the Mutex in Arc for shared ownership across threads
//   3. Arc::clone before moving into each thread
//   4. .lock().unwrap() inside the thread to access the data
//
// YOUR TASK:
// Fix each function so the code compiles and runs correctly.
// ========================================

use std::sync::Mutex;
use std::thread;

// FIX: Mutex is not enough for sharing across threads. Mutex is not Clone
// and cannot be moved into multiple threads. Wrap it in Arc.
fn shared_counter() -> i32 {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..5 {
        // FIX: We try to move `counter` into multiple threads, but it can
        // only be moved once. Use Arc to share ownership.
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // FIX: Access the final value from the counter.
    *counter.lock().unwrap()
}

// FIX: This function has multiple issues:
// 1. Uses Rc instead of Arc (Rc is not thread-safe)
// 2. Missing lock() call to access Mutex contents
// 3. Missing clone for each thread
use std::rc::Rc;

fn parallel_push() -> Vec<i32> {
    let data = Rc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let data_ref = data;  // FIX: Need to clone, and use Arc instead of Rc
        let handle = thread::spawn(move || {
            data_ref.push(i);  // FIX: Need to lock() before accessing
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // FIX: Extract the vector from Arc<Mutex<Vec<i32>>>
    let mut result = data.lock().unwrap().clone();
    result.sort();
    result
}

// FIX: The Mutex guard is held across an await point (well, across a long
// operation). While this compiles, it's bad practice. More importantly here,
// the types are wrong -- fix the Arc/Mutex usage.
fn concurrent_sum() -> i64 {
    let sum = Mutex::new(0i64);
    let numbers = vec![10, 20, 30, 40, 50];
    let mut handles = vec![];

    for &num in &numbers {
        // FIX: Need Arc for thread-safe shared ownership
        let handle = thread::spawn(move || {
            let mut total = sum.lock().unwrap();
            *total += num;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *sum.lock().unwrap()
}

// FIX: Multiple issues with collecting thread results into shared state.
fn parallel_squares() -> Vec<i64> {
    let results = Mutex::new(Vec::new());
    let mut handles = vec![];

    for i in 1..=4i64 {
        // FIX: Need Arc, need proper cloning
        let handle = thread::spawn(move || {
            let square = i * i;
            results.lock().unwrap().push(square);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut result = results.lock().unwrap().clone();
    result.sort();
    result
}

fn main() {
    // Test shared_counter
    let count = shared_counter();
    assert_eq!(count, 5);

    // Test parallel_push
    let data = parallel_push();
    assert_eq!(data, vec![0, 1, 2, 3, 4]);

    // Test concurrent_sum
    let total = concurrent_sum();
    assert_eq!(total, 150);

    // Test parallel_squares
    let squares = parallel_squares();
    assert_eq!(squares, vec![1, 4, 9, 16]);

    println!("All shared state tests passed!");
}
