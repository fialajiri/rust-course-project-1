use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Command {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    Reverse,
    Alternating,
    CsvFile,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lowercase" => Ok(Command::Lowercase),
            "uppercase" => Ok(Command::Uppercase),
            "no-spaces" => Ok(Command::NoSpaces),
            "slugify" => Ok(Command::Slugify),
            "reverse" => Ok(Command::Reverse),
            "alternating" => Ok(Command::Alternating),
            "csv-file" => Ok(Command::CsvFile),
            _ => Err(format!("Invalid transform type: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct TransformMessage {
    pub command: Command,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_from_str() {       
        assert_eq!(Command::from_str("lowercase").unwrap(), Command::Lowercase);
        assert_eq!(Command::from_str("UPPERCASE").unwrap(), Command::Uppercase);
        assert_eq!(Command::from_str("no-spaces").unwrap(), Command::NoSpaces);
        assert_eq!(Command::from_str("slugify").unwrap(), Command::Slugify);
        assert_eq!(Command::from_str("reverse").unwrap(), Command::Reverse);
        assert_eq!(
            Command::from_str("alternating").unwrap(),
            Command::Alternating
        );
        assert_eq!(Command::from_str("csv-file").unwrap(), Command::CsvFile);
        
        assert!(Command::from_str("invalid").is_err());
    }
}
