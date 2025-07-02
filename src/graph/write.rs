use std::{collections::HashMap, rc::Rc};
use anyhow::anyhow;

use super::{prefab::{Prefab, Opt}, Block, Graph, Port};
use crate::{game, grammar::{Expression, Statement}};

pub fn statements(statements: &Vec<Statement>, prefabs: &HashMap<String, Rc<Prefab>>) -> anyhow::Result<Graph> {
    statements.into_iter().rev().fold(Ok(None), |after, current| {
        statement(current, after?, prefabs)
    })
}

pub fn statement(statement: &Statement, after: Graph, prefabs: &HashMap<String, Rc<Prefab>>) -> anyhow::Result<Graph> {
    match statement {
        Statement::Invocation { name, inputs, outputs, callbacks } => {
            let prefab = prefabs.get(name);
            let Some(prefab) = prefab else {
                return Err(anyhow!(format!("Couldn't find prefab with name {}", name)))
            };

            let after_port = Port { offset: prefab.after_port(), graph: after };

            let mut i = 0;

            let inps: Vec<Port> = prefab.inputs.iter().map(|inp| {
                let input = inputs.get(i);
                let offset = prefab.input_port(i as u16);
                i += 1;
                let Some(input) = input else {
                    return Ok(Port {
                        graph: None,
                        offset,
                    });
                };

                let graph = expression(&input.value, prefabs)?;
                Ok(Port {
                    graph,
                    offset,
                })
            }).collect::<anyhow::Result<_>>()?;

            let opts: Vec<Option<game::OptData>> = prefab.opts.iter().map(|opt| {
                let input = inputs.get(i);
                i += 1;
                let Some(input) = input else {
                    return Ok(None);
                };
                let data = option(opt, &input.value)?;
                Ok(data)
            }).collect::<anyhow::Result<_>>()?;
 
            let block = Block {
                prefab: Rc::clone(&prefab),
                callbacks: vec![after_port],
                inps,
                opts,
            };

            Ok(Some(Rc::new(block)))
        },
        Statement::Assignement { value, outputs } => todo!(),
        Statement::Definition { name, inputs, outputs, callbacks, statements } => todo!(),
        Statement::Comment(_) => Ok(after),
    }
}

pub fn expression(expression: &Expression, prefabs: &HashMap<String, Rc<Prefab>>) -> anyhow::Result<Graph> {
    Ok(match expression {
        Expression::Skip => todo!(),
        Expression::Float(_) => todo!(),
        Expression::Integer(_) => todo!(),
        Expression::Boolean(_) => todo!(),
        Expression::String(_) => todo!(),
        Expression::Call { name, inputs } => {
            let prefab = prefabs.get(name);
            let Some(prefab) = prefab else {
                return Err(anyhow!(format!("Couldn't find prefab with name {}", name)))
            };

            let block = Block {
                prefab: Rc::clone(&prefab),
                callbacks: vec![],
                inps: Vec::default(),
                opts: Vec::default(),
            };

            Some(Rc::new(block))
        },
        Expression::Variable { modifier, name } => todo!(),
    })
}

pub fn option(opt: &Opt, expression: &Expression) -> anyhow::Result<Option<game::OptData>> {
    match opt.kind {
        game::OptKind::Int8 => {
            match expression {
                &Expression::Integer(value) => Ok(Some(game::OptData::Int8(value.try_into()?))),
                _ => unimplemented!()
            }
        },
        game::OptKind::Int16 => {
            match expression {
                &Expression::Integer(value) => Ok(Some(game::OptData::Int16(value.try_into()?))),
                _ => unimplemented!()
            }
        },
        game::OptKind::Float32 => {
            match expression {
                &Expression::Float(value) => Ok(Some(game::OptData::Float32(value as f32))),
                &Expression::Integer(value) => Ok(Some(game::OptData::Float32(value as f32))),
                _ => unimplemented!()
            }
        },
        game::OptKind::Vec => todo!(),
        game::OptKind::Name => {
            match expression {
                Expression::String(value) => Ok(Some(game::OptData::Name(value.to_string()))),
                _ => unimplemented!()
            }
        },
        game::OptKind::Execute => {
            match expression {
                Expression::String(value) => Ok(Some(game::OptData::Execute(value.to_string()))),
                _ => unimplemented!()
            }
        },
        game::OptKind::Input => {
            match expression {
                Expression::String(value) => Ok(Some(game::OptData::Input(value.to_string()))),
                _ => unimplemented!()
            }
        },
        game::OptKind::This => {
            match expression {
                Expression::String(value) => Ok(Some(game::OptData::This(value.to_string()))),
                _ => unimplemented!()
            }
        },
        game::OptKind::Pointer => {
            match expression {
                Expression::String(value) => Ok(Some(game::OptData::Pointer(value.to_string()))),
                _ => unimplemented!()
            }
        },
        game::OptKind::Object => {
            match expression {
                Expression::String(value) => Ok(Some(game::OptData::Object(value.to_string()))),
                _ => unimplemented!()
            }
        },
        game::OptKind::Output => {
            match expression {
                Expression::String(value) => Ok(Some(game::OptData::Output(value.to_string()))),
                _ => unimplemented!()
            }
        },
        game::OptKind::Unknown(kind) => {
            match expression {
                Expression::String(value) => Ok(Some(game::OptData::Unknown(kind, value.to_string()))),
                _ => unimplemented!()
            }
        },
    }
}