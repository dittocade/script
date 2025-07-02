pub mod write;
pub mod prefab;

use crate::{game, position::Position};
use prefab::Prefab;
use std::rc::Rc;

pub type Graph = Option<Rc<Block>>;

#[derive(Debug)]
pub struct Block {
    pub prefab: Rc<Prefab>,
    pub callbacks: Vec<Port>,
    pub inps: Vec<Port>,
    pub opts: Vec<Option<game::OptData>>,
}

#[derive(Debug)]
pub struct Port {
    pub offset: Position<u16>,
    pub graph: Graph,
}