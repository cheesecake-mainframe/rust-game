// ========================================
// Exercise: Trait Bounds (FixCompilerError)
// ========================================
// Difficulty: Intermediate
// Module: 11 - Generics & Traits
//
// CONCEPT:
// Trait bounds constrain generic types. You can specify them in several ways:
//   - Inline: fn foo<T: Display + Clone>(x: T)
//   - Where clause: fn foo<T>(x: T) where T: Display + Clone
//   - impl Trait: fn foo(x: impl Display)
//
// You can also require multiple bounds, use trait bounds on structs and
// impl blocks, and combine them with lifetime parameters.
//
// YOUR TASK:
// Fix every function/struct/impl so the code compiles. Look for missing
// trait bounds, incorrect where clauses, and missing derives.
//
// HINTS:
// - Some functions need `where` clauses for readability
// - Some structs need trait bounds on their impl blocks
// - `ToString` is automatically implemented for anything implementing `Display`
// ========================================

use std::fmt::{Debug, Display};

// FIX: This function needs trait bounds so it can display and compare items.
fn display_if_greater<T>(a: T, b: T) {
    if a > b {
        println!("{} is greater than {}", a, b);
    } else {
        println!("{} is not greater than {}", a, b);
    }
}

// FIX: This function uses a where clause, but the bounds are wrong/missing.
fn summarize_pair<A, B>(first: A, second: B) -> String
where
    A: Debug,
{
    format!("Pair: ({}, {})", first, second)
}

// FIX: This struct holds a value that must be displayable and comparable.
// The struct definition is fine, but the impl block is missing bounds.
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }

    fn show(&self) {
        println!("Wrapped: {}", self.value);
    }

    fn is_greater_than(&self, other: &T) -> bool {
        self.value > *other
    }
}

// FIX: This function takes impl Trait arguments but the traits are wrong.
fn print_all(items: &[impl Debug]) {
    for item in items {
        println!("Item: {}", item);
    }
}

// FIX: This function returns a value that must implement multiple traits.
// The return type needs to satisfy the bounds used in the function.
fn create_default_and_display<T>() -> T {
    let value = T::default();
    println!("Created default: {}", value);
    value
}

// FIX: This function uses both generic bounds and a concrete return type.
fn largest_to_string<T>(list: &[T]) -> String {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest.to_string()
}

// FIX: This function requires that T can be collected into a Vec and sorted.
fn sort_and_display<T>(mut items: Vec<T>) -> Vec<T>
where
    T: Debug,
{
    items.sort();
    println!("Sorted: {:?}", items);
    items
}

fn main() {
    // Test display_if_greater
    display_if_greater(10, 5);
    display_if_greater(2.5, 3.7);

    // Test summarize_pair
    let summary = summarize_pair(42, "hello");
    println!("{}", summary);

    // Test Wrapper
    let w = Wrapper::new(100);
    w.show();
    assert!(w.is_greater_than(&50));
    assert!(!w.is_greater_than(&200));

    // Test print_all
    print_all(&[1, 2, 3]);

    // Test create_default_and_display
    let val: i32 = create_default_and_display();
    assert_eq!(val, 0);

    // Test largest_to_string
    assert_eq!(largest_to_string(&[1, 5, 3, 9, 2]), "9");
    assert_eq!(largest_to_string(&["apple", "cherry", "banana"]), "cherry");

    // Test sort_and_display
    let sorted = sort_and_display(vec![3, 1, 4, 1, 5]);
    assert_eq!(sorted, vec![1, 1, 3, 4, 5]);

    println!("All trait bound tests passed!");
}
