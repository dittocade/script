use itertools::Itertools;

use super::blocks::{calculate_ranges, Blocks};
use crate::game::Wire;

pub type Wires = Vec<([[i32; 3]; 2], [[u16; 3]; 2])>;

pub fn resolve_wires(wires: Wires, blocks: &Blocks) -> Option<Vec<Wire>> {
    if wires.is_empty() {
        return None;
    }

    let [min, _, _] = calculate_ranges(&blocks);

    let wires = wires
        .iter()
        .map(|&([from, to], offsets)| {
            let from = [
                (from[0] - min[0]) as u16,
                (from[1] - min[1]) as u16,
                (from[2] - min[2]) as u16,
            ];
            let to = [
                (to[0] - min[0]) as u16,
                (to[1] - min[1]) as u16,
                (to[2] - min[2]) as u16,
            ];
            Wire {
                positions: [from, to],
                offsets,
            }
        })
        .collect_vec();

    Some(wires)
}
