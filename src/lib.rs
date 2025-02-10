use csv::{ReaderBuilder, StringRecord};
use slug::slugify;
use std::error::Error;
use std::fmt;

pub fn run(args: &[String]) -> Result<String, Box<dyn Error>> {
    if args.len() < 3 {
        return Err("Not enough arguments. Usage: <transform_type> <text>".into());
    }

    let transform_type = &args[1];
    let text = &args[2];

    match transform_type.as_str() {
        "lowercase" => lowercase(text),
        "uppercase" => uppercase(text),
        "no-spaces" => no_spaces(text),
        "slugify" => slugify_text(text),
        "reverse" => reverse_text(text),
        "alternating" => alternating(text),
        "csv" => csv_transform(text),
        _ => Err(format!("Invalid transform type: {}", transform_type).into()),
    }
}

pub fn validate_input(text: &str) -> Result<String, Box<dyn Error>> {
    if text.trim().is_empty() {
        return Err("Input text cannot be empty".into());
    }
    Ok(text.to_string())
}

pub fn lowercase(text: &str) -> Result<String, Box<dyn Error>> {
    let validated_text = validate_input(text)?;
    Ok(validated_text.to_lowercase())
}

pub fn uppercase(text: &str) -> Result<String, Box<dyn Error>> {
    let validated_text = validate_input(text)?;
    Ok(validated_text.to_uppercase())
}

pub fn no_spaces(text: &str) -> Result<String, Box<dyn Error>> {
    let validated_text = validate_input(text)?;
    Ok(validated_text.replace(" ", ""))
}

pub fn slugify_text(text: &str) -> Result<String, Box<dyn Error>> {
    let validated_text = validate_input(text)?;
    Ok(slugify(validated_text))
}

pub fn reverse_text(text: &str) -> Result<String, Box<dyn Error>> {
    let validated_text = validate_input(text)?;
    Ok(validated_text.chars().rev().collect())
}

pub fn alternating(text: &str) -> Result<String, Box<dyn Error>> {
    let validated_text = validate_input(text)?;
    Ok(validated_text
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect())
}

pub fn csv_transform(text: &str) -> Result<String, Box<dyn Error>> {
    let validated_text = validate_input(text)?;
    let csv = Csv::new(&validated_text)?;
    Ok(csv.to_string())
}

#[derive(Debug)]
pub struct Csv {
    headers: StringRecord,
    records: Vec<StringRecord>,
}

impl Csv {
    pub fn new(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new()
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(input.as_bytes());

        let headers = reader.headers()?.clone();
        let records: Vec<StringRecord> = reader.records().filter_map(|r| r.ok()).collect();

        if records.is_empty() {
            return Err("No valid data rows found in CSV".into());
        }

        Ok(Csv { headers, records })
    }

    fn get_column_widths(&self) -> Vec<usize> {
        let mut widths = vec![0; self.headers.len()];

        // Check headers
        for (i, header) in self.headers.iter().enumerate() {
            widths[i] = widths[i].max(header.len());
        }

        // Check all records
        for record in &self.records {
            for (i, field) in record.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(field.trim().len());
                }
            }
        }

        widths
    }
}

impl fmt::Display for Csv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let widths = self.get_column_widths();
        let num_columns = self.headers.len();

        // Print headers
        for (i, header) in self.headers.iter().enumerate() {
            if i > 0 {
                write!(f, " | ")?;
            }
            write!(f, "{:width$}", header.trim(), width = widths[i])?;
        }
        writeln!(f)?;

        // Print separator
        for (i, width) in widths.iter().enumerate() {
            if i > 0 {
                write!(f, "-+-")?;
            }
            write!(f, "{}", "-".repeat(*width))?;
        }
        writeln!(f)?;

        // Print records
        for record in &self.records {
            for i in 0..num_columns {
                if i > 0 {
                    write!(f, " | ")?;
                }
                let field = record.get(i).map_or("", |f| f.trim());
                write!(f, "{:width$}", field, width = widths[i])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input_empty() {
        assert!(validate_input("").is_err());
    }

    #[test]
    fn test_validate_input_single_character() {
        assert!(validate_input("a").is_ok());
    }

    #[test]
    fn test_run_lowercase() {
        let string = "HELLO";
        assert_eq!(lowercase(&string).unwrap(), "hello");
    }

    #[test]
    fn test_run_uppercase() {
        let string = "hello";
        assert_eq!(uppercase(&string).unwrap(), "HELLO");
    }

    #[test]
    fn test_run_no_spaces() {
        let string = "hello world";
        assert_eq!(no_spaces(&string).unwrap(), "helloworld");
    }

    #[test]
    fn test_run_slugify() {
        let string = "Hello World!";
        assert_eq!(slugify_text(&string).unwrap(), "hello-world");
    }

    #[test]
    fn test_run_reverse() {
        let string = "hello";
        assert_eq!(reverse_text(&string).unwrap(), "olleh");
    }

    #[test]
    fn test_run_alternating() {
        let string = "hello";
        assert_eq!(alternating(&string).unwrap(), "HeLlO");
    }

    #[test]
    fn test_run_csv() {
        let string = "Name,Age\nJohn,30\nJane,25";
        assert!(csv_transform(&string).unwrap().contains("Name | Age"));
        assert!(csv_transform(&string).unwrap().contains("John | 30"));
        assert!(csv_transform(&string).unwrap().contains("Jane | 25"));
    }

    #[test]
    fn test_run_invalid_transform() {
        let args = vec![
            "program".to_string(),
            "invalid".to_string(),
            "hello".to_string(),
        ];
        assert!(run(&args).is_err());
    }

    #[test]
    fn test_run_not_enough_args() {
        let args = vec!["program".to_string(), "lowercase".to_string()];
        assert!(run(&args).is_err());
    }

    #[test]
    fn test_csv_with_invalid_input() {
        let args = vec![
            "program".to_string(),
            "csv".to_string(),
            "invalid,csv,format\nwith,too,many,columns".to_string(),
        ];

        assert!(run(&args).is_ok());
    }

    #[test]
    fn test_csv_with_varying_columns() {
        let csv_data = "\
Name,Age,City,Extra
John Doe,30,New York,Something
Jane Smith,25,Los Angeles
Bob,45,Chicago,Extra,More";

        let result = csv_transform(csv_data).unwrap();
        assert!(result.contains("John Doe"));
        assert!(result.contains("Jane Smith"));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_csv_with_whitespace() {
        let csv_data = "Name  ,  Age  ,  City  \n  John Doe  ,  30  ,  New York  ";
        let result = csv_transform(csv_data).unwrap();
        assert!(result.contains("John Doe"));
        assert!(result.contains("30"));
        assert!(result.contains("New York"));
    }

    #[test]
    fn test_csv_extra_columns() {
        let csv_data = "Name,Age\n\
            John,30,extra,columns\n\
            Jane,25,more,stuff\n";
        let result = csv_transform(csv_data).unwrap();
        assert!(!result.contains("|  |")); // Should not have extra pipes
        assert!(!result.contains("extra")); // Should not show extra columns
        assert!(!result.contains("more")); // Should not show extra columns
    }
}
