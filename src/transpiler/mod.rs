use ndarray::Array3;

use crate::{game::{Chunk, Collider, Game, Kind}, parser::grammar::Statement};

pub fn transpile(grammar: Vec<Statement>) -> Game {
    let mut game = Game::default();

    let blocks: Vec<Vec<Vec<u8>>> = Vec::new();

    for statement in grammar {
        match statement {
            Statement::Invocation { name, inputs, outputs, callbacks } => {
                match name.as_str() {
                    "set_score" => {
                        
                    },
                    _ => todo!(),
                }
                todo!()
            },
            Statement::Assignement { value, outputs } => {
                todo!()
            },
            Statement::Definition { name, inputs, outputs, callbacks, statements } => {
                todo!()
            },
            Statement::Comment(_) => {
                todo!()
            },
        }
    }

    let level = Chunk {
        is_locked: false,
        kind: Kind::Level,
        name: Some("New Level".to_string()),
        collider: Collider::default(),
        multi: None,
        color: Some(0x1a),
        faces: None,
        blocks: Some(Array3::default([0, 0, 0])),
        values: None,
        wires: None,
    };

    game.chunks.push(level);

    game
}