# Rust developer course

## Homework I

- install Rust ✓
- set up the development environment ✓ 
- set up git repository ✓
- modify the code a tiny bit ✓
- build and run the code ✓


## Homework II
- added slug with cargo add command (0.1.6) ✓
- collect string vector from standard input ✓
- parse the CLI arguments using TextTransformArgs struct ✓
- default funcionality - lowercase, uppercase, no-spaces and slugify ✓
- bonus funcionality - reverse and case alternation ✓
- unit tests for string transformation ✓


## Homework III
- the main function calls the run function which examine the first argument, identify the operation and call the appropriete function ✓
- display the operation output or an error ✓
- each operation has dedicated function ✓
- these functions should validatate and parse arguments ✓
- these functions should return `Result<String, Box<dyn Error>>` ✓
- use `format!()` and `println!()` macros ✓
- present the selected operation and any errors via `eprintln!()` ✓
- added aditional csv operation ✓
- interpret the input string as csv the first line as headers ✓
- print the output in table layout ✓
- handle lenght greater then 16 characters ✓
- created csv struct and implemented a `Display` trait ✓
- added csv crate ✓


