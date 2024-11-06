mod ast;
mod errors;
mod parser;
mod repl;
mod scanner;
mod token;

fn main() {
    repl::start();
}
