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

#[test]
fn test_lesson_screen_renders() {
    let mut tui = make_test_tui();
    tui.screen = Screen::Lesson("01_getting_started".to_string());
    render_screen(&tui);
}

/// Renders the real markdown path: an actual loaded lesson going through
/// tui-markdown and `Paragraph::line_count`. Without loading the lesson the
/// test only exercises the "no lesson yet" fallback branch.
#[test]
fn test_lesson_screen_renders_real_markdown() {
    let mut tui = make_test_tui();
    let module = tui
        .app
        .catalog
        .get_module("01_getting_started")
        .expect("module 01 should exist")
        .clone();

    // Skip if content has not been authored yet — the screen is still correct,
    // it just takes the fallback branch covered by the test above.
    if let Some(lesson) = rust_game::lesson::load(&module).expect("lesson load must not error") {
        assert!(!lesson.body.is_empty(), "lesson body should have content");
        assert!(!lesson.title.is_empty(), "lesson title should be extracted");
        tui.current_lesson = Some(lesson);
        tui.screen = Screen::Lesson("01_getting_started".to_string());
        render_screen(&tui);

        // The renderer must have published a clamp bound for the key handler.
        // A ~660-word lesson in an 80x24 viewport always overflows.
        assert!(
            tui.lesson_max_scroll.get() > 0,
            "renderer should compute a non-zero max scroll for a full lesson"
        );
    }
}

/// Scrolling past the end must clamp rather than run into blank space.
#[test]
fn test_lesson_scroll_clamps_at_the_end() {
    let mut tui = make_test_tui();
    let module = tui
        .app
        .catalog
        .get_module("01_getting_started")
        .expect("module 01 should exist")
        .clone();

    if let Some(lesson) = rust_game::lesson::load(&module).expect("lesson load must not error") {
        tui.current_lesson = Some(lesson);
        tui.screen = Screen::Lesson("01_getting_started".to_string());
        tui.lesson_scroll = u16::MAX; // absurd offset
        render_screen(&tui); // must not panic, and must clamp
        assert!(tui.lesson_max_scroll.get() < u16::MAX);
    }
}

#[test]
fn test_lesson_screen_unknown_module_renders() {
    let mut tui = make_test_tui();
    tui.screen = Screen::Lesson("nonexistent_module".to_string());
    render_screen(&tui);
}
