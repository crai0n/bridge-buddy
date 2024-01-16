use crate::primitives::card::Rank;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SuitField(u16);

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
}

#[cfg(test)]
mod test {

    use crate::dds::card_manager::suit_tracker::SuitField;
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
}
