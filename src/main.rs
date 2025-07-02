mod cli;
mod game;
mod token;
mod grammar;
mod transpiler;
mod position;
mod graph;

fn main() {
    cli::run().unwrap();
}
