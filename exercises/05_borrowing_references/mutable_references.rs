// Exercise: Mutable References
// Type: fix_compiler_error
// Difficulty: intermediate
//
// A mutable reference (&mut T) lets you MODIFY a borrowed value.
// Rules: you can have EITHER one &mut OR any number of & — never both.
//
// Fix the compiler errors.

fn add_exclamation(s: &String) {
    // TODO: This function tries to modify s, but the reference is immutable.
    // Fix the function signature and the call site.
    s.push_str("!");
}

fn main() {
    let s = String::from("hello");

    // TODO: Fix the call to match the fixed function signature
    add_exclamation(&s);

    println!("{}", s);
}
