use crate::{
    game::{Chunk, Collider, Color, Direction, FacesExt, Game, Kind, Part},
    lexer, parser,
    transpiler::transpile_game,
};
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use ndarray::{array, Array3, Array4};
use std::{
    fmt::Debug,
    fs::File,
    io::{stdout, BufReader, BufWriter, Read, Write},
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
        #[clap()]
        path: String,

        /// Where to store the output
        #[clap(short, long)]
        out: Option<String>,

        /// How to encode the output
        #[clap(short, long, default_value_t, value_enum)]
        encoding: Encoding,
    },

    /// Load a game binary
    Load {
        /// Path of the game to load
        #[clap()]
        path: String,

        /// Where to store the output
        #[clap(short, long)]
        out: Option<String>,

        /// How to encode the output
        #[clap(short, long, default_value_t, value_enum)]
        encoding: Encoding,

        /// How to decode the input
        #[clap(short, long, default_value_t, value_enum)]
        decoding: Decoding,
    },

    Generate {
        /// Where to store the output
        #[clap(short, long)]
        out: Option<String>,

        /// How to encode the output
        #[clap(short, long, default_value_t, value_enum)]
        encoding: Encoding,
    },
}

#[derive(ValueEnum, Default, Clone, Debug)]
pub enum Encoding {
    #[default]
    Zlib,
    Raw,
    Debug,
}

#[derive(ValueEnum, Default, Clone, Debug)]
pub enum Decoding {
    #[default]
    Zlib,
    Raw,
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Build {
            path,
            out,
            encoding,
        } => {
            let content = std::fs::read_to_string(&path)?;
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
                return Ok(());
            };

            let grammar = terminated(parser::statements0, lexer::token::Kind::EndOfFile)
                .parse(TokenSlice::new(&tokens))
                .unwrap();

            let game = transpile_game(grammar)?;

            let writer: Box<dyn Write> = match out {
                Some(out) => Box::new(File::create_new(out)?),
                None => Box::new(stdout()),
            };
            let mut writer = BufWriter::new(writer);
            write_game_with_encoding(&mut writer, game, encoding)?;
        }

        Command::Load {
            path,
            out,
            encoding,
            decoding,
        } => {
            let file = File::open(&path)?;
            let reader: Box<dyn Read> = match decoding {
                Decoding::Zlib => Box::new(ZlibDecoder::new(file)),
                Decoding::Raw => Box::new(file),
            };
            let mut reader = BufReader::new(reader);

            let game = Game::read(&mut reader)?;

            let Ok(0) = reader.read(&mut [0; 1]) else {
                panic!("not all game data could be read!");
            };

            let writer: Box<dyn Write> = match out {
                Some(out) => Box::new(File::create_new(out)?),
                None => Box::new(stdout()),
            };
            let mut writer = BufWriter::new(writer);
            write_game_with_encoding(&mut writer, game, encoding)?;
        }

        Command::Generate { out, encoding } => {
            let sizes = [("S", 1), ("M", 2), ("L", 3), ("XL", 4)];

            let kinds = [
                ("Script Block", [Color::Black, Color::Gray4, Color::Gray3]),
                ("Math Block", [Color::Gray4, Color::Gray3, Color::Gray2]),
                (
                    "Execute Block",
                    [Color::DarkYellow, Color::Yellow, Color::LightYellow],
                ),
            ];

            let width = 2;

            let mut chunk = 598;
            let mut chunks = Vec::new();

            let mut blocks = Vec::new();

            for (name, [primary, secondary, tertriary]) in kinds {
                let id = chunk;

                for i in 0..width {
                    let mut faces = Array4::zeros((6, 8, 8, 8));

                    for x in 0..(if i == width - 1 { 7 } else { 8 }) {
                        for y in 0..3 {
                            for z in 0..7 {
                                faces.fill_voxel((z, y, x), Color::Black as u8);
                            }
                        }
                    }

                    for x in 0..(if i == width - 1 { 7 } else { 8 })  {
                        for z in 0..7 {
                            *faces.get_mut((Direction::Up as usize, z, 2, x)).unwrap() = secondary as u8;
                            *faces.get_mut((Direction::Down as usize, z, 0, x)).unwrap() = primary as u8;
                        }
                    }

                    for x in 0..(if i == width - 1 { 7 } else { 8 })  {
                        for y in 0..3 {
                            *faces.get_mut((Direction::South as usize, 0, y, x)).unwrap() = primary as u8;
                            *faces.get_mut((Direction::North as usize, 6, y, x)).unwrap() = primary as u8;
                        }
                    }

                    for z in 0..7  {
                        for y in 0..3 {
                            if i == 0 {
                                *faces.get_mut((Direction::West as usize, z, y, 0)).unwrap() = primary as u8;
                            } else if i == 1 {
                                *faces.get_mut((Direction::East as usize, z, y, 6)).unwrap() = primary as u8;
                            }
                        }
                    }

                    for x in 0..(if i == width - 1 { 7 } else { 8 }) {
                        *faces.get_mut((Direction::Up as usize, 0, 2, x)).unwrap() = primary as u8;
                    }

                    for x in 0..(if i == width - 1 { 7 } else { 8 }) {
                        *faces.get_mut((Direction::Up as usize, 6, 2, x)).unwrap() = tertriary as u8;
                    }

                    if i == width - 1 {
                        for z in 0..7 {
                            *faces.get_mut((Direction::Up as usize, z, 2, 6)).unwrap() = tertriary as u8;
                        }
                        *faces.get_mut((Direction::Up as usize, 0, 2, 6)).unwrap() = secondary as u8;
                    } else if i == 0 {
                        for z in 0..6 {
                            *faces.get_mut((Direction::Up as usize, z, 2, 0)).unwrap() = primary as u8;
                        }
                        *faces.get_mut((Direction::Up as usize, 6, 2, 0)).unwrap() = secondary as u8;
                    }

                    let block = Chunk {
                        blocks: None,
                        collider: Collider::Default,
                        color: None,
                        faces: Some(faces),
                        is_locked: false,
                        kind: Kind::Default,
                        name: (id == chunk).then(|| name.to_string()),
                        opts: None,
                        part: Some(Part {
                            id,
                            offset: [i as u8, 0, 0],
                        }),
                        wires: None
                    };

                    blocks.push(block);

                    chunks.push(chunk);
                    chunk += 1;
                }
            }

            let level = Chunk {
                blocks: Some(Array3::from_shape_vec((1, 1, chunks.len()), chunks)?),
                collider: Collider::Default,
                color: Some(Color::Blue as u8),
                faces: None,
                is_locked: false,
                kind: crate::game::Kind::Level,
                name: Some("New Level".to_string()),
                opts: None,
                part: None,
                wires: None,
            };

            let chunks: Vec<Chunk> = [vec![level], blocks].concat();

            let game = Game {
                chunks,
                ..Default::default()
            };

            let writer: Box<dyn Write> = match out {
                Some(out) => Box::new(File::create_new(out)?),
                None => Box::new(stdout()),
            };
            let mut writer = BufWriter::new(writer);
            write_game_with_encoding(&mut writer, game, encoding)?;
        }
    }
    Ok(())
}

fn write_game_with_encoding(
    mut writer: &mut impl Write,
    game: Game,
    encoding: Encoding,
) -> Result<()> {
    match encoding {
        Encoding::Zlib => {
            let mut writer = ZlibEncoder::new(writer, Compression::best());
            game.write(&mut writer)?;
        }
        Encoding::Raw => {
            game.write(&mut writer)?;
        }
        Encoding::Debug => {
            write!(writer, "{:#?}", game)?;
        }
    }
    Ok(())
}
