use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};

/// Application events — keys and periodic ticks.
pub enum AppEvent {
    Key(KeyEvent),
    /// A bracketed paste arriving as one block rather than per-character keys.
    Paste(String),
    Tick,
}

/// Poll for events with a tick rate.
///
/// Returns `Some(AppEvent)` if an event occurred, or `None` on error.
/// The tick_rate controls how often the UI refreshes (for timers, animations).
pub fn poll_event(tick_rate: Duration) -> Result<AppEvent> {
    if event::poll(tick_rate)? {
        match event::read()? {
            Event::Key(key) => {
                // Windows reports both press and release; without this every key
                // fires twice — two hints burned, two rows skipped, two resets.
                // On Linux `kind` is always `Press`, so this is a no-op there.
                if key.kind == KeyEventKind::Press {
                    return Ok(AppEvent::Key(key));
                }
            }
            Event::Paste(text) => return Ok(AppEvent::Paste(text)),
            _ => {}
        }
    }
    Ok(AppEvent::Tick)
}
