use project_1::run;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match run(&args) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
