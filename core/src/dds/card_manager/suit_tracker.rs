use crate::primitives::card::Denomination;

pub struct SuitTracker(u16);

#[allow(dead_code)]
impl SuitTracker {
    pub fn empty() -> Self {
        Self(0u16)
    }

    pub fn for_n_cards_per_suit(n: usize) -> Self {
        let mask = (1u16 << (13 - n)) - 1;
        Self(mask)
    }

    pub fn add_rank(&mut self, rank: Denomination) {
        self.0 |= rank as u16;
    }

    // pub fn promote_cards_based_on_played_cards(self, played: &SuitTracker) -> RelativeTracker {
    //     let my_field = self.field();
    //     let played_field = played.field();
    //
    //     let mut ranks = 0u64;
    //
    //     for suit_index in 0..4 {
    //         for index in 0..16 {
    //             let cursor = 1 << index << (suit_index * 16);
    //             if my_field & cursor != 0 {
    //                 let played_field = played_field >> (suit_index * 16);
    //                 let shifted = (played_field as u16) >> index;
    //                 let pop_count = shifted.count_ones();
    //                 let rank_index = index + pop_count;
    //                 ranks |= 1 << rank_index << (suit_index * 16);
    //             }
    //         }
    //     }
    //     RelativeTracker::from_u64(ranks)
    // }

    #[allow(dead_code)]
    pub fn union(&self, other: &Self) -> Self {
        let new = self.0 | other.0;
        Self(new)
    }
}
