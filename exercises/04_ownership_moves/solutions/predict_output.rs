// Solution: Predict the Output
//
// String is moved into takes_ownership, so it prints "Got: hello".
// i32 implements Copy, so makes_copy gets a copy and prints "Copy: 42".
// Then we print the return values and x (which is still valid because i32 is Copy).

const ANSWER: &str = r#"Got: hello
Copy: 42
Length: 5
x is still: 42
y is: 43"#;

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
