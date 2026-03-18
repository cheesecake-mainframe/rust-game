// Exercise: Result Type
// Type: fix_compiler_error
// Difficulty: intermediate
//
// `Result<T, E>` is Rust's way of handling recoverable errors:
//   - Ok(value)  — the operation succeeded
//   - Err(error) — the operation failed
//
// Using .unwrap() on a Result will panic if it's an Err.
// Instead, use `match` or `?` to handle errors gracefully.
//
// Fix the compiler errors by replacing unwrap() with proper
// error handling using match or the ? operator.

use std::num::ParseIntError;

// TODO: Fix — this function uses unwrap() but should return a Result.
fn parse_age(input: &str) -> u32 {
    let age = input.trim().parse::<u32>().unwrap();
    age
}

// TODO: Fix — this function should propagate errors, not panic.
fn parse_and_double(input: &str) -> Result<u32, ParseIntError> {
    let n = input.parse::<u32>().unwrap();
    Ok(n * 2)
}

// TODO: Fix — the return type doesn't match the function body.
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(a / b)
}

// TODO: Fix — this function chains operations that can fail but
// doesn't handle the errors correctly.
fn process_input(input: &str) -> Result<String, String> {
    let number: i32 = input.parse().unwrap();
    let result = divide(number, number - 1).unwrap();
    Ok(format!("Result: {}", result))
}

fn main() {
    match parse_and_double("21") {
        Ok(val) => println!("Doubled: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 3) {
        Ok(val) => println!("10 / 3 = {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
