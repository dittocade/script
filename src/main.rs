use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use game::Data;
use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
    thread,
};
use winnow::{combinator::terminated, stream::TokenSlice, Parser as _};

mod game;
mod lexer;
mod parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The command to run
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Compile a script
    Build {
        /// Path of the script to compile
        #[clap(short, long)]
        path: String,
    },

    /// Decode a game binary
    Decode {
        /// Path of the game to decode
        #[clap(short, long)]
        path: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Build { path } => {
            let path = Path::new(&path);
            let content = std::fs::read_to_string(path).unwrap();
            let content = content.as_str();
            let tokens = lexer::tokens.parse(content);
            let Ok(tokens) = tokens else {
                match tokens {
                    Ok(tokens) => println!("{:#?}", tokens),
                    Err(error) => {
                        let message = error.inner().to_string();
                        let input = (*error.input()).to_owned();
                        let span = error.char_span();
                        let message = annotate_snippets::Level::Error.title(&message).snippet(
                            annotate_snippets::Snippet::source(&input)
                                .fold(true)
                                .annotation(annotate_snippets::Level::Error.span(span.clone())),
                        );
                        let renderer = annotate_snippets::Renderer::plain();
                        let rendered = renderer.render(message);
                        println!("{}", rendered);
                    }
                };
                return;
            };
            let grammar = terminated(parser::statements0, lexer::token::Kind::EndOfFile)
                .parse(TokenSlice::new(&tokens));
            println!("{:?}", grammar);
        }

        Command::Decode { path } => {
            let path = Path::new(&path);
            let mut file = File::open(path).unwrap();
            let mut file = ZlibDecoder::new(file);
            let mut file = BufReader::new(file);
            let game = game::Game::read(&mut file).unwrap();
            let Ok(0) = file.read(&mut [0; 1]) else {
                panic!("not all game data could be read!");
            };
            println!("{:?}", game);
        }
    }
}
