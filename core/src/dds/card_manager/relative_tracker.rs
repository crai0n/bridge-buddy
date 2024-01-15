use crate::dds::card_manager::card_tracker::{CardTracker, SUIT_ARRAY};
use crate::primitives::Suit;
use strum::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RelativeTracker([u16; 4]);

impl RelativeTracker {
    #[allow(dead_code)]
    pub fn from_u16s(val: [u16; 4]) -> Self {
        Self(val)
    }

    #[allow(dead_code)]
    pub fn from_u64(val: u64) -> Self {
        let field = [val as u16, (val >> 16) as u16, (val >> 32) as u16, (val >> 48) as u16];
        Self(field)
    }

    pub fn field(&self) -> u64 {
        SUIT_ARRAY.iter().fold(0u64, |total, suit| {
            total | (*self.suit_state(*suit) as u64) << (*suit as usize * 16)
        })
    }

    pub fn count_high_cards(&self) -> u8 {
        Suit::iter().fold(0, |high_cards, suit| high_cards + self.count_high_cards_in_suit(suit))
    }

    pub fn count_high_cards_in_suit(&self, suit: Suit) -> u8 {
        let mut field = *self.suit_state(suit);
        field <<= 3; // make Ace the leading bit
        field.leading_ones() as u8
    }

    pub fn count_high_cards_per_suit(&self) -> [u8; 4] {
        SUIT_ARRAY.map(|suit| self.count_high_cards_in_suit(suit))
    }

    pub fn suit_state(&self, suit: Suit) -> &u16 {
        &self.0[suit as usize]
    }

    pub fn suit_state_mut(&mut self, suit: Suit) -> &mut u16 {
        &mut self.0[suit as usize]
    }

    pub fn absolute_cards_given_played_cards(self, played: &CardTracker) -> RelativeTracker {
        let mut ranks = 0u64;

        for suit in Suit::iter() {
            let my_field = self.suit_state(suit);
            let played_field = played.suit_state(suit);

            for index in 0..16 {
                let cursor = 1 << index;
                if my_field & cursor != 0 {
                    let shifted = played_field >> index;
                    let pop_count = shifted.count_ones();
                    let rank_index = index + pop_count;
                    ranks |= 1 << rank_index
                }
            }
        }
        RelativeTracker::from_u64(ranks)
    }
}
