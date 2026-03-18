use ratatui::prelude::*;
use ratatui::widgets::*;

use super::super::ui::TuiApp;

pub fn render(frame: &mut Frame, tui: &TuiApp, exercise_id: &str) {
    let area = frame.area();

    let exercise = match tui.app.catalog.get_exercise(exercise_id) {
        Some(e) => e,
        None => {
            frame.render_widget(Paragraph::new("Exercise not found"), area);
            return;
        }
    };

    let outer = Layout::vertical([
        Constraint::Length(3), // Header
        Constraint::Min(6),   // AI context content
        Constraint::Length(3), // Footer
    ])
    .split(area);

    // Header
    let header = Paragraph::new(format!("  AI Context: {}", exercise.name))
        .block(Block::bordered().title(" /hint-ai Output "))
        .style(Style::new().fg(Color::Cyan));
    frame.render_widget(header, outer[0]);

    // Generate AI context
    let source = std::fs::read_to_string(&exercise.file_path)
        .unwrap_or_else(|_| "Could not read exercise file.".into());

    let context = crate::ai::context_formatter::format_ai_context(
        exercise,
        &source,
        None,
        &tui.app.catalog,
        &tui.app.state,
    );

    let content = Paragraph::new(context)
        .block(Block::bordered().title(" Copy this to your AI tutor "))
        .wrap(Wrap { trim: false })
        .scroll((tui.ai_context_scroll, 0));
    frame.render_widget(content, outer[1]);

    // Footer
    let footer_text =
        " [j/k] Scroll  [Esc] Back  [q] Quit  |  Copy the text above into your AI chat";
    let footer = Paragraph::new(footer_text)
        .block(Block::bordered())
        .style(Style::new().fg(Color::DarkGray));
    frame.render_widget(footer, outer[2]);
}
