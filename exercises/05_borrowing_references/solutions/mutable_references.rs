fn add_exclamation(s: &mut String) {
    s.push_str("!");
}

fn main() {
    let mut s = String::from("hello");

    add_exclamation(&mut s);

    println!("{}", s);
}
