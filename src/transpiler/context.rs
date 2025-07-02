use crate::{game, position::Position};
use ndarray::Array3;
use std::{cmp::{max, min}, num::TryFromIntError};

#[derive(Debug, Default)]
pub struct Context {
    pub blocks: Vec<Block>,
    pub wires: Vec<Wire>,
    pub opts: Vec<Opt>,
}

#[derive(Debug)]
pub struct Block {
    pub parts: Array3<u16>,
    pub position: Position<i32>,
}

impl Block {
    pub fn dim(&self) -> Position<usize> {
        let pos: Position<usize> = self.parts.dim().into();
        pos.to_reversed()
    }

    pub fn min(&self) -> Position<i32> {
        self.position
    }

    pub fn max(&self) -> Position<i32> {
        self.position + self.dim().try_into().unwrap()
    }

    pub fn before_port(&self) -> Port {
        Port {
            position: self.position,
            offset: Position::new(0o03, 0o01, self.dim().z as u16 * 0o10 - 0o02),
        }
    }

    pub fn after_port(&self) -> Port {
        Port {
            position: self.position,
            offset: Position::new(0x03, 0o01, 0o00),
        }
    }

    pub fn output_port(&self, index: u16) -> Port {
        Port {
            position: self.position,
            offset: Position::new(
                (self.dim().x as u16 - index) * 0o10 - 0o02,
                0o01,
                self.dim().z as u16 * 0o10 - 0o05,
            ),
        }
    }

    pub fn input_port(&self, index: u16) -> Port {
        Port {
            position: self.position,
            offset: Position::new(0o00, 0o01, (self.dim().z as u16 - index) * 0o10 - 0o05),
        }
    }
}

#[derive(Debug)]
pub struct Wire {
    pub from: Port,
    pub to: Port,
}

#[derive(Debug)]
pub struct Port {
    pub position: Position<i32>,
    pub offset: Position<u16>,
}

#[derive(Debug)]
pub struct Opt {
    pub index: u8,
    pub position: Position<i32>,
    pub data: game::OptData,
}

#[derive(Debug, Default)]
pub struct Size {
    pub min: Position<i32>,
    pub max: Position<i32>,
}

impl Size {
    pub fn new(blocks: &Vec<Block>) -> Self {
        let mut size = Self::default();
        size.update(blocks);
        size
    }

    pub fn dim(&self) -> Position<u16> {
        (self.max - self.min).try_into().unwrap()
    }

    pub fn update(&mut self, blocks: &Vec<Block>) {
        for block in blocks {
            self.min.x = min(self.min.x, block.min().x);
            self.min.y = min(self.min.y, block.min().y);
            self.min.z = min(self.min.z, block.min().z);
            self.max.x = max(self.max.x, block.max().x);
            self.max.y = max(self.max.y, block.max().y);
            self.max.z = max(self.max.z, block.max().z);
        }
    }

    pub fn resolve_position(&self, position: Position<i32>) -> Result<Position<u16>, TryFromIntError> {
        (position - self.min).try_into()
    }
}

impl TryInto<game::Game> for Context {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<game::Game, Self::Error> {
        let size = Size::new(&self.blocks);

        let dim: Position<usize> = size.dim().to_reversed().try_into()?;
        let dim: (usize, usize, usize) = dim.into();

        let mut blocks: ndarray::ArrayBase<ndarray::OwnedRepr<u16>, ndarray::Dim<[usize; 3]>> =
            Array3::zeros(dim);

        for block in self.blocks.iter() {
            let pos = block.position;
            for (offset, &block) in block.parts.indexed_iter() {
                let offset = Position::from(offset).to_reversed().try_into()?;
                let pos: Position<usize> = (pos + offset).try_into()?;
                let pos: (usize, usize, usize) = pos.to_reversed().into();
                blocks[pos] = block;
            }
        }

        let blocks = Some(blocks).filter(|b| !b.is_empty());

        let opts= self
            .opts
            .iter()
            .map(|opt| Ok(game::Opt {
                index: opt.index,
                position: size.resolve_position(opt.position)?.into(),
                data: opt.data.clone(),
            }))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let opts = if opts.is_empty() { None } else { Some(opts) };

        let wires = self
            .wires
            .iter()
            .map(|wire| Ok(game::Wire {
                from: game::Port {
                    position: size.resolve_position(wire.from.position)?.into(),
                    offset: wire.from.offset.into(),
                },
                to: game::Port {
                    position: size.resolve_position(wire.to.position)?.into(),
                    offset: wire.to.offset.into(),
                },
            }))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let wires = if wires.is_empty() { None } else { Some(wires) };

        let level = game::Chunk {
            is_locked: false,
            kind: game::Kind::Level,
            name: Some("New Level".to_string()),
            collider: game::Collider::default(),
            part: None,
            color: Some(0x1a),
            faces: None,
            blocks,
            opts,
            wires,
        };

        let game = game::Game {
            chunks: vec![level],
            ..game::Game::default()
        };

        Ok(game)
    }
}