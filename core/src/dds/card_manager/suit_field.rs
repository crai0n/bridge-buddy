use crate::dds::card_manager::relative_rank::RelativeRank;
use crate::primitives::card::rank::N_RANKS;
use crate::primitives::card::Rank;
use lazy_static::lazy_static;
use std::ops::BitXor;
use strum::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SuitField(u16);

lazy_static! {
    static ref TO_RELATIVE_GIVEN_PLAYED: [[RelativeRank; 13]; 8192] = {
        let mut rr = [[RelativeRank::OutOfPlay; 13]; 8192];
        for rank in Rank::iter().take(13) {
            for played in 0u16..8192 {
                rr[played as usize][rank as usize] = relative_from_rank(rank, played)
            }
        }
        rr
    };
    static ref TO_ABSOLUTE_GIVEN_PLAYED: [[Option<Rank>; 13]; 8192] = {
        let mut ar = [[None; 13]; 8192];
        for relrank in RelativeRank::iter().take(13) {
            for played in 0u16..8192 {
                ar[played as usize][relrank as usize] = try_absolute_from_relative(relrank, played)
            }
        }
        ar
    };
}

fn try_absolute_from_relative(relative: RelativeRank, played: u16) -> Option<Rank> {
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
    let relative = SuitField::u16_from_rank(rank);

    if relative & played != 0 {
        return RelativeRank::OutOfPlay;
    }

    let index = rank as u16;

    let shifted = played >> index;
    let pop_count = shifted.count_ones() as u16;

    RelativeRank::from(index + pop_count)
}

#[allow(dead_code)]
impl SuitField {
    pub fn empty() -> Self {
        Self(0u16)
    }

    #[allow(dead_code)]
    pub fn from_u16(val: u16) -> Self {
        Self(val)
    }

    pub fn for_n_cards_per_suit(n: usize) -> Self {
        let mask = (1u16 << (13 - n)) - 1;
        Self(mask)
    }

    const fn u16_from_rank(rank: Rank) -> u16 {
        1 << (rank as usize)
    }

    pub fn add_rank(&mut self, rank: Rank) {
        self.0 |= Self::u16_from_rank(rank);
    }

    pub fn remove_rank(&mut self, rank: Rank) {
        self.0 &= !Self::u16_from_rank(rank);
    }

    pub fn contains_rank(&self, rank: Rank) -> bool {
        self.0 & Self::u16_from_rank(rank) != 0
    }

    pub fn count_cards(&self) -> u8 {
        self.0.count_ones() as u8
    }

    fn count_high_cards(&self) -> u8 {
        let field = self.0 << 3; // make Ace the leading bit
        field.leading_ones() as u8
    }

    pub fn count_high_cards_given_played_cards(&self, played: &SuitField) -> u8 {
        let relative = self.relative_ranks_given_played_ranks(played);
        relative.count_high_cards()
    }

    pub fn all_contained_ranks(&self) -> Vec<Rank> {
        let mut vec = vec![];

        let mut tracking_field = self.0;

        while tracking_field != 0 {
            let lowest_bit = tracking_field & (!tracking_field + 1);
            tracking_field &= !lowest_bit;
            let index = lowest_bit.ilog2();
            let rank = Rank::from((index % 16) as u16);
            vec.push(rank)
        }

        vec
    }

    pub fn non_equivalent_moves(&self, played_cards: &SuitField) -> Vec<Rank> {
        let ranks = self.relative_ranks_given_played_ranks(played_cards);

        let tops = ranks.only_tops_of_sequences(); // marks only the highest of a sequence

        let absolute = tops.absolute_ranks_given_played_ranks(played_cards);

        absolute.all_contained_ranks()
    }

    pub fn only_tops_of_sequences(self) -> Self {
        let field = self.0;
        Self::from_u16(!(field >> 1) & field)
    }

    fn absolute_ranks_given_played_ranks(&self, played: &Self) -> Self {
        let relative = self.0;
        let played = played.0;

        let mut abs = 0u16;

        let mut index = 0;

        while index < 16 {
            if played & (1 << index) == 0 {
                let shifted = played >> index;
                let pop_count = shifted.count_ones();

                if relative & (1 << (index + pop_count)) != 0 {
                    abs |= 1 << index
                }
            }
            index += 1;
        }
        Self(abs)
    }

    fn relative_ranks_given_played_ranks(&self, played: &Self) -> Self {
        let absolute = self.0;
        let played = played.0;

        let mut ranks = 0u16;

        for index in 0..16 {
            let cursor = 1 << index;
            if absolute & cursor != 0 {
                let shifted = played >> index;
                let pop_count = shifted.count_ones();
                let rank_index = index + pop_count;
                ranks |= 1 << rank_index;
            }
        }

        Self(ranks)
    }

    #[allow(dead_code)]
    pub fn union(&self, other: &Self) -> Self {
        let new = self.0 | other.0;
        Self(new)
    }

    #[allow(dead_code)]
    pub fn highest_rank(&self) -> Option<Rank> {
        Rank::iter().rev().find(|&rank| self.0 & Self::u16_from_rank(rank) != 0)
    }

    #[allow(dead_code)]
    pub fn lowest_rank(&self) -> Option<Rank> {
        Rank::iter().find(|&rank| self.0 & Self::u16_from_rank(rank) != 0)
    }

    #[allow(dead_code)]
    pub fn win_ranks(&self, least_win: Rank) -> Self {
        let mask = Self::u16_from_rank(least_win) - 1;
        let new = self.0 & !mask;
        Self(new)
    }

    pub fn all_lower_than(rank: Rank) -> Self {
        let mask = Self::u16_from_rank(rank) - 1;
        Self(mask)
    }

    pub fn all_higher_than(rank: Rank) -> Self {
        let mask = 2 * Self::u16_from_rank(rank) - 1;
        let mask = Self::ALL_RANKS & !mask;
        Self(mask)
    }

    pub fn lowest_relative_rank(&self) -> Option<Rank> {
        match self.0.count_ones() {
            0 => None,
            i if i < 13 => Some(Rank::from(13 - i as u16)),
            _ => unreachable!(),
        }
    }

    pub fn win_rank_mask(&self) -> u32 {
        let all_set = (1u32 << (N_RANKS * 2)) - 1;
        all_set.bitxor(all_set >> self.0.count_ones())
    }

    pub const ALL_RANKS: u16 = 0b0001_1111_1111_1111;
}

// pub struct HighCard {
//     rank: Rank,
//     player: Seat,
// }

#[cfg(test)]
mod test {
    use super::{TO_ABSOLUTE_GIVEN_PLAYED, TO_RELATIVE_GIVEN_PLAYED};

    use crate::dds::card_manager::relative_rank::RelativeRank;
    use crate::dds::card_manager::suit_field::SuitField;
    use crate::primitives::card::Rank;
    use test_case::test_case;

    #[test_case(0b0000_0011_0000_1000, 0b0000_1100_0110_0110, 0b0000_1100_1000_0000)]
    #[test_case(0b0000_0011_0000_1001, 0b0000_1100_0110_0110, 0b0000_1100_1100_0000)]
    #[test_case(0b0000_0011_1001_0110, 0b0001_1000_0000_1001, 0b0000_1110_0111_0000)]
    fn rank_field(my_field: u16, played_field: u16, expected: u16) {
        let card_tracker = SuitField::from_u16(my_field);

        let played = SuitField::from_u16(played_field);

        let expected = SuitField::from_u16(expected);

        assert_eq!(card_tracker.relative_ranks_given_played_ranks(&played), expected)
    }

    #[test_case(0b0000_0011_0000_1000, Some(Rank::Five))]
    #[test_case(0b0000_0000_0000_0000, None)]
    #[test_case(0b0000_0011_1000_0000, Some(Rank::Nine))]
    fn lowest_rank(my_field: u16, expected: Option<Rank>) {
        let suit_field = SuitField::from_u16(my_field);
        assert_eq!(suit_field.lowest_rank(), expected);
    }

    #[test_case(0b0000_0011_0000_1000, Some(Rank::Jack))]
    #[test_case(0b0000_1011_0000_1000, Some(Rank::King))]
    #[test_case(0b0000_0000_0000_1000, Some(Rank::Five))]
    fn highest_rank(my_field: u16, expected: Option<Rank>) {
        let suit_field = SuitField::from_u16(my_field);
        assert_eq!(suit_field.highest_rank(), expected);
    }

    #[test_case(0b0000_0011_0000_1000, Rank::Two, 0b0000_0011_0000_1000)]
    #[test_case(0b0000_0011_0000_1000, Rank::Three, 0b0000_0011_0000_1000)]
    #[test_case(0b0000_0011_0000_1000, Rank::Four, 0b0000_0011_0000_1000)]
    #[test_case(0b0000_0011_0000_1000, Rank::Five, 0b0000_0011_0000_1000)]
    #[test_case(0b0000_0011_0000_1000, Rank::Ace, 0b0000_0000_0000_0000)]
    fn win_ranks(my_field: u16, win_ranks: Rank, expected: u16) {
        let suit_field = SuitField::from_u16(my_field);
        assert_eq!(suit_field.win_ranks(win_ranks), SuitField::from_u16(expected));
    }

    #[test_case(Rank::Two, 0b0001_1111_1111_1110)]
    #[test_case(Rank::Three, 0b0001_1111_1111_1100)]
    #[test_case(Rank::Ace, 0b0000_0000_0000_0000)]
    fn all_higher_than(rank: Rank, expected: u16) {
        assert_eq!(SuitField::all_higher_than(rank), SuitField::from_u16(expected));
    }

    #[test_case(Rank::Two, 0b0000_0000_0000_0000)]
    #[test_case(Rank::Seven, 0b0000_0000_0001_1111)]
    #[test_case(Rank::Ace, 0b0000_1111_1111_1111)]
    fn all_lower_than(rank: Rank, expected: u16) {
        assert_eq!(SuitField::all_lower_than(rank), SuitField::from_u16(expected));
    }

    #[test_case(Rank::Two, 0b0000_0011_0000_1000, RelativeRank::Five)]
    #[test_case(Rank::Two, 0b0000_0011_0100_1000, RelativeRank::Six)]
    #[test_case(Rank::Two, 0b0000_0011_0100_1001, RelativeRank::OutOfPlay)]
    fn relative_given_played(rank: Rank, played: u16, expected: RelativeRank) {
        let relative = TO_RELATIVE_GIVEN_PLAYED[played as usize][rank as usize];
        assert_eq!(relative, expected)
    }

    #[test_case(RelativeRank::Five, 0b0000_0011_0000_1000, Some(Rank::Two))]
    #[test_case(RelativeRank::Six, 0b0000_0011_0100_1000, Some(Rank::Two))]
    #[test_case(RelativeRank::Jack, 0b0000_0011_0100_1001, Some(Rank::Nine))]
    fn absolute_given_played(rank: RelativeRank, played: u16, expected: Option<Rank>) {
        let relative = TO_ABSOLUTE_GIVEN_PLAYED[played as usize][rank as usize];
        assert_eq!(relative, expected)
    }
}
