use std::time::{Duration, Instant};

use crate::app::App;
use crate::exercise::types::{Exercise, ExerciseStatus};
use crate::game::xp::{self, XpAward};
use crate::state::game_state::ModuleState;

/// Everything a caller needs to render after an exercise is completed.
///
/// Callers render *from this value* rather than computing an award of their
/// own. That is the whole point: `complete_exercise` is the authority on
/// whether XP is granted, and before this type existed four separate call sites
/// each formatted a "+N XP" message that the state layer might then decline —
/// so pressing verify twice on a solved exercise reported the award twice while
/// `player.xp` moved once.
#[derive(Debug, Clone)]
pub struct Outcome {
    /// False when the exercise was already completed. `award` is zeroed.
    pub is_new: bool,
    pub award: XpAward,
    pub level_up: Option<(u32, u32)>,
    /// `(module name, theme name)` when this completion finished the module.
    pub module_completed: Option<(String, String)>,
    pub elapsed: Option<Duration>,
    /// Set when the award could not be written to disk. The XP is real in
    /// memory but will not survive a restart, and the student needs telling —
    /// silently reporting "+15 XP" over a failed save is its own small lie.
    pub save_error: Option<String>,
}

/// The award nobody earned. `streak_multiplier` is 1.0 rather than 0.0 because
/// callers render it as `{:.2}x`, and `0.00x` reads as a bug.
fn no_award() -> XpAward {
    XpAward {
        base: 0,
        first_try_bonus: 0,
        time_trial_bonus: 0,
        streak_multiplier: 1.0,
        total: 0,
    }
}

/// Record a successful completion and return what actually happened.
///
/// The single place the verify → record → award → streak → level-up → module
/// completion → persist sequence lives.
///
/// `started_at` is when the student began working, not when verification
/// started — it drives the time-trial bonus. Only watch mode has a meaningful
/// value; the CLI and one-shot paths pass `None`.
///
/// Takes `&mut App` and an owned-or-borrowed `&Exercise` that must **not** be
/// borrowed out of `app.catalog` — every caller already holds a clone. Passing
/// a catalog reference alongside the mutable borrow is an E0502 conflict, so
/// keep those clones even if they look redundant.
pub fn award_completion(
    app: &mut App,
    exercise: &Exercise,
    started_at: Option<Instant>,
) -> Outcome {
    let exercise_id = exercise.id.as_str();
    let elapsed = started_at.map(|s| s.elapsed());

    app.state.record_attempt(exercise_id);
    let attempts = app
        .state
        .exercises
        .get(exercise_id)
        .map(|e| e.attempts)
        .unwrap_or(1);

    // The time-trial bonus needs the time the student spent, which only exists
    // when the caller tracked a start. Passing verification duration here — as
    // the CLI used to — always beats the limit and grants the bonus for free.
    let time_taken = match (exercise.time_limit_secs, elapsed) {
        (Some(_), Some(d)) => Some(d),
        _ => None,
    };

    let old_level = app.state.player.level;
    let award = xp::calculate_xp(exercise, attempts, time_taken, app.streak.current);
    let is_new = app
        .state
        .complete_exercise(exercise_id, award.total, elapsed.map(|d| d.as_secs()));

    if !is_new {
        let save_error = app.save().err().map(|e| format!("{:#}", e));
        return Outcome {
            is_new: false,
            award: no_award(),
            level_up: None,
            module_completed: None,
            elapsed,
            save_error,
        };
    }

    app.streak.increment();

    if let Some(d) = elapsed {
        app.state.player.total_time_played_secs += d.as_secs();
    }

    let new_level = app.state.player.level;
    let level_up = (new_level > old_level).then_some((old_level, new_level));
    let module_completed = mark_module_if_complete(app, &exercise.module_id);

    let save_error = app.save().err().map(|e| format!("{:#}", e));

    Outcome {
        is_new: true,
        award,
        level_up,
        module_completed,
        elapsed,
        save_error,
    }
}

/// Mark a module completed if this was its last exercise, returning its name
/// and theme the first time only, so the celebration fires once.
fn mark_module_if_complete(app: &mut App, module_id: &str) -> Option<(String, String)> {
    let exercise_ids: Vec<String> = app
        .catalog
        .exercises_for_module(module_id)
        .iter()
        .map(|e| e.id.clone())
        .collect();

    if exercise_ids.is_empty() {
        return None;
    }
    if !exercise_ids
        .iter()
        .all(|id| app.exercise_status(id) == ExerciseStatus::Completed)
    {
        return None;
    }
    if app
        .state
        .modules
        .get(module_id)
        .map(|m| m.completed)
        .unwrap_or(false)
    {
        return None; // already celebrated
    }

    let names = app
        .catalog
        .get_module(module_id)
        .map(|m| (m.name.clone(), m.theme_name.clone()));

    let entry = app
        .state
        .modules
        .entry(module_id.to_string())
        .or_insert_with(|| ModuleState {
            unlocked: true,
            completed: false,
            unlocked_at: None,
            completed_at: None,
        });
    entry.completed = true;
    entry.completed_at = Some(chrono::Utc::now());

    names
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_exercise(app: &App) -> Exercise {
        app.catalog.exercises()[0].clone()
    }

    #[test]
    fn second_award_is_declined_and_zeroed() {
        let mut app = App::new_for_testing().unwrap();
        let ex = first_exercise(&app);

        let first = award_completion(&mut app, &ex, None);
        assert!(first.is_new);
        assert!(first.award.total > 0);
        let xp_after_first = app.state.player.xp;

        let second = award_completion(&mut app, &ex, None);
        assert!(!second.is_new, "already-completed exercise must not re-award");
        assert_eq!(
            second.award.total, 0,
            "a declined award must be zeroed so callers cannot render it"
        );
        assert_eq!(
            app.state.player.xp, xp_after_first,
            "XP must not move on the second award"
        );
    }

    #[test]
    fn a_successful_award_reports_no_save_error() {
        let mut app = App::new_for_testing().unwrap();
        let ex = first_exercise(&app);
        let outcome = award_completion(&mut app, &ex, None);
        assert!(
            outcome.save_error.is_none(),
            "a normal award must not report a persistence failure"
        );
    }

    #[test]
    fn first_try_earns_the_bonus() {
        let mut app = App::new_for_testing().unwrap();
        let ex = first_exercise(&app);
        let outcome = award_completion(&mut app, &ex, None);
        assert!(outcome.is_new);
        assert!(
            outcome.award.first_try_bonus > 0,
            "completing on attempt 1 should earn the first-try bonus"
        );
    }

    #[test]
    fn time_trial_bonus_requires_a_start_time() {
        let mut app = App::new_for_testing().unwrap();
        let timed = app
            .catalog
            .exercises()
            .iter()
            .find(|e| e.time_limit_secs.is_some())
            .expect("at least one time-trial exercise exists")
            .clone();

        // No start time -> no bonus, rather than a bonus granted by accident.
        let outcome = award_completion(&mut app, &timed, None);
        assert_eq!(outcome.award.time_trial_bonus, 0);

        // A start time inside the limit -> bonus.
        let mut app2 = App::new_for_testing().unwrap();
        let outcome2 = award_completion(&mut app2, &timed, Some(Instant::now()));
        assert!(outcome2.award.time_trial_bonus > 0);
    }

    #[test]
    fn testing_app_never_writes_state() {
        // Guards the reason `persist` exists: these tests call save() through
        // award_completion, and save_state resolves a process-global path.
        let mut app = App::new_for_testing().unwrap();
        let ex = first_exercise(&app);
        let before = std::fs::read(crate::state::persistence::state_file_path()).ok();
        award_completion(&mut app, &ex, None);
        let after = std::fs::read(crate::state::persistence::state_file_path()).ok();
        assert_eq!(before, after, "new_for_testing must not touch the real state file");
    }
}
