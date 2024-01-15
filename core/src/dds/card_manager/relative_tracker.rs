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

    #[allow(dead_code)]
    pub fn field(&self) -> u64 {
        SUIT_ARRAY.iter().fold(0u64, |total, suit| {
            total | (*self.suit_state(*suit) as u64) << (*suit as usize * 16)
        })
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn suit_state_mut(&mut self, suit: Suit) -> &mut u16 {
        &mut self.0[suit as usize]
    }

    pub fn only_tops_of_sequences(self) -> Self {
        let tops = self.0.map(|field| !(field >> 1) & field);

        Self::from_u16s(tops)
    }

    #[allow(dead_code)]
    pub fn absolute_cards_given_played_cards(&self, played: &CardTracker) -> CardTracker {
        let fields = SUIT_ARRAY.map(|suit| {
            let relative = *self.suit_state(suit);
            let played = *played.suit_state(suit);

            Self::absolute_denominations_given_played_denominations(relative, played)
        });

        CardTracker::from_u16s(fields)
    }

    fn absolute_denominations_given_played_denominations(relative: u16, played: u16) -> u16 {
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
        abs
    }
}
