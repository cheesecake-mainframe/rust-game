//! The embedded code editor used by watch mode.
//!
//! Owns everything about the student's in-memory buffer: loading it, writing it
//! back atomically, deciding whether a change on disk came from us or from
//! somewhere else, and the small indentation shim edtui does not provide.
//!
//! The editor state lives behind a `RefCell` because `screens::render` takes
//! `&TuiApp` while edtui's `EditorView::new` needs `&mut EditorState`. That is
//! the same constraint the `Cell<u16>` scroll fields already work around, with
//! one extra hazard: a borrow held across a call back into `TuiApp` panics.
//! Every borrow in this file is taken in the tightest possible scope.

use std::cell::{Cell, RefCell, RefMut};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent};
use edtui::actions::search::StartSearch;
use edtui::actions::{Chainable, FindNext, InsertChar, StopSearch, SwitchMode};
use edtui::events::{KeyEventRegister, KeyInput};
use edtui::syntect::highlighting::{Theme, ThemeSet};
use edtui::syntect::parsing::{SyntaxReference, SyntaxSet};
use edtui::{
    EditorEventHandler, EditorMode, EditorState, Lines, SyntaxHighlighter, SYNTAX_SET, THEME_SET,
};

/// Spaces per indent level — Rust's convention, and what Tab inserts.
pub const INDENT: usize = 4;

/// One of edtui's bundled themes. Note the hyphen: edtui renames syntect's
/// `base16-ocean.dark` to `base16-ocean-dark`, and its own doc example still
/// shows the old spelling.
const THEME_NAME: &str = "base16-ocean-dark";

pub struct EditorSession {
    state: RefCell<EditorState>,
    /// Built once. Rebuilding it per keystroke would allocate a ~60-entry
    /// keymap five times a second.
    handler: RefCell<EditorEventHandler>,
    path: PathBuf,
    /// Hash of the content we last wrote, so the watcher's event for our own
    /// save is not mistaken for somebody else's edit.
    last_write: Option<u64>,
    /// After a failed save the buffer is the only good copy, so an external
    /// change must not be allowed to reload over it.
    last_save_failed: bool,
    dirty: Cell<bool>,
    // Resolved once. `SyntaxHighlighter` is not `Clone` and `EditorView` takes
    // it by value, so the parts are cached and the highlighter is rebuilt per
    // frame from them — two `Arc` clones and two cheap struct clones.
    //
    // `None` when the theme or the Rust syntax could not be resolved. Colour is
    // a nicety; losing it must not cost the student their editor.
    highlight: Option<HighlightParts>,
}

struct HighlightParts {
    theme: Theme,
    theme_set: Arc<ThemeSet>,
    syntax: SyntaxReference,
    syntax_set: Arc<SyntaxSet>,
}

impl EditorSession {
    /// Open a file for editing. Fails on unreadable or non-UTF-8 content; the
    /// caller is expected to fall back to the read-only watch behavior.
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let mut state = EditorState::new(Lines::from(content.as_str()));
        // Load-bearing. `EditorState` defaults to `EditorMode::Normal`, and
        // edtui's modeless keymap registers *no* Normal-mode bindings — the
        // character-insert path is gated on Insert mode. Without this line
        // every keystroke, plain letters included, is a silent no-op.
        state.mode = EditorMode::Insert;

        let theme_set = THEME_SET.clone();
        let syntax_set = SYNTAX_SET.clone();
        let highlight = match (
            theme_set.themes.get(THEME_NAME).cloned(),
            syntax_set.find_syntax_by_extension("rs").cloned(),
        ) {
            (Some(theme), Some(syntax)) => Some(HighlightParts {
                theme,
                theme_set,
                syntax,
                syntax_set,
            }),
            // Highlighting is optional; the editor is not.
            _ => None,
        };

        Ok(Self {
            state: RefCell::new(state),
            handler: RefCell::new(emacs_handler_with_search_on_ctrl_f()),
            path: path.to_path_buf(),
            last_write: Some(hash_str(&content)),
            last_save_failed: false,
            dirty: Cell::new(false),
            highlight,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.get()
    }

    pub fn last_save_failed(&self) -> bool {
        self.last_save_failed
    }

    /// Borrow the state for rendering. Must not be held across a call back into
    /// `TuiApp` — see the module comment.
    pub fn state_mut(&self) -> RefMut<'_, EditorState> {
        self.state.borrow_mut()
    }

    /// A fresh highlighter for this frame, or `None` if highlighting is
    /// unavailable. Cheap: `Arc` clones plus two struct clones. See the field
    /// comment for why it cannot simply be stored.
    pub fn highlighter(&self) -> Option<SyntaxHighlighter> {
        let h = self.highlight.as_ref()?;
        Some(SyntaxHighlighter::with_sets(
            h.theme.clone(),
            h.theme_set.clone(),
            h.syntax.clone(),
            h.syntax_set.clone(),
        ))
    }

    /// Write the buffer to disk atomically.
    ///
    /// A plain `fs::write` truncates before writing, so a failure partway
    /// through leaves a truncated file *and* fires the watcher — and because
    /// disk wins on an external change, that truncated file would be reloaded
    /// over the only good copy of the student's work. Writing to a sibling temp
    /// file and renaming makes the file either wholly old or wholly new. The
    /// watcher already watches the parent directory for exactly this pattern.
    pub fn save(&mut self) -> Result<()> {
        let content = self.buffer_text();
        let dir = self.path.parent().unwrap_or_else(|| Path::new("."));

        let result = (|| -> Result<()> {
            let mut tmp = tempfile::NamedTempFile::new_in(dir)
                .with_context(|| format!("Failed to create a temp file in {}", dir.display()))?;
            tmp.write_all(content.as_bytes())
                .context("Failed to write the buffer")?;
            tmp.flush().context("Failed to flush the buffer")?;
            // `NamedTempFile` is created 0600 and `persist` keeps that mode, so
            // without this the first save quietly tightens the file's
            // permissions from whatever the workspace copy had.
            if let Ok(meta) = fs::metadata(&self.path) {
                let _ = tmp.as_file().set_permissions(meta.permissions());
            }
            tmp.persist(&self.path)
                .map_err(|e| anyhow::anyhow!("{}", e))
                .with_context(|| format!("Failed to replace {}", self.path.display()))?;
            Ok(())
        })();

        match result {
            Ok(()) => {
                self.last_write = Some(hash_str(&content));
                self.last_save_failed = false;
                self.dirty.set(false);
                Ok(())
            }
            Err(e) => {
                // Keep `dirty` set: the buffer is still the only good copy.
                self.last_save_failed = true;
                Err(e)
            }
        }
    }

    /// Is the file on disk still exactly what we last wrote?
    ///
    /// Content only, deliberately not mtime: content is what decides whether a
    /// reload is needed, and mtime granularity varies by filesystem.
    pub fn disk_matches_last_write(&self) -> bool {
        match fs::read_to_string(&self.path) {
            Ok(content) => Some(hash_str(&content)) == self.last_write,
            // Unreadable or gone. Reloading nothing over a live buffer would be
            // worse than doing nothing, so report "no external change".
            Err(_) => true,
        }
    }

    /// Replace the buffer with what is on disk.
    ///
    /// The cursor deliberately returns to the top. After a reset the file is
    /// usually much shorter, and edtui indexes rows directly, so carrying a
    /// stale deep cursor over is an out-of-bounds hazard.
    pub fn reload_from_disk(&mut self) -> Result<()> {
        let content = fs::read_to_string(&self.path)
            .with_context(|| format!("Failed to re-read {}", self.path.display()))?;

        let mut state = EditorState::new(Lines::from(content.as_str()));
        // A newly constructed state is back in Normal mode, where the modeless
        // keymap binds nothing — same trap as in `load`.
        state.mode = EditorMode::Insert;
        *self.state.borrow_mut() = state;

        self.last_write = Some(hash_str(&content));
        self.last_save_failed = false;
        self.dirty.set(false);
        Ok(())
    }

    /// Is the editor currently in its incremental-search mode?
    pub fn is_searching(&self) -> bool {
        self.state.borrow().mode == EditorMode::Search
    }

    /// Leave search without leaving the editor — what `Ctrl+g` does.
    pub fn cancel_search(&self) {
        let mut state = self.state.borrow_mut();
        state.execute(StopSearch);
        state.execute(SwitchMode(EditorMode::Insert));
    }

    /// Feed a key to the editor. Returns true if the buffer actually changed.
    ///
    /// Navigation keys must not mark the buffer dirty — that would show a false
    /// modified marker and cause a pointless identical write on exit — so the
    /// decision is made by comparing content, not by classifying keys.
    pub fn on_key(&self, key: KeyEvent) -> bool {
        let before = self.buffer_hash();

        // Both interventions below are only correct while keys are going into
        // the buffer. In search mode they go into the query instead, and Enter
        // there jumps the cursor to the match — so an indent computed from the
        // pre-jump line lands in the middle of the student's code, and Tab
        // types spaces into it rather than the query.
        let in_insert = self.state.borrow().mode == EditorMode::Insert;

        // Tab would otherwise insert a literal '\t', mixing tabs and the spaces
        // the indent shim adds within one file.
        if in_insert && key.code == KeyCode::Tab {
            let mut state = self.state.borrow_mut();
            for _ in 0..INDENT {
                state.execute(InsertChar(' '));
            }
        } else {
            // Compute the indent *before* the line is split.
            let indent = if in_insert && key.code == KeyCode::Enter {
                self.indent_for_new_line()
            } else {
                0
            };

            self.handler
                .borrow_mut()
                .on_key_event(key, &mut self.state.borrow_mut());

            if indent > 0 {
                let mut state = self.state.borrow_mut();
                for _ in 0..indent {
                    state.execute(InsertChar(' '));
                }
            }
        }

        let changed = self.buffer_hash() != before;
        if changed {
            self.dirty.set(true);
        }
        changed
    }

    /// Insert a bracketed paste verbatim. The indent shim never sees these
    /// newlines, which is what stops pasted code from staircasing.
    pub fn on_paste(&self, text: String) -> bool {
        let before = self.buffer_hash();
        self.handler
            .borrow()
            .on_paste_event(text, &mut self.state.borrow_mut());
        let changed = self.buffer_hash() != before;
        if changed {
            self.dirty.set(true);
        }
        changed
    }

    /// The buffer as it would be written to disk.
    pub fn buffer_text(&self) -> String {
        self.state.borrow().lines.to_string()
    }

    fn buffer_hash(&self) -> u64 {
        hash_str(&self.buffer_text())
    }

    /// How many spaces a newly opened line should start with: the current
    /// line's leading spaces, plus one level if the text before the cursor ends
    /// in an opening brace.
    ///
    /// Deliberately not language-aware beyond that. It exists so nested code
    /// does not start at column zero, not to be rustfmt.
    fn indent_for_new_line(&self) -> usize {
        let state = self.state.borrow();
        let row = state.cursor.row;
        let col = state.cursor.col;
        let text = state.lines.to_string();

        let Some(line) = text.split('\n').nth(row) else {
            return 0;
        };
        let lead = line.chars().take_while(|c| *c == ' ').count();
        let prefix: String = line.chars().take(col).collect();
        let extra = if prefix.trim_end().ends_with('{') {
            INDENT
        } else {
            0
        };
        lead + extra
    }
}

/// edtui's modeless keymap, with search moved off `Ctrl+S`.
///
/// `Ctrl+S` is the game's save key, so search has to go somewhere; `Ctrl+F` for
/// "find" is the more conventional binding anyway, and edtui's `Ctrl+F`
/// (character forward) is covered by the Right arrow.
///
/// Two registrations move, not one. edtui binds `Ctrl+S` in insert mode to
/// *start* a search and again in search mode to *repeat* it — relocating only
/// the first would leave a search you could open but only walk backwards.
fn emacs_handler_with_search_on_ctrl_f() -> EditorEventHandler {
    let mut handler = EditorEventHandler::emacs_mode();
    let k = &mut handler.key_handler;

    k.remove(&KeyEventRegister::i(vec![KeyInput::ctrl('s')]));
    k.remove(&KeyEventRegister::s(vec![KeyInput::ctrl('s')]));

    // The original is a chain: bare `StartSearch` would begin a search without
    // switching mode, so the query would be typed into the buffer instead.
    k.insert(
        KeyEventRegister::i(vec![KeyInput::ctrl('f')]),
        StartSearch.chain(SwitchMode(EditorMode::Search)),
    );
    k.insert(KeyEventRegister::s(vec![KeyInput::ctrl('f')]), FindNext);

    handler
}

fn hash_str(s: &str) -> u64 {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}
