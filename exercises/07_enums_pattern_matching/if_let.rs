// Exercise: if let and let else
// Type: fix_compiler_error
// Difficulty: intermediate
//
// When you only care about ONE variant of an enum, a full `match`
// is verbose. Rust provides shorthand:
//
//   if let Some(x) = value { ... }      — run block only if it matches
//   let Some(x) = value else { ... };   — bind x or run the else block
//
// Fix the code below to use `if let` or `let else` where appropriate.
// The compiler errors indicate where the current code is wrong.

fn process_config(setting: Option<String>) {
    // TODO: Fix — this match is overly verbose and has a syntax error.
    // Use `if let` instead.
    match setting {
        Some(val) => {
            println!("Config value: {}", val);
        }
        // Error: match arms must return the same type
        None => return ()
    };
}

fn get_username(id: u32) -> Option<String> {
    match id {
        1 => Some("alice".to_string()),
        2 => Some("bob".to_string()),
        _ => None,
    }
}

fn greet_user(id: u32) {
    // TODO: Fix — this code tries to use `let else` but the syntax is wrong.
    // `let else` requires an irrefutable diverging block (must return/break/panic).
    let name = get_username(id) else {
        println!("User not found");
        // Error: the else block must diverge (return, break, panic, etc.)
    };

    println!("Hello, {}!", name);
}

enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    Color(u8, u8, u8),
}

fn handle_command(cmd: Command) {
    // TODO: Fix — only the Echo command needs special handling here,
    // but this match is missing arms and won't compile.
    match cmd {
        Command::Echo(msg) => println!("Echo: {}", msg),
        Command::Move { x, y } => println!("Move to ({}, {})", x, y),
    }
}

fn main() {
    process_config(Some("dark_mode=true".to_string()));
    process_config(None);

    greet_user(1);
    greet_user(99);

    handle_command(Command::Echo("hello".to_string()));
    handle_command(Command::Quit);
    handle_command(Command::Color(255, 0, 0));
}
