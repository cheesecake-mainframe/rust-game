use ratatui::prelude::*;
use ratatui::widgets::*;

/// Render a centered popup overlay on top of the current screen.
///
/// Used for level-up and module completion celebrations.
/// The overlay auto-dismisses after 3 seconds (managed by TuiApp).
pub fn render_centered(
    frame: &mut Frame,
    title: &str,
    lines: Vec<Line>,
    border_color: Color,
    width: u16,
    height: u16,
) {
    let area = frame.area();
    let popup_width = width.min(area.width.saturating_sub(4));
    let popup_height = height.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    frame.render_widget(Clear, popup_area);

    let block = Block::bordered()
        .border_style(Style::new().fg(border_color))
        .title(format!(" {} ", title));

    frame.render_widget(Paragraph::new(lines).block(block), popup_area);
}

/// Render the level-up celebration overlay.
pub fn render_level_up(frame: &mut Frame, old_level: u32, new_level: u32) {
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "     ★  LEVEL UP!  ★",
            Style::new().fg(Color::Yellow).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("     Level {} → Level {}", old_level, new_level),
            Style::new().fg(Color::Cyan).bold(),
        )),
        Line::from(""),
    ];
    render_centered(frame, "Congratulations!", lines, Color::Yellow, 40, 7);
}

/// Render the module completion celebration overlay.
pub fn render_module_complete(frame: &mut Frame, module_name: &str, theme_name: &str) {
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "     ★  MODULE COMPLETE!  ★",
            Style::new().fg(Color::Magenta).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("     {}", module_name),
            Style::new().fg(Color::Cyan).bold(),
        )),
        Line::from(Span::styled(
            format!("     {}", theme_name),
            Style::new().fg(Color::DarkGray).italic(),
        )),
        Line::from(""),
    ];
    render_centered(frame, "Well Done!", lines, Color::Magenta, 44, 8);
}
