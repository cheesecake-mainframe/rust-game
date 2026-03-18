fn main() {
    let greeting = String::from("Hello");
    let first_char = greeting.chars().next().unwrap();
    println!("First character: {}", first_char);

    let mut message = String::from("Hello");
    message.push('!');
    message.push_str(" world");
    println!("{}", message);

    let first = String::from("Hello");
    let second = String::from(" world");
    let combined = first + &second;
    println!("{}", combined);

    let name = String::from("Ferris");
    print_length(&name);

    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);

    let emoji = String::from("🦀 Rust");
    let first_part = &emoji[0..4]; // The crab emoji is 4 bytes
    println!("First part: {}", first_part);
}

fn print_length(s: &str) -> usize {
    println!("'{}' has {} characters", s, s.len());
    s.len()
}
