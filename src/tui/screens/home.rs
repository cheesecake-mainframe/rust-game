use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::exercise::types::ExerciseStatus;
use crate::game::level;
use crate::tui::widgets::xp_gauge;
use super::super::ui::TuiApp;

pub fn render(frame: &mut Frame, tui: &TuiApp) {
    let area = frame.area();

    // Outer layout: title bar, main content, footer
    let outer = Layout::vertical([
        Constraint::Length(3),  // Title bar
        Constraint::Min(6),    // Main content
        Constraint::Length(3), // Footer
    ])
    .split(area);

    // Title bar — XP gauge widget
    let title = format!(
        " RUST-GAME  |  Level {}  |  {} XP  |  Streak: {} ",
        tui.app.state.player.level,
        tui.app.state.player.xp,
        tui.app.streak.current,
    );
    xp_gauge::render(frame, outer[0], tui.app.state.player.xp, title);

    // Main content: module list + stats panel
    let main_layout = Layout::horizontal([
        Constraint::Percentage(60),
        Constraint::Percentage(40),
    ])
    .split(outer[1]);

    // Module list
    render_module_list(frame, tui, main_layout[0]);

    // Stats panel
    render_stats_panel(frame, tui, main_layout[1]);

    // Footer
    let mut footer_text = " [Enter] Select  [n] Next  [s] Stats  [q] Quit";
    if let Some(msg) = &tui.status_message {
        footer_text = Box::leak(msg.clone().into_boxed_str());
    }
    let footer = Paragraph::new(footer_text)
        .block(Block::bordered())
        .style(Style::new().fg(Color::DarkGray));
    frame.render_widget(footer, outer[2]);
}

fn render_module_list(frame: &mut Frame, tui: &TuiApp, area: Rect) {
    let unlocked = tui.app.compute_unlocked_modules();

    let items: Vec<ListItem> = tui
        .app
        .catalog
        .modules()
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let is_unlocked = unlocked.get(&m.id).copied().unwrap_or(false);
            let exercises = tui.app.catalog.exercises_for_module(&m.id);
            let completed = exercises
                .iter()
                .filter(|e| tui.app.exercise_status(&e.id) == ExerciseStatus::Completed)
                .count();
            let total = exercises.len();

            let icon = if !is_unlocked {
                "🔒"
            } else if completed == total && total > 0 {
                "✅"
            } else if completed > 0 {
                "🔶"
            } else {
                "  "
            };

            let progress = if total > 0 {
                format!(" ({}/{})", completed, total)
            } else {
                String::new()
            };

            let text = format!("{} {} — {}{}", icon, m.name, m.theme_name, progress);
            let style = if i == tui.selected_module {
                Style::new().fg(Color::Yellow).bold()
            } else if !is_unlocked {
                Style::new().fg(Color::DarkGray)
            } else {
                Style::new()
            };
            ListItem::new(text).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::bordered().title(" Modules "))
        .highlight_style(Style::new().fg(Color::Yellow).bold());
    frame.render_widget(list, area);
}

fn render_stats_panel(frame: &mut Frame, tui: &TuiApp, area: Rect) {
    let p = &tui.app.state.player;
    let completed = tui.app.state.exercises_completed();
    let total = tui.app.catalog.total_exercises();
    let xp_next = level::xp_to_next_level(p.xp);

    let text = vec![
        Line::from(vec![
            Span::styled("Level:     ", Style::new().fg(Color::DarkGray)),
            Span::styled(format!("{}", p.level), Style::new().bold()),
        ]),
        Line::from(vec![
            Span::styled("XP:        ", Style::new().fg(Color::DarkGray)),
            Span::styled(format!("{}", p.xp), Style::new().bold()),
            Span::styled(format!("  ({} to next)", xp_next), Style::new().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("Streak:    ", Style::new().fg(Color::DarkGray)),
            Span::styled(format!("{}", tui.app.streak.current), Style::new().fg(Color::Yellow).bold()),
            Span::styled(format!("  (best: {})", tui.app.streak.best), Style::new().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("Progress:  ", Style::new().fg(Color::DarkGray)),
            Span::styled(format!("{}/{}", completed, total), Style::new().bold()),
        ]),
        Line::from(""),
        Line::from(Span::styled("Recent Activity", Style::new().fg(Color::Cyan).bold())),
        Line::from(Span::styled("─────────────────", Style::new().fg(Color::DarkGray))),
    ];

    // Show last few completed exercises
    let mut recent: Vec<(&String, &crate::state::game_state::ExerciseState)> = tui
        .app
        .state
        .exercises
        .iter()
        .filter(|(_, s)| s.status == ExerciseStatus::Completed)
        .collect();
    recent.sort_by(|a, b| b.1.completed_at.cmp(&a.1.completed_at));

    let mut lines = text;
    for (id, state) in recent.iter().take(5) {
        let name = tui
            .app
            .catalog
            .get_exercise(id)
            .map(|e| e.name.as_str())
            .unwrap_or(id);
        lines.push(Line::from(vec![
            Span::styled("  ✓ ", Style::new().fg(Color::Green)),
            Span::raw(format!("{} ", name)),
            Span::styled(format!("+{}xp", state.xp_earned), Style::new().fg(Color::Yellow)),
        ]));
    }

    let panel = Paragraph::new(lines)
        .block(Block::bordered().title(" Player Stats "));
    frame.render_widget(panel, area);
}
