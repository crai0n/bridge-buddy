use bridge_buddy_core::primitives::card::rank::N_RANKS;
use bridge_buddy_core::primitives::card::Rank;

use std::ops::BitXor;
use strum::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SuitField(pub(crate) u16);

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

    pub const fn u16_from_rank(rank: Rank) -> u16 {
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

    pub fn count_cards(&self) -> usize {
        self.0.count_ones() as usize
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
    #[allow(dead_code)]
    pub fn cards_lower_than(&self, rank: Rank) -> Self {
        let mask = 2 * Self::u16_from_rank(rank) - 1;
        Self(mask & self.0)
    }

    pub fn all_higher_than(rank: Rank) -> Self {
        let mask = 2 * Self::u16_from_rank(rank) - 1;
        let mask = Self::ALL_RANKS & !mask;
        Self(mask)
    }

    #[allow(dead_code)]
    pub fn cards_higher_than(&self, rank: Rank) -> Self {
        let mask = 2 * Self::u16_from_rank(rank) - 1;
        let mask = Self::ALL_RANKS & !mask;
        Self(mask & self.0)
    }

    pub fn win_rank_mask(&self) -> u32 {
        let all_set = (1u32 << (N_RANKS * 2)) - 1;
        all_set.bitxor(all_set >> self.0.count_ones())
    }

    pub const ALL_RANKS: u16 = 0b0001_1111_1111_1111;

    pub fn has_higher_ranks_than_other(&self, other: &Self) -> bool {
        self.0 > other.0
    }
}

// pub struct HighCard {
//     rank: Rank,
//     player: Seat,
// }

#[cfg(test)]
mod test {

    use super::SuitField;
    use bridge_buddy_core::primitives::card::Rank;
    use test_case::test_case;

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
}
