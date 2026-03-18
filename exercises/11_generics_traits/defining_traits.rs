// ========================================
// Exercise: Defining Traits (ImplementFromScratch)
// ========================================
// Difficulty: Intermediate
// Module: 11 - Generics & Traits
//
// CONCEPT:
// Traits define shared behavior -- like interfaces in other languages.
// You can define a trait with method signatures, then implement it for
// any type. Traits can also have default method implementations.
//
// YOUR TASK:
// 1. Define a trait called `Describable` with:
//    - A required method: `fn describe(&self) -> String`
//    - A default method: `fn short_description(&self) -> String` that returns
//      the first 20 characters of `describe()` followed by "..." if longer
//      than 20 chars, or the full string if 20 chars or fewer.
//
// 2. Define a trait called `Measurable` with:
//    - A required method: `fn length(&self) -> usize`
//    - A required method: `fn is_empty(&self) -> bool`
//
// 3. Implement `Describable` for:
//    - `Book` struct (fields: title, author, pages)
//    - `Circle` struct (fields: radius)
//    - `Playlist` struct (fields: name, songs: Vec<String>)
//
// 4. Implement `Measurable` for:
//    - `Playlist` (length = number of songs)
//
// The structs are defined for you. Implement the traits!
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

// TODO: Define the `Describable` trait here.
// It should have:
//   - fn describe(&self) -> String       (required)
//   - fn short_description(&self) -> String  (default implementation)

// TODO: Define the `Measurable` trait here.
// It should have:
//   - fn length(&self) -> usize
//   - fn is_empty(&self) -> bool

// TODO: Implement `Describable` for `Book`.
// describe() should return: "<title> by <author>, <pages> pages"

// TODO: Implement `Describable` for `Circle`.
// describe() should return: "Circle with radius <radius>"
// (Use one decimal place for radius, e.g., "Circle with radius 5.0")

// TODO: Implement `Describable` for `Playlist`.
// describe() should return: "Playlist '<name>' with <n> songs"

// TODO: Implement `Measurable` for `Playlist`.
// length() returns number of songs, is_empty() returns true if no songs.

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
        // "Circle with radius 5.0" is 22 chars, so it should be truncated
        assert_eq!(circle.short_description(), "Circle with radius 5...");
    }

    #[test]
    fn test_short_description_exact_20() {
        let circle = Circle { radius: 42.0 };
        // "Circle with radius 42.0" is 23 chars -> truncated
        let desc = circle.short_description();
        assert!(desc.ends_with("..."));
        assert_eq!(desc.len(), 23); // 20 chars + "..."
    }

    #[test]
    fn test_short_description_no_truncation() {
        let circle = Circle { radius: 1.0 };
        // "Circle with radius 1.0" is 22 chars -> truncated
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
        assert_eq!(short.len(), 23); // 20 + "..."
    }
}
