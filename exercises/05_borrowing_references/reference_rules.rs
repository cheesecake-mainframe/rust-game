// Exercise: Reference Rules
// Type: fix_compiler_error
// Difficulty: intermediate
//
// Rust's borrowing rules (enforced at compile time):
// 1. You can have EITHER one mutable reference OR any number of
//    immutable references — but not both at the same time.
// 2. References must always be valid (no dangling references).
//
// Fix each function so it compiles.

fn rule_one() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; // TODO: Can't have &mut while & exists

    println!("{}, {}, {}", r1, r2, r3);
}

fn rule_two() {
    let r;
    {
        let s = String::from("temporary");
        r = &s; // TODO: s is dropped at end of block — r would dangle
    }
    println!("{}", r);
}

fn main() {
    rule_one();
    rule_two();
}
