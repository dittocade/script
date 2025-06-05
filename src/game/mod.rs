use std::{fs::File, io::{self, Read}, iter::repeat_with};


#[derive(Debug)]
pub struct Game {
    pub app_version: u16,
    pub title: String,
    pub author: String,
    pub description: String,
    pub id_offset: u16,
    pub chunks: Vec<Chunk>,
}

impl Game {
    pub fn read(file: &mut File) -> Result<Self, Box<dyn std::error::Error>> {
        let game = Self {
            app_version: read_u16(file)?,
            title: read_string(file)?,
            author: read_string(file)?,
            description: read_string(file)?,
            id_offset: read_u16(file)?,
            chunks: read_vec(file, Chunk::read)?,
        };
        Ok(game)
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub is_locked: bool,
    pub kind: Option<u8>,
    pub name: Option<String>,
    pub collider: Option<u8>,
    pub id: Option<u16>,
    pub offset: Option<[u8; 3]>,
    pub color: Option<u8>,
    pub faces: Option<[[[[u8; 6]; 8]; 8]; 8]>,
    pub blocks: Option<Vec<Vec<Vec<u16>>>>,
    pub values: Option<Vec<Value>>,
    pub wires: Option<Vec<Wire>>,
}

impl Chunk {
    pub fn read(file: &mut File) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_flags(file)?;
        let [
            has_wires,
            has_values,
            has_blocks,
            has_faces,
            is_multi,
            has_collider,
            is_locked,
            _,
            has_color,
            _,
            _,
            has_name,
            has_kind,
            ..
        ] = flags[..] else {
            unreachable!()
        };
        let chunk = Self {
            is_locked,
            kind: if has_kind { Some(read_u8(file)?) } else { None },
            name: if has_name { Some(read_string(file)?)} else {None},
            collider: if has_collider {Some(read_u8(file)?)} else {None},
            id: if is_multi {Some(read_u16(file)?)} else {None},
            offset: if is_multi {Some([read_u8(file)?, read_u8(file)?, read_u8(file)?])} else {None},
            color: if has_color {Some(read_u8(file)?)} else {None},
            faces: if has_faces { Some(read_faces(file)?)} else {None},
            blocks: if has_blocks {Some(read_blocks(file)?)} else {None},
            values: if has_values {Some(read_vec(file, Value::read)?)} else {None},
            wires: if has_wires {Some(read_vec(file, Wire::read)?)} else {None},
        };
        Ok(chunk)
    }
}

#[derive(Debug)]
pub struct Value {
    pub index: u8,
    pub position: [u16; 3],
    pub data: Data,
}

impl Value {
    pub fn read(file: &mut File) -> Result<Self, Box<dyn std::error::Error>> {
        let index = read_u8(file)?;
        let kind = read_u8(file)?;
        Ok(Self {
            index,
            position: [read_u16(file)?, read_u16(file)?, read_u16(file)?],
            data: Data::read(file, kind)?,
        })
    }
}

#[derive(Debug)]
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
}

impl Data {
    pub fn read(file: &mut File, kind: u8) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(match kind {
            0x01 => Data::Int8(read_u8(file)?),
            0x02 => Data::Int16(read_u16(file)?),
            0x04 => Data::Float32(read_f32(file)?),
            0x05 => Data::Vec([read_f32(file)?, read_f32(file)?, read_f32(file)?]),
            0x06 => Data::String(read_string(file)?),
            0x07 => Data::Execute(read_string(file)?),
            0x08 => Data::Input(read_string(file)?),
            0x09 => Data::This(read_string(file)?),
            0x0A => Data::Pointer(read_string(file)?),
            0x10 => Data::Object(read_string(file)?),
            0x11 => Data::Output(read_string(file)?),
            i => panic!("invalid data kind {}!", i),
        })
    }
}

#[derive(Debug)]
pub struct Wire {
    pub position: [[u16; 2]; 3],
    pub offset: [[u16; 2]; 3],
}

impl Wire {
    pub fn read(file: &mut File) -> Result<Self, Box<dyn std::error::Error>>  {
        Ok(Self {
            position: [[read_u16(file)?, read_u16(file)?],[read_u16(file)?, read_u16(file)?],[read_u16(file)?, read_u16(file)?],],
            offset: [[read_u16(file)?, read_u16(file)?],[read_u16(file)?, read_u16(file)?],[read_u16(file)?, read_u16(file)?],],
        })
    }
}

fn read_u8(file: &mut File) -> io::Result<u8> {
    let mut buffer = [0; 1];
    file.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

fn read_u16(file: &mut File) -> io::Result<u16> {
    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)?;
    Ok(u16::from_le_bytes(buffer))
}

fn read_f32(file: &mut File) -> io::Result<f32> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    Ok(f32::from_le_bytes(buffer))
}

fn read_string(file: &mut File) -> Result<String,  Box<dyn std::error::Error>> {
    let length = read_u8(file)?;
    let mut buffer = Vec::with_capacity(length as usize);
    file.take(length as u64).read_to_end(&mut buffer)?;
    let string = String::from_utf8(buffer)?;
    Ok(string)
}

fn read_flags(file: &mut File) -> io::Result<Vec<bool>> {
    let flags = read_u16(file)?;
    let flags = (0..16).map(|i| flags & (0b1 << i) != 0).collect();
    Ok(flags)
}

fn read_vec<T>(file: &mut File, reader: fn(&mut File) -> Result<T, Box<dyn std::error::Error>>) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let length = read_u16(file)?;
    repeat_with(|| reader(file)).take(length.into()).collect()
}

fn read_faces(file: &mut File) -> io::Result<[[[[u8; 6]; 8]; 8]; 8]> {
    let mut faces = [[[[0; 6]; 8]; 8]; 8];
    let mut face @ mut x @ mut y @ mut z = 0usize;
    let colors = repeat_with(|| read_u8(file));
    for color in colors {
        faces[z][y][x][face] = color?;
        face += 1;
        if face == 6 {
            face = 0;
            x += 1;
            if x == 8 {
                x = 0;
                y += 1;
                if y == 8 {
                    y = 0;
                    z += 1;
                    if z == 8 {
                        break;
                    }
                }
            }
        }
    }
    Ok(faces)
}

fn read_blocks(file: &mut File) -> io::Result<Vec<Vec<Vec<u16>>>> {
    let x = read_u16(file)?;
    let y = read_u16(file)?;
    let z = read_u16(file)?;
    repeat_with(
        || repeat_with(
            || repeat_with(
                || read_u16(file)
            ).take(z.into()).collect()
        ).take(y.into()).collect()
    ).take(x.into()).collect()
}