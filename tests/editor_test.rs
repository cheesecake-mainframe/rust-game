//! Tests for the embedded editor session.
//!
//! Several of these guard defects that would be invisible at a glance: an
//! editor that renders perfectly but ignores every keystroke, and a save that
//! fails in a way that lets the buffer be overwritten by its own wreckage.

use std::fs;
use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use edtui::EditorMode;
use rust_game::tui::editor::EditorSession;

fn scratch(content: &str) -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("exercise.rs");
    fs::write(&path, content).expect("write");
    (dir, path)
}

fn key(c: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
}

fn press(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn type_str(ed: &EditorSession, s: &str) {
    for c in s.chars() {
        ed.on_key(key(c));
    }
}

/// The load-bearing one. `EditorState` defaults to Normal mode, and edtui's
/// modeless keymap has no Normal-mode bindings at all — a session left in the
/// default mode renders correctly and ignores everything typed into it.
#[test]
fn a_loaded_session_is_in_insert_mode_and_accepts_typing() {
    let (_dir, path) = scratch("fn main() {}\n");
    let ed = EditorSession::load(&path).unwrap();

    assert_eq!(ed.state_mut().mode, EditorMode::Insert);

    type_str(&ed, "x");
    assert!(
        ed.buffer_text().contains('x'),
        "typing must reach the buffer; got {:?}",
        ed.buffer_text()
    );
    assert!(ed.is_dirty());
}

#[test]
fn navigation_keys_do_not_mark_the_buffer_dirty() {
    let (_dir, path) = scratch("fn main() {}\n");
    let ed = EditorSession::load(&path).unwrap();

    ed.on_key(press(KeyCode::Right));
    ed.on_key(press(KeyCode::End));
    ed.on_key(press(KeyCode::Home));

    assert!(
        !ed.is_dirty(),
        "moving the cursor is not an edit; a false dirty flag causes a pointless write on exit"
    );
}

#[test]
fn save_writes_the_buffer_to_disk() {
    let (_dir, path) = scratch("fn main() {}\n");
    let mut ed = EditorSession::load(&path).unwrap();

    ed.on_key(press(KeyCode::End));
    type_str(&ed, "// hi");
    ed.save().unwrap();

    let on_disk = fs::read_to_string(&path).unwrap();
    assert!(on_disk.contains("// hi"), "got {:?}", on_disk);
    assert!(!ed.is_dirty(), "a successful save clears dirty");
}

/// Our own save fires the watcher too. If that were treated as an external
/// change the buffer would be reloaded, discarding anything typed while the
/// compile ran.
#[test]
fn our_own_save_is_not_seen_as_an_external_change() {
    let (_dir, path) = scratch("fn main() {}\n");
    let mut ed = EditorSession::load(&path).unwrap();

    type_str(&ed, "x");
    ed.save().unwrap();

    assert!(ed.disk_matches_last_write());
}

#[test]
fn an_external_write_is_seen_as_an_external_change() {
    let (_dir, path) = scratch("fn main() {}\n");
    let mut ed = EditorSession::load(&path).unwrap();

    ed.save().unwrap();
    fs::write(&path, "fn main() { /* somebody else */ }\n").unwrap();

    assert!(!ed.disk_matches_last_write());
}

#[test]
fn reload_replaces_the_buffer_clears_dirty_and_stays_in_insert_mode() {
    let (_dir, path) = scratch("fn main() {}\n");
    let mut ed = EditorSession::load(&path).unwrap();

    type_str(&ed, "unsaved");
    assert!(ed.is_dirty());

    fs::write(&path, "fn other() {}\n").unwrap();
    ed.reload_from_disk().unwrap();

    assert!(ed.buffer_text().contains("fn other()"));
    assert!(!ed.is_dirty());
    // A freshly constructed EditorState is back in Normal mode, where nothing
    // is bound — the same trap as on first load.
    assert_eq!(ed.state_mut().mode, EditorMode::Insert);

    type_str(&ed, "z");
    assert!(ed.buffer_text().contains('z'), "the editor must still work after a reload");
}

/// A failed save must leave the buffer marked dirty and flagged, because it is
/// then the only good copy and must not be reloaded over.
#[test]
#[cfg(unix)]
fn a_failed_save_keeps_the_buffer_dirty() {
    use std::os::unix::fs::PermissionsExt;

    let (dir, path) = scratch("fn main() {}\n");
    let mut ed = EditorSession::load(&path).unwrap();
    type_str(&ed, "x");

    // The atomic save creates a temp file beside the target, so it is the
    // directory permission that fails it, not the file's.
    fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o555)).unwrap();
    let result = ed.save();
    fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o755)).unwrap();

    if result.is_ok() {
        // Running as root ignores the permission bits; the assertion below
        // would be meaningless, so skip rather than report a false pass.
        eprintln!("skipped: directory permissions are not enforced for this user");
        return;
    }
    assert!(ed.is_dirty(), "a failed save must not clear dirty");
    assert!(ed.last_save_failed(), "a failed save must be recorded");
}

#[test]
fn auto_indent_adds_a_level_after_an_open_brace() {
    let (_dir, path) = scratch("fn main() {");
    let ed = EditorSession::load(&path).unwrap();

    ed.on_key(press(KeyCode::End));
    ed.on_key(press(KeyCode::Enter));
    type_str(&ed, "let x = 1;");

    assert_eq!(ed.buffer_text(), "fn main() {\n    let x = 1;");
}

#[test]
fn auto_indent_matches_the_previous_line() {
    let (_dir, path) = scratch("        deeply_nested();");
    let ed = EditorSession::load(&path).unwrap();

    ed.on_key(press(KeyCode::End));
    ed.on_key(press(KeyCode::Enter));
    type_str(&ed, "next();");

    assert_eq!(
        ed.buffer_text(),
        "        deeply_nested();\n        next();"
    );
}

#[test]
fn tab_inserts_spaces_not_a_tab_character() {
    let (_dir, path) = scratch("x");
    let ed = EditorSession::load(&path).unwrap();

    ed.on_key(press(KeyCode::End));
    ed.on_key(press(KeyCode::Tab));

    let text = ed.buffer_text();
    assert!(!text.contains('\t'), "mixing tabs with the shim's spaces: {:?}", text);
    assert_eq!(text, "x    ");
}

/// A pasted block must arrive verbatim — the indent shim must not see its
/// newlines and staircase the result.
#[test]
fn a_paste_is_inserted_verbatim() {
    let (_dir, path) = scratch("");
    let ed = EditorSession::load(&path).unwrap();

    ed.on_paste("fn a() {\n    b();\n}".to_string());

    let text = ed.buffer_text();
    assert!(text.contains("    b();"), "got {:?}", text);
    assert!(!text.contains("        b();"), "paste was re-indented: {:?}", text);
}

fn ctrl(c: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
}

/// Search routes keys into the query, not the buffer. Enter there jumps the
/// cursor to the match — and an indent computed from the pre-jump line used to
/// land in the middle of the student's code, which the auto-save then persisted.
#[test]
fn confirming_a_search_does_not_modify_the_buffer() {
    let (_dir, path) = scratch("fn main() {\n    let value = 1;\n}\n");
    let ed = EditorSession::load(&path).unwrap();
    let before = ed.buffer_text();

    ed.on_key(ctrl('f'));
    type_str(&ed, "value");
    ed.on_key(press(KeyCode::Enter));

    assert_eq!(
        before,
        ed.buffer_text(),
        "searching must not edit the code being searched"
    );
}

#[test]
fn tab_during_a_search_does_not_edit_the_buffer() {
    let (_dir, path) = scratch("fn main() {\n    let value = 1;\n}\n");
    let ed = EditorSession::load(&path).unwrap();
    let before = ed.buffer_text();

    ed.on_key(ctrl('f'));
    ed.on_key(press(KeyCode::Tab));

    assert_eq!(before, ed.buffer_text(), "Tab belongs to the query, not the code");
}

/// Search moved to Ctrl+F because Ctrl+S is the save key. edtui binds Ctrl+S in
/// *two* modes — start-search and repeat-search — so relocating only the first
/// would leave a search that opens but cannot advance.
#[test]
fn ctrl_f_starts_a_search_and_repeats_it() {
    let (_dir, path) = scratch("let a = 1;\nlet b = 2;\nlet c = 3;\n");
    let ed = EditorSession::load(&path).unwrap();
    let before = ed.buffer_text();

    ed.on_key(ctrl('f'));
    assert!(ed.is_searching(), "Ctrl+F must enter search mode");

    type_str(&ed, "let");
    let first = ed.state_mut().cursor.row;

    // Repeat: this is the registration that would be missing if only the
    // insert-mode binding had been moved.
    ed.on_key(ctrl('f'));
    let second = ed.state_mut().cursor.row;

    assert_ne!(first, second, "a second Ctrl+F must advance to the next match");
    assert_eq!(before, ed.buffer_text(), "searching must not edit the buffer");
}

#[test]
fn ctrl_s_no_longer_starts_a_search() {
    let (_dir, path) = scratch("let a = 1;\n");
    let ed = EditorSession::load(&path).unwrap();

    ed.on_key(ctrl('s'));

    assert!(
        !ed.is_searching(),
        "Ctrl+S belongs to the game now; the editor must not claim it"
    );
}
