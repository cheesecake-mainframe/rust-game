// ========================================
// Solution: Shared State
// ========================================

use std::sync::{Arc, Mutex};
use std::thread;

// Fix: Wrap Mutex in Arc for thread-safe shared ownership.
fn shared_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        // Fix: Arc::clone creates another owner (cheap atomic refcount increment).
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock() returns a MutexGuard -- exclusive access until it's dropped.
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // All threads are joined, so we're the only accessor.
    // Bind to a variable so the MutexGuard is dropped before counter.
    let result = *counter.lock().unwrap();
    result
}

// Fix: Use Arc (not Rc), clone for each thread, and lock() before accessing.
fn parallel_push() -> Vec<i32> {
    let data = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let data_clone = Arc::clone(&data); // Fix: Clone the Arc for each thread
        let handle = thread::spawn(move || {
            data_clone.lock().unwrap().push(i); // Fix: lock() to access the Vec
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut result = data.lock().unwrap().clone();
    result.sort();
    result
}

// Fix: Wrap sum in Arc<Mutex<_>> and clone for each thread.
fn concurrent_sum() -> i64 {
    let sum = Arc::new(Mutex::new(0i64));
    let numbers = vec![10, 20, 30, 40, 50];
    let mut handles = vec![];

    for &num in &numbers {
        let sum_clone = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            let mut total = sum_clone.lock().unwrap();
            *total += num;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *sum.lock().unwrap();
    result
}

// Fix: Wrap in Arc and clone for each thread.
fn parallel_squares() -> Vec<i64> {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 1..=4i64 {
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let square = i * i;
            results_clone.lock().unwrap().push(square);
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
