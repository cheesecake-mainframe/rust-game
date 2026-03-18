// ========================================
// Exercise: Channels (ImplementFromScratch)
// ========================================
// Difficulty: Intermediate
// Module: 15 - Concurrency
//
// CONCEPT:
// Channels provide message-passing concurrency in Rust. The std::sync::mpsc
// module gives you multi-producer, single-consumer channels:
//   - mpsc::channel() -> (Sender<T>, Receiver<T>)
//   - tx.send(value)  -> sends a value (moves ownership to receiver)
//   - rx.recv()       -> blocks until a value arrives (returns Result)
//   - tx.clone()      -> creates another sender for the same channel
//   - rx can be used as an iterator: for msg in rx { ... }
//
// The "mpsc" stands for Multiple Producer, Single Consumer:
//   - You can clone the Sender to create multiple producers
//   - There is only one Receiver
//   - When all Senders are dropped, the Receiver's iterator ends
//
// YOUR TASK:
// Implement each function using channels for inter-thread communication.
// ========================================

use std::sync::mpsc;
use std::thread;

/// Spawns a thread that sends a single message back to the main thread.
/// Returns the received message.
fn send_single_message() -> String {
    todo!()
}

/// Spawns a thread that sends all numbers from 1 to `count` through a channel.
/// The main thread receives and collects them into a vector.
/// Returns the vector of received numbers.
fn send_sequence(count: u32) -> Vec<u32> {
    todo!()
}

/// Spawns `num_producers` threads. Each producer sends its ID (0-based index)
/// through the channel. The main thread collects all IDs and returns them sorted.
///
/// Hint: Clone the sender for each producer thread. The original sender
/// must be dropped so the receiver knows when all producers are done.
fn multiple_producers(num_producers: usize) -> Vec<usize> {
    todo!()
}

/// Each of `num_producers` threads sends the numbers from 1 to `nums_per_producer`.
/// The main thread receives ALL numbers and returns their sum.
///
/// For example: 3 producers each sending 1..=4 means the sum is 3 * (1+2+3+4) = 30.
fn parallel_sum(num_producers: usize, nums_per_producer: u32) -> u64 {
    todo!()
}

/// Spawns a worker thread that receives strings through a channel,
/// converts each to uppercase, and sends the result back through a second channel.
/// The main thread sends all input words, then collects all uppercase results.
///
/// Hint: You need TWO channels -- one for input, one for output.
fn uppercase_worker(words: Vec<&str>) -> Vec<String> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_single_message() {
        let msg = send_single_message();
        assert_eq!(msg, "hello from thread");
    }

    #[test]
    fn test_send_sequence() {
        let result = send_sequence(5);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_send_sequence_large() {
        let result = send_sequence(100);
        let expected: Vec<u32> = (1..=100).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_producers() {
        let ids = multiple_producers(5);
        assert_eq!(ids, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_multiple_producers_single() {
        let ids = multiple_producers(1);
        assert_eq!(ids, vec![0]);
    }

    #[test]
    fn test_parallel_sum() {
        // 3 producers, each sends 1+2+3+4 = 10, total = 30
        assert_eq!(parallel_sum(3, 4), 30);
    }

    #[test]
    fn test_parallel_sum_single_producer() {
        // 1 producer sends 1+2+3+4+5 = 15
        assert_eq!(parallel_sum(1, 5), 15);
    }

    #[test]
    fn test_parallel_sum_many_producers() {
        // 10 producers, each sends 1..=10 (sum=55), total = 550
        assert_eq!(parallel_sum(10, 10), 550);
    }

    #[test]
    fn test_uppercase_worker() {
        let result = uppercase_worker(vec!["hello", "world", "rust"]);
        assert_eq!(result, vec!["HELLO", "WORLD", "RUST"]);
    }

    #[test]
    fn test_uppercase_worker_empty() {
        let result = uppercase_worker(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_uppercase_worker_single() {
        let result = uppercase_worker(vec!["test"]);
        assert_eq!(result, vec!["TEST"]);
    }
}
