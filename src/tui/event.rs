use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent};

/// Application events — keys and periodic ticks.
pub enum AppEvent {
    Key(KeyEvent),
    Tick,
}

/// Poll for events with a tick rate.
///
/// Returns `Some(AppEvent)` if an event occurred, or `None` on error.
/// The tick_rate controls how often the UI refreshes (for timers, animations).
pub fn poll_event(tick_rate: Duration) -> Result<AppEvent> {
    if event::poll(tick_rate)? {
        if let Event::Key(key) = event::read()? {
            return Ok(AppEvent::Key(key));
        }
    }
    Ok(AppEvent::Tick)
}
