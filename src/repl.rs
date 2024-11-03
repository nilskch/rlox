use std::io;
use std::io::Write;

use crate::scanner::Scanner;

const PROMPT: &str = ">> ";

pub fn start() {
    println!("Welcome to the Lox Programming Language!");
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut scanner = Scanner::new(&input);
        match scanner.scan_tokens() {
            Ok(tokens) => {
                for token in tokens {
                    println!("{:?}", token);
                }
            }
            Err(errors) => {
                for err in errors {
                    let diagnostic = miette::Report::new(err);
                    println!("{:?}", diagnostic);
                }
            }
        }
    }
}
