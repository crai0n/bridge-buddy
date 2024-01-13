use crate::primitives::card::Denomination;
use crate::primitives::{Card, Hand, Suit};
use strum::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct CardTracker(u64);
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RelativeTracker(u64);

impl RelativeTracker {
    #[allow(dead_code)]
    pub fn from_u64(field: u64) -> Self {
        Self(field)
    }

    pub fn field(&self) -> u64 {
        self.0
    }
}

impl CardTracker {
    const SUIT_MASKS: [u64; 4] = [0xFFFF, 0xFFFF_0000, 0xFFFF_0000_0000, 0xFFFF_0000_0000_0000];

    pub fn empty() -> Self {
        Self(0u64)
    }

    #[allow(dead_code)]
    pub fn from_u64(field: u64) -> Self {
        Self(field)
    }

    #[allow(dead_code)]
    pub fn union(&self, other: &Self) -> Self {
        let new = self.0 | other.0;
        Self(new)
    }

    #[allow(dead_code)]
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
        self.0 |= Self::u64_from_card(card)
    }

    pub fn remove_card(&mut self, card: Card) {
        self.0 &= !Self::u64_from_card(card)
    }

    #[allow(dead_code)]
    pub fn contains_card(&self, card: Card) -> bool {
        self.0 & Self::u64_from_card(card) != 0
    }

    #[allow(dead_code)]
    pub fn contained_cards(&self) -> Vec<Card> {
        let mut vec = vec![];
        for suit in Suit::iter() {
            for denomination in Denomination::iter() {
                let interesting_bit = 1 << (denomination as usize + 16 * suit as usize);
                if self.0 & interesting_bit != 0 {
                    vec.push(Card { denomination, suit });
                }
            }
        }
        vec
    }

    pub fn all_contained_cards(&self) -> Vec<Card> {
        let mut vec = vec![];
        let mut tracking_field = self.0;

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
        let suit_tracker = self.only_suit(suit);

        suit_tracker.all_contained_cards()
    }

    pub fn field(&self) -> u64 {
        self.0
    }

    pub fn only_tops_of_sequences(self) -> Self {
        let field = self.0;
        let tops = !(field >> 1) & field;

        CardTracker::from_u64(tops)
    }

    pub fn only_suit(self, suit: Suit) -> Self {
        let only_suit = self.0 & Self::SUIT_MASKS[suit as usize];
        CardTracker::from_u64(only_suit)
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
        assert_eq!(tracker.0, 1);
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
