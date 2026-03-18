// ========================================
// Exercise: Predict the Output
// ========================================
// Type: Reverse Engineering (Fill-in)
// Difficulty: Intermediate
// Module: 04 - Ownership & Moves
//
// CONCEPT:
// Read the code below carefully. Trace through the ownership transfers
// and function calls to determine exactly what will be printed.
//
// YOUR TASK:
// Replace the "???" in ANSWER with the exact program output.
// Each println! produces one line. Separate lines with \n.
// Do NOT include a trailing newline.
// ========================================

/// Your prediction — replace ??? with the exact output.
const ANSWER: &str = r#"???"#;

fn takes_ownership(s: String) -> usize {
    println!("Got: {}", s);
    s.len()
}

fn makes_copy(x: i32) -> i32 {
    println!("Copy: {}", x);
    x + 1
}

fn main() {
    let s = String::from("hello");
    let x = 42;

    let len = takes_ownership(s);
    let y = makes_copy(x);

    println!("Length: {}", len);
    println!("x is still: {}", x);
    println!("y is: {}", y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prediction() {
        let expected = "Got: hello\nCopy: 42\nLength: 5\nx is still: 42\ny is: 43";
        assert_eq!(
            ANSWER, expected,
            "\nYour prediction doesn't match the actual output.\n\
             Expected:\n{}\n\nYour answer:\n{}",
            expected, ANSWER
        );
    }
}
