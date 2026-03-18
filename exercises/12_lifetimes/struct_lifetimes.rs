// ========================================
// Exercise: Struct Lifetimes (FixCompilerError)
// ========================================
// Difficulty: Intermediate
// Module: 12 - Lifetimes
//
// CONCEPT:
// When a struct holds a reference, it MUST have a lifetime parameter.
// This tells Rust: "this struct cannot outlive the data it references."
//
// Syntax:
//   struct Foo<'a> {
//       field: &'a str,
//   }
//
// Any impl block for such a struct must also declare the lifetime:
//   impl<'a> Foo<'a> { ... }
//
// YOUR TASK:
// Fix all the struct definitions and impl blocks by adding the
// necessary lifetime parameters. Don't change the field types or
// method bodies -- just add lifetime annotations.
// ========================================

// FIX: This struct holds a reference to a string slice but has no lifetime.
struct Excerpt {
    text: &str,
}

impl Excerpt {
    fn new(text: &str) -> Excerpt {
        Excerpt { text }
    }

    fn words(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

// FIX: This struct holds two references with potentially different lifetimes.
struct Comparison {
    left: &str,
    right: &str,
}

impl Comparison {
    fn new(left: &str, right: &str) -> Comparison {
        Comparison { left, right }
    }

    fn longer(&self) -> &str {
        if self.left.len() >= self.right.len() {
            self.left
        } else {
            self.right
        }
    }

    fn are_equal(&self) -> bool {
        self.left == self.right
    }
}

// FIX: This struct holds a reference to a slice of integers.
struct Stats {
    data: &[i32],
}

impl Stats {
    fn new(data: &[i32]) -> Stats {
        Stats { data }
    }

    fn sum(&self) -> i32 {
        self.data.iter().sum()
    }

    fn max(&self) -> Option<&i32> {
        self.data.iter().max()
    }

    fn min(&self) -> Option<&i32> {
        self.data.iter().min()
    }

    fn average(&self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }
        self.sum() as f64 / self.data.len() as f64
    }
}

// FIX: This struct has a mix of owned data and borrowed data.
struct Article {
    title: String,         // owned
    content: &str,         // borrowed
    author_name: &str,     // borrowed
}

impl Article {
    fn new(title: String, content: &str, author_name: &str) -> Article {
        Article {
            title,
            content,
            author_name,
        }
    }

    fn summary(&self) -> String {
        format!(
            "'{}' by {} ({} chars)",
            self.title,
            self.author_name,
            self.content.len()
        )
    }
}

fn main() {
    // Test Excerpt
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt::new(&novel);
    println!("Excerpt: '{}' ({} words)", excerpt.text, excerpt.words());
    assert_eq!(excerpt.words(), 6);

    // Test Comparison
    let comp = Comparison::new("hello", "world!");
    println!("Longer: {}", comp.longer());
    assert_eq!(comp.longer(), "world!");
    assert!(!comp.are_equal());

    let comp2 = Comparison::new("same", "same");
    assert!(comp2.are_equal());

    // Test Stats
    let numbers = vec![4, 7, 2, 9, 1, 5];
    let stats = Stats::new(&numbers);
    assert_eq!(stats.sum(), 28);
    assert_eq!(stats.max(), Some(&9));
    assert_eq!(stats.min(), Some(&1));
    assert!((stats.average() - 4.666666).abs() < 0.001);

    // Test Article
    let content = String::from("Rust is a systems programming language...");
    let author = String::from("Jane Doe");
    let article = Article::new(
        String::from("Why Rust?"),
        &content,
        &author,
    );
    println!("{}", article.summary());
    assert!(article.summary().contains("Why Rust?"));
    assert!(article.summary().contains("Jane Doe"));

    println!("All struct lifetime tests passed!");
}
