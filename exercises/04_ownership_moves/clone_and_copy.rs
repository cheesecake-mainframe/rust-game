// Exercise: Clone and Copy
// Type: fix_compiler_error
// Difficulty: intermediate
//
// Types that live on the STACK (integers, bools, chars, floats) implement
// the `Copy` trait — they are automatically copied, not moved.
//
// Types on the HEAP (String, Vec, etc.) must be explicitly `.clone()`d
// if you want a copy.
//
// Fix the compiler errors by understanding which types Copy and which move.

fn main() {
    // Integers implement Copy — this works fine
    let x = 42;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Booleans implement Copy too
    let a = true;
    let b = a;
    println!("a = {}, b = {}", a, b);

    // Strings do NOT implement Copy — they MOVE
    let s1 = String::from("owned");
    let s2 = s1;
    // TODO: Fix this — s1 has been moved
    println!("s1 = {}, s2 = {}", s1, s2);

    // Vec does NOT implement Copy either
    let v1 = vec![1, 2, 3];
    let v2 = v1;
    // TODO: Fix this — v1 has been moved
    println!("v1 = {:?}, v2 = {:?}", v1, v2);
}
