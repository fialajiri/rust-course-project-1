use crate::command::{Command, TransformMessage};
use std::io::{self, BufRead, Write};
use std::str::FromStr;

pub fn display_available_commands() {
    println!("Available commands:");
    println!("  lowercase   - Convert text to lowercase");
    println!("  uppercase   - Convert text to uppercase");
    println!("  no-spaces   - Remove all spaces from text");
    println!("  slugify     - Convert text to URL-friendly format");
    println!("  reverse     - Reverse the text");
    println!("  alternating - Alternate between upper and lowercase");    
    println!("  csv-file    - Read and format CSV from file");
    println!("\nUsage:");
    println!("  <command> <text>");
    println!("  csv-file <path/to/file.csv>");
    println!("\nType 'quit' to exit\n");
}

pub fn process_user_input(tx: flume::Sender<TransformMessage>) {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut input = String::new();

    loop {
        input.clear();
        print!("> ");
        if io::stdout().flush().is_err() {
            eprintln!("Failed to flush stdout");
            break;
        }

        match stdin_lock.read_line(&mut input) {
            Ok(0) => {
                println!("\nGoodbye!");
                break;
            }
            Ok(_) => {
                let input = input.trim();
                if input == "quit" {
                    println!("Goodbye!");
                    break;
                }
                handle_input_line(input, &tx);
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

pub fn parse_input(input: &str) -> Result<TransformMessage, String> {
    let mut parts = input.splitn(2, ' ');
    match (parts.next(), parts.next()) {
        (Some(cmd), Some(text)) => {
            let command = Command::from_str(cmd)?;
            Ok(TransformMessage {
                command,
                text: text.to_string(),
            })
        }
        _ => Err("Invalid input format. Use: <command> <text>".to_string()),
    }
}

fn handle_input_line(input: &str, tx: &flume::Sender<TransformMessage>) {
    match parse_input(input) {
        Ok(message) => {
            if tx.send(message).is_err() {
                eprintln!("Channel closed unexpectedly");
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_valid() {
        let input = "uppercase hello";
        let result = parse_input(input).unwrap();
        assert!(matches!(result.command, Command::Uppercase));
        assert_eq!(result.text, "hello");
    }

    #[test]
    fn test_parse_input_invalid_format() {
        let input = "uppercase";
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn test_parse_input_invalid_command() {
        let input = "invalid hello";
        assert!(parse_input(input).is_err());
    }
}
