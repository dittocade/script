use ndarray::{Array3, Array4};
use std::fmt::Debug;

pub mod read;
pub mod write;

#[derive(Debug)]
#[allow(unused)]
pub struct Game {
    pub app_version: u16,
    pub title: String,
    pub author: String,
    pub description: String,
    pub id_offset: u16,
    pub chunks: Vec<Chunk>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            app_version: 32,
            title: "New Game".to_string(),
            author: "Unknown".to_string(),
            description: "A Fancade game".to_string(),
            id_offset: 597,
            chunks: Default::default(),
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct Chunk {
    pub is_locked: bool,
    pub kind: Option<u8>,
    pub name: Option<String>,
    pub collider: Option<u8>,
    pub multi: Option<Multi>,
    pub color: Option<u8>,
    pub faces: Option<Array4<u8>>,
    pub blocks: Option<Array3<u16>>,
    pub values: Option<Vec<Value>>,
    pub wires: Option<Vec<Wire>>,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Multi {
    pub id: u16,
    pub offset: [u8; 3],
}

#[derive(Debug)]
#[allow(unused)]
pub struct Value {
    pub index: u8,
    pub position: [u16; 3],
    pub data: Data,
}

#[derive(Debug)]
#[allow(unused)]
pub enum Data {
    Int8(u8),
    Int16(u16),
    Float32(f32),
    Vec([f32; 3]),
    String(String),
    Execute(String),
    Input(String),
    This(String),
    Pointer(String),
    Object(String),
    Output(String),
    Unknown(u8, String),
}

#[derive(Debug)]
#[allow(unused)]
pub struct Wire {
    pub position: [[u16; 2]; 3],
    pub offset: [[u16; 2]; 3],
}
