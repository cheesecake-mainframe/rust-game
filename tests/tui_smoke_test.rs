//! TUI Smoke Tests
//!
//! Uses ratatui's TestBackend to verify each screen renders without panicking.
//! These are fast tests — no compilation or file I/O beyond loading the catalog.

use ratatui::backend::TestBackend;
use ratatui::Terminal;

use rust_game::app::App;
use rust_game::tui::ui::{TuiApp, Screen};
use rust_game::tui::screens;

fn make_test_tui() -> TuiApp {
    let app = App::new_for_testing().expect("Failed to create test app");
    TuiApp::new(app)
}

fn render_screen(tui: &TuiApp) {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| screens::render(frame, tui))
        .expect("Render should not panic");
}

#[test]
fn test_home_screen_renders() {
    let tui = make_test_tui();
    render_screen(&tui);
}

#[test]
fn test_stats_screen_renders() {
    let mut tui = make_test_tui();
    tui.screen = Screen::Stats;
    render_screen(&tui);
}

#[test]
fn test_module_view_renders() {
    let mut tui = make_test_tui();
    tui.screen = Screen::ModuleView("01_getting_started".to_string());
    render_screen(&tui);
}

#[test]
fn test_exercise_view_renders() {
    let mut tui = make_test_tui();
    tui.screen = Screen::ExerciseView("01_getting_started/hello_world".to_string());
    render_screen(&tui);
}

#[test]
fn test_exercise_view_unknown_id_renders() {
    let mut tui = make_test_tui();
    tui.screen = Screen::ExerciseView("nonexistent/exercise".to_string());
    render_screen(&tui);
}

#[test]
fn test_mcq_screen_renders() {
    let mut tui = make_test_tui();
    tui.screen = Screen::MultipleChoice("07_enums_pattern_matching/predict_match".to_string());
    render_screen(&tui);
}

#[test]
fn test_ai_context_screen_renders() {
    let mut tui = make_test_tui();
    tui.screen = Screen::AiContext("01_getting_started/hello_world".to_string());
    render_screen(&tui);
}
