// Exercise: First Function
// Type: implement_from_scratch
// Difficulty: beginner
//
// Write a function called `greet` that takes a name (&str)
// and returns a greeting String.
//
// Example: greet("Ferris") should return "Hello, Ferris!"

// TODO: Implement the `greet` function here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_ferris() {
        assert_eq!(greet("Ferris"), "Hello, Ferris!");
    }

    #[test]
    fn test_greet_world() {
        assert_eq!(greet("world"), "Hello, world!");
    }

    #[test]
    fn test_greet_empty() {
        assert_eq!(greet(""), "Hello, !");
    }
}
