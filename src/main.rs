mod cli;
mod game;
mod lexer;
mod parser;
mod transpiler;

fn main() {
    cli::run().unwrap();
}
