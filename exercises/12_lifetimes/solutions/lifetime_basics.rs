// ========================================
// Solution: Lifetime Basics
// ========================================

// Both inputs tied to the same lifetime since either could be returned.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

// Single reference input -- lifetime elision handles this automatically,
// but being explicit works too.
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

// Both inputs could be returned, so both share the same lifetime.
fn choose_non_empty<'a>(primary: &'a str, fallback: &'a str) -> &'a str {
    if !primary.is_empty() {
        primary
    } else {
        fallback
    }
}

// Single reference input -- elision works, but explicit is fine.
fn get_element<'a>(items: &'a [i32], index: usize) -> &'a i32 {
    &items[index]
}

// Both returned references come from the same slice.
fn first_and_last<'a>(items: &'a [i32]) -> (&'a i32, &'a i32) {
    let first = &items[0];
    let last = &items[items.len() - 1];
    (first, last)
}

// The return value only depends on `text`, so it gets its own lifetime.
// `_config` has an independent lifetime.
fn extract_with_config<'a, 'b>(_config: &'a str, text: &'b str) -> &'b str {
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
