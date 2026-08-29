use std::path::{Path, PathBuf};
use std::{fs, thread, time::Duration};

use anyhow::{Context, Result};

use super::game_state::GameState;

const STATE_DIR_NAME: &str = "rust-game";
const STATE_FILE_NAME: &str = "state.json";
const BACKUP_FILE_NAME: &str = "state.json.bak";
pub(crate) const CURRENT_VERSION: u32 = 2;

/// Determine the state file path.
///
/// Uses platform-specific user data directory (via `dirs` crate):
/// - Linux: ~/.local/share/rust-game/state.json
/// - macOS: ~/Library/Application Support/rust-game/state.json
/// - Windows: %APPDATA%/rust-game/state.json
///
/// Falls back to .rust-game-state.json in current directory if dirs fails.
pub fn state_file_path() -> PathBuf {
    // An explicit override lets the state file live in a synced directory, so
    // progress follows the player between machines. Names a *file*, not a
    // directory — the `.bak` sits beside it via `with_file_name`.
    if let Ok(path) = std::env::var("RUST_GAME_STATE") {
        if !path.is_empty() {
            return PathBuf::from(path);
        }
    }
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
    load_from(&state_file_path())
}

/// Load game state from a specific path.
///
/// Split out from [`load_state`] so the recovery behavior below can be tested
/// without touching the real state file, which lives at a process-global path.
pub fn load_from(path: &Path) -> Result<GameState> {
    if !path.exists() {
        return Ok(GameState::new());
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read state file: {}", path.display()))?;

    let mut recovered_from_backup = false;
    let state: GameState = match serde_json::from_str(&content) {
        Ok(s) => s,
        Err(parse_err) => {
            // The tool wrote this file; refusing to start over it would strand
            // the player with a backup sitting unread beside it.
            let backup = path.with_file_name(BACKUP_FILE_NAME);
            let recovered = fs::read_to_string(&backup)
                .ok()
                .and_then(|b| serde_json::from_str::<GameState>(&b).ok());

            match recovered {
                Some(state) => {
                    recovered_from_backup = true;
                    eprintln!(
                        "Warning: {} is corrupt ({}).\n  \
                         Recovered from {}{}.\n  \
                         Progress since that backup is lost.",
                        path.display(),
                        parse_err,
                        backup.display(),
                        backup_age(&backup)
                    );
                    state
                }
                None => {
                    return Err(anyhow::Error::new(parse_err).context(format!(
                        "Failed to parse state file: {}, and no usable backup at {}. \
                         Delete the state file to start fresh.",
                        path.display(),
                        backup.display()
                    )));
                }
            }
        }
    };

    // Version check — applied to whichever state we ended up with, including a
    // recovered backup. Every backup written so far predates the v2 migration,
    // so skipping this would silently regress the in-memory version.
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
        // Do not back up when we just recovered *from* the backup: that would
        // copy the corrupt primary over the only good copy we have.
        if !recovered_from_backup {
            backup_state(path)?;
        }
        let migrated = migrate_state(state);
        // Write back to the path we loaded from, not the global one — otherwise
        // loading any v1 fixture from a temp dir rewrites the player's real state.
        save_state_to(&migrated, path)?;
        return Ok(migrated);
    }

    Ok(state)
}

/// Save the game state to disk atomically.
///
/// Writes to a temp file first, then renames (atomic on Unix).
/// On Windows, retries once after 100ms if persist() fails.
pub fn save_state(state: &GameState) -> Result<()> {
    save_state_to(state, &state_file_path())
}

/// Save game state to a specific path. Split out from [`save_state`] for the
/// same reason as [`load_from`] — the default path is process-global.
pub fn save_state_to(state: &GameState, path: &Path) -> Result<()> {
    let path = path.to_path_buf();

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

/// Describe when a backup was written, so the player can judge what was lost.
fn backup_age(backup: &Path) -> String {
    match fs::metadata(backup).and_then(|m| m.modified()) {
        Ok(t) => {
            let dt: chrono::DateTime<chrono::Local> = t.into();
            format!(" (saved {})", dt.format("%Y-%m-%d %H:%M"))
        }
        Err(_) => String::new(),
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
    // Migration chain: apply each step in order.
    // v1 → v2 added `lessons`. Nothing to move: `#[serde(default)]` on the field
    // already supplied an empty map when the v1 file was parsed, so the step is
    // just the version bump. The caller backs the file up before we get here.
    if state.version == 1 {
        state.version = 2;
    }
    // When CURRENT_VERSION becomes 3, add:
    //   if state.version == 2 { /* migrate v2 → v3 fields */; state.version = 3; }

    state.version = CURRENT_VERSION;
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corrupt_state_recovers_from_backup() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join(STATE_FILE_NAME);
        let backup = tmp.path().join(BACKUP_FILE_NAME);

        // A good backup beside a corrupt primary.
        let mut good = GameState::new();
        good.complete_exercise("ex1", 42, None);
        fs::write(&backup, serde_json::to_string_pretty(&good).unwrap()).unwrap();
        fs::write(&path, "{ this is not json at all").unwrap();

        let recovered = load_from(&path).expect("should recover rather than fail");
        assert_eq!(
            recovered.player.xp, 42,
            "recovery must load the backup's progress, not a fresh state"
        );
        assert_eq!(recovered.version, CURRENT_VERSION);
    }

    #[test]
    fn test_recovery_from_a_v1_backup_preserves_the_backup() {
        // Every backup written before the v2 migration is a v1 file, so this is
        // the realistic recovery case. It must migrate without clobbering the
        // backup it just read, and without writing to the global state path.
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join(STATE_FILE_NAME);
        let backup = tmp.path().join(BACKUP_FILE_NAME);

        let v1 = r#"{"version": 1, "player": {"xp": 77, "level": 3, "current_streak": 1,
            "best_streak": 4, "total_time_played_secs": 0, "last_exercise_at": null},
            "exercises": {}, "modules": {}}"#;
        fs::write(&backup, v1).unwrap();
        fs::write(&path, "not json").unwrap();

        let recovered = load_from(&path).expect("should recover and migrate");
        assert_eq!(recovered.player.xp, 77);
        assert_eq!(recovered.version, CURRENT_VERSION);

        // The backup must still be readable — it is the only good copy.
        let after = fs::read_to_string(&backup).unwrap();
        assert!(
            serde_json::from_str::<GameState>(&after).is_ok(),
            "recovery must not overwrite the backup it recovered from"
        );
    }

    #[test]
    fn test_corrupt_state_with_no_backup_still_errors() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join(STATE_FILE_NAME);
        fs::write(&path, "{ not json").unwrap();
        assert!(
            load_from(&path).is_err(),
            "without a usable backup there is nothing to recover"
        );
    }

    #[test]
    fn test_fresh_state_uses_current_version() {
        // Writing v1 while CURRENT_VERSION is 2 made every new player take the
        // migration branch on next launch, which backs up — overwriting the
        // last good backup with the just-created empty state.
        assert_eq!(GameState::new().version, CURRENT_VERSION);
    }

    #[test]
    fn test_v1_state_migrates_to_v2_with_empty_lessons() {
        // A version-1 file predates the `lessons` map entirely.
        let json = r#"{"version": 1, "player": {"xp": 50, "level": 2, "current_streak": 3,
            "best_streak": 3, "total_time_played_secs": 0, "last_exercise_at": null},
            "exercises": {}, "modules": {}}"#;

        let state: GameState = serde_json::from_str(json)
            .expect("serde(default) should let a v1 file parse before migration");
        assert_eq!(state.version, 1);
        assert!(state.lessons.is_empty());

        let migrated = migrate_state(state);
        assert_eq!(migrated.version, CURRENT_VERSION);
        assert_eq!(migrated.version, 2);
        assert!(migrated.lessons.is_empty());
        // Progress must survive the migration untouched.
        assert_eq!(migrated.player.xp, 50);
        assert_eq!(migrated.player.level, 2);
        assert_eq!(migrated.player.current_streak, 3);
    }

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
        assert!(json.contains(&format!("\"version\": {}", CURRENT_VERSION)));
        assert!(json.contains("\"xp\": 0"));
    }
}
