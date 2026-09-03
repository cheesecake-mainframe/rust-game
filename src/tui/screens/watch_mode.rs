use edtui::{EditorTheme, EditorView, LineNumbers};
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::exercise::types::{Exercise, ExerciseStatus};
use crate::state::game_state::EditorLayout;
use crate::tui::widgets::{xp_gauge, hint_panel, overlay};
use super::super::ui::{TuiApp, WatchStatus};

/// Below this many rows there is not enough space for a useful split, so the
/// editor takes what there is and the compiler output moves behind Ctrl+E.
const MIN_ROWS_FOR_SPLIT: u16 = 20;

/// Below this, edtui is not rendered at all.
///
/// This is not cosmetic. With no usable content width its line wrapper spins
/// forever — `split_at(0)` never shrinks the remaining span — and because it
/// hangs rather than panics, the panic hook never restores the terminal.
const MIN_EDITOR_COLS: u16 = 12;
const MIN_EDITOR_ROWS: u16 = 3;

/// Which way to split, given the preference and the space available.
///
/// A stored preference always wins; before the student has ever pressed the
/// toggle, the terminal decides. 120 columns is where two panes stop mangling
/// compiler output — roughly half of real rustc lines exceed 38 columns.
fn resolved_layout(tui: &TuiApp, area: Rect) -> EditorLayout {
    tui.app
        .state
        .preferences
        .editor_layout
        .unwrap_or(if area.width >= 120 {
            EditorLayout::SideBySide
        } else {
            EditorLayout::Stacked
        })
}

pub fn render(frame: &mut Frame, tui: &TuiApp, exercise_id: &str) {
    let area = frame.area();

    let exercise = match tui.app.catalog.get_exercise(exercise_id) {
        Some(e) => e,
        None => {
            frame.render_widget(Paragraph::new("Exercise not found"), area);
            return;
        }
    };

    let hints_h = hint_panel::height(tui.hints_revealed);

    // Publish it for the key handler: the toggle needs to know what it is
    // toggling away from, and this is the only place the width is known.
    let layout = resolved_layout(tui, area);
    tui.resolved_layout.set(layout);

    if tui.errors_expanded {
        // Compiler output gets the whole screen; the editor is off duty.
        let rows = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(hints_h),
            Constraint::Length(4),
        ])
        .split(area);
        render_header(frame, rows[0], tui, exercise, exercise_id);
        render_errors(frame, rows[1], tui);
        hint_panel::render(frame, rows[2], &exercise.hints, tui.hints_revealed, "Ctrl+G");
        render_footer(frame, rows[3], tui, true);
    } else if layout == EditorLayout::SideBySide {
        // Code left, compiler output right. The row floor below guards a
        // *vertical* split and is irrelevant here, so it is not consulted.
        let rows = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(hints_h),
            Constraint::Length(4),
        ])
        .split(area);
        let cols =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(rows[1]);
        render_header(frame, rows[0], tui, exercise, exercise_id);
        render_editor(frame, cols[0], tui, exercise);
        render_errors(frame, cols[1], tui);
        hint_panel::render(frame, rows[2], &exercise.hints, tui.hints_revealed, "Ctrl+G");
        render_footer(frame, rows[3], tui, false);
    } else if area.height < MIN_ROWS_FOR_SPLIT {
        let rows = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(hints_h),
            Constraint::Length(4),
        ])
        .split(area);
        render_header(frame, rows[0], tui, exercise, exercise_id);
        render_editor(frame, rows[1], tui, exercise);
        hint_panel::render(frame, rows[2], &exercise.hints, tui.hints_revealed, "Ctrl+G");
        render_footer(frame, rows[3], tui, false);
    } else {
        let rows = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(3), // code
            Constraint::Fill(2), // compiler output
            Constraint::Length(hints_h),
            Constraint::Length(4),
        ])
        .split(area);
        render_header(frame, rows[0], tui, exercise, exercise_id);
        render_editor(frame, rows[1], tui, exercise);
        render_errors(frame, rows[2], tui);
        hint_panel::render(frame, rows[3], &exercise.hints, tui.hints_revealed, "Ctrl+G");
        render_footer(frame, rows[4], tui, false);
    }
}

fn render_header(
    frame: &mut Frame,
    area: Rect,
    tui: &TuiApp,
    exercise: &Exercise,
    exercise_id: &str,
) {
    let status = tui.app.exercise_status(exercise_id);
    let status_icon = match status {
        ExerciseStatus::Completed => "COMPLETED",
        _ => "EDITING",
    };
    let p = &tui.app.state.player;
    let header_title = format!(
        " {} | {:?} | {} | Lvl {} | Streak: {} ",
        exercise.name, exercise.exercise_type, status_icon,
        p.level, tui.app.streak.current,
    );
    xp_gauge::render(frame, area, p.xp, header_title);
}

/// The code pane.
///
/// `editor == None` is a supported state — a file that is unreadable or not
/// UTF-8 still gets a watch session, just not an editable one — so this never
/// unwraps.
fn render_editor(frame: &mut Frame, area: Rect, tui: &TuiApp, exercise: &Exercise) {
    if area.width < MIN_EDITOR_COLS || area.height < MIN_EDITOR_ROWS {
        frame.render_widget(
            Paragraph::new("  Terminal too narrow to edit.").block(Block::bordered()),
            area,
        );
        return;
    }

    let Some(ed) = tui.editor.as_ref() else {
        let block = Block::bordered().title(" Watch Mode — external editor ");
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  This file could not be opened for editing here.",
                Style::new().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  Edit it in your own editor — saves still verify automatically:",
                Style::new().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                format!("  {}", tui.app.workspace.working_path(exercise).display()),
                Style::new().fg(Color::Yellow),
            )),
        ];
        frame.render_widget(Paragraph::new(text).block(block), area);
        return;
    };

    let title = format!(
        " {}{} ",
        ed.path().display(),
        if ed.is_dirty() { "  •unsaved" } else { "" }
    );
    let block = Block::bordered()
        .title(title)
        .border_style(Style::new().fg(if ed.is_dirty() {
            Color::Yellow
        } else {
            Color::DarkGray
        }));

    // The state borrow is confined to this call — see the note in tui::editor.
    let mut state = ed.state_mut();
    let view = EditorView::new(&mut state)
        .theme(EditorTheme::default().block(block))
        .syntax_highlighter(ed.highlighter())
        .line_numbers(LineNumbers::Absolute)
        .wrap(true);
    frame.render_widget(view, area);
}

/// The compiler-output pane, unchanged in substance from before the editor
/// existed: every line kept, scrollable, because rustc's `help:` and its
/// suggested fix routinely land past line 20.
fn render_errors(frame: &mut Frame, area: Rect, tui: &TuiApp) {
    let verify_block = Block::bordered().title(" Compiler output ");
    match &tui.watch_status {
        WatchStatus::Watching => {
            let text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  Waiting for a save...",
                    Style::new().fg(Color::DarkGray).italic(),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  Edit the code above and press Ctrl+S to save — verification runs on save.",
                    Style::new().fg(Color::DarkGray),
                )),
            ];
            frame.render_widget(Paragraph::new(text).block(verify_block), area);
        }
        WatchStatus::Verifying => {
            let text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  Verifying...",
                    Style::new().fg(Color::Yellow).bold(),
                )),
            ];
            frame.render_widget(Paragraph::new(text).block(verify_block), area);
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
                "  Press Ctrl+N for the next exercise, or Esc to leave.",
                Style::new().fg(Color::DarkGray),
            )));
            frame.render_widget(Paragraph::new(lines).block(verify_block), area);
        }
        WatchStatus::Failed(output) => {
            let mut lines = vec![
                Line::from(""),
                Line::from(Span::styled("  FAILED", Style::new().fg(Color::Red).bold())),
                Line::from(""),
            ];
            for line in output.lines() {
                lines.push(Line::from(Span::styled(
                    format!("  {}", line),
                    Style::new().fg(Color::Red),
                )));
            }

            let inner = verify_block.inner(area);
            let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
            let total = paragraph.line_count(inner.width.max(1));
            let max_scroll =
                total.saturating_sub(inner.height as usize).min(u16::MAX as usize) as u16;
            tui.watch_scroll_max.set(max_scroll);
            let offset = tui.watch_scroll.min(max_scroll);

            frame.render_widget(paragraph.block(verify_block).scroll((offset, 0)), area);
        }
    }
}

fn render_footer(frame: &mut Frame, area: Rect, tui: &TuiApp, expanded: bool) {
    let lines: Vec<Line> = if let Some(msg) = &tui.status_message {
        vec![Line::from(msg.as_str())]
    } else if expanded {
        vec![
            Line::from(" [j/k] Scroll   [Esc] Back to the editor   [Ctrl+C] Quit"),
            Line::from(" Showing full compiler output."),
        ]
    } else {
        vec![
            Line::from(
                " Ctrl+S Save  Ctrl+D Diff  Ctrl+E Errors  Ctrl+J Layout  Esc Leave",
            ),
            Line::from(
                " Ctrl+G Hint  Ctrl+L Lesson  Ctrl+A AI  Ctrl+N Next  Ctrl+X Reset  Ctrl+C Quit",
            ),
        ]
    };
    let footer = Paragraph::new(lines)
        .block(Block::bordered())
        .style(Style::new().fg(Color::DarkGray));
    frame.render_widget(footer, area);
}

/// Render the level-up overlay on top of any screen — delegates to overlay widget.
pub fn render_level_up_overlay(frame: &mut Frame, old_level: u32, new_level: u32) {
    overlay::render_level_up(frame, old_level, new_level);
}
