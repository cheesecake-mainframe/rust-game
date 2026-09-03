//! The side-by-side comparison view.
//!
//! The shape of a row is the thing under test. A unified diff would represent a
//! changed line as a deletion followed by an insertion — two rows, each half
//! empty. Side-by-side needs them paired on one row, and a test that only counts
//! rows cannot tell the two apart.

use rust_game::tui::screens::{CompareView, RowKind};

#[test]
fn identical_files_produce_only_unchanged_rows() {
    let src = "fn main() {\n    println!(\"hi\");\n}\n";
    let view = CompareView::build(src, src);

    assert_eq!(view.differing(), 0);
    assert!(view.rows.iter().all(|r| r.kind == RowKind::Same));
    assert_eq!(view.rows.len(), 3);
}

#[test]
fn a_changed_line_becomes_one_paired_row() {
    let yours = "fn main() {\n    println(\"hi\");\n}\n";
    let reference = "fn main() {\n    println!(\"hi\");\n}\n";
    let view = CompareView::build(yours, reference);

    assert_eq!(view.differing(), 1, "exactly one line differs");
    let changed: Vec<_> = view
        .rows
        .iter()
        .filter(|r| r.kind != RowKind::Same)
        .collect();
    assert_eq!(changed.len(), 1, "and it must be ONE row, not two half-rows");
    let row = changed[0];
    assert_eq!(row.kind, RowKind::Changed);
    assert!(row.left.is_some() && row.right.is_some(), "both sides present");
    assert!(row.left.as_ref().unwrap().contains("println("));
    assert!(row.right.as_ref().unwrap().contains("println!("));
}

#[test]
fn an_empty_working_copy_is_all_additions() {
    let view = CompareView::build("", "fn main() {}\n");

    assert!(view.rows.iter().all(|r| r.kind == RowKind::Added));
    assert!(view.rows.iter().all(|r| r.left.is_none()));
    assert_eq!(view.rows.len(), 1);
}

#[test]
fn extra_lines_in_your_code_are_removals() {
    let view = CompareView::build("a\nb\nc\n", "a\nc\n");

    let removed: Vec<_> = view
        .rows
        .iter()
        .filter(|r| r.kind == RowKind::Removed)
        .collect();
    assert_eq!(removed.len(), 1);
    assert_eq!(removed[0].left.as_deref(), Some("b"));
    assert!(removed[0].right.is_none());
}

/// A replace where the sides are uneven must pad, not drop.
#[test]
fn an_uneven_replacement_pads_the_shorter_side() {
    let view = CompareView::build("x\n", "y\nz\n");

    assert_eq!(view.rows.len(), 2, "the longer side sets the row count");
    assert!(view.rows.iter().all(|r| r.right.is_some()));
    assert_eq!(
        view.rows.iter().filter(|r| r.left.is_some()).count(),
        1,
        "the shorter side is padded with None, not dropped"
    );
}

/// The renderer turns these straight into spans; an embedded newline would
/// render as a control character.
#[test]
fn rows_carry_no_line_endings() {
    let view = CompareView::build("a\r\nb\n", "a\nc\n");
    for row in &view.rows {
        for s in [&row.left, &row.right].into_iter().flatten() {
            assert!(!s.contains('\n'), "got {:?}", s);
            assert!(!s.contains('\r'), "got {:?}", s);
        }
    }
}
