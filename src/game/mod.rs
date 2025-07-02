use anyhow::anyhow;
use ndarray::{Array3, Array4};
use std::{fmt::Debug, mem::transmute};

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
            app_version: 31,
            title: "New Game".to_string(),
            author: "Unknown".to_string(),
            description: "A Fancade game".to_string(),
            id_offset: 597,
            chunks: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
#[allow(unused)]
pub struct Face{
    color: Color,
    glued: bool,
}

impl TryFrom<u8> for Face {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Face {
            glued: value & 0b1000_0000 != 0,
            color: (value & 0b0111_1111).try_into()?,
        })
    }
}

impl Into<u8> for Face {
    fn into(self) -> u8 {
        self.color as u8 + (self.glued as u8 & 0b1000_0000)
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
#[repr(u8)]
pub enum Color {
    Empty = 0x00,
    Black = 0x01,
    Gray4 = 0x02,
    Gray3 = 0x03,
    Gray2 = 0x04,
    Gray1 = 0x05,
    White = 0x06,
    DarkBrown = 0x07,
    Brown = 0x08,
    LightBrown = 0x09,
    DarkTan = 0x0A,
    Tan = 0x0B,
    LightTan = 0x0C,
    DarkRed = 0x0D,
    Red = 0x0E,
    LightRed = 0x0F,
    DarkOrange = 0x10,
    Orange = 0x11,
    LightOrange = 0x12,
    DarkYellow = 0x13,
    Yellow = 0x14,
    LightYellow = 0x15,
    DarkGreen = 0x16,
    Green = 0x17,
    LightGreen = 0x18,
    DarkBlue = 0x19,
    Blue = 0x1A,
    LightBlue = 0x1B,
}

impl TryFrom<u8> for Color {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (value <= 0x1B) {
            Ok(unsafe {transmute(value)})
        } else {
            Err(anyhow!(format!("Couldn't convert {} to Color!", value)))
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub enum Direction {
    East = 0,
    West = 1,
    Up = 2,
    Down = 3,
    North = 4,
    South = 5,
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

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub struct Part {
    pub id: u16,
    pub offset: [u8; 3],
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Opt {
    pub index: u8,
    pub position: [u16; 3],
    pub data: OptData,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum OptData {
    Int8(u8),
    Int16(u16),
    Float32(f32),
    Vec([f32; 3]),
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

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub struct Wire {
    pub from: Port,
    pub to: Port,
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub struct Port {
    pub position: [u16; 3],
    pub offset: [u16; 3],
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

pub trait FacesExt {
    #[allow(unused)]
    fn fill_voxel(&mut self, voxel: (usize, usize, usize), color: u8);
}

impl FacesExt for Array4<u8> {
    fn fill_voxel(&mut self, (z, y, x): (usize, usize, usize), color: u8) {
        for side in 0..6 {
            *self.get_mut((side, z, y, x)).unwrap() = color;
        }
    }
}