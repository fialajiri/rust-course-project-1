use crate::TransformMessage;
use std::io::{self, Write};

pub fn process_messages(rx: flume::Receiver<TransformMessage>) {
    for message in rx.iter() {
        match crate::process_command(&message.command, &message.text) {
            Ok(result) => {
                println!("{}", result);
                if io::stdout().flush().is_err() {
                    eprintln!("Failed to flush stdout");
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                if io::stderr().flush().is_err() {
                    eprintln!("Failed to flush stderr");
                }
            }
        }
    }
}
