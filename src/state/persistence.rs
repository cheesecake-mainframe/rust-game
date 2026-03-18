use std::path::{Path, PathBuf};
use std::{fs, thread, time::Duration};

use anyhow::{Context, Result};

use super::game_state::GameState;

const STATE_DIR_NAME: &str = "rust-game";
const STATE_FILE_NAME: &str = "state.json";
const BACKUP_FILE_NAME: &str = "state.json.bak";
const CURRENT_VERSION: u32 = 1;

/// Determine the state file path.
///
/// Uses platform-specific user data directory (via `dirs` crate):
/// - Linux: ~/.local/share/rust-game/state.json
/// - macOS: ~/Library/Application Support/rust-game/state.json
/// - Windows: %APPDATA%/rust-game/state.json
///
/// Falls back to .rust-game-state.json in current directory if dirs fails.
pub fn state_file_path() -> PathBuf {
    if let Some(data_dir) = dirs::data_dir() {
        data_dir.join(STATE_DIR_NAME).join(STATE_FILE_NAME)
    } else {
        eprintln!(
            "Warning: Could not determine user data directory; \
             storing state in current directory."
        );
        PathBuf::from(".rust-game-state.json")
    }
}

/// Load the game state from disk.
///
/// If the file doesn't exist, returns a fresh default state.
/// If the file exists but has a newer version, returns an error.
/// If the file has an older version, auto-migrates and backs up the old file.
pub fn load_state() -> Result<GameState> {
    let path = state_file_path();

    if !path.exists() {
        return Ok(GameState::new());
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read state file: {}", path.display()))?;

    let state: GameState = serde_json::from_str(&content).with_context(|| {
        format!(
            "Failed to parse state file: {}. \
             If the file is corrupted, delete it to start fresh.",
            path.display()
        )
    })?;

    // Version check
    if state.version > CURRENT_VERSION {
        anyhow::bail!(
            "State file version {} is newer than this version of rust-game (expects version {}). \
             Please update rust-game.",
            state.version,
            CURRENT_VERSION
        );
    }

    // Migration: auto-upgrade old state files with backup
    if state.version < CURRENT_VERSION {
        backup_state(&path)?;
        let migrated = migrate_state(state);
        save_state(&migrated)?;
        return Ok(migrated);
    }

    Ok(state)
}

/// Save the game state to disk atomically.
///
/// Writes to a temp file first, then renames (atomic on Unix).
/// On Windows, retries once after 100ms if persist() fails.
pub fn save_state(state: &GameState) -> Result<()> {
    let path = state_file_path();

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create state directory: {}", parent.display()))?;
    }

    let json = serde_json::to_string_pretty(state)
        .context("Failed to serialize game state")?;

    // Atomic write: write to temp file, then rename
    let dir = path.parent().unwrap_or(Path::new("."));
    let mut tmp = tempfile::NamedTempFile::new_in(dir)
        .context("Failed to create temporary state file")?;

    use std::io::Write;
    tmp.write_all(json.as_bytes())
        .context("Failed to write temporary state file")?;
    tmp.flush().context("Failed to flush temporary state file")?;

    // persist() does an atomic rename
    match tmp.persist(&path) {
        Ok(_) => Ok(()),
        Err(e) => {
            // On Windows, persist can fail if the file is open.
            // Retry once after 100ms.
            eprintln!(
                "Warning: atomic save failed ({}), retrying...",
                e.error
            );
            thread::sleep(Duration::from_millis(100));
            let mut tmp2 = tempfile::NamedTempFile::new_in(dir)
                .context("Failed to create retry temp file")?;
            tmp2.write_all(json.as_bytes())?;
            tmp2.flush()?;
            tmp2.persist(&path)
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Failed to save state after retry: {}. \
                         Progress from this session may be lost.",
                        e.error
                    )
                })?;
            Ok(())
        }
    }
}

/// Back up the current state file before migration.
fn backup_state(path: &Path) -> Result<()> {
    let backup = path.with_file_name(BACKUP_FILE_NAME);
    fs::copy(path, &backup)
        .with_context(|| format!("Failed to back up state file to {}", backup.display()))?;
    Ok(())
}

/// Migrate a state file from an older version to the current version.
///
/// Each version bump adds a migration step. Migrations are applied sequentially
/// so a version 1 state file will pass through every intermediate migration
/// to reach the current version.
fn migrate_state(mut state: GameState) -> GameState {
    // Migration chain: apply each step in order
    // Currently version 1 is the only version, so this is a no-op placeholder.
    // When CURRENT_VERSION becomes 2, add:
    //   if state.version == 1 { /* migrate v1 → v2 fields */; state.version = 2; }
    // When CURRENT_VERSION becomes 3, add:
    //   if state.version == 2 { /* migrate v2 → v3 fields */; state.version = 3; }

    state.version = CURRENT_VERSION;
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_roundtrip() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join(STATE_DIR_NAME).join(STATE_FILE_NAME);

        let mut state = GameState::new();
        state.complete_exercise("ex1", 25, Some(45));
        state.player.current_streak = 3;
        state.player.best_streak = 5;

        // Save to the temp path
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let json = serde_json::to_string_pretty(&state).unwrap();
        fs::write(&path, &json).unwrap();

        // Load it back
        let content = fs::read_to_string(&path).unwrap();
        let loaded: GameState = serde_json::from_str(&content).unwrap();

        assert_eq!(loaded.player.xp, 25);
        assert_eq!(loaded.player.current_streak, 3);
        assert_eq!(loaded.player.best_streak, 5);
        assert_eq!(loaded.exercises_completed(), 1);
    }

    #[test]
    fn test_corrupt_state_file() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("state.json");
        fs::write(&path, "not json at all {{{").unwrap();

        let content = fs::read_to_string(&path).unwrap();
        let result: std::result::Result<GameState, _> = serde_json::from_str(&content);
        assert!(result.is_err());
    }

    #[test]
    fn test_version_too_new() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("state.json");
        let json = r#"{"version": 999, "player": {"xp": 0, "level": 1, "current_streak": 0, "best_streak": 0, "total_time_played_secs": 0, "last_exercise_at": null}, "exercises": {}, "modules": {}}"#;
        fs::write(&path, json).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        let state: GameState = serde_json::from_str(&content).unwrap();
        assert_eq!(state.version, 999);
        // The load_state() function would reject this, but here we test parsing
    }

    #[test]
    fn test_migrate_state_updates_version() {
        let mut state = GameState::new();
        state.version = 0; // pretend it's an older version
        state.player.xp = 100;
        let migrated = migrate_state(state);
        assert_eq!(migrated.version, CURRENT_VERSION);
        assert_eq!(migrated.player.xp, 100); // data preserved
    }

    #[test]
    fn test_backup_state_creates_bak_file() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("state.json");
        fs::write(&path, r#"{"version":1}"#).unwrap();

        backup_state(&path).unwrap();

        let backup = tmp.path().join(BACKUP_FILE_NAME);
        assert!(backup.exists());
        assert_eq!(fs::read_to_string(&backup).unwrap(), r#"{"version":1}"#);
    }

    #[test]
    fn test_fresh_state_serializes() {
        let state = GameState::new();
        let json = serde_json::to_string_pretty(&state).unwrap();
        assert!(json.contains("\"version\": 1"));
        assert!(json.contains("\"xp\": 0"));
    }
}
