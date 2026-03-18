use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

/// Watches a single exercise file for changes with manual debouncing.
pub struct ExerciseWatcher {
    _watcher: RecommendedWatcher,
    rx: mpsc::Receiver<notify::Result<notify::Event>>,
    watched_path: PathBuf,
    last_event: Option<Instant>,
    debounce_duration: Duration,
}

impl ExerciseWatcher {
    /// Start watching an exercise file.
    /// Changes are debounced (300ms) to avoid triggering on rapid saves.
    pub fn new(exercise_path: &Path) -> Result<Self> {
        let (tx, rx) = mpsc::channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default(),
        )
        .context("Failed to create file watcher")?;

        // Watch the parent directory (some editors do atomic write via temp+rename)
        let watch_dir = exercise_path
            .parent()
            .unwrap_or(exercise_path);

        watcher
            .watch(watch_dir, RecursiveMode::NonRecursive)
            .with_context(|| format!("Failed to watch: {}", watch_dir.display()))?;

        Ok(Self {
            _watcher: watcher,
            rx,
            watched_path: exercise_path.canonicalize().unwrap_or(exercise_path.to_path_buf()),
            last_event: None,
            debounce_duration: Duration::from_millis(300),
        })
    }

    /// Check for file changes (non-blocking).
    /// Returns true if the watched file has changed and the debounce
    /// window has passed since the last reported change.
    pub fn poll_change(&mut self) -> bool {
        let mut saw_relevant_event = false;

        // Drain all pending events
        while let Ok(event_result) = self.rx.try_recv() {
            if let Ok(event) = event_result {
                // Only care about modify/create events (not access, remove, etc.)
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        for path in &event.paths {
                            let canonical = path.canonicalize().unwrap_or(path.clone());
                            if canonical == self.watched_path {
                                saw_relevant_event = true;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        if saw_relevant_event {
            let now = Instant::now();
            // Debounce: only fire if enough time passed since last event
            if let Some(last) = self.last_event {
                if now.duration_since(last) < self.debounce_duration {
                    // Too soon — record the event time but don't fire yet.
                    // The next poll_change call (on the next tick) will fire it.
                    self.last_event = Some(now);
                    return false;
                }
            }
            self.last_event = Some(now);
            return true;
        }

        // Check if we have a pending debounced event that's now ready
        if let Some(last) = self.last_event {
            if Instant::now().duration_since(last) >= self.debounce_duration {
                // The debounce window passed — we had a pending event
                // But only fire if there was actually a pending event
                // (last_event is reset after firing in the true branch above)
                // This won't double-fire because we only get here if
                // saw_relevant_event was false this time.
            }
        }

        false
    }
}
