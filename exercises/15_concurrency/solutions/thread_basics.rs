// ========================================
// Solution: Thread Basics
// ========================================

use std::thread;

// Fix: Add `move` to transfer ownership of `name` into the thread.
fn greet_from_thread() -> String {
    let name = String::from("Rustacean");

    let handle = thread::spawn(move || {
        format!("Hello from thread, {}!", name)
    });

    handle.join().unwrap()
}

// Fix: Clone `numbers` before moving it into the thread.
// The original stays available after the thread takes the clone.
fn sum_in_thread() -> (i32, Vec<i32>) {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_clone = numbers.clone();

    let handle = thread::spawn(move || {
        numbers_clone.iter().sum::<i32>()
    });

    let sum = handle.join().unwrap();
    (sum, numbers)
}

// Fix: Clone `message` for each thread so each gets its own owned copy.
fn parallel_greetings() -> Vec<String> {
    let message = String::from("Hello");
    let mut handles = Vec::new();

    for i in 0..3 {
        let msg = message.clone(); // Each thread gets its own clone
        let handle = thread::spawn(move || {
            format!("{} from thread {}!", msg, i)
        });
        handles.push(handle);
    }

    let mut results: Vec<String> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    results.sort();
    results
}

// Fix: Join the thread handle and return its u64 result.
fn compute_in_background() -> u64 {
    let handle = thread::spawn(|| {
        let mut sum = 0u64;
        for i in 1..=100 {
            sum += i;
        }
        sum
    });

    handle.join().unwrap()
}

// Fix: Give each thread its own owned chunk of data.
// Clone/split the data before spawning threads so each thread owns its portion.
fn parallel_transform() -> Vec<i32> {
    let data = vec![1, 2, 3, 4, 5, 6];

    let chunk1: Vec<i32> = data[0..3].to_vec();
    let chunk2: Vec<i32> = data[3..6].to_vec();

    let handle1 = thread::spawn(move || {
        chunk1.iter().map(|x| x * 2).collect::<Vec<i32>>()
    });

    let handle2 = thread::spawn(move || {
        chunk2.iter().map(|x| x * 2).collect::<Vec<i32>>()
    });

    let mut results = Vec::new();
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
