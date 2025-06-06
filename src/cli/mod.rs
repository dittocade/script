use clap::{Parser, Subcommand};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use crate::{game::Game, lexer, parser};
use std::{
    fmt::Debug, fs::File, io::{BufReader, BufWriter, Read, Write}, path::Path
};
use winnow::{combinator::terminated, stream::TokenSlice, Parser as _};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The command to run
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Compile a script
    Build {
        /// Path of the script to compile
        #[clap(short, long)]
        path: String,

        /// Path to store the compiled script at
        #[clap(short, long)]
        out: Option<String>,
    },

    /// Decode a game binary
    Decode {
        /// Path of the game to decode
        #[clap(short, long)]
        path: String,

        /// Path to store the decoded game at
        #[clap(short, long)]
        out: Option<String>,
    },

    /// Decode and re-encode a game binary
    Recode {
        /// Path of the game to decode
        #[clap(short, long)]
        path: String,

        /// Path to store the re-encoded game at
        #[clap(short, long)]
        out: Option<String>,
    },
}

pub fn run() {
    let args = Args::parse();

    match args.command {
        Command::Build { path, out} => {
            let content = std::fs::read_to_string(&path).unwrap();
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

            let mut writer = File::create_new(out.unwrap_or(path + ".out")).unwrap();
            write!(writer, "{:?}", grammar).unwrap();
        }

        Command::Decode { path, out } => {
            let file = File::open(&path).unwrap();
            let reader = ZlibDecoder::new(file);
            let mut reader = BufReader::new(reader);
            let game = Game::read(&mut reader).unwrap();
            let Ok(0) = reader.read(&mut [0; 1]) else {
                panic!("not all game data could be read!");
            };

            let mut writer = File::create_new(out.unwrap_or(path + ".out")).unwrap();
            write!(writer, "{:?}", game).unwrap()
        }

        Command::Recode { path, out } => {
            let reader = File::open(&path).unwrap();
            let reader = ZlibDecoder::new(reader);
            let mut reader = BufReader::new(reader);
            let game = Game::read(&mut reader).unwrap();
            let writer = File::create_new(out.unwrap_or(path + ".out")).unwrap();
            let writer = BufWriter::new(writer);
            let mut writer = ZlibEncoder::new(writer, Compression::best());
            game.write(&mut writer).unwrap();
        }
    }
}
