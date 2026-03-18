// Exercise: Expressions
// Type: fix_compiler_error
// Difficulty: beginner
//
// In Rust, almost everything is an expression that returns a value.
// An `if` block, a `match`, even a bare block `{ ... }` returns a value.
//
// Fix the compiler errors below. Remember:
// - The last expression in a block (WITHOUT a semicolon) is the return value.
// - Adding a semicolon turns an expression into a statement (returns `()`).

fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

fn absolute_value(n: i32) -> i32 {
    // TODO: Fix this — the block should return a value, not `()`
    if n < 0 {
        -n;
    } else {
        n;
    }
}

fn classify_number(n: i32) -> &'static str {
    // TODO: Fix this — match should return a value
    let result = match n {
        0 => "zero",
        1..=100 => "positive",
        _ => "negative",
    }
    result
}

fn main() {
    println!("{} is even: {}", 4, is_even(4));
    println!("|{}| = {}", -5, absolute_value(-5));
    println!("{} is {}", 42, classify_number(42));
}
