use anyhow::{anyhow, Result};
use context::Size;
use itertools::Itertools;
use ndarray::Array3;
use std::{
    cmp::min,
    collections::HashMap, rc::Rc,
};

use crate::{
    game::{self, Chunk, Collider, Game, Kind, OptData, OptKind}, grammar::{Expression, Input, Statement}, graph::prefab::{prefabs, by_name, Prefab}, position::Position, transpiler::context::{Block, Context, Port, Wire}
};

mod context;

pub fn transpile_statements(statements: Vec<Statement>, ctx: &mut Context, prefabs: &HashMap<String, Rc<Prefab>>) -> Result<()> {
    let mut prev = None::<Block>;
    for statement in statements {
        let current = transpile_statement(statement, ctx, prefabs)?;

        if let Some(current) = current {
            if let Some(ref prev) = prev {
                ctx.wires.push(Wire {
                    from: prev.after_port(),
                    to: current.before_port(),
                })
            }
            prev = Some(current);
        }
    }
    Ok(())
}

pub fn transpile_statement(statement: Statement, ctx: &mut Context, prefabs: &HashMap<String, Rc<Prefab>>) -> Result<Option<Block>> {
    Ok(None)
}

pub fn transpile_game(statements: Vec<Statement>) -> Result<Game> {
    let mut ctx = Context::default();
    let prefabs = by_name(prefabs());

    transpile_statements(statements, &mut ctx, &prefabs)?;
    
    ctx.try_into()
}
