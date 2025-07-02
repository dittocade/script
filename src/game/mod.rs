pub mod read;
pub mod write;

use ndarray::{Array3, Array4};
use std::fmt::Debug;
use crate::position::Position;

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
            app_version: 31,
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
    pub part: Option<Part>,
    pub color: Option<u8>,
    pub faces: Option<Array4<u8>>,
    pub blocks: Option<Array3<u16>>,
    pub opts: Option<Vec<Opt>>,
    pub wires: Option<Vec<Wire>>,
}

#[derive(Debug, Clone, Copy, Default)]
#[allow(unused)]
pub enum Kind {
    #[default]
    Default,
    Physics,
    Script,
    Level,
}

impl From<Option<u8>> for Kind {
    fn from(value: Option<u8>) -> Self {
        match value {
            None => Kind::Default,
            Some(value) => match value {
                0x01 => Kind::Physics,
                0x02 => Kind::Script,
                0x03 => Kind::Level,
                _ => panic!("invalid chunk kind!"),
            },
        }
    }
}

impl Into<Option<u8>> for Kind {
    fn into(self) -> Option<u8> {
        match self {
            Kind::Default => None,
            Kind::Physics => Some(0x01),
            Kind::Script => Some(0x02),
            Kind::Level => Some(0x03),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[allow(unused)]
pub enum Collider {
    #[default]
    Default,
    Passthrough,
    Sphere,
}

impl From<Option<u8>> for Collider {
    fn from(value: Option<u8>) -> Self {
        match value {
            None => Collider::Default,
            Some(value) => match value {
                0x00 => Collider::Passthrough,
                0x02 => Collider::Sphere,
                _ => panic!("invalid chunk collider!"),
            },
        }
    }
}

impl Into<Option<u8>> for Collider {
    fn into(self) -> Option<u8> {
        match self {
            Collider::Default => None,
            Collider::Passthrough => Some(0x00),
            Collider::Sphere => Some(0x02),
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct Part {
    pub id: u16,
    pub offset: Position<u8>,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Opt {
    pub index: u8,
    pub position: Position<u16>,
    pub data: OptData,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum OptData {
    Int8(u8),
    Int16(u16),
    Float32(f32),
    Vec(Position<f32>),
    Name(String),
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
pub enum OptKind {
    Int8,
    Int16,
    Float32,
    Vec,
    Name,
    Execute,
    Input,
    This,
    Pointer,
    Object,
    Output,
    Unknown(u8), // TODO: find out what these types of data are used for
}

#[derive(Debug)]
#[allow(unused)]
pub struct Wire {
    pub from: Port,
    pub to: Port,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Port {
    pub position: Position<u16>,
    pub offset: Position<u16>,
}

#[derive(Debug)]
#[allow(unused)]
pub enum WireKind {
    Execute,
    Value(ValueKind),
}

#[derive(Debug)]
#[allow(unused)]
pub enum ValueKind {
    Raw(RawKind),
    Reference(RawKind),
}

#[derive(Debug)]
#[allow(unused)]
pub enum RawKind {
    Number,
    Vector,
    Rotation,
    Truth,
    Object,
    Constraint,
}
