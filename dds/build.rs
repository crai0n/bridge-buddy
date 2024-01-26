use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use bridge_buddy_core::game::card_manager::suit_field;
use bridge_buddy_core::game::card_manager::suit_field::SuitField;
use bridge_buddy_core::primitives::card::relative_rank::RelativeRank;
use bridge_buddy_core::primitives::card::Rank;
use strum::IntoEnumIterator;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let mut relative_map = phf_codegen::Map::new();

    for played in 0u16..8192 {
        for rank in Rank::iter() {
            let field = SuitField::u16_from_rank(rank) as u32;
            let key = field << 16 | played as u32;
            let rel_rank = suit_field::relative_from_rank(rank, played);
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
            let abs_rank = suit_field::try_absolute_from_relative(rel_rank, played);
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
