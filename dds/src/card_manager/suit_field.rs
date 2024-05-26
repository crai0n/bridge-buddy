use bridge_buddy_core::primitives::card::rank::N_RANKS;
use bridge_buddy_core::primitives::card::Rank;

use std::ops::BitXor;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SuitField(u16);

impl From<SuitField> for u16 {
    fn from(value: SuitField) -> Self {
        value.0
    }
}

#[allow(dead_code)]
impl SuitField {
    pub fn empty() -> Self {
        Self(0u16)
    }

    pub fn masked(&self, mask: u16) -> Self {
        Self(self.0 & mask)
    }

    #[allow(dead_code)]
    pub fn from_u16(val: u16) -> Self {
        Self(val)
    }

    pub fn for_n_cards_per_suit(n: usize) -> Self {
        let mask = (1u16 << (13 - n)) - 1;
        Self(mask)
    }

    pub const fn is_void(&self) -> bool {
        self.0 == 0
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

    pub fn contains_rank(&self, rank: &Rank) -> bool {
        self.0 & Self::u16_from_rank(*rank) != 0
    }

    pub fn count_cards(&self) -> usize {
        self.0.count_ones() as usize
    }

    #[allow(dead_code)]
    pub fn all_contained_ranks(&self) -> Vec<Rank> {
        let mut vec = Vec::with_capacity(self.0.count_ones() as usize);

        let mut tracking_field = self.0;

        while tracking_field != 0 {
            let lowest_bit = Self::lowest_bit(tracking_field);
            tracking_field &= !lowest_bit;
            let rank = Self::u16_to_rank(lowest_bit).unwrap();
            vec.push(rank)
        }

        vec
    }

    #[allow(dead_code)]
    pub fn union(&self, other: &Self) -> Self {
        let new = self.0 | other.0;
        Self(new)
    }

    fn u16_to_rank(input: u16) -> Option<Rank> {
        match input {
            0b0000_0000_0000_0000 => None,
            0b0001_0000_0000_0000 => Some(Rank::Ace),
            0b0000_1000_0000_0000 => Some(Rank::King),
            0b0000_0100_0000_0000 => Some(Rank::Queen),
            0b0000_0010_0000_0000 => Some(Rank::Jack),
            0b0000_0001_0000_0000 => Some(Rank::Ten),
            0b0000_0000_1000_0000 => Some(Rank::Nine),
            0b0000_0000_0100_0000 => Some(Rank::Eight),
            0b0000_0000_0010_0000 => Some(Rank::Seven),
            0b0000_0000_0001_0000 => Some(Rank::Six),
            0b0000_0000_0000_1000 => Some(Rank::Five),
            0b0000_0000_0000_0100 => Some(Rank::Four),
            0b0000_0000_0000_0010 => Some(Rank::Three),
            0b0000_0000_0000_0001 => Some(Rank::Two),
            _ => panic!("Not a valid Rank!"),
        }
    }

    #[allow(dead_code)]
    pub fn highest_rank(&self) -> Option<Rank> {
        let leading_zeros = self.0.leading_zeros();
        match leading_zeros {
            0..=15 => Rank::try_from(15 - leading_zeros).ok(),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn lowest_rank(&self) -> Option<Rank> {
        Self::u16_to_rank(Self::lowest_bit(self.0))
    }

    #[allow(dead_code)]
    pub fn take_lowest_rank(&mut self) -> Option<Rank> {
        let lowest_bit = Self::lowest_bit(self.0);
        self.0 &= !lowest_bit;
        Self::u16_to_rank(lowest_bit)
    }

    #[allow(dead_code)]
    pub fn take_highest_rank(&mut self) -> Option<Rank> {
        match self.0.leading_zeros() {
            0..=15 => {
                let highest_bit_position = 15 - self.0.leading_zeros();
                self.0 &= !(1 << highest_bit_position);
                Rank::try_from(highest_bit_position).ok()
            }
            _ => None,
        }
    }

    fn lowest_bit(val: u16) -> u16 {
        match val {
            0 => 0,
            v => v & (!v + 1),
        }
    }

    #[allow(dead_code)]
    pub fn win_ranks(&self, least_win: Rank) -> Self {
        let mask = Self::u16_from_rank(least_win) - 1;

        self.masked(!mask)
    }

    pub fn all_lower_than(rank: Rank) -> Self {
        let mask = Self::u16_from_rank(rank) - 1;
        Self(mask)
    }
    #[allow(dead_code)]
    pub fn cards_lower_than(&self, rank: Rank) -> Self {
        let mask = 2 * Self::u16_from_rank(rank) - 1;
        self.masked(mask)
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
        self.masked(mask)
    }

    pub fn win_rank_mask(&self) -> u32 {
        let all_set = (1u32 << (N_RANKS * 2)) - 1;
        all_set.bitxor(all_set >> self.0.count_ones())
    }

    pub const ALL_RANKS: u16 = 0b0001_1111_1111_1111;

    pub fn has_higher_ranks_than_other(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    pub fn iter(&self) -> SuitFieldIterator {
        self.into_iter()
    }
}

impl IntoIterator for SuitField {
    type Item = Rank;
    type IntoIter = SuitFieldIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        SuitFieldIntoIterator { suit_field: self }
    }
}

pub struct SuitFieldIntoIterator {
    suit_field: SuitField,
}

impl Iterator for SuitFieldIntoIterator {
    type Item = Rank;
    fn next(&mut self) -> Option<Rank> {
        self.suit_field.take_lowest_rank()
    }
}

impl DoubleEndedIterator for SuitFieldIntoIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.suit_field.take_highest_rank()
    }
}

impl<'a> IntoIterator for &'a SuitField {
    type Item = Rank;
    type IntoIter = SuitFieldIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SuitFieldIterator {
            suit_field: self,
            mask: 0,
        }
    }
}

pub struct SuitFieldIterator<'a> {
    suit_field: &'a SuitField,
    mask: u16,
}

impl<'a> Iterator for SuitFieldIterator<'a> {
    type Item = Rank;

    fn next(&mut self) -> Option<Self::Item> {
        let masked = self.suit_field.masked(!self.mask);
        let lowest_bit = SuitField::lowest_bit(masked.0);
        self.mask |= lowest_bit;
        SuitField::u16_to_rank(lowest_bit)
    }
}

impl<'a> DoubleEndedIterator for SuitFieldIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let masked = self.suit_field.masked(!self.mask);
        match masked.0.leading_zeros() {
            0..=15 => {
                let highest_bit = 1 << (15 - masked.0.leading_zeros());
                self.mask |= highest_bit;
                SuitField::u16_to_rank(highest_bit)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::SuitField;
    use bridge_buddy_core::primitives::card::Rank;

    use test_case::test_case;

    #[test_case(0b0000_0000_0000_0000, 0b0000_0000_0000_0000)]
    #[test_case(0b0000_1000_0000_0000, 0b0000_1000_0000_0000)]
    #[test_case(0b0001_1000_0000_0000, 0b0000_1000_0000_0000)]
    #[test_case(0b0001_1001_0000_0000, 0b0000_0001_0000_0000)]
    #[test_case(0b0001_1001_0000_0100, 0b0000_0000_0000_0100)]
    fn lowest_bit(my_field: u16, expected: u16) {
        assert_eq!(SuitField::lowest_bit(my_field), expected)
    }

    #[test_case(0b0000_0000_0000_0000, None, None)]
    #[test_case(0b0000_1000_0000_0000, Some(Rank::King), None)]
    #[test_case(0b0001_1000_0000_0000, Some(Rank::Ace), Some(Rank::King))]
    #[test_case(0b0000_0001_0000_0100, Some(Rank::Ten), Some(Rank::Four))]
    #[test_case(0b0000_0000_0000_0100, Some(Rank::Four), None)]
    fn take_highest_rank(my_field: u16, expected1: Option<Rank>, expected2: Option<Rank>) {
        let mut suit_field = SuitField::from_u16(my_field);
        assert_eq!(suit_field.take_highest_rank(), expected1);
        assert_eq!(suit_field.take_highest_rank(), expected2);
    }

    #[test_case(0b0000_0000_0000_0000, None)]
    #[test_case(0b0000_1000_0000_0000, Some(Rank::King))]
    #[test_case(0b0001_1000_0000_0000, Some(Rank::Ace))]
    #[test_case(0b0000_0001_0000_0100, Some(Rank::Ten))]
    #[test_case(0b0000_0000_0000_0100, Some(Rank::Four))]
    fn highest_rank(my_field: u16, expected: Option<Rank>) {
        let suit_field = SuitField::from_u16(my_field);
        assert_eq!(suit_field.highest_rank(), expected);
        assert_eq!(suit_field.highest_rank(), expected);
    }

    #[test_case(0b0000_0011_0000_1000, Some(Rank::Five))]
    #[test_case(0b0000_0000_0000_0000, None)]
    #[test_case(0b0000_0011_1000_0000, Some(Rank::Nine))]
    fn lowest_rank(my_field: u16, expected: Option<Rank>) {
        let suit_field = SuitField::from_u16(my_field);
        assert_eq!(suit_field.lowest_rank(), expected);
    }

    #[test_case(0b0000_0011_0000_1010)]
    #[test_case(0b0001_0000_0000_0101)]
    #[test_case(0b0000_0011_1000_0001)]
    #[test_case(0b0001_0011_1000_0010)]
    #[test_case(0b0000_1011_0000_1000)]
    fn into_iterator(my_field: u16) {
        let suit_field = SuitField::from_u16(my_field);
        itertools::assert_equal(suit_field, suit_field.all_contained_ranks())
    }

    #[test_case(0b0000_0011_0000_1010)]
    #[test_case(0b0001_0000_0000_0101)]
    #[test_case(0b0000_0011_1000_0001)]
    #[test_case(0b0001_0011_1000_0010)]
    #[test_case(0b0000_1011_0000_1000)]
    fn iterator(my_field: u16) {
        let suit_field = SuitField::from_u16(my_field);
        itertools::assert_equal(suit_field.iter(), suit_field.all_contained_ranks());
    }

    #[test_case(0b0000_0011_0000_1010)]
    #[test_case(0b0001_0000_0000_0101)]
    #[test_case(0b0000_0011_1000_0001)]
    #[test_case(0b0001_0011_1000_0010)]
    #[test_case(0b0000_1011_0000_1000)]
    fn rev_iterator(my_field: u16) {
        let suit_field = SuitField::from_u16(my_field);
        itertools::assert_equal(
            suit_field.iter().rev(),
            suit_field.all_contained_ranks().into_iter().rev(),
        );
    }

    #[test_case(0b0000_0011_0000_1010)]
    #[test_case(0b0001_0000_0000_0101)]
    #[test_case(0b0000_0011_1000_0001)]
    #[test_case(0b0001_0011_1000_0010)]
    #[test_case(0b0000_1011_0000_1000)]
    fn rev_into_iterator(my_field: u16) {
        let suit_field = SuitField::from_u16(my_field);
        itertools::assert_equal(
            suit_field.into_iter().rev(),
            suit_field.all_contained_ranks().into_iter().rev(),
        )
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
