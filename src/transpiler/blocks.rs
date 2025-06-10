use anyhow::{anyhow, Result};
use fnv::FnvHashMap;
use itertools::Itertools;
use ndarray::Array3;

pub type Blocks = FnvHashMap<[i32; 3], u16>;

pub trait BlocksExt {
    fn try_insert_parts(&mut self, offset: [i32; 3], parts: &Array3<u16>) -> Result<()>;
}

impl BlocksExt for Blocks {
    fn try_insert_parts(&mut self, offset: [i32; 3], parts: &Array3<u16>) -> Result<()> {
        for ((z, y, x), &block) in parts.indexed_iter() {
            let pos = [
                x as i32 + offset[0],
                y as i32 + offset[1],
                z as i32 + offset[2],
            ];
            let prev = self.insert(pos, block);
            if let Some(prev) = prev {
                return Err(anyhow!(
                    "Tried to write block {} at {:?}, found existing block {}!",
                    block,
                    pos,
                    prev
                ));
            }
        }
        Ok(())
    }
}

pub fn calculate_ranges(blocks: &Blocks) -> [[i32; 3]; 3] {
    if blocks.is_empty() {
        return [[0; 3], [0; 3], [0; 3]];
    }

    let (x_min, x_max) = blocks
        .keys()
        .map(|pos| pos[0])
        .minmax()
        .into_option()
        .unwrap_or_default();
    let x_size = x_max - x_min + 1;
    let (y_min, y_max) = blocks
        .keys()
        .map(|pos| pos[1])
        .minmax()
        .into_option()
        .unwrap_or_default();
    let y_size = y_max - y_min + 1;
    let (z_min, z_max) = blocks
        .keys()
        .map(|pos| pos[2])
        .minmax()
        .into_option()
        .unwrap_or_default();
    let z_size = z_max - z_min + 1;

    [
        [x_min, y_min, z_min],
        [x_max, y_max, z_max],
        [x_size, y_size, z_size],
    ]
}

pub fn resolve_blocks(blocks: &Blocks) -> Option<Array3<u16>> {
    let [min, _, size] = calculate_ranges(&blocks);
    if size == [0; 3] {
        return None;
    }

    let mut resolved = Array3::<u16>::zeros([size[2] as usize, size[1] as usize, size[0] as usize]);

    for ([x, y, z], block) in blocks {
        resolved[[
            (z - min[2]) as usize,
            (y - min[1]) as usize,
            (x - min[0]) as usize,
        ]] = *block;
    }

    Some(resolved)
}
