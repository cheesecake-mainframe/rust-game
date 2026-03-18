// ========================================
// Solution: Struct Lifetimes
// ========================================

struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn new(text: &'a str) -> Excerpt<'a> {
        Excerpt { text }
    }

    fn words(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

struct Comparison<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Comparison<'a> {
    fn new(left: &'a str, right: &'a str) -> Comparison<'a> {
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

struct Stats<'a> {
    data: &'a [i32],
}

impl<'a> Stats<'a> {
    fn new(data: &'a [i32]) -> Stats<'a> {
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

struct Article<'a> {
    title: String,
    content: &'a str,
    author_name: &'a str,
}

impl<'a> Article<'a> {
    fn new(title: String, content: &'a str, author_name: &'a str) -> Article<'a> {
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
