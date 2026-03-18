// Exercise: Immutable by Default
// Type: fix_compiler_error
// Difficulty: beginner
//
// In Rust, variables are immutable by default.
// Fix the compiler error below without changing the println! lines.

fn main() {
    let x = 5;
    println!("x is: {}", x);
    x = 10; // TODO: Fix this — you can't reassign an immutable variable!
    println!("x is now: {}", x);
}
