// ========================================
// Solution: Defining Traits
// ========================================

struct Book {
    title: String,
    author: String,
    pages: u32,
}

struct Circle {
    radius: f64,
}

struct Playlist {
    name: String,
    songs: Vec<String>,
}

trait Describable {
    fn describe(&self) -> String;

    fn short_description(&self) -> String {
        let full = self.describe();
        if full.len() > 20 {
            format!("{}...", &full[..20])
        } else {
            full
        }
    }
}

trait Measurable {
    fn length(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("{} by {}, {} pages", self.title, self.author, self.pages)
    }
}

impl Describable for Circle {
    fn describe(&self) -> String {
        format!("Circle with radius {:.1}", self.radius)
    }
}

impl Describable for Playlist {
    fn describe(&self) -> String {
        format!("Playlist '{}' with {} songs", self.name, self.songs.len())
    }
}

impl Measurable for Playlist {
    fn length(&self) -> usize {
        self.songs.len()
    }

    fn is_empty(&self) -> bool {
        self.songs.is_empty()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_describe() {
        let book = Book {
            title: String::from("The Rust Book"),
            author: String::from("Steve Klabnik"),
            pages: 560,
        };
        assert_eq!(book.describe(), "The Rust Book by Steve Klabnik, 560 pages");
    }

    #[test]
    fn test_circle_describe() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.describe(), "Circle with radius 5.0");
    }

    #[test]
    fn test_playlist_describe() {
        let playlist = Playlist {
            name: String::from("Coding Jams"),
            songs: vec![
                String::from("Song A"),
                String::from("Song B"),
                String::from("Song C"),
            ],
        };
        assert_eq!(
            playlist.describe(),
            "Playlist 'Coding Jams' with 3 songs"
        );
    }

    #[test]
    fn test_short_description_short_text() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.short_description(), "Circle with radius 5...");
    }

    #[test]
    fn test_short_description_exact_20() {
        let circle = Circle { radius: 42.0 };
        let desc = circle.short_description();
        assert!(desc.ends_with("..."));
        assert_eq!(desc.len(), 23);
    }

    #[test]
    fn test_short_description_no_truncation() {
        let circle = Circle { radius: 1.0 };
        let short = circle.short_description();
        assert!(short.ends_with("..."));
    }

    #[test]
    fn test_playlist_measurable_length() {
        let playlist = Playlist {
            name: String::from("My Mix"),
            songs: vec![
                String::from("Track 1"),
                String::from("Track 2"),
            ],
        };
        assert_eq!(playlist.length(), 2);
    }

    #[test]
    fn test_playlist_measurable_empty() {
        let empty_playlist = Playlist {
            name: String::from("Empty"),
            songs: vec![],
        };
        assert!(empty_playlist.is_empty());
        assert_eq!(empty_playlist.length(), 0);
    }

    #[test]
    fn test_playlist_measurable_not_empty() {
        let playlist = Playlist {
            name: String::from("Party"),
            songs: vec![String::from("Banger")],
        };
        assert!(!playlist.is_empty());
    }

    #[test]
    fn test_book_short_description() {
        let book = Book {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik"),
            pages: 560,
        };
        let short = book.short_description();
        assert!(short.ends_with("..."));
        assert_eq!(short.len(), 23);
    }
}
