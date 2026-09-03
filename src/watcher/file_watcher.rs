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
    /// A change has been seen but not yet reported. Without this the debounce
    /// swallowed saves outright: a second event inside the window updated
    /// `last_event`, returned false, and nothing ever fired it.
    pending: bool,
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
            pending: false,
            debounce_duration: Duration::from_millis(300),
        })
    }

    /// Check for file changes (non-blocking).
    ///
    /// Purely trailing-edge: a burst of events fires exactly once, `debounce`
    /// after the last of them. Firing on the leading edge as well would double
    /// verify, because editors routinely emit a truncate and a write as two
    /// separate events that can land in different polls.
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
            // Restart the window; report nothing yet.
            self.pending = true;
            self.last_event = Some(Instant::now());
            return false;
        }

        // Quiet poll: fire once the window has elapsed.
        if self.pending {
            if let Some(last) = self.last_event {
                if Instant::now().duration_since(last) >= self.debounce_duration {
                    self.pending = false;
                    self.last_event = None;
                    return true;
                }
            }
        }

        false
    }
}
