// ========================================
// Exercise: Lifetime Basics (FixCompilerError)
// ========================================
// Difficulty: Intermediate
// Module: 12 - Lifetimes
//
// CONCEPT:
// When a function returns a reference, Rust needs to know how long that
// reference is valid. Lifetime annotations (like 'a) tell the compiler
// the relationship between the lifetimes of references in function
// parameters and return values.
//
// Rules:
// - Every reference has a lifetime
// - If a function returns a reference, it must come from one of the inputs
// - Lifetime annotations don't change how long references live; they describe
//   relationships so the compiler can check safety
//
// YOUR TASK:
// Add lifetime annotations to make these functions compile.
// Do NOT change the function bodies -- only add lifetime parameters.
//
// HINTS:
// - The syntax is: fn foo<'a>(x: &'a str) -> &'a str
// - When two parameters could be the source, think about which one
//   the return value actually comes from
// - Sometimes both parameters need the same lifetime
// ========================================

// FIX: Returns a reference to the longer of two string slices.
// Both inputs must live at least as long as the return value.
fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

// FIX: Returns the first word of a string (up to first space or the whole string).
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

// FIX: Returns whichever string slice is non-empty, preferring `primary`.
fn choose_non_empty(primary: &str, fallback: &str) -> &str {
    if !primary.is_empty() {
        primary
    } else {
        fallback
    }
}

// FIX: Returns a reference to the element at the given index.
fn get_element(items: &[i32], index: usize) -> &i32 {
    &items[index]
}

// FIX: Returns references to the first and last elements of a slice.
fn first_and_last(items: &[i32]) -> (&i32, &i32) {
    let first = &items[0];
    let last = &items[items.len() - 1];
    (first, last)
}

// FIX: This function takes a reference and a string slice, returning
// the string slice. The lifetime of the return value is tied only to `text`.
// The `_config` parameter's lifetime is independent.
fn extract_with_config(_config: &str, text: &str) -> &str {
    // We only return something derived from `text`
    if text.len() > 5 {
        &text[..5]
    } else {
        text
    }
}

fn main() {
    // Test longest
    let result;
    {
        let s1 = String::from("long string");
        let s2 = String::from("short");
        result = longest(s1.as_str(), s2.as_str());
        assert_eq!(result, "long string");
    }

    // Test first_word
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    assert_eq!(word, "hello");

    let single = String::from("rust");
    assert_eq!(first_word(&single), "rust");

    // Test choose_non_empty
    assert_eq!(choose_non_empty("hello", "world"), "hello");
    assert_eq!(choose_non_empty("", "fallback"), "fallback");

    // Test get_element
    let nums = vec![10, 20, 30, 40];
    assert_eq!(*get_element(&nums, 2), 30);

    // Test first_and_last
    let (first, last) = first_and_last(&nums);
    assert_eq!(*first, 10);
    assert_eq!(*last, 40);

    // Test extract_with_config
    let config = "some config";
    let text = "hello, world!";
    assert_eq!(extract_with_config(config, text), "hello");

    let short_text = "hi";
    assert_eq!(extract_with_config(config, short_text), "hi");

    println!("All lifetime basics tests passed!");
}
