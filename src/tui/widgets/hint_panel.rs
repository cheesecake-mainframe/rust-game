use ratatui::prelude::*;
use ratatui::widgets::*;

/// Render the hint panel showing progressively revealed hints.
///
/// Shows "Press <key> to reveal a hint" when none revealed, then numbered hints
/// up to `revealed` count. The key is a parameter because watch mode gives the
/// keyboard to the editor and moves the hint command to `Ctrl+G`.
pub fn render(frame: &mut Frame, area: Rect, hints: &[String], revealed: usize, key_label: &str) {
    let title = format!(" Hints ({}/{}) ", revealed, hints.len());
    let mut lines: Vec<Line> = Vec::new();

    if revealed == 0 && !hints.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  Press {} to reveal a hint.", key_label),
            Style::new().fg(Color::DarkGray),
        )));
    }

    for (i, hint) in hints.iter().take(revealed).enumerate() {
        lines.push(Line::from(vec![
            Span::styled(format!("  {}. ", i + 1), Style::new().fg(Color::Yellow)),
            Span::raw(hint),
        ]));
    }

    let widget = Paragraph::new(lines).block(Block::bordered().title(title));
    frame.render_widget(widget, area);
}

/// Calculate the height needed for the hint panel.
pub fn height(revealed: usize) -> u16 {
    if revealed > 0 {
        (revealed as u16) + 3
    } else {
        3
    }
}
