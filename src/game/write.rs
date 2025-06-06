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
        let has_wires = self.wires.is_some();
        let has_values = self.values.is_some();
        let has_blocks = self.blocks.is_some();
        let has_faces = self.faces.is_some();
        let is_multi = self.multi.is_some();
        let has_collider = self.collider.is_some();
        let has_color = self.color.is_some();
        let has_name = self.name.is_some();
        let has_kind = self.kind.is_some();
        
        let flags = [
            has_wires,
            has_values,
            has_blocks,
            has_faces,
            is_multi,
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

        if let Some(kind) = self.kind {
            write_u8(file, kind)?;
        }
        if let Some(name) = &self.name {
            write_string(file, &name)?;
        }
        if let Some(collider) = self.collider {
            write_u8(file, collider)?;
        }
        if let Some(multi) = &self.multi {
            write_u16(file, multi.id)?;
            file.write_all(&multi.offset)?;
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
        if let Some(values) = &self.values {
            write_values(file, &values)?;
        }
        if let Some(wires) = &self.wires {
            write_wires(file, &wires)?;
        }
        Ok(())
    }
}

impl Value {
    pub fn write(&self, file: &mut impl Write) -> io::Result<()> {
        write_u8(file, self.index)?;

        let kind = match self.data {
            Data::Int8(_) => 0x01,
            Data::Int16(_) => 0x02,
            Data::Float32(_) => 0x04,
            Data::Vec(_) => 0x05,
            Data::String(_) => 0x06,
            Data::Execute(_) => 0x07,
            Data::Input(_) => 0x08,
            Data::This(_) => 0x09,
            Data::Pointer(_) => 0x0A,
            Data::Object(_) => 0x10,
            Data::Output(_) => 0x11,
            Data::Unknown(kind, _) => kind,
        };
        write_u8(file, kind)?;

        let position: Vec<_> = self
            .position
            .iter()
            .flat_map(|pos| pos.to_le_bytes())
            .collect();
        file.write_all(&position[..])?;
        match &self.data {
            Data::Int8(data) => write_u8(file, *data),
            Data::Int16(data) => write_u16(file, *data),
            Data::Float32(data) => write_f32(file, *data),
            Data::Vec(data) => {
                let data: Vec<_> = data.iter().flat_map(|pos| pos.to_le_bytes()).collect();
                file.write_all(&data[..])
            }
            Data::String(data) => write_string(file, &data),
            Data::Execute(data) => write_string(file, &data),
            Data::Input(data) => write_string(file, &data),
            Data::This(data) => write_string(file, &data),
            Data::Pointer(data) => write_string(file, &data),
            Data::Object(data) => write_string(file, &data),
            Data::Output(data) => write_string(file, &data),
            Data::Unknown(_, data) => write_string(file, &data),
        }
    }
}

impl Wire {
    pub fn write(&self, file: &mut impl Write) -> io::Result<()> {
        let position: Vec<_> = self
            .position
            .iter()
            .flatten()
            .flat_map(|pos| pos.to_le_bytes())
            .collect();
        file.write_all(&position[..])?;
        let offset: Vec<_> = self
            .offset
            .iter()
            .flatten()
            .flat_map(|pos| pos.to_le_bytes())
            .collect();
        file.write_all(&offset[..])
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
    let (x, y, z) = blocks.dim();
    let dimensions: Vec<_> = [x as u16, y as u16, z as u16].iter().flat_map(|v| v.to_le_bytes()).collect();
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

fn write_values(file: &mut impl Write, values: &Vec<Value>) -> io::Result<()> {
    write_u16(file, values.len() as u16)?;
    values.iter().map(|value| value.write(file)).collect()
}

fn write_wires(file: &mut impl Write, wires: &Vec<Wire>) -> io::Result<()> {
    write_u16(file, wires.len() as u16)?;
    wires.iter().map(|wire| wire.write(file)).collect()
}
