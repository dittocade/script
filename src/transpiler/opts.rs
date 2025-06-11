use itertools::Itertools;

use super::blocks::{calculate_ranges, Blocks};
use crate::game::{Opt, OptData};

pub type Opts = Vec<(u8, [i32; 3], OptData)>;

pub fn resolve_opts(opts: Opts, blocks: &Blocks) -> Option<Vec<Opt>> {
    if opts.is_empty() {
        return None;
    }

    let [min, _, _] = calculate_ranges(&blocks);

    let opts = opts
        .iter()
        .map(|(i, pos, data)| {
            let pos = [
                (pos[0] - min[0]) as u16,
                (pos[1] - min[1]) as u16,
                (pos[2] - min[2]) as u16,
            ];
            Opt {
                index: *i,
                position: pos,
                data: data.clone(),
            }
        })
        .collect_vec();

    Some(opts)
}
