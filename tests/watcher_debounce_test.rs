//! Debounce behaviour of the exercise file watcher.
//!
//! The first test guards a real defect: a save arriving shortly after a
//! previous one used to be recorded and then never reported, so the file was
//! verified once and the student's next save silently did nothing.
//!
//! `notify` delivers events asynchronously, so every assertion polls with a
//! timeout. Asserting on a single `poll_change()` call would flake.

use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use rust_game::watcher::file_watcher::ExerciseWatcher;

fn poll_until_fire(w: &mut ExerciseWatcher, timeout: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if w.poll_change() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    false
}

fn touch(path: &Path, content: &str) {
    fs::write(path, content).expect("write");
}

#[test]
fn a_save_inside_the_debounce_window_still_fires() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("exercise.rs");
    touch(&path, "fn main() {}\n");

    let mut w = ExerciseWatcher::new(&path).unwrap();

    touch(&path, "fn main() { /* 1 */ }\n");
    assert!(
        poll_until_fire(&mut w, Duration::from_secs(5)),
        "the first save must fire"
    );

    // Immediately after that fire, i.e. inside the debounce window. The old
    // implementation recorded this event and dropped it on the floor.
    touch(&path, "fn main() { /* 2 */ }\n");
    assert!(
        poll_until_fire(&mut w, Duration::from_secs(5)),
        "a save shortly after the previous one must still fire"
    );
}

#[test]
fn a_burst_of_saves_fires_exactly_once() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("exercise.rs");
    touch(&path, "fn main() {}\n");

    let mut w = ExerciseWatcher::new(&path).unwrap();

    for i in 0..5 {
        touch(&path, &format!("fn main() {{ /* {} */ }}\n", i));
        std::thread::sleep(Duration::from_millis(20));
    }

    let mut fires = 0;
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(3) {
        if w.poll_change() {
            fires += 1;
        }
        std::thread::sleep(Duration::from_millis(20));
    }

    assert_eq!(
        fires, 1,
        "a burst of saves should compile once, not once per event"
    );
}

#[test]
fn a_quiet_watcher_never_fires() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("exercise.rs");
    touch(&path, "fn main() {}\n");

    let mut w = ExerciseWatcher::new(&path).unwrap();

    assert!(
        !poll_until_fire(&mut w, Duration::from_secs(1)),
        "nothing changed, so nothing should fire"
    );
}
