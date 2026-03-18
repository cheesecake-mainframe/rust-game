// Exercise: Move Semantics
// Type: fix_compiler_error
// Difficulty: intermediate
//
// When you pass a String to a function, ownership moves INTO the function.
// After the call, the original variable is no longer valid.
//
// Fix all the compiler errors without removing any println! calls.

fn print_length(s: String) {
    println!("'{}' has {} characters", s, s.len());
}

fn main() {
    let greeting = String::from("Hello, Rust!");

    print_length(greeting);

    // TODO: Fix this — `greeting` was moved into print_length
    println!("Original: {}", greeting);

    let name = String::from("Ferris");
    let name2 = name;

    // TODO: Fix this — `name` was moved to `name2`
    println!("name = {}, name2 = {}", name, name2);
}
