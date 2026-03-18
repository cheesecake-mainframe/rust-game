fn rule_one() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // r1 and r2 are no longer used after this point,
    // so we can now take a mutable reference
    let r3 = &mut s;
    println!("{}", r3);
}

fn rule_two() {
    let s = String::from("temporary");
    let r = &s;
    println!("{}", r);
}

fn main() {
    rule_one();
    rule_two();
}
