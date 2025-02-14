use project_1::{display_available_commands, parse_input, process_messages, process_user_input};
use std::{env, thread};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let (tx, rx) = flume::unbounded();

    if args.is_empty() {
        display_available_commands();
        let input_thread = thread::spawn(move || process_user_input(tx));
        process_messages(rx);
        input_thread.join().unwrap();
        return;
    }

    // CLI mode
    let command_line = args.join(" ");
    if let Ok(message) = parse_input(&command_line) {
        tx.send(message).unwrap();
        drop(tx);
        process_messages(rx);
    } else {
        eprintln!("Error: Invalid command format");
        std::process::exit(1);
    }
}
