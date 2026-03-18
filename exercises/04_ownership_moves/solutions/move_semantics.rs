fn print_length(s: &str) {
    println!("'{}' has {} characters", s, s.len());
}

fn main() {
    let greeting = String::from("Hello, Rust!");

    print_length(&greeting);

    println!("Original: {}", greeting);

    let name = String::from("Ferris");
    let name2 = name.clone();

    println!("name = {}, name2 = {}", name, name2);
}
