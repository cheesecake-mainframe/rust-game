// Exercise: Immutable References
// Type: fix_compiler_error
// Difficulty: intermediate
//
// A reference (&T) lets you READ a value without taking ownership.
// The original owner keeps the value — the reference just borrows it.
//
// In Python/JS, this is automatic. In Rust, you must be explicit.
// Fix the compiler errors.

fn calculate_length(s: String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hello");

    // TODO: Fix this — calculate_length takes ownership of s,
    // so we can't use s afterward. Change the function to take a reference.
    let len = calculate_length(s);

    println!("The length of '{}' is {}.", s, len);
}
