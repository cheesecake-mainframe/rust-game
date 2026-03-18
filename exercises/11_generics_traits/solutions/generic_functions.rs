// ========================================
// Solution: Generic Functions
// ========================================

use std::fmt::{Debug, Display};
use std::ops::Add;

fn print_value<T: Display>(value: T) {
    println!("The value is: {}", value);
}

fn max_of_two<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

fn clone_and_debug<T: Clone + Debug>(value: &T) -> T {
    let cloned = value.clone();
    println!("Debug: {:?}", cloned);
    cloned
}

fn add_values<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn unwrap_or_default<T: Default>(opt: Option<T>) -> T {
    match opt {
        Some(v) => v,
        None => T::default(),
    }
}

fn find_minimum<T: PartialOrd + Display>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        return None;
    }
    let mut min = &items[0];
    for item in &items[1..] {
        if item < min {
            min = item;
        }
    }
    println!("Minimum found: {}", min);
    Some(min)
}

fn main() {
    // Test print_value
    print_value(42);
    print_value("hello");
    print_value(3.14);

    // Test max_of_two
    assert_eq!(max_of_two(10, 20), 20);
    assert_eq!(max_of_two(3.5, 2.1), 3.5);

    // Test clone_and_debug
    let original = String::from("Rust");
    let cloned = clone_and_debug(&original);
    assert_eq!(original, cloned);

    // Test add_values
    assert_eq!(add_values(3, 7), 10);
    assert_eq!(add_values(1.5, 2.5), 4.0);

    // Test unwrap_or_default
    assert_eq!(unwrap_or_default(Some(42)), 42);
    assert_eq!(unwrap_or_default::<i32>(None), 0);
    assert_eq!(unwrap_or_default::<String>(None), String::new());

    // Test find_minimum
    assert_eq!(find_minimum(&[5, 3, 8, 1, 9]), Some(&1));
    assert_eq!(find_minimum(&[2.5, 1.1, 3.7]), Some(&1.1));
    assert_eq!(find_minimum::<i32>(&[]), None);

    println!("All generic function tests passed!");
}
