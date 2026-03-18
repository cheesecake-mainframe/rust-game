// ========================================
// Solution: Channels
// ========================================

use std::sync::mpsc;
use std::thread;

/// Spawns a thread that sends a single message, receives it on the main thread.
fn send_single_message() -> String {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello from thread".to_string()).unwrap();
    });

    rx.recv().unwrap()
}

/// Sends numbers 1..=count from a thread and collects them.
fn send_sequence(count: u32) -> Vec<u32> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=count {
            tx.send(i).unwrap();
        }
        // tx is dropped here, which closes the channel
    });

    // Using the receiver as an iterator: it yields values until the channel closes
    rx.into_iter().collect()
}

/// Multiple producer threads each send their ID. Results collected and sorted.
fn multiple_producers(num_producers: usize) -> Vec<usize> {
    let (tx, rx) = mpsc::channel();

    for id in 0..num_producers {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(id).unwrap();
            // tx_clone is dropped when the thread ends
        });
    }

    // IMPORTANT: Drop the original sender! Otherwise the receiver will wait
    // forever since there's still a live sender (the original).
    drop(tx);

    let mut ids: Vec<usize> = rx.into_iter().collect();
    ids.sort();
    ids
}

/// Multiple producers each send 1..=nums_per_producer. Sum everything.
fn parallel_sum(num_producers: usize, nums_per_producer: u32) -> u64 {
    let (tx, rx) = mpsc::channel();

    for _ in 0..num_producers {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for i in 1..=nums_per_producer {
                tx_clone.send(i as u64).unwrap();
            }
        });
    }

    // Drop original sender so the receiver iterator will terminate
    drop(tx);

    rx.into_iter().sum()
}

/// Two-channel pipeline: main -> worker (strings) -> main (uppercased).
fn uppercase_worker(words: Vec<&str>) -> Vec<String> {
    // Channel 1: main thread sends words TO the worker
    let (input_tx, input_rx) = mpsc::channel::<String>();
    // Channel 2: worker sends uppercased words BACK to main
    let (output_tx, output_rx) = mpsc::channel::<String>();

    // Spawn the worker thread
    thread::spawn(move || {
        // Process each word received from the input channel
        for word in input_rx {
            output_tx.send(word.to_uppercase()).unwrap();
        }
        // When input_rx's iterator ends (all senders dropped), the loop exits
        // and output_tx is dropped, signaling the output channel is done
    });

    // Send all words to the worker
    for word in &words {
        input_tx.send(word.to_string()).unwrap();
    }
    // Drop the sender to close the input channel
    drop(input_tx);

    // Collect all results from the worker
    output_rx.into_iter().collect()
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
        assert_eq!(parallel_sum(3, 4), 30);
    }

    #[test]
    fn test_parallel_sum_single_producer() {
        assert_eq!(parallel_sum(1, 5), 15);
    }

    #[test]
    fn test_parallel_sum_many_producers() {
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
