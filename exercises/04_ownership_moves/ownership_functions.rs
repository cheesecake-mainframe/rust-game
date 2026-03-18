// Exercise: Ownership and Functions
// Type: implement_from_scratch
// Difficulty: intermediate
//
// Implement the functions below with correct ownership semantics.
// Read the test cases to understand what each function should do.
//
// Key insight: when a function TAKES ownership, the caller can't use
// the value after the call. When a function RETURNS ownership, the
// caller receives it.

/// Take ownership of a String and return it with "!" appended.
fn make_excited(s: String) -> String {
    todo!()
}

/// Take ownership of a Vec, add an element, and return it.
fn push_and_return(v: Vec<i32>, item: i32) -> Vec<i32> {
    todo!()
}

/// Return a new String (the function creates and owns it).
fn create_greeting(name: &str) -> String {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_excited() {
        let s = String::from("hello");
        let result = make_excited(s);
        assert_eq!(result, "hello!");
        // Note: `s` can no longer be used here — ownership was moved
    }

    #[test]
    fn test_push_and_return() {
        let v = vec![1, 2, 3];
        let result = push_and_return(v, 4);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_create_greeting() {
        let g = create_greeting("Ferris");
        assert_eq!(g, "Hello, Ferris!");
    }

    #[test]
    fn test_create_greeting_world() {
        let g = create_greeting("world");
        assert_eq!(g, "Hello, world!");
    }
}
