use ndarray::{Array, Array3, Array4};
use std::{
    io::{self, Read},
    iter::repeat_with,
};

use super::*;

impl Game {
    pub fn read(file: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            app_version: read_u16(file)?,
            title: read_string(file)?,
            author: read_string(file)?,
            description: read_string(file)?,
            id_offset: read_u16(file)?,
            chunks: read_chunks(file)?,
        })
    }
}

impl Chunk {
    pub fn read(file: &mut impl Read) -> io::Result<Self> {
        let flags = read_flags(file)?;
        let [has_wires, has_values, has_blocks, has_faces, is_part, has_collider, is_locked, _, has_color, _, _, has_name, has_kind, ..] =
            flags[..]
        else {
            unreachable!()
        };
        Ok(Self {
            is_locked,
            kind: (if has_kind { Some(read_u8(file)?) } else { None }).into(),
            name: if has_name {
                Some(read_string(file)?)
            } else {
                None
            },
            collider: (if has_collider {
                Some(read_u8(file)?)
            } else {
                None
            })
            .into(),
            part: if is_part {
                Some(Part {
                    id: read_u16(file)?,
                    offset: [read_u8(file)?, read_u8(file)?, read_u8(file)?],
                })
            } else {
                None
            },
            color: if has_color {
                Some(read_u8(file)?)
            } else {
                None
            },
            faces: if has_faces {
                Some(read_faces(file)?)
            } else {
                None
            },
            blocks: if has_blocks {
                Some(read_blocks(file)?)
            } else {
                None
            },
            opts: if has_values {
                Some(read_values(file)?)
            } else {
                None
            },
            wires: if has_wires {
                Some(read_wires(file)?)
            } else {
                None
            },
        })
    }
}

impl Opt {
    pub fn read(file: &mut impl Read) -> io::Result<Self> {
        let index = read_u8(file)?;
        let kind = read_u8(file)?;
        Ok(Self {
            index,
            position: [read_u16(file)?, read_u16(file)?, read_u16(file)?],
            data: match kind {
                0x01 => OptData::Int8(read_u8(file)?),
                0x02 => OptData::Int16(read_u16(file)?),
                0x04 => OptData::Float32(read_f32(file)?),
                0x05 => OptData::Vec([read_f32(file)?, read_f32(file)?, read_f32(file)?]),
                0x06 => OptData::Name(read_string(file)?),
                0x07 => OptData::Execute(read_string(file)?),
                0x08 => OptData::Input(read_string(file)?),
                0x09 => OptData::This(read_string(file)?),
                0x0A => OptData::Pointer(read_string(file)?),
                0x10 => OptData::Object(read_string(file)?),
                0x11 => OptData::Output(read_string(file)?),
                _ => OptData::Unknown(kind, read_string(file)?),
            },
        })
    }
}

impl Wire {
    pub fn read(file: &mut impl Read) -> io::Result<Self> {
        let from_position = [read_u16(file)?, read_u16(file)?, read_u16(file)?];
        let to_position = [read_u16(file)?, read_u16(file)?, read_u16(file)?];
        let from_offset = [read_u16(file)?, read_u16(file)?, read_u16(file)?];
        let to_offset = [read_u16(file)?, read_u16(file)?, read_u16(file)?];
        Ok(Self {
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
}

fn read_u8(file: &mut impl Read) -> io::Result<u8> {
    let mut buffer = [0; 1];
    file.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

fn read_u16(file: &mut impl Read) -> io::Result<u16> {
    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)?;
    Ok(u16::from_le_bytes(buffer))
}

fn read_f32(file: &mut impl Read) -> io::Result<f32> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    Ok(f32::from_le_bytes(buffer))
}

fn read_string(file: &mut impl Read) -> io::Result<String> {
    let length = read_u8(file)?;
    let mut buffer = Vec::with_capacity(length as usize);
    file.take(length as u64).read_to_end(&mut buffer)?;
    let string = String::from_utf8(buffer).unwrap();
    Ok(string)
}

fn read_flags(file: &mut impl Read) -> io::Result<Vec<bool>> {
    let flags = read_u16(file)?;
    let flags = (0..16).map(|i| flags & (0b1 << i) != 0).collect();
    Ok(flags)
}

fn read_faces(file: &mut impl Read) -> io::Result<Array4<u8>> {
    let dimensions = [6, 8, 8, 8];
    let capacity = dimensions.iter().product();
    let mut data = vec![0; capacity];
    file.read_exact(&mut data[..])?;
    let data = Array4::from_shape_vec(dimensions, data).unwrap();
    Ok(data)
}

fn read_blocks(file: &mut impl Read) -> io::Result<Array3<u16>> {
    let [z, y, x] = [
        read_u16(file)?.into(),
        read_u16(file)?.into(),
        read_u16(file)?.into(),
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

fn read_chunks(file: &mut impl Read) -> io::Result<Vec<Chunk>> {
    let length = read_u16(file)?;
    repeat_with(|| Chunk::read(file))
        .take(length.into())
        .collect()
}

fn read_values(file: &mut impl Read) -> io::Result<Vec<Opt>> {
    let length = read_u16(file)?;
    repeat_with(|| Opt::read(file))
        .take(length.into())
        .collect()
}

fn read_wires(file: &mut impl Read) -> io::Result<Vec<Wire>> {
    let length = read_u16(file)?;
    repeat_with(|| Wire::read(file))
        .take(length.into())
        .collect()
}
