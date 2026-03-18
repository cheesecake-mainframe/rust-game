#[derive(Debug, Clone, PartialEq)]
struct Song {
    title: String,
    artist: String,
}

struct Playlist {
    name: String,
    songs: Vec<Song>,
}

impl Playlist {
    fn new(name: &str) -> Playlist {
        Playlist {
            name: name.to_string(),
            songs: Vec::new(),
        }
    }

    fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }

    fn song_count(&self) -> usize {
        self.songs.len()
    }

    fn find_by_artist(&self, artist: &str) -> Vec<&Song> {
        self.songs.iter().filter(|s| s.artist == artist).collect()
    }

    fn remove_song(&mut self, title: &str) -> Option<Song> {
        if let Some(pos) = self.songs.iter().position(|s| s.title == title) {
            Some(self.songs.remove(pos))
        } else {
            None
        }
    }
}

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
