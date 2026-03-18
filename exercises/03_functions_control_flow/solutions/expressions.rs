fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

fn absolute_value(n: i32) -> i32 {
    if n < 0 {
        -n
    } else {
        n
    }
}

fn classify_number(n: i32) -> &'static str {
    let result = match n {
        0 => "zero",
        1..=100 => "positive",
        _ => "negative",
    };
    result
}

fn main() {
    println!("{} is even: {}", 4, is_even(4));
    println!("|{}| = {}", -5, absolute_value(-5));
    println!("{} is {}", 42, classify_number(42));
}
