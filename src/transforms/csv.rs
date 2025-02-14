use csv::{ReaderBuilder, StringRecord};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

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

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read CSV file: {}", e))?;
        Self::new(&content)
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

// Not used anymore, but kept for future reference
#[allow(dead_code)]
pub fn csv_transform(text: &str) -> Result<String, Box<dyn Error>> {
    let csv = Csv::new(text)?;
    Ok(csv.to_string())
}

pub fn csv_from_file(path: &str) -> Result<String, Box<dyn Error>> {
    let csv = Csv::from_file(path)?;
    Ok(csv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_csv(dir: &TempDir, content: &str) -> String {
        let file_path = dir.path().join("test.csv");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", content.trim()).unwrap();
        file_path.to_str().unwrap().to_string()
    }

    #[test]
    fn test_csv_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let csv_content = "\
Name,Age,City
John,30,New York
Jane,25,London";

        let file_path = create_test_csv(&temp_dir, csv_content);
        let result = csv_from_file(&file_path).unwrap();

        assert!(result.contains("Name | Age | City"));
        assert!(result.contains("John | 30  | New York"));
        assert!(result.contains("Jane | 25  | London"));
    }

    #[test]
    fn test_csv_from_file_not_found() {
        let result = csv_from_file("nonexistent.csv");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to read CSV file"));
    }

    #[test]
    fn test_csv() {
        let string = "Name,Age\nJohn,30\nJane,25";
        let result = csv_transform(string).unwrap();
        assert!(result.contains("Name | Age"));
        assert!(result.contains("John | 30"));
        assert!(result.contains("Jane | 25"));
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
}
