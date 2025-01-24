use project_1::{run, TextTransformArgs};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let text_transform_args = TextTransformArgs::new(&args).unwrap_or_else(|err| {
        println!("Usage: <transform_type> <text>");
        println!("Transform_type: lowercase, uppercase, no-spaces, slugify, reverse, alternating");
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    match run(text_transform_args) {
        Ok(result) => println!("The result of your transformation is: {}", result),
        Err(e) => {
            println!("Application error: {e}");
            std::process::exit(1);
        }
    }
}
