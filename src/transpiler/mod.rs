use anyhow::{anyhow, Result};
use blocks::Blocks;
use fnv::FnvHashMap;
use itertools::join;
use opts::{resolve_opts, Opts};
use std::cmp::min;
use wires::{resolve_wires, Wires};

use crate::{
    game::{Chunk, Collider, Game, Kind, Opt, OptData, OptKind},
    parser::grammar::{Expression, Input, Statement},
    transpiler::{blocks::{resolve_blocks, BlocksExt}, scripts::{get_scripts, Script}},
};

mod blocks;
mod wires;
mod scripts;
mod opts;

pub fn transpile_statements(grammar: Vec<Statement>) -> Result<Game> {
    let mut game = Game::default();

    let scripts = get_scripts();
    let mut blocks = FnvHashMap::default();
    let mut opts = Vec::new();
    let mut wires = Vec::new();
    let mut prev_pos = None;
    let mut z = 0;
    let mut x = 0;

    for statement in grammar {
        match statement {
            Statement::Invocation {
                name,
                inputs,
                outputs,
                callbacks,
            } => {
                let Some(script) = scripts.iter().find(|script| script.name == name) else {
                    todo!();
                };

                let height = script.parts.dim().0 as i32;
                let new_z = z - height;
                // origin position
                let pos = [x, 0, new_z];

                // connect before and after wires
                if let Some(prev_pos) = prev_pos {
                    wires.push((
                        [prev_pos, pos],
                        [
                            [0x03, 0o01, 0o00],
                            [0o03, 0o01, 0o06 + (height as u16 - 1) * 8],
                        ],
                    ));
                }

                // insert block
                blocks.try_insert_parts(pos, &script.parts)?;

                // transpile inputs
                for (i, input) in inputs.iter().enumerate() {
                    transpile_expression(
                        &input.value,
                        &scripts,
                        &mut blocks,
                        &mut opts,
                        &mut wires,
                        &mut x,
                        &mut z,
                        pos,
                        [0o00, 0o01, 0o03 + (height as u16 - i as u16 - 1) * 8],
                    )?;
                }

                // calculate lower z bound
                z = min(z, new_z);
                prev_pos = Some(pos);
            }
            Statement::Assignement { value, outputs } => {
                todo!()
            }
            Statement::Definition {
                name,
                inputs,
                outputs,
                callbacks,
                statements,
            } => {
                todo!()
            }
            Statement::Comment(_) => {
                todo!()
            }
        }
    }

    let wires = resolve_wires(wires, &blocks);
    let opts = resolve_opts(opts, &blocks);
    let level = Chunk {
        is_locked: false,
        kind: Kind::Level,
        name: Some("New Level".to_string()),
        collider: Collider::default(),
        part: None,
        color: Some(0x1a),
        faces: None,
        blocks: resolve_blocks(blocks),
        opts,
        wires
    };

    game.chunks.push(level);

    Ok(game)
}

pub fn transpile_expression(
    value: &Expression,
    scripts: &Vec<Script>,
    blocks: &mut Blocks,
    opts: &mut Opts,
    wires: &mut Wires,
    x: &mut i32,
    z: &mut i32,
    prev_pos: [i32; 3],
    prev_offset: [u16; 3],
) -> Result<()> {
    match &value {
        Expression::Skip => (),
        Expression::Float(value) => {
            transpile_expression(
                &Expression::Call {
                    name: "number".to_string(),
                    inputs: vec![Input { value: Expression::Float(*value), label: None}],
                },
                scripts,
                blocks,
                opts,
                wires,
                x,
                z,
                prev_pos,
                prev_offset,
            )?;
        },
        Expression::Integer(value) => {
            transpile_expression(
                &Expression::Call {
                    name: "number".to_string(),
                    inputs: vec![Input { value: Expression::Float(*value as f64), label: None}],
                },
                scripts,
                blocks,
                opts,
                wires,
                x,
                z,
                prev_pos,
                prev_offset,
            )?;
        },
        Expression::Boolean(_) => todo!(),
        Expression::String(_) => unimplemented!(),
        Expression::Call { name, inputs } => {
            let Some(script) = scripts.iter().find(|script| &script.name == name) else {
                todo!();
            };

            // add x padding
            *x -= 1;

            let height = script.parts.dim().0 as i32;
            let width = script.parts.dim().2 as i32;
            let new_z = *z - height;
            *x -= width;
            // origin position
            let pos = [*x, 0, *z - height];

            wires.push((
                [pos, prev_pos],
                [
                    [
                        0x06 + (width as u16 - 1) * 8,
                        0o01,
                        0o03 + (height as u16 - 1) * 8,
                    ],
                    prev_offset,
                ],
            ));
            blocks.try_insert_parts(pos, &script.parts)?;
        
            for (i, inp) in script.inputs.iter().enumerate() {
                if let Some(input) = inputs.get(i) {
                    transpile_expression(
                        &input.value,
                        scripts,
                        blocks,
                        opts,
                        wires,
                        x,
                        z,
                        pos,
                        [0o00, 0o01, 0o03 + (height as u16 - i as u16 - 1) * 8],
                    )?;
                }
            }

            for (i, opt) in script.options.iter().enumerate() {
                if let Some(input) = inputs.get(i + script.inputs.len()) {
                    if let Expression::Skip = input.value {
                        continue;
                    }

                    let data = match opt.kind {
                        OptKind::Int8 => {
                            let Expression::Integer(value) = input.value else {
                                unimplemented!()
                            };

                            OptData::Int8(value.try_into()?)
                        },
                        OptKind::Int16 => {
                            let Expression::Integer(value) = input.value else {
                                unimplemented!()
                            };

                            OptData::Int16(value.try_into()?)
                        },
                        OptKind::Float32 => {
                            match input.value {
                                Expression::Integer(value) => OptData::Float32(value as f32),
                                Expression::Float(value) => OptData::Float32(value as f32),
                                _ => unimplemented!()
                            }
                        },
                        OptKind::Vec => todo!(),
                        OptKind::Name => {
                            let Expression::String(value) = &input.value else {
                                unimplemented!()
                            };

                            OptData::Name(title_case(value))
                        },
                        OptKind::Execute => todo!(),
                        OptKind::Input => todo!(),
                        OptKind::This => todo!(),
                        OptKind::Pointer => todo!(),
                        OptKind::Object => todo!(),
                        OptKind::Output => todo!(),
                        OptKind::Unknown(_) => todo!(),
                    };
                    opts.push((i as u8, pos, data))
                }
            }

            *x += width;
            *z = min(*z, new_z);

            // undo x padding
            *x += 1;
        }
        Expression::Variable { modifier, name } => todo!(),
    };
    Ok(())
}


fn title_case(s: &str) -> String {
    let words = s.split('_').map(|s| {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    });

    return join(words, " ")
}