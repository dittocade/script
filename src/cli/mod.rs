use crate::{
    game::{Chunk, Collider, Color, Direction, Game, Kind, Part},
    lexer, parser,
    transpiler::transpile_game,
};
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use ndarray::{s, Array3, Array4};
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
                ("Script", [Color::Black as u8, Color::Gray4 as u8, Color::Gray3 as u8]),
                ("Math", [Color::Gray4 as u8, Color::Gray3 as u8, Color::Gray2 as u8]),
                (
                    "Execute",
                    [Color::DarkYellow as u8, Color::Yellow as u8, Color::LightYellow as u8],
                ),
                (
                    "Number",
                    [Color::DarkBlue as u8, Color::Blue as u8, Color::LightBlue as u8],
                ),
                (
                    "Object",
                    [Color::DarkPink as u8, Color::Pink as u8, Color::LightPink as u8],
                ),
                (
                    "Vector",
                    [Color::DarkGreen as u8, Color::Green as u8, Color::LightGreen as u8],
                ),
                (
                    "Rotation",
                    [Color::DarkOrange as u8, Color::Orange as u8, Color::LightOrange as u8],
                ), 
                (
                    "Truth",
                    [Color::DarkRed as u8, Color::Red as u8, Color::LightRed as u8],
                ),
                (
                    "Constraint",
                    [Color::Gray2 as u8, Color::Gray1 as u8, Color::White as u8],
                ),
            ];

            let width = 2;

            let mut chunk = 598;
            let mut chunks = Vec::new();
            let mut blocks = Array3::zeros((sizes.iter().map(|(_, height)| height).sum(), 1, width * kinds.len()));
            let mut x = 0;

            for (kind_name, [primary, secondary, tertriary]) in kinds {
                let mut z = 0;

                for (size_name, height) in sizes {
                    let id = chunk;
                    for part_z in 0..height {
                        for part_x in 0..width {
                            let is_westernmost = part_x == 0;
                            let is_easternmost = part_x == width - 1;
                            let is_southernmost = part_z == 0;
                            let is_northernmost = part_z == height - 1;

                            let part_depth = if is_northernmost { 7 } else { 8 };
                            let part_height = 3;
                            let part_width = if is_easternmost { 7 } else { 8 };

                            let mut faces = Array4::zeros((6, 8, 8, 8));

                            faces.slice_mut(s![0..6, 0..part_depth, 0..part_height, 0..part_width]).fill(Color::Black as u8);

                            faces.slice_mut(s![Direction::Up as usize, 0..part_depth, part_height - 1, 0..part_width]).fill(secondary);
                            faces.slice_mut(s![Direction::Down as usize, 0..part_depth, 0, 0..part_width]).fill(primary);

                            if is_westernmost {
                                faces.slice_mut(s![Direction::Up as usize, 0..part_depth, part_height - 1, 0]).fill(primary);
                                faces.slice_mut(s![Direction::West as usize, 0..part_depth, 0..part_height, 0]).fill(primary);
                            }

                            if is_easternmost {
                                faces.slice_mut(s![Direction::Up as usize, 0..part_depth, part_height - 1, part_width - 1]).fill(tertriary);
                                faces.slice_mut(s![Direction::East as usize, 0..part_depth, 0..part_height, part_width - 1]).fill(primary);
                            }

                            if is_southernmost {
                                faces.slice_mut(s![Direction::Up as usize, 0, part_height - 1, 0..part_width],).fill(primary);
                                faces.slice_mut(s![Direction::South as usize, 0, 0..part_height, 0..part_width]).fill(primary);
                            }

                            if is_northernmost {
                                faces.slice_mut(s![Direction::Up as usize, part_depth - 1, part_height - 1, 0..part_width]).fill(tertriary);
                                faces.slice_mut(s![Direction::North as usize, part_depth - 1, 0..part_height, 0..part_width]).fill(primary);
                            }

                            if is_westernmost && is_northernmost {
                                faces[(Direction::Up as usize, part_depth - 1, part_height - 1, 0)] = secondary;
                            }

                            if is_easternmost && is_southernmost {
                                faces[(Direction::Up as usize, 0, part_height - 1, part_width - 1)] = secondary;
                            }

                            let block = Chunk {
                                blocks: None,
                                collider: Collider::Passthrough,
                                color: None,
                                faces: Some(faces),
                                is_locked: false,
                                kind: Kind::Script,
                                name: (id == chunk).then(|| format!("{} {}", kind_name, size_name)),
                                opts: None,
                                part: Some(Part {
                                    id,
                                    offset: [part_x as u8, 0, part_z as u8],
                                }),
                                wires: None
                            };

                            blocks[(z + part_z, 0, x + part_x)] = chunk;

                            chunks.push(block);
                            chunk += 1;
                        }
                    }

                    z += height;
                }


                x += width;
            }

            let level = Chunk {
                blocks: Some(blocks),
                collider: Collider::Default,
                color: Some(1),
                faces: None,
                is_locked: false,
                kind: Kind::Level,
                name: Some("Scripts".to_string()),
                opts: None,
                part: None,
                wires: None,
            };

            let chunks: Vec<Chunk> = [vec![level], chunks].concat();

            let game = Game {
                chunks,
                title: "Script Templates".to_string(),
                author: "Bricked".to_string(),
                description: "Templates to quickly create script blocks!".to_string(),
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
