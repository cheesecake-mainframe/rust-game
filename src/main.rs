mod cli;

use clap::Parser;

use rust_game::app::App;
use rust_game::runner;
use rust_game::tui;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::CleanCache) => {
            // CleanCache doesn't need the full app (no catalog/state)
            runner::sandbox::clean_all_sandboxes(&std::path::PathBuf::from(".rust-game-cache"))
                .map(|_| println!("Sandbox cache cleaned."))
        }
        _ => run_with_app(cli.command),
    };

    if let Err(e) = result {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn run_with_app(command: Option<Commands>) -> anyhow::Result<()> {
    let mut app = App::init()?;

    match command {
        None | Some(Commands::Tui) => {
            tui::ui::run(app)
        }
        Some(Commands::List { module }) => {
            app.cmd_list(module.as_deref());
            Ok(())
        }
        Some(Commands::Verify { exercise }) => {
            app.cmd_verify(&exercise)
        }
        Some(Commands::Watch { exercise }) => {
            // CLI watch: verify then print result. TUI watch mode is in the TUI.
            if let Some(id) = exercise {
                app.cmd_verify(&id)
            } else {
                app.cmd_next()
            }
        }
        Some(Commands::Hint { exercise }) => {
            app.cmd_hint(&exercise)
        }
        Some(Commands::Stats) => {
            app.cmd_stats();
            Ok(())
        }
        Some(Commands::Next) => {
            app.cmd_next()
        }
        Some(Commands::Reset { exercise, all }) => {
            app.cmd_reset(exercise.as_deref(), all)
        }
        Some(Commands::HintAi { exercise }) => {
            app.cmd_hint_ai(&exercise)
        }
        Some(Commands::Lesson { module, mark_read }) => {
            app.cmd_lesson(&module, mark_read)
        }
        Some(Commands::Solution { exercise }) => {
            app.cmd_solution(&exercise)
        }
        Some(Commands::CleanCache) => {
            unreachable!("Handled before App::init()")
        }
    }
}
