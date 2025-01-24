use slug::slugify;

pub fn run(args: TextTransformArgs) -> Result<String, String> {
    match args.transform_type.as_str() {
        "lowercase" => Ok(args.text.to_lowercase()),
        "uppercase" => Ok(args.text.to_uppercase()),
        "no-spaces" => Ok(args.text.replace(" ", "")),
        "slugify" => Ok(slugify(args.text)),
        "reverse" => Ok(args.text.chars().rev().collect()),
        "alternating" => Ok(alternate_case(&args.text)),
        _ => Err(format!("Invalid transform type: {}", args.transform_type)),
    }
}

pub fn alternate_case(text: &str) -> String {
    text.chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect()
}

#[derive(Debug, PartialEq)]
pub struct TextTransformArgs {
    pub transform_type: String,
    pub text: String,
}

impl TextTransformArgs {
    pub fn new(args: &[String]) -> Result<TextTransformArgs, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let transform_type = args[1].clone();
        let text = args[2].clone();

        Ok(TextTransformArgs {
            transform_type,
            text,
        })
    }
}

#[test]
fn test_run_lowercase() {
    let text_transform_args = TextTransformArgs {
        transform_type: "lowercase".to_string(),
        text: "HELLO".to_string(),
    };
    assert_eq!(run(text_transform_args).unwrap(), "hello");
}

#[test]
fn test_run_uppercase() {
    let text_transform_args = TextTransformArgs {
        transform_type: "uppercase".to_string(),
        text: "hello".to_string(),
    };

    assert_eq!(run(text_transform_args).unwrap(), "HELLO");
}

#[test]
fn test_run_no_spaces() {
    let text_transform_args = TextTransformArgs {
        transform_type: "no-spaces".to_string(),
        text: "hello world".to_string(),
    };

    assert_eq!(run(text_transform_args).unwrap(), "helloworld");
}

#[test]
fn test_run_slugify() {
    let text_transform_args = TextTransformArgs {
        transform_type: "slugify".to_string(),
        text: "Hello World!".to_string(),
    };

    assert_eq!(run(text_transform_args).unwrap(), "hello-world");
}

#[test]
fn test_run_reverse() {
    let text_transform_args = TextTransformArgs {
        transform_type: "reverse".to_string(),
        text: "hello".to_string(),
    };

    assert_eq!(run(text_transform_args).unwrap(), "olleh");
}

#[test]
fn test_run_alternating() {
    let text_transform_args = TextTransformArgs {
        transform_type: "alternating".to_string(),
        text: "hello".to_string(),
    };

    assert_eq!(run(text_transform_args).unwrap(), "HeLlO");
}
