use anyhow::{anyhow, Result};
use itertools::Itertools;
use ndarray::Array3;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

use crate::{
    game::{self, Chunk, Collider, Game, Kind, OptData, OptKind},
    parser::grammar::{Expression, Input, Statement},
    transpiler::prefabs::{get_prefabs, Prefab},
};

mod blocks;
mod opts;
mod prefabs;
mod wires;

pub fn transpile_statements(statements: Vec<Statement>, ctx: &mut Context, prefabs: &HashMap<String, Prefab>) -> Result<()> {
    let mut prev = None::<Block>;
    for statement in statements {
        let new = transpile_statement(statement, ctx, prefabs)?;

        if let Some(new) = new {
            if let Some(ref prev) = prev {
                ctx.wires.push(Wire {
                    from: Port {
                        position: prev.position,
                        offset: [0x03, 0o01, 0o00],
                    },
                    to: Port {
                        position: new.position,
                        offset: [0o03, 0o01, 0o06 + (new.parts.dim().0 as u16 - 1) * 8],
                    },
                })
            }
            prev = Some(new);
        }
    }
    Ok(())
}

pub fn transpile_statement(statement: Statement, ctx: &mut Context, prefabs: &HashMap<String, Prefab>) -> Result<Option<Block>> {
    match statement {
        Statement::Invocation {
            name,
            inputs,
            outputs,
            callbacks,
        } => {
            let Some(prefab) = prefabs.get(&name) else {
                return Err(anyhow!("Can't find prefab with name: {}", name));
            };

            let height = prefab.parts.dim().0 as i32;
            let mut pos = ctx.pos.clone();
            ctx.pos[0] -= 1;
            pos[2] -= height;

            let block = Block {
                parts: prefab.parts.clone(),
                position: pos,
            };
            ctx.blocks.push(block.clone());

            transpile_inputs(prefab, &block, inputs, ctx, prefabs)?;

            ctx.pos[0] += 1;
            ctx.pos[2] = min(ctx.pos[2], pos[2]);
            Ok(Some(block))
        }
        Statement::Assignement { value, outputs } => todo!(),
        Statement::Definition {
            name,
            inputs,
            outputs,
            callbacks,
            statements,
        } => todo!(),
        Statement::Comment(value) => {
            ctx.pos[2] -= 1;

            for line in textwrap::wrap(value.as_str(), 16) {
                let prefab = prefabs.get(&"comment".to_string()).unwrap();
                let height = prefab.parts.dim().0 as i32;
                ctx.pos[2] -= height;
                ctx.pos[0] += 1;
                ctx.blocks.push(Block {
                    parts: prefab.parts.clone(),
                    position: ctx.pos,
                });
                ctx.opts.push(Opt {
                    data: OptData::Name(line.to_string()),
                    position: ctx.pos,
                    index: 0,
                });
                ctx.pos[0] -= 1;
            }

            Ok(None)
        }
    }
}

pub fn transpile_expression(expression: Expression, ctx: &mut Context, prefabs: &HashMap<String, Prefab>) -> Result<Option<Block>> {
    match expression {
        Expression::Skip => {
            ctx.pos[2] -= 1;
            Ok(None)
        }
        Expression::Float(value) => transpile_expression(
            Expression::Call {
                name: "number".to_string(),
                inputs: vec![Input {
                    label: None,
                    value: Expression::Float(value),
                }],
            },
            ctx,
            prefabs
        ),
        Expression::Integer(value) => transpile_expression(Expression::Float(value as f64), ctx, prefabs),
        Expression::Boolean(value) => transpile_expression(
            Expression::Call {
                name: value.to_string(),
                inputs: Vec::new(),
            },
            ctx,
            prefabs
        ),
        Expression::String(value) => {
            return Err(anyhow!(
                "Can't parse string literal to expression: {}",
                value
            ))
        }
        Expression::Call { name, inputs } => {
            let Some(prefab) = prefabs.get(&name) else {
                return Err(anyhow!("Can't find prefab with name: {}", name));
            };

            let height = prefab.parts.dim().0 as i32;
            let width = prefab.parts.dim().2 as i32;
            ctx.pos[0] -= width;
            let mut pos = ctx.pos.clone();

            ctx.pos[0] -= 1;
            pos[2] -= height;

            let block = Block {
                parts: prefab.parts.clone(),
                position: pos,
            };
            ctx.blocks.push(block.clone());

            transpile_inputs(prefab, &block, inputs, ctx, prefabs)?;

            ctx.pos[0] += width + 1;
            ctx.pos[2] = min(ctx.pos[2], pos[2]);
            Ok(Some(block))
        }
        Expression::Variable { modifier, name } => todo!(),
    }
}

pub fn transpile_inputs(prefab: &Prefab, block: &Block, inputs: Vec<Input>, ctx: &mut Context, prefabs: &HashMap<String, Prefab>) -> Result<()> {
    for (i, input) in inputs.into_iter().enumerate() {
        if i < prefab.inputs.len() {
            let prev = transpile_expression(input.value, ctx, prefabs)?;
            if let Some(prev) = prev {
                ctx.wires.push(Wire {
                    from: Port {
                        position: prev.position,
                        offset: [
                            0x06 + (prev.parts.dim().2 as u16 - 1) * 8,
                            0o01,
                            0o03 + (prev.parts.dim().0 as u16 - 1) * 8,
                        ],
                    },
                    to: Port {
                        position: block.position,
                        offset: [0o00, 0o01, 0o03 + (block.parts.dim().0 as u16 - i as u16 - 1) * 8],
                    },
                })
            }
        } else {
            let i = i - prefab.inputs.len();
            if i < prefab.options.len() {
                transpile_option( &prefab.options[i], &input.value, i as u8, block.position, ctx)?;
            } else {
                return Err(anyhow!("Too many inputs were provided!"))
            }
        }
    }
    Ok(())
}

pub fn transpile_option(opt: &prefabs::Opt, value: &Expression, index: u8,position: [i32; 3],  ctx: &mut Context) -> Result<()> {
    let data = match opt.kind {
        OptKind::Int8 => match value {
            &Expression::Integer(value) => OptData::Int8(value.try_into()?),
            &Expression::Boolean(value) => OptData::Int8(value.into()),
            _ => unimplemented!(),
        },
        OptKind::Int16 => match value {
            &Expression::Integer(value) => OptData::Int16(value.try_into()?),
            _ => unimplemented!(),
        },
        OptKind::Float32 => match value {
            &Expression::Integer(value) => OptData::Float32(value as f32),
            &Expression::Float(value) => OptData::Float32(value as f32),
            _ => unimplemented!(),
        },
        OptKind::Vec => todo!(),
        OptKind::Name => match value {
            Expression::String(value) => OptData::Name(value.to_string()),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    
    ctx.opts.push(Opt {
        data,
        index,
        position
    });
    Ok(())
}

pub fn transpile_game(statements: Vec<Statement>) -> Result<Game> {
    let mut ctx = Context::default();
    let prefabs = get_prefabs();

    transpile_statements(statements, &mut ctx, &prefabs)?;

    let size = Size::new(&ctx.blocks);
    let mut blocks: ndarray::ArrayBase<ndarray::OwnedRepr<u16>, ndarray::Dim<[usize; 3]>> =
        Array3::zeros(size.dim());
    for block in ctx.blocks.iter() {
        let pos = block.position;
        for ((z, y, x), &block) in block.parts.indexed_iter() {
            let [x, y, z] =
                size.resolve_position(&[pos[0] + x as i32, pos[1] + y as i32, pos[2] + z as i32]);
            blocks[[z as usize, y as usize, x as usize]] = block;
        }
    }
    let blocks = if blocks.is_empty() {
        None
    } else {
        Some(blocks)
    };

    let opts = ctx
        .opts
        .iter()
        .map(|opt| game::Opt {
            index: opt.index,
            position: size.resolve_position(&opt.position),
            data: opt.data.clone(),
        })
        .collect_vec();
    let opts = if opts.is_empty() { None } else { Some(opts) };

    let wires = ctx
        .wires
        .iter()
        .map(|wire| game::Wire {
            from: game::Port {
                position: size.resolve_position(&wire.from.position),
                offset: wire.from.offset,
            },
            to: game::Port {
                position: size.resolve_position(&wire.to.position),
                offset: wire.to.offset,
            },
        })
        .collect_vec();
    let wires = if wires.is_empty() { None } else { Some(wires) };

    let level = Chunk {
        is_locked: false,
        kind: Kind::Level,
        name: Some("New Level".to_string()),
        collider: Collider::default(),
        part: None,
        color: Some(0x1a),
        faces: None,
        blocks,
        opts,
        wires,
    };

    let game = Game {
        chunks: vec![level],
        ..Game::default()
    };

    Ok(game)
}

#[derive(Debug, Default)]
pub struct Context {
    pub pos: [i32; 3],
    pub blocks: Vec<Block>,
    pub wires: Vec<Wire>,
    pub opts: Vec<Opt>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub parts: Array3<u16>,
    pub position: [i32; 3],
}

#[derive(Debug)]
pub struct Wire {
    pub from: Port,
    pub to: Port,
}

#[derive(Debug)]
pub struct Port {
    pub position: [i32; 3],
    pub offset: [u16; 3],
}

#[derive(Debug)]
pub struct Opt {
    pub index: u8,
    pub position: [i32; 3],
    pub data: OptData,
}

#[derive(Debug, Default)]
pub struct Size {
    pub min: [i32; 3],
    pub max: [i32; 3],
}

impl Size {
    fn new(blocks: &Vec<Block>) -> Self {
        let mut size = Self::default();
        size.update(blocks);
        size
    }

    fn update(&mut self, blocks: &Vec<Block>) {
        for block in blocks {
            self.min[0] = min(self.min[0], block.position[0]);
            self.min[1] = min(self.min[1], block.position[1]);
            self.min[2] = min(self.min[2], block.position[2]);
            self.max[0] = max(self.max[0], block.position[0] + block.parts.dim().2 as i32);
            self.max[1] = max(self.max[1], block.position[1] + block.parts.dim().1 as i32);
            self.max[2] = max(self.max[2], block.position[2] + block.parts.dim().0 as i32);
        }
    }

    fn dim(&self) -> [usize; 3] {
        [
            (self.max[2] - self.min[2]) as usize,
            (self.max[1] - self.min[1]) as usize,
            (self.max[0] - self.min[0]) as usize,
        ]
    }

    fn resolve_position(&self, position: &[i32; 3]) -> [u16; 3] {
        [
            (position[0] - self.min[0]) as u16,
            (position[1] - self.min[1]) as u16,
            (position[2] - self.min[2]) as u16,
        ]
    }
}
