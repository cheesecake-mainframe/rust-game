use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::exercise::types::ExerciseStatus;
use crate::tui::widgets::{xp_gauge, hint_panel, overlay};
use super::super::ui::{TuiApp, WatchStatus};

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
        Constraint::Min(6),    // Verification output
        Constraint::Length(hint_panel::height(tui.hints_revealed)),
        Constraint::Length(3), // Footer
    ])
    .split(area);

    // Header with XP bar — uses xp_gauge widget
    let status = tui.app.exercise_status(exercise_id);
    let status_icon = match status {
        ExerciseStatus::Completed => "COMPLETED",
        _ => "WATCHING",
    };
    let p = &tui.app.state.player;
    let header_title = format!(
        " {} | {:?} | {} | Lvl {} | Streak: {} ",
        exercise.name, exercise.exercise_type, status_icon,
        p.level, tui.app.streak.current,
    );
    xp_gauge::render(frame, outer[0], p.xp, header_title);

    // Main verification output area
    let verify_block = Block::bordered().title(" Watch Mode — Saves auto-verify ");
    match &tui.watch_status {
        WatchStatus::Watching => {
            let text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  Watching for changes...",
                    Style::new().fg(Color::DarkGray).italic(),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  File: ", Style::new().fg(Color::DarkGray)),
                    Span::styled(
                        tui.app.workspace.working_path(exercise).display().to_string(),
                        Style::new().fg(Color::Yellow),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "  Edit the file in your editor, save, and verification runs automatically.",
                    Style::new().fg(Color::DarkGray),
                )),
            ];
            frame.render_widget(
                Paragraph::new(text).block(verify_block),
                outer[1],
            );
        }
        WatchStatus::Verifying => {
            let text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  Verifying...",
                    Style::new().fg(Color::Yellow).bold(),
                )),
            ];
            frame.render_widget(
                Paragraph::new(text).block(verify_block),
                outer[1],
            );
        }
        WatchStatus::Passed(msg) => {
            let mut lines = vec![Line::from("")];
            for line in msg.lines() {
                lines.push(Line::from(Span::styled(
                    format!("  {}", line),
                    Style::new().fg(Color::Green).bold(),
                )));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Press 'n' for next exercise or 'Esc' to exit watch mode.",
                Style::new().fg(Color::DarkGray),
            )));
            frame.render_widget(
                Paragraph::new(lines).block(verify_block),
                outer[1],
            );
        }
        WatchStatus::Failed(output) => {
            // Every line, scrollable. rustc's `help:` and its suggested fix
            // routinely land past line 20, and a cargo test assertion diff
            // deeper still — truncating hides exactly what the student needs.
            let mut lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  FAILED",
                    Style::new().fg(Color::Red).bold(),
                )),
                Line::from(""),
            ];
            for line in output.lines() {
                lines.push(Line::from(Span::styled(
                    format!("  {}", line),
                    Style::new().fg(Color::Red),
                )));
            }

            let inner = verify_block.inner(outer[1]);
            let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
            let total = paragraph.line_count(inner.width);
            let max_scroll =
                total.saturating_sub(inner.height as usize).min(u16::MAX as usize) as u16;
            tui.watch_scroll_max.set(max_scroll);
            let offset = tui.watch_scroll.min(max_scroll);

            frame.render_widget(
                paragraph.block(verify_block).scroll((offset, 0)),
                outer[1],
            );
        }
    }

    // Hints panel — uses hint_panel widget
    hint_panel::render(frame, outer[2], &exercise.hints, tui.hints_revealed);

    // Footer
    let footer_text = if let Some(msg) = &tui.status_message {
        msg.as_str()
    } else {
        " [j/k] Scroll  [a] AI  [l] Lesson  [h] Hint  [n] Next  [x] Reset  [Esc] Exit  [q] Quit"
    };
    let footer = Paragraph::new(footer_text)
        .block(Block::bordered())
        .style(Style::new().fg(Color::DarkGray));
    frame.render_widget(footer, outer[3]);
}

/// Render the level-up overlay on top of any screen — delegates to overlay widget.
pub fn render_level_up_overlay(frame: &mut Frame, old_level: u32, new_level: u32) {
    overlay::render_level_up(frame, old_level, new_level);
}
