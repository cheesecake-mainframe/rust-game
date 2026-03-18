use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::exercise::types::ExerciseStatus;
use crate::tui::widgets::hint_panel;
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
        Constraint::Length(5),  // Exercise info
        Constraint::Min(4),    // Hints / verification output
        Constraint::Length(3), // Footer
    ])
    .split(area);

    // Header
    let status = tui.app.exercise_status(exercise_id);
    let status_str = match status {
        ExerciseStatus::Completed => " ✓ COMPLETED",
        ExerciseStatus::InProgress => " > IN PROGRESS",
        ExerciseStatus::Available => " ○ AVAILABLE",
        ExerciseStatus::Locked => " · LOCKED",
    };
    let header_text = format!("  {}  |  {:?}  |  {} XP  |{}",
        exercise.name, exercise.exercise_type, exercise.base_xp, status_str);
    let header = Paragraph::new(header_text)
        .block(Block::bordered().title(" Exercise "))
        .style(Style::new().fg(Color::Cyan));
    frame.render_widget(header, outer[0]);

    // Exercise info
    let mut info_lines = vec![
        Line::from(vec![
            Span::styled("  Description: ", Style::new().fg(Color::DarkGray)),
            Span::raw(&exercise.description),
        ]),
        Line::from(vec![
            Span::styled("  File:        ", Style::new().fg(Color::DarkGray)),
            Span::styled(
                exercise.file_path.display().to_string(),
                Style::new().fg(Color::Yellow),
            ),
        ]),
    ];
    if let Some(flavor) = &exercise.flavor_text {
        info_lines.push(Line::from(vec![
            Span::styled("  ", Style::new()),
            Span::styled(flavor.as_str(), Style::new().italic().fg(Color::DarkGray)),
        ]));
    }
    let info = Paragraph::new(info_lines).block(Block::bordered());
    frame.render_widget(info, outer[1]);

    // Main area: hints + verification output
    let main_layout = Layout::vertical([
        Constraint::Min(3),    // Verification output
        Constraint::Length(hint_panel::height(tui.hints_revealed)),
    ])
    .split(outer[2]);

    // Verification output
    let verify_block = Block::bordered().title(" Verification ");
    let verify_content = if let Some(output) = &tui.verify_output {
        let style = if tui.verify_passed == Some(true) {
            Style::new().fg(Color::Green)
        } else {
            Style::new().fg(Color::Red)
        };
        Paragraph::new(output.as_str())
            .style(style)
            .block(verify_block)
            .wrap(Wrap { trim: false })
    } else {
        Paragraph::new("  Press 'v' to verify this exercise.")
            .style(Style::new().fg(Color::DarkGray))
            .block(verify_block)
    };
    frame.render_widget(verify_content, main_layout[0]);

    // Hints — uses hint_panel widget
    hint_panel::render(frame, main_layout[1], &exercise.hints, tui.hints_revealed);

    // Footer
    let footer_text = if let Some(msg) = &tui.status_message {
        msg.clone()
    } else {
        " [w] Watch  [v] Verify  [h] Hint  [o] Open  [n] Next  [Esc] Back  [q] Quit".into()
    };
    let footer = Paragraph::new(footer_text)
        .block(Block::bordered())
        .style(Style::new().fg(Color::DarkGray));
    frame.render_widget(footer, outer[3]);
}
