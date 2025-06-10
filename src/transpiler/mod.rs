use anyhow::Result;
use blocks::Blocks;
use fnv::FnvHashMap;
use std::cmp::min;
use wires::{resolve_wires, Wires};

use crate::{
    game::{Chunk, Collider, Game, Kind},
    parser::grammar::{Expression, Input, Statement},
    transpiler::{blocks::{resolve_blocks, BlocksExt}, scripts::{get_scripts, Script}},
};

mod blocks;
mod wires;
mod scripts;

pub fn transpile_statements(grammar: Vec<Statement>) -> Result<Game> {
    let mut game = Game::default();

    let scripts = get_scripts();
    let mut blocks = FnvHashMap::default();
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
                    transpile_input(
                        input,
                        &scripts,
                        &mut blocks,
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

    let level = Chunk {
        is_locked: false,
        kind: Kind::Level,
        name: Some("New Level".to_string()),
        collider: Collider::default(),
        part: None,
        color: Some(0x1a),
        faces: None,
        blocks: resolve_blocks(&blocks),
        values: None,
        wires: resolve_wires(&wires, &blocks),
    };

    game.chunks.push(level);

    Ok(game)
}

pub fn transpile_input(
    input: &Input,
    scripts: &Vec<Script>,
    blocks: &mut Blocks,
    wires: &mut Wires,
    x: &mut i32,
    z: &mut i32,
    prev_pos: [i32; 3],
    prev_offset: [u16; 3],
) -> Result<()> {
    match &input.value {
        Expression::Skip => todo!(),
        Expression::Float(_) => todo!(),
        Expression::Integer(_) => todo!(),
        Expression::Boolean(_) => todo!(),
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
        
            for (i, input) in inputs.iter().enumerate() {
                transpile_input(
                    input,
                    scripts,
                    blocks,
                    wires,
                    x,
                    z,
                    pos,
                    [0o00, 0o01, 0o03 + (height as u16 - i as u16 - 1) * 8],
                )?;
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
