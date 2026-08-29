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
        Constraint::Length(3),  // Header
        Constraint::Min(6),    // Code preview
        Constraint::Length((exercise.multiple_choice_options.len() as u16) + 4), // Options
        Constraint::Length(3), // Footer
    ])
    .split(area);

    // Header
    let header = Paragraph::new(format!("  {}  |  Multiple Choice  |  {} XP", exercise.name, exercise.base_xp))
        .block(Block::bordered().title(" Predict the Output "))
        .style(Style::new().fg(Color::Cyan));
    frame.render_widget(header, outer[0]);

    // Code preview — read exercise file and display
    let code = std::fs::read_to_string(tui.app.workspace.source_path(exercise))
        .unwrap_or_else(|_| "Could not read exercise file.".into());

    // Strip the header comments to show only the code
    let code_lines: Vec<&str> = code
        .lines()
        .skip_while(|l| l.starts_with("//") || l.is_empty())
        .collect();
    let code_display = code_lines.join("\n");

    let code_widget = Paragraph::new(code_display)
        .block(Block::bordered().title(" Code "))
        .style(Style::new().fg(Color::White))
        .wrap(Wrap { trim: false });
    frame.render_widget(code_widget, outer[1]);

    // Options
    let mut option_lines: Vec<Line> = Vec::new();
    if let Some(ref msg) = tui.mcq_feedback {
        option_lines.push(Line::from(Span::styled(
            format!("  {}", msg),
            if tui.mcq_correct == Some(true) {
                Style::new().fg(Color::Green).bold()
            } else {
                Style::new().fg(Color::Red).bold()
            },
        )));
    } else {
        option_lines.push(Line::from(Span::styled(
            "  What does this program print?",
            Style::new().fg(Color::Yellow),
        )));
    }

    for (i, opt) in exercise.multiple_choice_options.iter().enumerate() {
        let is_selected = i == tui.selected_mcq_option;
        let prefix = if is_selected { "▸ " } else { "  " };
        let style = if is_selected {
            Style::new().fg(Color::Cyan).bold()
        } else {
            Style::new().fg(Color::White)
        };
        // Show option on a single line, replacing newlines with ␊ for display
        let display_text = opt.text.replace('\n', " ↵ ");
        option_lines.push(Line::from(vec![
            Span::styled(format!("  {}{}: ", prefix, opt.label), style),
            Span::styled(display_text, style),
        ]));
    }

    let options = Paragraph::new(option_lines)
        .block(Block::bordered().title(" Select Answer "));
    frame.render_widget(options, outer[2]);

    // Footer
    let footer_text = " [j/k] Navigate  [Enter] Confirm  [Esc] Back  [q] Quit";
    let footer = Paragraph::new(footer_text)
        .block(Block::bordered())
        .style(Style::new().fg(Color::DarkGray));
    frame.render_widget(footer, outer[3]);
}
