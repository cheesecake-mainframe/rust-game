use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::exercise::types::ExerciseStatus;
use crate::game::level;
use crate::tui::widgets::xp_gauge;
use super::super::ui::TuiApp;

pub fn render(frame: &mut Frame, tui: &TuiApp) {
    let area = frame.area();

    let outer = Layout::vertical([
        Constraint::Length(3),  // Header
        Constraint::Min(8),    // Stats
        Constraint::Length(3), // Footer
    ])
    .split(area);

    // Header
    let header = Paragraph::new("  Detailed Statistics")
        .block(Block::bordered().title(" Stats "))
        .style(Style::new().fg(Color::Cyan));
    frame.render_widget(header, outer[0]);

    // Stats content
    let p = &tui.app.state.player;
    let completed = tui.app.state.exercises_completed();
    let total = tui.app.catalog.total_exercises();
    let progress_pct = if total > 0 {
        (completed as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };
    let xp_next = level::xp_to_next_level(p.xp);

    let content_layout = Layout::vertical([
        Constraint::Length(3),  // XP gauge
        Constraint::Min(6),    // Text stats
    ])
    .split(outer[1]);

    // XP progress bar — uses xp_gauge widget
    xp_gauge::render_labeled(
        frame,
        content_layout[0],
        p.xp,
        p.level,
        format!("{} XP ({} to next level)", p.xp, xp_next),
    );

    // Text stats
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Streak:       ", Style::new().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", tui.app.streak.current),
                Style::new().fg(Color::Yellow).bold(),
            ),
            Span::styled(
                format!("  (best: {})", tui.app.streak.best),
                Style::new().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Completed:    ", Style::new().fg(Color::DarkGray)),
            Span::styled(
                format!("{}/{} ({}%)", completed, total, progress_pct),
                Style::new().bold(),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Time Played:  ", Style::new().fg(Color::DarkGray)),
            Span::raw(format!("{} minutes", p.total_time_played_secs / 60)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Module Progress",
            Style::new().fg(Color::Cyan).bold(),
        )),
        Line::from(Span::styled(
            "  ─────────────────────────────",
            Style::new().fg(Color::DarkGray),
        )),
    ];

    // Per-module stats
    for module in tui.app.catalog.modules() {
        let exercises = tui.app.catalog.exercises_for_module(&module.id);
        let mod_completed = exercises
            .iter()
            .filter(|e| tui.app.exercise_status(&e.id) == ExerciseStatus::Completed)
            .count();
        let mod_total = exercises.len();
        let mod_xp: u32 = exercises
            .iter()
            .filter_map(|e| tui.app.state.exercises.get(&e.id))
            .map(|s| s.xp_earned)
            .sum();

        let icon = if mod_completed == mod_total && mod_total > 0 {
            "✓"
        } else if mod_completed > 0 {
            ">"
        } else {
            " "
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  [{}] ", icon), Style::new().fg(Color::Green)),
            Span::raw(format!(
                "{}: {}/{}",
                module.name, mod_completed, mod_total
            )),
            if mod_xp > 0 {
                Span::styled(format!("  (+{} XP)", mod_xp), Style::new().fg(Color::Yellow))
            } else {
                Span::raw("")
            },
        ]));
    }

    let stats = Paragraph::new(lines).block(Block::bordered());
    frame.render_widget(stats, content_layout[1]);

    // Footer
    let footer = Paragraph::new(" [Esc] Back  [q] Quit")
        .block(Block::bordered())
        .style(Style::new().fg(Color::DarkGray));
    frame.render_widget(footer, outer[2]);
}
