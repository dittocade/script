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
    pub kind: Kind,
    pub name: Option<String>,
    pub collider: Collider,
    pub multi: Option<Multi>,
    pub color: Option<u8>,
    pub faces: Option<Array4<u8>>,
    pub blocks: Option<Array3<u16>>,
    pub values: Option<Vec<Value>>,
    pub wires: Option<Vec<Wire>>,
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum Kind {
    Rigid,
    Physics,
    Script,
    Level,
}

impl From<Option<u8>> for Kind {
    fn from(value: Option<u8>) -> Self {
        match value {
            None => Kind::Rigid,
            Some(value) => match value {
                0x01 => Kind::Physics,
                0x02 => Kind::Script,
                0x03 => Kind::Level,
                _ => panic!("invalid chunk kind!")
            }
        }
    }
}

impl Into<Option<u8>> for Kind {
    fn into(self) -> Option<u8> {
        match self {
            Kind::Rigid => None,
            Kind::Physics => Some(0x01),
            Kind::Script => Some(0x02),
            Kind::Level => Some(0x03),
        }
    }
}

impl Default for Kind {
    fn default() -> Self {
        Self::from(None)
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum Collider {
    Box,
    Passthrough,
    Sphere,
}

impl From<Option<u8>> for Collider {
    fn from(value: Option<u8>) -> Self {
        match value {
            None => Collider::Box,
            Some(value) => match value {
                0x00 => Collider::Passthrough,
                0x02 => Collider::Sphere,
                _ => panic!("invalid chunk collider!")
            }
        }
    }
}

impl Into<Option<u8>> for Collider {
    fn into(self) -> Option<u8> {
        match self {
            Collider::Box => None,
            Collider::Passthrough => Some(0x00),
            Collider::Sphere => Some(0x02),
        }
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self::from(None)
    }
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
