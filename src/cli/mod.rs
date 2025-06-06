use clap::{Parser, Subcommand, ValueEnum};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use crate::{game::Game, lexer, parser, transpiler::transpile};
use std::{
    error::Error, fmt::Debug, fs::File, io::{stdout, BufReader, BufWriter, Read, Write}, path::Path
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

        /// How to encode the compiled script
        #[clap(short, long)]
        encoding: Encoding,
    },

    /// Load a game binary
    Load {
        /// Path of the game to decode
        #[clap(short, long)]
        path: String,

        /// Path to store the compiled script at
        #[clap(short, long)]
        out: Option<String>,

        /// How to encode the game
        #[clap(short, long)]
        encoding: Encoding,
    },
}

#[derive(ValueEnum, Default, Clone, Debug)]
pub enum Encoding {
    #[default]
    Zlib,
    Raw,
    Zip,
    Debug,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Command::Build { path, out, encoding} => {
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
                        panic!("{}", rendered);
                    }
                };
                return Ok(())
            };

            let grammar = terminated(parser::statements0, lexer::token::Kind::EndOfFile)
                .parse(TokenSlice::new(&tokens)).unwrap();

            let game = transpile(grammar);

            let writer: Box<dyn Write> = match out {
                Some(out) => Box::new(File::create_new(out).unwrap()),
                None => Box::new(stdout()),
            };
            let mut writer = BufWriter::new(writer);
            write_game_with_encoding(&mut writer, game, encoding);
        }

        Command::Load { path, out, encoding } => {
            let file = File::open(&path).unwrap();
            let reader = ZlibDecoder::new(file);
            let mut reader = BufReader::new(reader);

            let game = Game::read(&mut reader).unwrap();

            let Ok(0) = reader.read(&mut [0; 1]) else {
                panic!("not all game data could be read!");
            };

            let writer: Box<dyn Write> = match out {
                Some(out) => Box::new(File::create_new(out).unwrap()),
                None => Box::new(stdout()),
            };
            let mut writer = BufWriter::new(writer);
            write_game_with_encoding(&mut writer, game, encoding);
        }
    }
    Ok(())
}

fn write_game_with_encoding(mut writer: &mut impl Write, game: Game, encoding: Encoding) {
    match encoding {
        Encoding::Zlib => {
            let mut writer = ZlibEncoder::new(writer, Compression::best());
            game.write(&mut writer).unwrap();
        },
        Encoding::Raw => {
            game.write(&mut writer).unwrap();
        },
        Encoding::Zip => {
            unimplemented!();
        },
        Encoding::Debug => {
            write!(writer, "{:#?}", game).unwrap();
        },
    }
}