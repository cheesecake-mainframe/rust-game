//! Side-by-side comparison of the student's code against the reference solution.
//!
//! The rows are computed once, when the screen opens — never in the renderer,
//! which ticks five times a second.

use ratatui::prelude::*;
use ratatui::widgets::*;
use similar::{DiffOp, TextDiff};

use super::super::ui::TuiApp;

/// Below this width two columns of Rust are unreadable, so the reference is
/// shown alone.
const MIN_COLS_FOR_SPLIT: u16 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowKind {
    Same,
    /// Only in the student's code.
    Removed,
    /// Only in the reference.
    Added,
    /// Both sides have a line and they differ.
    Changed,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub left: Option<String>,
    pub right: Option<String>,
    pub kind: RowKind,
}

#[derive(Debug, Clone)]
pub struct CompareView {
    pub rows: Vec<Row>,
}

impl CompareView {
    /// Build aligned two-column rows.
    ///
    /// Deliberately not `iter_all_changes`: that yields a *unified* stream of
    /// changes, where a replaced line appears as a deletion followed by an
    /// insertion. Side-by-side needs them paired on one row, which is what
    /// walking `ops()` gives.
    ///
    /// The inputs are tokenized here with `lines()` rather than by
    /// `TextDiff::from_lines`, so the op indices address exactly these vectors
    /// and no line carries a trailing newline into the renderer.
    pub fn build(yours: &str, reference: &str) -> Self {
        let old: Vec<&str> = yours.lines().collect();
        let new: Vec<&str> = reference.lines().collect();
        let diff = TextDiff::from_slices(&old, &new);

        let mut rows = Vec::new();
        for op in diff.ops() {
            match *op {
                DiffOp::Equal {
                    old_index,
                    new_index,
                    len,
                } => {
                    for i in 0..len {
                        rows.push(Row {
                            left: Some(old[old_index + i].to_string()),
                            right: Some(new[new_index + i].to_string()),
                            kind: RowKind::Same,
                        });
                    }
                }
                DiffOp::Delete {
                    old_index, old_len, ..
                } => {
                    for i in 0..old_len {
                        rows.push(Row {
                            left: Some(old[old_index + i].to_string()),
                            right: None,
                            kind: RowKind::Removed,
                        });
                    }
                }
                DiffOp::Insert {
                    new_index, new_len, ..
                } => {
                    for i in 0..new_len {
                        rows.push(Row {
                            left: None,
                            right: Some(new[new_index + i].to_string()),
                            kind: RowKind::Added,
                        });
                    }
                }
                DiffOp::Replace {
                    old_index,
                    old_len,
                    new_index,
                    new_len,
                } => {
                    // Pair them up; the shorter side runs out first and is padded.
                    for i in 0..old_len.max(new_len) {
                        rows.push(Row {
                            left: (i < old_len).then(|| old[old_index + i].to_string()),
                            right: (i < new_len).then(|| new[new_index + i].to_string()),
                            kind: RowKind::Changed,
                        });
                    }
                }
            }
        }
        Self { rows }
    }

    /// How many rows differ in any way.
    pub fn differing(&self) -> usize {
        self.rows.iter().filter(|r| r.kind != RowKind::Same).count()
    }
}

pub fn render(frame: &mut Frame, tui: &TuiApp, exercise_id: &str) {
    let area = frame.area();
    let name = tui
        .app
        .catalog
        .get_exercise(exercise_id)
        .map(|e| e.name.clone())
        .unwrap_or_else(|| exercise_id.to_string());

    let outer = Layout::vertical([Constraint::Min(4), Constraint::Length(3)]).split(area);

    let Some(view) = tui.compare_diff.as_ref() else {
        frame.render_widget(
            Paragraph::new("  Nothing to compare.").block(Block::bordered()),
            outer[0],
        );
        return;
    };

    if area.width < MIN_COLS_FOR_SPLIT {
        // Two 28-column columns of Rust are unreadable; show the reference alone.
        let lines: Vec<Line> = view
            .rows
            .iter()
            .filter_map(|r| r.right.as_ref())
            .map(|s| Line::from(Span::raw(format!(" {}", s))))
            .collect();
        render_pane(
            frame,
            outer[0],
            tui,
            lines,
            format!(" Reference — {} (too narrow to split) ", name),
        );
        render_footer(frame, outer[1], tui, view);
        return;
    }

    let cols = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(outer[0]);

    let left: Vec<Line> = view.rows.iter().map(|r| row_line(r, true)).collect();
    let right: Vec<Line> = view.rows.iter().map(|r| row_line(r, false)).collect();

    render_pane(frame, cols[0], tui, left, format!(" Your code — {} ", name));
    render_pane(frame, cols[1], tui, right, " Reference solution ".to_string());
    render_footer(frame, outer[1], tui, view);
}

/// One side of one row. A missing line renders as an empty tinted gutter so the
/// two columns stay aligned.
fn row_line(row: &Row, left: bool) -> Line<'static> {
    let text = if left { &row.left } else { &row.right };
    let style = match row.kind {
        RowKind::Same => Style::new().fg(Color::DarkGray),
        RowKind::Removed => Style::new().fg(Color::Red),
        RowKind::Added => Style::new().fg(Color::Green),
        RowKind::Changed => Style::new().fg(Color::Yellow),
    };
    match text {
        Some(s) => Line::from(Span::styled(format!(" {}", s), style)),
        None => Line::from(Span::styled("  ~", Style::new().fg(Color::DarkGray).dim())),
    }
}

fn render_pane(frame: &mut Frame, area: Rect, tui: &TuiApp, lines: Vec<Line>, title: String) {
    let block = Block::bordered().title(title);
    let inner = block.inner(area);
    let total = lines.len();
    let max_scroll = total
        .saturating_sub(inner.height as usize)
        .min(u16::MAX as usize) as u16;
    // Both columns publish the same max; they scroll on one shared offset.
    tui.compare_scroll_max.set(max_scroll);
    let offset = tui.compare_scroll.min(max_scroll);
    frame.render_widget(
        Paragraph::new(lines).block(block).scroll((offset, 0)),
        area,
    );
}

fn render_footer(frame: &mut Frame, area: Rect, tui: &TuiApp, view: &CompareView) {
    let text = if let Some(msg) = &tui.status_message {
        msg.clone()
    } else {
        let n = view.differing();
        format!(
            " {} differing line{}   [j/k] Scroll   [Esc] Back to the editor   [Ctrl+C] Quit",
            n,
            if n == 1 { "" } else { "s" }
        )
    };
    frame.render_widget(
        Paragraph::new(text)
            .block(Block::bordered())
            .style(Style::new().fg(Color::DarkGray)),
        area,
    );
}
