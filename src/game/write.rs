use ndarray::{Array3, Array4};
use std::io::{self, Write};

use super::*;

impl Game {
    #[allow(unused)]
    pub fn write(&self, file: &mut impl Write) -> io::Result<()> {
        write_u16(file, self.app_version)?;
        write_string(file, &self.title)?;
        write_string(file, &self.author)?;
        write_string(file, &self.description)?;
        write_u16(file, self.id_offset)?;
        write_chunks(file, &self.chunks)
    }
}

impl Chunk {
    #[allow(unused)]
    pub fn write(&self, file: &mut impl Write) -> io::Result<()> {
        let collider: Option<u8> = self.collider.into();
        let kind: Option<u8> = self.kind.into();
        let has_wires = self.wires.is_some();
        let has_values = self.opts.is_some();
        let has_blocks = self.blocks.is_some();
        let has_faces = self.faces.is_some();
        let is_part = self.part.is_some();
        let has_collider = collider.is_some();
        let has_color = self.color.is_some();
        let has_name = self.name.is_some();
        let has_kind = kind.is_some();

        let flags = [
            has_wires,
            has_values,
            has_blocks,
            has_faces,
            is_part,
            has_collider,
            self.is_locked,
            false,
            has_color,
            false,
            false,
            has_name,
            has_kind,
            false,
            false,
            false,
        ];
        write_flags(file, flags)?;

        if let Some(kind) = kind {
            write_u8(file, kind)?;
        }
        if let Some(name) = &self.name {
            write_string(file, &name)?;
        }
        if let Some(collider) = collider {
            write_u8(file, collider)?;
        }
        if let Some(multi) = &self.part {
            write_u16(file, multi.id)?;
            file.write_all(&multi.offset.into_slice()[..])?;
        }
        if let Some(color) = self.color {
            write_u8(file, color)?;
        }
        if let Some(faces) = &self.faces {
            write_faces(file, faces)?;
        }
        if let Some(blocks) = &self.blocks {
            write_blocks(file, blocks)?;
        }
        if let Some(values) = &self.opts {
            write_values(file, &values)?;
        }
        if let Some(wires) = &self.wires {
            write_wires(file, &wires)?;
        }
        Ok(())
    }
}

impl Opt {
    pub fn write(&self, file: &mut impl Write) -> io::Result<()> {
        write_u8(file, self.index)?;

        let kind = match self.data {
            OptData::Int8(_) => 0x01,
            OptData::Int16(_) => 0x02,
            OptData::Float32(_) => 0x04,
            OptData::Vec(_) => 0x05,
            OptData::Name(_) => 0x06,
            OptData::Execute(_) => 0x07,
            OptData::Input(_) => 0x08,
            OptData::This(_) => 0x09,
            OptData::Pointer(_) => 0x0A,
            OptData::Object(_) => 0x10,
            OptData::Output(_) => 0x11,
            OptData::Unknown(kind, _) => kind,
        };
        write_u8(file, kind)?;

        let position: Vec<_> = self
            .position
            .into_slice()
            .iter()
            .flat_map(|pos| pos.to_le_bytes())
            .collect();
        file.write_all(&position[..])?;
        match &self.data {
            OptData::Int8(data) => write_u8(file, *data),
            OptData::Int16(data) => write_u16(file, *data),
            OptData::Float32(data) => write_f32(file, *data),
            OptData::Vec(data) => {
                let data: Vec<_> = data.into_slice().iter().flat_map(|pos| pos.to_le_bytes()).collect();
                file.write_all(&data[..])
            }
            OptData::Name(data) => write_string(file, &data),
            OptData::Execute(data) => write_string(file, &data),
            OptData::Input(data) => write_string(file, &data),
            OptData::This(data) => write_string(file, &data),
            OptData::Pointer(data) => write_string(file, &data),
            OptData::Object(data) => write_string(file, &data),
            OptData::Output(data) => write_string(file, &data),
            OptData::Unknown(_, data) => write_string(file, &data),
        }
    }
}

impl Wire {
    pub fn write(&self, file: &mut impl Write) -> io::Result<()> {
        let positions: Vec<_> = [self.from.position.into_slice(), self.to.position.into_slice()]
            .iter()
            .flatten()
            .flat_map(|pos| pos.to_le_bytes())
            .collect();
        file.write_all(&positions[..])?;

        let offsets: Vec<_> = [self.from.offset.into_slice(), self.to.offset.into_slice()]
            .iter()
            .flatten()
            .flat_map(|pos| pos.to_le_bytes())
            .collect();
        file.write_all(&offsets[..])
    }
}

fn write_u8(file: &mut impl Write, data: u8) -> io::Result<()> {
    file.write_all(&data.to_le_bytes())
}

fn write_u16(file: &mut impl Write, data: u16) -> io::Result<()> {
    file.write_all(&data.to_le_bytes())
}

fn write_f32(file: &mut impl Write, data: f32) -> io::Result<()> {
    file.write_all(&data.to_le_bytes())
}

fn write_string(file: &mut impl Write, string: &String) -> io::Result<()> {
    write_u8(file, string.len() as u8)?;
    file.write_all(string.as_bytes())
}

fn write_flags(file: &mut impl Write, flags: [bool; 16]) -> io::Result<()> {
    let flags = (0..16).map(|i| (flags[i] as u16) << i).sum();
    write_u16(file, flags)
}

fn write_faces(file: &mut impl Write, faces: &Array4<u8>) -> io::Result<()> {
    let faces = faces.as_slice().unwrap();
    file.write_all(faces)
}

fn write_blocks(file: &mut impl Write, blocks: &Array3<u16>) -> io::Result<()> {
    let (z, y, x) = blocks.dim();
    let dimensions: Vec<_> = [x as u16, y as u16, z as u16]
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    file.write_all(&dimensions[..])?;

    let blocks: Vec<_> = blocks
        .iter()
        .flat_map(|block| block.to_le_bytes())
        .collect();
    file.write_all(&blocks[..])
}

fn write_chunks(file: &mut impl Write, chunks: &Vec<Chunk>) -> io::Result<()> {
    write_u16(file, chunks.len() as u16)?;
    chunks.iter().map(|chunk| chunk.write(file)).collect()
}

fn write_values(file: &mut impl Write, values: &Vec<Opt>) -> io::Result<()> {
    write_u16(file, values.len() as u16)?;
    values.iter().map(|value| value.write(file)).collect()
}

fn write_wires(file: &mut impl Write, wires: &Vec<Wire>) -> io::Result<()> {
    write_u16(file, wires.len() as u16)?;
    wires.iter().map(|wire| wire.write(file)).collect()
}
