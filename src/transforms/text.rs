use std::error::Error;

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
    Ok(slug::slugify(validated_text))
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
    fn test_lowercase() {
        let string = "HELLO";
        assert_eq!(lowercase(string).unwrap(), "hello");
    }

    #[test]
    fn test_uppercase() {
        let string = "hello";
        assert_eq!(uppercase(string).unwrap(), "HELLO");
    }

    #[test]
    fn test_no_spaces() {
        let string = "hello world";
        assert_eq!(no_spaces(string).unwrap(), "helloworld");
    }

    #[test]
    fn test_slugify() {
        let string = "Hello World!";
        assert_eq!(slugify_text(string).unwrap(), "hello-world");
    }

    #[test]
    fn test_reverse() {
        let string = "hello";
        assert_eq!(reverse_text(string).unwrap(), "olleh");
    }

    #[test]
    fn test_alternating() {
        let string = "hello";
        assert_eq!(alternating(string).unwrap(), "HeLlO");
    }
}
