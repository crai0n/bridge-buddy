use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::card::Rank;
use strum::IntoEnumIterator;

fn main() {
    create_relative_map();
    create_absolute_map();
}

fn create_absolute_map() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("absolute_map.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let mut absolute_map = phf_codegen::Map::new();

    for played in 0u16..8192 {
        for virtual_rank in VirtualRank::iter().take(13) {
            let field = 1u32 << (virtual_rank as u8);
            let key = field << 16 | played as u32;
            let abs_rank = try_absolute_from_virtual_rank(virtual_rank, played);
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

fn create_relative_map() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("relative_map.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let mut relative_map = phf_codegen::Map::new();

    for played in 0u16..8192 {
        for rank in Rank::iter() {
            let field = 1u32 << (rank as u8);
            let key = field << 16 | played as u32;
            let rel_rank = relative_from_rank(rank, played);
            let value = format!("VirtualRank::{:?}", rel_rank);
            relative_map.entry(key, &value);
        }
    }

    writeln!(
        &mut file,
        "static RELATIVE: phf::Map<u32, VirtualRank> = {};",
        relative_map.build()
    )
    .unwrap();
}

fn try_absolute_from_virtual_rank(virtual_rank: VirtualRank, played: u16) -> Option<Rank> {
    if virtual_rank == VirtualRank::OutOfPlay {
        return None;
    }

    let rel_index = virtual_rank as u16;
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

fn relative_from_rank(rank: Rank, played: u16) -> VirtualRank {
    let relative = 1u16 << rank as usize;

    if relative & played != 0 {
        return VirtualRank::OutOfPlay;
    }

    let index = rank as u16;

    let shifted = played >> index;
    let pop_count = shifted.count_ones() as u16;

    VirtualRank::from(index + pop_count)
}
