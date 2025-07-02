use ndarray::{Array, Array3, Array4};
use std::{
    io::{self, Read},
    iter::repeat_with,
};

use super::*;

pub fn game(file: &mut impl Read) -> io::Result<Game> {
    Ok(Game {
        app_version: u16(file)?,
        title: string(file)?,
        author: string(file)?,
        description: string(file)?,
        id_offset: u16(file)?,
        chunks: chunks(file)?,
    })
}

pub fn chunk(file: &mut impl Read) -> io::Result<Chunk> {
    let flags = flags(file)?;
    let [has_wires, has_values, has_blocks, has_faces, is_part, has_collider, is_locked, _, has_color, _, _, has_name, has_kind, ..] =
        flags[..]
    else {
        unreachable!()
    };

    Ok(Chunk {
        is_locked,
        kind: (if has_kind { Some(u8(file)?) } else { None }).into(),
        name: if has_name {
            Some(string(file)?)
        } else {
            None
        },
        collider: (if has_collider {
            Some(u8(file)?)
        } else {
            None
        })
        .into(),
        part: if is_part {
            Some(Part {
                id: u16(file)?,
                offset: Position::new(u8(file)?, u8(file)?, u8(file)?),
            })
        } else {
            None
        },
        color: if has_color {
            Some(u8(file)?)
        } else {
            None
        },
        faces: if has_faces {
            Some(faces(file)?)
        } else {
            None
        },
        blocks: if has_blocks {
            Some(blocks(file)?)
        } else {
            None
        },
        opts: if has_values {
            Some(opts(file)?)
        } else {
            None
        },
        wires: if has_wires {
            Some(wires(file)?)
        } else {
            None
        },
    })
}

pub fn opt(file: &mut impl Read) -> io::Result<Opt> {
    let index = u8(file)?;
    let kind = u8(file)?;

    Ok(Opt {
        index,
        position: Position::new(u16(file)?, u16(file)?, u16(file)?),
        data: match kind {
            0x01 => OptData::Int8(u8(file)?),
            0x02 => OptData::Int16(u16(file)?),
            0x04 => OptData::Float32(f32(file)?),
            0x05 => OptData::Vec(Position::new(f32(file)?, f32(file)?, f32(file)?)),
            0x06 => OptData::Name(string(file)?),
            0x07 => OptData::Execute(string(file)?),
            0x08 => OptData::Input(string(file)?),
            0x09 => OptData::This(string(file)?),
            0x0A => OptData::Pointer(string(file)?),
            0x10 => OptData::Object(string(file)?),
            0x11 => OptData::Output(string(file)?),
            _ => OptData::Unknown(kind, string(file)?),
        },
    })
}

pub fn wire(file: &mut impl Read) -> io::Result<Wire> {
    let from_position = Position::new(u16(file)?, u16(file)?, u16(file)?);
    let to_position = Position::new(u16(file)?, u16(file)?, u16(file)?);
    let from_offset = Position::new(u16(file)?, u16(file)?, u16(file)?);
    let to_offset = Position::new(u16(file)?, u16(file)?, u16(file)?);

    Ok(Wire {
        from: Port {
            position: from_position,
            offset: from_offset,
        },
        to: Port {
            position: to_position,
            offset: to_offset,
        },
    })
}

fn u8(file: &mut impl Read) -> io::Result<u8> {
    let mut buffer = [0; 1];
    file.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

fn u16(file: &mut impl Read) -> io::Result<u16> {
    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)?;
    Ok(u16::from_le_bytes(buffer))
}

fn f32(file: &mut impl Read) -> io::Result<f32> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    Ok(f32::from_le_bytes(buffer))
}

fn string(file: &mut impl Read) -> io::Result<String> {
    let length = u8(file)?;
    let mut buffer = Vec::with_capacity(length as usize);
    file.take(length as u64).read_to_end(&mut buffer)?;
    let string = String::from_utf8(buffer).unwrap();
    Ok(string)
}

fn flags(file: &mut impl Read) -> io::Result<Vec<bool>> {
    let flags = u16(file)?;
    let flags = (0..16).map(|i| flags & (0b1 << i) != 0).collect();
    Ok(flags)
}

fn faces(file: &mut impl Read) -> io::Result<Array4<u8>> {
    let dimensions = [6, 8, 8, 8];
    let capacity = dimensions.iter().product();
    let mut data = vec![0; capacity];
    file.read_exact(&mut data[..])?;
    let data = Array4::from_shape_vec(dimensions, data).unwrap();
    Ok(data)
}

fn blocks(file: &mut impl Read) -> io::Result<Array3<u16>> {
    let [z, y, x] = [
        u16(file)?.into(),
        u16(file)?.into(),
        u16(file)?.into(),
    ];
    let dimensions = [x, y, z];
    let capacity: usize = dimensions.iter().product();
    let mut data = vec![0; capacity * 2];
    file.read_exact(&mut data[..])?;
    let data = data[..].chunks(2).map(|v: &[u8]| match v {
        &[a, b] => u16::from_le_bytes([a, b]),
        _ => unreachable!(),
    });
    let data = Array::from_iter(data);
    let data = data.into_shape_with_order(dimensions).unwrap();
    Ok(data)
}

fn chunks(file: &mut impl Read) -> io::Result<Vec<Chunk>> {
    let length = u16(file)?;
    repeat_with(|| chunk(file))
        .take(length.into())
        .collect()
}

fn opts(file: &mut impl Read) -> io::Result<Vec<Opt>> {
    let length = u16(file)?;
    repeat_with(|| opt(file))
        .take(length.into())
        .collect()
}

fn wires(file: &mut impl Read) -> io::Result<Vec<Wire>> {
    let length = u16(file)?;
    repeat_with(|| wire(file))
        .take(length.into())
        .collect()
}
