mod home;
mod module_view;
mod exercise_view;
mod watch_mode;
mod stats;
mod multiple_choice;
mod ai_context;

use ratatui::prelude::*;
use super::ui::{TuiApp, Screen};

/// Render the current screen.
pub fn render(frame: &mut Frame, tui: &TuiApp) {
    match &tui.screen {
        Screen::Home => home::render(frame, tui),
        Screen::ModuleView(module_id) => module_view::render(frame, tui, module_id),
        Screen::ExerciseView(exercise_id) => exercise_view::render(frame, tui, exercise_id),
        Screen::WatchMode(exercise_id) => watch_mode::render(frame, tui, exercise_id),
        Screen::Stats => stats::render(frame, tui),
        Screen::MultipleChoice(exercise_id) => multiple_choice::render(frame, tui, exercise_id),
        Screen::AiContext(exercise_id) => ai_context::render(frame, tui, exercise_id),
    }

    // Celebration overlays render on top of any screen
    if let Some((old, new)) = tui.level_up_from {
        if tui.is_level_up_visible() {
            watch_mode::render_level_up_overlay(frame, old, new);
        }
    }
    if let Some((ref name, ref theme)) = tui.module_complete_info {
        if tui.is_module_complete_visible() {
            crate::tui::widgets::overlay::render_module_complete(frame, name, theme);
        }
    }
}
