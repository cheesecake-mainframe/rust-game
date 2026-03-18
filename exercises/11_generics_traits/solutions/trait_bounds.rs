// ========================================
// Solution: Trait Bounds
// ========================================

use std::fmt::{Debug, Display};

fn display_if_greater<T: Display + PartialOrd>(a: T, b: T) {
    if a > b {
        println!("{} is greater than {}", a, b);
    } else {
        println!("{} is not greater than {}", a, b);
    }
}

fn summarize_pair<A, B>(first: A, second: B) -> String
where
    A: Display,
    B: Display,
{
    format!("Pair: ({}, {})", first, second)
}

struct Wrapper<T> {
    value: T,
}

impl<T: Display + PartialOrd> Wrapper<T> {
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

fn print_all(items: &[impl Display]) {
    for item in items {
        println!("Item: {}", item);
    }
}

fn create_default_and_display<T: Default + Display>() -> T {
    let value = T::default();
    println!("Created default: {}", value);
    value
}

fn largest_to_string<T: PartialOrd + Display>(list: &[T]) -> String {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest.to_string()
}

fn sort_and_display<T>(mut items: Vec<T>) -> Vec<T>
where
    T: Debug + Ord,
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
