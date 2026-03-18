use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::exercise::types::ExerciseStatus;
use super::super::ui::TuiApp;

pub fn render(frame: &mut Frame, tui: &TuiApp, module_id: &str) {
    let area = frame.area();

    let module = match tui.app.catalog.get_module(module_id) {
        Some(m) => m,
        None => {
            frame.render_widget(Paragraph::new("Module not found"), area);
            return;
        }
    };

    let outer = Layout::vertical([
        Constraint::Length(3),  // Header
        Constraint::Min(4),    // Exercise list
        Constraint::Length(3), // Footer
    ])
    .split(area);

    // Header
    let header = Paragraph::new(format!("  {} — {}", module.name, module.flavor_text))
        .block(Block::bordered().title(format!(" {} ", module.theme_name)))
        .style(Style::new().fg(Color::Cyan));
    frame.render_widget(header, outer[0]);

    // Exercise list
    let exercises = tui.app.catalog.exercises_for_module(module_id);
    let items: Vec<ListItem> = exercises
        .iter()
        .enumerate()
        .map(|(i, ex)| {
            let status = tui.app.exercise_status(&ex.id);
            let (icon, style) = match status {
                ExerciseStatus::Completed => ("✓", Style::new().fg(Color::Green)),
                ExerciseStatus::InProgress => (">", Style::new().fg(Color::Yellow)),
                ExerciseStatus::Available => (" ", Style::new()),
                ExerciseStatus::Locked => ("·", Style::new().fg(Color::DarkGray)),
            };

            let xp_str = tui
                .app
                .state
                .exercises
                .get(&ex.id)
                .filter(|s| s.status == ExerciseStatus::Completed)
                .map(|s| format!("  +{}xp", s.xp_earned))
                .unwrap_or_default();

            let type_str = format!("{:?}", ex.exercise_type);

            let text = format!(
                " [{}] {} ({}){}",
                icon, ex.name, type_str, xp_str
            );

            let final_style = if i == tui.selected_exercise {
                style.bold().reversed()
            } else {
                style
            };

            ListItem::new(text).style(final_style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::bordered().title(" Exercises "));
    frame.render_widget(list, outer[1]);

    // Footer
    let footer = Paragraph::new(" [Enter] View  [Esc] Back  [q] Quit")
        .block(Block::bordered())
        .style(Style::new().fg(Color::DarkGray));
    frame.render_widget(footer, outer[2]);
}
