use bridge_buddy_core::game::card_manager::suit_field::SuitField;
use bridge_buddy_core::primitives::card::relative_rank::RelativeRank;
use bridge_buddy_core::primitives::card::Rank;

pub mod dds;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn find_relative(absolute: Rank, played: u16) -> RelativeRank {
    let field = SuitField::u16_from_rank(absolute) as u32;
    let key = field << 16 | played as u32;
    *RELATIVE.get(&key).unwrap()
}

pub fn find_absolute(relative: RelativeRank, played: u16) -> Option<Rank> {
    let field = 1u32 << relative as usize;
    let key = field << 16 | played as u32;
    *ABSOLUTE.get(&key).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Rank::Two, 0b0000_0011_0000_1000, RelativeRank::Five)]
    #[test_case(Rank::Two, 0b0000_0011_0100_1000, RelativeRank::Six)]
    #[test_case(Rank::Two, 0b0000_0011_0100_1001, RelativeRank::OutOfPlay)]
    fn relative_given_played(rank: Rank, played: u16, expected: RelativeRank) {
        let relative = find_relative(rank, played);
        assert_eq!(relative, expected);
    }

    #[test_case(RelativeRank::Five, 0b0000_0011_0000_1000, Some(Rank::Two))]
    #[test_case(RelativeRank::Six, 0b0000_0011_0100_1000, Some(Rank::Two))]
    #[test_case(RelativeRank::Jack, 0b0000_0011_0100_1001, Some(Rank::Nine))]
    fn absolute_given_played(rank: RelativeRank, played: u16, expected: Option<Rank>) {
        let relative = find_absolute(rank, played);
        assert_eq!(relative, expected);
    }
}
