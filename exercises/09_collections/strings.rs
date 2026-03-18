// Exercise: Strings
// Type: fix_compiler_error
// Difficulty: intermediate
//
// Rust has two main string types:
//   - String    — owned, growable, heap-allocated
//   - &str      — borrowed string slice (a view into a String or literal)
//
// Common mistakes:
//   - Indexing a String by byte position (strings are UTF-8, not char arrays)
//   - Using &String where &str is expected
//   - Confusing push_str (appends &str) with push (appends a single char)
//
// Fix all the compiler errors below.

fn main() {
    // TODO: Fix — can't index a String with [0] in Rust.
    // Strings are UTF-8 encoded, so byte indexing is not allowed.
    let greeting = String::from("Hello");
    let first_char = greeting[0];
    println!("First character: {}", first_char);

    // TODO: Fix — push_str takes a &str, not a char.
    // push takes a single char, push_str takes a string slice.
    let mut message = String::from("Hello");
    message.push_str('!');
    message.push(" world");
    println!("{}", message);

    // TODO: Fix — can't concatenate two Strings with + like this.
    // The + operator takes ownership of the left side and borrows the right.
    let first = String::from("Hello");
    let second = String::from(" world");
    let combined = first + second;
    println!("{}", combined);

    // TODO: Fix — function expects &str but we're passing &String.
    // Actually this one auto-coerces, but the function signature is wrong.
    let name = String::from("Ferris");
    print_length(name);

    // TODO: Fix — can't modify a string slice.
    let mut s: &str = "hello";
    s.push_str(" world");
    println!("{}", s);

    // TODO: Fix — string slicing by byte index can panic on multi-byte chars.
    // This emoji is 4 bytes, so slicing at [0..1] will panic.
    let emoji = String::from("🦀 Rust");
    let first_part = &emoji[0..1]; // This panics!
    println!("First part: {}", first_part);
}

fn print_length(s: String) -> usize {
    // TODO: Fix — this function takes ownership unnecessarily.
    // Change it to borrow instead, so the caller can still use the string.
    println!("'{}' has {} characters", s, s.len());
    s.len()
}
