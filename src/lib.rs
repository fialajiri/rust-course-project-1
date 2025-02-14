mod command;
mod input;
mod processor;
mod transforms;

pub use command::{Command, TransformMessage};
pub use input::{display_available_commands, parse_input, process_user_input};
pub use processor::process_messages;
use std::error::Error;

pub fn process_command(command: &Command, text: &str) -> Result<String, Box<dyn Error>> {
    use transforms::*;

    match command {
        Command::Lowercase => lowercase(text),
        Command::Uppercase => uppercase(text),
        Command::NoSpaces => no_spaces(text),
        Command::Slugify => slugify_text(text),
        Command::Reverse => reverse_text(text),
        Command::Alternating => alternating(text),
        Command::CsvFile => csv_from_file(text),
    }
}
