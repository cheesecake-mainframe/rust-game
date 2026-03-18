// Exercise: What is Ownership?
// Type: fix_compiler_error
// Difficulty: intermediate
//
// In Rust, each value has exactly ONE owner. When the owner goes out
// of scope, the value is dropped. When you assign a String to another
// variable, ownership MOVES — the original variable can no longer be used.
//
// This is the biggest difference from Python/JS where everything is
// garbage collected and you can have unlimited references.
//
// Fix the compiler error below.

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // TODO: This line causes an error — s1 has been MOVED to s2
    println!("s1 = {}, s2 = {}", s1, s2);
}
