use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use bridge_buddy_core::primitives::card::relative_rank::RelativeRank;
use bridge_buddy_core::primitives::card::Rank;
use strum::IntoEnumIterator;

fn try_absolute_from_relative(relative: RelativeRank, played: u16) -> Option<Rank> {
    if relative == RelativeRank::OutOfPlay {
        return None;
    }

    let rel_index = relative as u16;
    let mut index = 0;

    while index <= rel_index {
        if played & (1 << index) == 0 {
            let shifted = played >> index;
            let pop_count = shifted.count_ones() as u16;

            if rel_index == index + pop_count {
                return Some(Rank::from(index));
            }
        }
        index += 1;
    }
    None
}

fn relative_from_rank(rank: Rank, played: u16) -> RelativeRank {
    let relative = 1u16 << rank as usize;

    if relative & played != 0 {
        return RelativeRank::OutOfPlay;
    }

    let index = rank as u16;

    let shifted = played >> index;
    let pop_count = shifted.count_ones() as u16;

    RelativeRank::from(index + pop_count)
}

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let mut relative_map = phf_codegen::Map::new();

    for played in 0u16..8192 {
        for rank in Rank::iter() {
            let field = 1u32 << (rank as u8);
            let key = field << 16 | played as u32;
            let rel_rank = relative_from_rank(rank, played);
            let value = format!("RelativeRank::{:?}", rel_rank);
            relative_map.entry(key, &value);
        }
    }

    writeln!(
        &mut file,
        "static RELATIVE: phf::Map<u32, RelativeRank> = {};",
        relative_map.build()
    )
    .unwrap();

    let mut absolute_map = phf_codegen::Map::new();

    for played in 0u16..8192 {
        for rel_rank in RelativeRank::iter().take(13) {
            let field = 1u32 << (rel_rank as u8);
            let key = field << 16 | played as u32;
            let abs_rank = try_absolute_from_relative(rel_rank, played);
            let value = match abs_rank {
                Some(abs) => format!("Some(Rank::{:?})", abs),
                None => "None".to_string(),
            };
            absolute_map.entry(key, &value);
        }
    }

    writeln!(
        &mut file,
        "static ABSOLUTE: phf::Map<u32, Option<Rank>> = {};",
        absolute_map.build()
    )
    .unwrap();
}
