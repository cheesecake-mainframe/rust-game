use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::game::level;

/// Render an XP progress gauge with a custom block title.
///
/// Used on the dashboard, stats screen, and watch-mode header to show
/// level progression as a green bar over dark gray.
pub fn render(frame: &mut Frame, area: Rect, xp: u64, title: String) {
    let progress = level::level_progress(xp);
    let gauge = Gauge::default()
        .block(Block::bordered().title(title).title_style(Style::new().bold().fg(Color::Cyan)))
        .ratio(progress.clamp(0.0, 1.0))
        .gauge_style(Style::new().fg(Color::Green).bg(Color::DarkGray));
    frame.render_widget(gauge, area);
}

/// Render a labeled XP gauge with explicit label text (for the stats screen).
pub fn render_labeled(frame: &mut Frame, area: Rect, xp: u64, level: u32, label: String) {
    let progress = level::level_progress(xp);
    let gauge = Gauge::default()
        .block(
            Block::bordered()
                .title(format!(" Level {} → Level {} ", level, level + 1)),
        )
        .ratio(progress.clamp(0.0, 1.0))
        .gauge_style(Style::new().fg(Color::Green).bg(Color::DarkGray))
        .label(label);
    frame.render_widget(gauge, area);
}
