use crate::primitives::card::Denomination;
use crate::primitives::{Card, Hand, Suit};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct CardTracker {
    tracking_field: u64,
}

// TODO: It would probably be nice to make a "relative" version of cardTracker, so that we can transform
// an absolute cardtracker to a relative one by calling something like relative_ranks(played_cards:) -> CardTracker {..}

impl CardTracker {
    pub fn empty() -> Self {
        Self { tracking_field: 0u64 }
    }

    pub fn from_field(tracking_field: u64) -> Self {
        Self { tracking_field }
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            tracking_field: self.tracking_field | other.tracking_field,
        }
    }

    pub fn from_cards(cards: &[Card]) -> Self {
        let mut tracker = Self::empty();

        for &card in cards {
            tracker.add_card(card)
        }
        tracker
    }

    pub fn from_hand<const N: usize>(hand: Hand<N>) -> Self {
        let mut tracker = Self::empty();

        for &card in hand.cards() {
            tracker.add_card(card)
        }

        tracker
    }

    const fn u64_from_card(card: Card) -> u64 {
        1 << (card.denomination as usize + 16 * card.suit as usize)
    }

    pub fn add_card(&mut self, card: Card) {
        self.tracking_field |= Self::u64_from_card(card)
    }

    pub fn remove_card(&mut self, card: Card) {
        self.tracking_field &= !Self::u64_from_card(card)
    }

    pub fn contains_card(&self, card: Card) -> bool {
        self.tracking_field & Self::u64_from_card(card) != 0
    }

    pub fn contained_cards(&self) -> Vec<Card> {
        let mut vec = vec![];
        for suit in Suit::iter() {
            for denomination in Denomination::iter() {
                let interesting_bit = 1 << (denomination as usize + 16 * suit as usize);
                if self.tracking_field & interesting_bit != 0 {
                    vec.push(Card { denomination, suit });
                }
            }
        }
        vec
    }

    pub fn all_contained_cards(&self) -> Vec<Card> {
        let mut vec = vec![];
        let mut tracking_field = self.tracking_field;

        while tracking_field != 0 {
            let lowest_bit = tracking_field & (!tracking_field + 1);
            tracking_field &= !lowest_bit;
            let index = lowest_bit.ilog2();
            let suit = Suit::from((index / 16) as u16);
            let denomination = Denomination::from((index % 16) as u16);
            vec.push(Card { suit, denomination })
        }

        vec
    }

    pub fn contained_cards_in_suit(&self, suit: Suit) -> Vec<Card> {
        let mut vec = vec![];
        let field = self.suit_state(suit);
        let mut interesting_bit = 1u16;
        for index in 0u8..16 {
            if field & interesting_bit != 0 {
                let denomination = Denomination::from((index % 16) as u16);
                vec.push(Card { suit, denomination })
            }
            interesting_bit <<= 1;
        }
        vec
    }

    pub fn field(&self) -> u64 {
        self.tracking_field
    }

    pub fn tops_of_sequences_field(&self) -> u64 {
        let field = self.tracking_field;

        !(field >> 1) & field
    }

    pub fn suit_state(&self, suit: Suit) -> u16 {
        (self.tracking_field >> (16 * suit as usize)) as u16
    }
}

#[cfg(test)]
mod test {
    use crate::dds::card_tracker::CardTracker;
    use crate::primitives::{Card, Hand, Suit};
    use itertools::Itertools;
    use std::str::FromStr;
    use test_case::test_case;

    #[test]
    fn card_tracker() {
        let mut tracker = CardTracker::empty();

        tracker.add_card(Card::from_str("C2").unwrap());
        assert_eq!(tracker.tracking_field, 1);
    }

    #[test_case("H:AQ,C:AQJ", Suit::Hearts)]
    #[test_case("H:AKQ,D:AT", Suit::Diamonds)]
    #[test_case("H:AKQT,S:T", Suit::Spades)]
    fn contained_cards_in_suit(hand_str: &str, suit: Suit) {
        let hand = Hand::<5>::from_str(hand_str).unwrap();
        let tracker = CardTracker::from_hand(hand);
        assert_eq!(tracker.contained_cards(), hand.cards().copied().collect_vec());
        assert_eq!(
            tracker.contained_cards_in_suit(suit),
            hand.cards_in(suit).copied().collect_vec()
        )
    }
}
