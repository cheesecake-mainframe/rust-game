use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rust-game")]
#[command(about = "A gamified Rust learning CLI — exercises, XP, time trials, boss battles")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Launch the TUI dashboard
    Tui,

    /// Watch a specific exercise for changes and auto-verify
    Watch {
        /// Exercise ID (e.g., "01_getting_started/hello_world")
        exercise: Option<String>,
    },

    /// Verify a specific exercise right now (no watch)
    Verify {
        /// Exercise ID
        exercise: String,
    },

    /// List all exercises with their status
    List {
        /// Filter by module
        #[arg(short, long)]
        module: Option<String>,
    },

    /// Show hints for an exercise
    Hint {
        /// Exercise ID
        exercise: String,
    },

    /// Show current progress and stats
    Stats,

    /// Jump to the next available exercise
    Next,

    /// Reset progress
    Reset {
        /// Reset a specific exercise
        #[arg(short, long)]
        exercise: Option<String>,
        /// Reset everything
        #[arg(long)]
        all: bool,
    },

    /// Format exercise context for pasting into an AI tutor
    HintAi {
        /// Exercise ID
        exercise: String,
    },

    /// Print a module's lesson
    Lesson {
        /// Module ID (e.g. "04_ownership_moves"), or any exercise ID within it
        module: String,
        /// Also mark the lesson as read. AI tutors run this after teaching.
        #[arg(long)]
        mark_read: bool,
    },

    /// Show the reference solution for an exercise
    Solution {
        /// Exercise ID
        exercise: String,
    },

    /// Wipe sandbox compilation caches
    CleanCache,
}
