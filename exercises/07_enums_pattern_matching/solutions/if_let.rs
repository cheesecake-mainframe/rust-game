fn process_config(setting: Option<String>) {
    if let Some(val) = setting {
        println!("Config value: {}", val);
    }
}

fn get_username(id: u32) -> Option<String> {
    match id {
        1 => Some("alice".to_string()),
        2 => Some("bob".to_string()),
        _ => None,
    }
}

fn greet_user(id: u32) {
    let Some(name) = get_username(id) else {
        println!("User not found");
        return;
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
    if let Command::Echo(msg) = cmd {
        println!("Echo: {}", msg);
    } else if let Command::Move { x, y } = cmd {
        println!("Move to ({}, {})", x, y);
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
