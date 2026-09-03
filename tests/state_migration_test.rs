//! State schema migration.
//!
//! The assertion that matters here is not the version number — it is that a
//! migration never loses progress. A schema bump that silently drops a player's
//! completed exercises would be indistinguishable from a corrupt file.
//!
//! Uses `persistence::load_from` rather than the `RUST_GAME_STATE` env var: the
//! variable is process-global and integration tests run in parallel threads.

use rust_game::state::persistence;

/// A hand-built v2 file: one completed exercise, one module, one read lesson.
const V2_STATE: &str = r#"{
  "version": 2,
  "player": {
    "xp": 355,
    "level": 5,
    "current_streak": 12,
    "best_streak": 12,
    "total_time_played_secs": 900,
    "last_exercise_at": "2026-09-02T04:45:08.435864688Z"
  },
  "exercises": {
    "01_getting_started/hello_world": {
      "status": "completed",
      "xp_earned": 15,
      "attempts": 2,
      "completed_at": "2026-09-02T04:45:08.435863155Z",
      "time_taken_secs": 42,
      "first_attempted_at": "2026-09-02T04:40:00.000000000Z"
    }
  },
  "modules": {
    "01_getting_started": {
      "unlocked": true,
      "completed": true,
      "unlocked_at": "2026-09-01T00:00:00.000000000Z",
      "completed_at": "2026-09-02T04:45:08.435863155Z"
    }
  },
  "lessons": {
    "01_getting_started": { "read": true, "read_at": "2026-09-02T04:43:43.222693745Z" }
  }
}"#;

#[test]
fn a_v2_file_migrates_to_v3_without_losing_anything() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.json");
    std::fs::write(&path, V2_STATE).unwrap();

    let state = persistence::load_from(&path).expect("a v2 file must load");

    assert_eq!(state.version, 3, "the file should have been migrated");

    // Everything that was there before must still be there.
    assert_eq!(state.player.xp, 355);
    assert_eq!(state.player.level, 5);
    assert_eq!(state.player.best_streak, 12);
    assert_eq!(state.exercises.len(), 1, "the completed exercise must survive");
    let ex = &state.exercises["01_getting_started/hello_world"];
    assert_eq!(ex.xp_earned, 15);
    assert_eq!(ex.attempts, 2);
    assert_eq!(state.modules.len(), 1, "the module record must survive");
    assert!(state.modules["01_getting_started"].completed);
    assert!(
        state.is_lesson_read("01_getting_started"),
        "the lesson-read flag must survive"
    );

    // And the new fields must arrive with sane defaults.
    assert!(!ex.solution_viewed, "a migrated exercise has not seen its solution");
    assert!(
        state.preferences.editor_layout.is_none(),
        "no layout preference until the student picks one"
    );
}

#[test]
fn the_migration_is_written_back_to_the_file_it_came_from() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.json");
    std::fs::write(&path, V2_STATE).unwrap();

    persistence::load_from(&path).unwrap();

    let on_disk = std::fs::read_to_string(&path).unwrap();
    assert!(on_disk.contains("\"version\": 3"), "got {}", &on_disk[..60.min(on_disk.len())]);

    // The pre-migration copy is kept, which is the only recovery path a bad
    // migration has.
    assert!(
        path.with_extension("json.bak").exists(),
        "a backup must be written before migrating"
    );
}

#[test]
fn a_v3_file_loads_unchanged() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.json");
    let v3 = V2_STATE.replace("\"version\": 2", "\"version\": 3");
    std::fs::write(&path, &v3).unwrap();

    let state = persistence::load_from(&path).unwrap();
    assert_eq!(state.version, 3);
    assert_eq!(state.exercises.len(), 1);
    assert!(
        !path.with_extension("json.bak").exists(),
        "an up-to-date file must not be backed up"
    );
}
