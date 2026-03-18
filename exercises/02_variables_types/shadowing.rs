// Exercise: Shadowing
// Type: fix_compiler_error
// Difficulty: beginner
//
// Rust allows "shadowing" — redeclaring a variable with `let`.
// Unlike mutation, shadowing lets you change the TYPE of a variable.
//
// Fix this code so it compiles. The variable `spaces` starts as a
// string but should become a number.

fn main() {
    let spaces = "   ";
    // TODO: Fix the line below. Hint: you can shadow `spaces` with a new `let`.
    let spaces: &str = spaces.len();
    println!("Number of spaces: {}", spaces);
}
