// Exercise: The Ownership Gauntlet (Boss Battle)
// Type: boss_battle
// Difficulty: intermediate
//
// === BOSS BATTLE ===
// Prove your mastery of ownership, moves, and returning values.
//
// Implement a `Playlist` that owns a Vec of `Song`s.
// You must handle ownership correctly in every method.

/// A song with a title and artist.
#[derive(Debug, Clone, PartialEq)]
struct Song {
    title: String,
    artist: String,
}

/// A playlist that owns its songs.
struct Playlist {
    name: String,
    songs: Vec<Song>,
}

// TODO: Implement these methods for Playlist:
//
// - new(name: &str) -> Playlist
//       Create an empty playlist.
//
// - add_song(&mut self, song: Song)
//       Add a song (takes ownership of it).
//
// - song_count(&self) -> usize
//       Return the number of songs.
//
// - find_by_artist(&self, artist: &str) -> Vec<&Song>
//       Return references to all songs by a given artist.
//
// - remove_song(&mut self, title: &str) -> Option<Song>
//       Remove and return a song by title (gives ownership back).

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_playlist() {
        let pl = Playlist::new("My Mix");
        assert_eq!(pl.song_count(), 0);
    }

    #[test]
    fn test_add_songs() {
        let mut pl = Playlist::new("Rock");
        pl.add_song(Song {
            title: "Bohemian Rhapsody".into(),
            artist: "Queen".into(),
        });
        pl.add_song(Song {
            title: "Stairway to Heaven".into(),
            artist: "Led Zeppelin".into(),
        });
        assert_eq!(pl.song_count(), 2);
    }

    #[test]
    fn test_find_by_artist() {
        let mut pl = Playlist::new("Mix");
        pl.add_song(Song { title: "Song A".into(), artist: "Artist 1".into() });
        pl.add_song(Song { title: "Song B".into(), artist: "Artist 2".into() });
        pl.add_song(Song { title: "Song C".into(), artist: "Artist 1".into() });

        let results = pl.find_by_artist("Artist 1");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].title, "Song A");
        assert_eq!(results[1].title, "Song C");
    }

    #[test]
    fn test_find_by_artist_none() {
        let pl = Playlist::new("Empty");
        let results = pl.find_by_artist("Nobody");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_remove_song() {
        let mut pl = Playlist::new("Mix");
        pl.add_song(Song { title: "Keep".into(), artist: "A".into() });
        pl.add_song(Song { title: "Remove Me".into(), artist: "B".into() });
        pl.add_song(Song { title: "Also Keep".into(), artist: "C".into() });

        let removed = pl.remove_song("Remove Me");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().artist, "B");
        assert_eq!(pl.song_count(), 2);
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut pl = Playlist::new("Mix");
        pl.add_song(Song { title: "Only Song".into(), artist: "A".into() });

        let removed = pl.remove_song("Not Here");
        assert!(removed.is_none());
        assert_eq!(pl.song_count(), 1);
    }
}
