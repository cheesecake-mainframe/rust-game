// ========================================
// Exercise: Generic Functions (FixCompilerError)
// ========================================
// Difficulty: Intermediate
// Module: 11 - Generics & Traits
//
// CONCEPT:
// Generic functions allow you to write code that works with many types.
// However, if you want to DO things with those types (print them, compare
// them, etc.), you need to add TRAIT BOUNDS to tell the compiler what
// capabilities the type must have.
//
// YOUR TASK:
// Fix every function signature below so that the code compiles.
// The function BODIES are correct -- only the signatures need fixing.
// You'll need trait bounds like: Display, PartialOrd, Clone, Debug, Add, Default.
//
// HINTS:
// - `std::fmt::Display` lets you print with `{}`
// - `std::fmt::Debug` lets you print with `{:?}`
// - `PartialOrd` lets you use `<`, `>`, `<=`, `>=`
// - `Clone` lets you call `.clone()`
// - `std::ops::Add<Output = T>` lets you use `+`
// - `Default` lets you call `T::default()`
// ========================================

// FIX: This function prints the value, but T has no Display bound.
fn print_value<T>(value: T) {
    println!("The value is: {}", value);
}

// FIX: This function finds the larger of two values, but T can't be compared.
fn max_of_two<T>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

// FIX: This function clones a value and prints the debug representation.
fn clone_and_debug<T>(value: &T) -> T {
    let cloned = value.clone();
    println!("Debug: {:?}", cloned);
    cloned
}

// FIX: This function adds two values together.
fn add_values<T>(a: T, b: T) -> T {
    a + b
}

// FIX: This function creates a default value if the option is None.
fn unwrap_or_default<T>(opt: Option<T>) -> T {
    match opt {
        Some(v) => v,
        None => T::default(),
    }
}

// FIX: This function finds the minimum in a slice. It needs multiple bounds.
fn find_minimum<T>(items: &[T]) -> Option<&T> {
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
