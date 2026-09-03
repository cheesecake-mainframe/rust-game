//! TUI Smoke Tests
//!
//! Uses ratatui's TestBackend to verify each screen renders without panicking.
//! These are fast tests — no compilation or file I/O beyond loading the catalog.

use ratatui::backend::TestBackend;
use ratatui::Terminal;

use rust_game::app::App;
use rust_game::tui::editor::EditorSession;
use rust_game::tui::ui::{TuiApp, Screen, WatchStatus};
use rust_game::tui::screens;
use rust_game::state::game_state::EditorLayout;

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

// ── Watch mode ────────────────────────────────────────────────────────────
//
// Watch mode is now the most complex screen in the app and had no render
// coverage at all. The no-editor case is the important one: a file that cannot
// be opened still gets a watch session, and the pane must degrade rather than
// unwrap.

fn watch_tui_with_editor() -> (tempfile::TempDir, TuiApp) {
    let mut tui = make_test_tui();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("hello_world.rs");
    std::fs::write(&path, "fn main() {\n    println!(\"hi\");\n}\n").unwrap();
    tui.editor = Some(EditorSession::load(&path).unwrap());
    tui.screen = Screen::WatchMode("01_getting_started/hello_world".to_string());
    (dir, tui)
}

#[test]
fn test_watch_mode_renders_with_an_editor() {
    let (_dir, tui) = watch_tui_with_editor();
    render_screen(&tui);
}

#[test]
fn test_watch_mode_renders_without_an_editor() {
    let mut tui = make_test_tui();
    tui.screen = Screen::WatchMode("01_getting_started/hello_world".to_string());
    assert!(tui.editor.is_none(), "this is the degraded path under test");
    render_screen(&tui);
}

#[test]
fn test_watch_mode_renders_a_long_failure() {
    let (_dir, mut tui) = watch_tui_with_editor();
    let long = (0..80).map(|i| format!("error line {}", i)).collect::<Vec<_>>().join("\n");
    tui.watch_status = WatchStatus::Failed(long);
    render_screen(&tui);
}

#[test]
fn test_watch_mode_renders_with_errors_expanded() {
    let (_dir, mut tui) = watch_tui_with_editor();
    tui.watch_status = WatchStatus::Failed("error[E0308]: mismatched types".to_string());
    tui.errors_expanded = true;
    render_screen(&tui);
}

/// A terminal too short for a split must still render.
#[test]
fn test_watch_mode_renders_in_a_short_terminal() {
    let (_dir, tui) = watch_tui_with_editor();
    let backend = ratatui::backend::TestBackend::new(80, 12);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| screens::render(frame, &tui))
        .expect("a short terminal must not panic");
}

/// A terminal too narrow for the editor must render, not hang. edtui's line
/// wrapper spins forever with no usable content width, and a hang leaves the
/// terminal in raw mode because the panic hook never runs.
#[test]
fn test_watch_mode_renders_in_a_very_narrow_terminal() {
    let (_dir, tui) = watch_tui_with_editor();
    for width in [4u16, 8, 11] {
        let backend = ratatui::backend::TestBackend::new(width, 24);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| screens::render(frame, &tui))
            .unwrap_or_else(|_| panic!("width {} must render", width));
    }
}

// ── Layout toggle ─────────────────────────────────────────────────────────

fn render_at(tui: &TuiApp, w: u16, h: u16) {
    let backend = ratatui::backend::TestBackend::new(w, h);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| screens::render(frame, tui))
        .unwrap_or_else(|_| panic!("{}x{} must render", w, h));
}

#[test]
fn test_watch_mode_renders_under_each_explicit_layout() {
    for layout in [EditorLayout::SideBySide, EditorLayout::Stacked] {
        let (_dir, mut tui) = watch_tui_with_editor();
        tui.app.state.preferences.editor_layout = Some(layout);
        render_at(&tui, 80, 30);
        render_at(&tui, 160, 30);
    }
}

/// Side by side halves the width, so the editor pane hits its floor sooner —
/// and below that floor edtui's line wrapper never returns.
#[test]
fn test_side_by_side_survives_a_narrow_terminal() {
    let (_dir, mut tui) = watch_tui_with_editor();
    tui.app.state.preferences.editor_layout = Some(EditorLayout::SideBySide);
    for w in [4u16, 8, 20, 40] {
        render_at(&tui, w, 24);
    }
}

/// With no stored preference the terminal decides.
#[test]
fn test_layout_defaults_by_width() {
    let (_dir, tui) = watch_tui_with_editor();
    assert!(tui.app.state.preferences.editor_layout.is_none());

    render_at(&tui, 80, 30);
    assert_eq!(tui.resolved_layout.get(), EditorLayout::Stacked);

    render_at(&tui, 160, 30);
    assert_eq!(tui.resolved_layout.get(), EditorLayout::SideBySide);
}

// ── Compare screen ────────────────────────────────────────────────────────

#[test]
fn test_compare_screen_renders() {
    let (_dir, mut tui) = watch_tui_with_editor();
    tui.compare_diff = Some(rust_game::tui::screens::CompareView::build(
        "fn main() {\n    println(\"hi\");\n}\n",
        "fn main() {\n    println!(\"hi\");\n}\n",
    ));
    tui.screen = Screen::Compare("01_getting_started/hello_world".to_string());
    render_at(&tui, 80, 24);
    render_at(&tui, 160, 40);
}

/// Two 25-column columns of Rust are unreadable, so the narrow path shows the
/// reference alone — it must still render.
#[test]
fn test_compare_screen_renders_narrow() {
    let (_dir, mut tui) = watch_tui_with_editor();
    tui.compare_diff = Some(rust_game::tui::screens::CompareView::build("a\n", "b\n"));
    tui.screen = Screen::Compare("01_getting_started/hello_world".to_string());
    render_at(&tui, 50, 24);
    render_at(&tui, 20, 12);
}

/// Opening the screen with nothing built must not panic.
#[test]
fn test_compare_screen_renders_without_a_diff() {
    let (_dir, mut tui) = watch_tui_with_editor();
    tui.compare_diff = None;
    tui.screen = Screen::Compare("01_getting_started/hello_world".to_string());
    render_at(&tui, 80, 24);
}
