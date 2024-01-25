use crate::dds::card_manager::suit_field::SuitField;

use crate::primitives::{Card, Hand, Suit};
use itertools::Itertools;
use std::fmt::Debug;
use strum::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct CardTracker([SuitField; 4]);

pub const SUIT_ARRAY: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

impl CardTracker {
    pub fn suit_state(&self, suit: &Suit) -> &SuitField {
        &self.0[*suit as usize]
    }

    pub fn suit_state_mut(&mut self, suit: &Suit) -> &mut SuitField {
        &mut self.0[*suit as usize]
    }

    pub fn empty() -> Self {
        Self([SuitField::empty(); 4])
    }

    #[allow(dead_code)]
    pub fn from_u16s(val: [u16; 4]) -> Self {
        let inner = val.map(SuitField::from_u16);
        Self(inner)
    }

    pub fn from_suit_fields(fields: [SuitField; 4]) -> Self {
        Self(fields)
    }

    pub fn from_u64(val: u64) -> Self {
        let field = [val as u16, (val >> 16) as u16, (val >> 32) as u16, (val >> 48) as u16];
        Self::from_u16s(field)
    }

    pub fn for_n_cards_per_suit(n: usize) -> Self {
        Self([SuitField::for_n_cards_per_suit(n); 4])
    }

    #[allow(dead_code)]
    pub fn union(&self, other: &Self) -> Self {
        let new = SUIT_ARRAY.map(|suit| self.suit_state(&suit).union(other.suit_state(&suit)));
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

    #[allow(dead_code)]
    pub fn count_cards_in_suit(&self, suit: &Suit) -> u8 {
        self.suit_state(suit).count_cards()
    }

    pub fn cards_per_suit(&self) -> [u8; 4] {
        self.0.map(|suit| suit.count_cards())
    }

    pub fn add_card(&mut self, card: Card) {
        self.suit_state_mut(&card.suit).add_rank(card.rank)
    }

    pub fn remove_card(&mut self, card: Card) {
        self.suit_state_mut(&card.suit).remove_rank(card.rank)
    }

    #[allow(dead_code)]
    pub fn contains_card(&self, card: &Card) -> bool {
        self.suit_state(&card.suit).contains_rank(card.rank)
    }

    pub fn all_contained_cards(&self) -> Vec<Card> {
        Suit::iter()
            .flat_map(|suit| {
                self.suit_state(&suit)
                    .all_contained_ranks()
                    .into_iter()
                    .map(move |rank| Card { suit, rank })
            })
            .collect_vec()
    }

    pub fn contained_cards_in_suit(&self, suit: &Suit) -> Vec<Card> {
        self.suit_state(suit)
            .all_contained_ranks()
            .iter()
            .map(|&rank| Card { suit: *suit, rank })
            .collect_vec()
    }

    pub fn only_tops_of_sequences(self) -> Self {
        let fields = self.0.map(|field| field.only_tops_of_sequences());
        Self::from_suit_fields(fields)
    }

    pub fn non_equivalent_moves(&self, played_cards: &CardTracker) -> Vec<Card> {
        Suit::iter()
            .flat_map(|suit| {
                self.suit_state(&suit)
                    .non_equivalent_moves(played_cards.suit_state(&suit))
                    .into_iter()
                    .map(move |rank| Card { suit, rank })
            })
            .collect_vec()
    }

    pub fn count_high_cards_per_suit_given_played_cards(&self, played_cards: &CardTracker) -> [u8; 4] {
        SUIT_ARRAY.map(|suit| {
            self.suit_state(&suit)
                .count_high_cards_given_played_cards(played_cards.suit_state(&suit))
        })
    }
}

#[cfg(test)]
mod test {
    use crate::dds::card_manager::card_tracker::CardTracker;

    use crate::primitives::{Card, Hand, Suit};
    use itertools::Itertools;
    use std::str::FromStr;
    use test_case::test_case;

    #[test]
    fn card_tracker() {
        let mut tracker = CardTracker::empty();

        tracker.add_card(Card::from_str("C2").unwrap());
        assert_eq!(tracker, CardTracker::from_u16s([1, 0, 0, 0]));
    }

    #[test_case(
        13,
        0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000
    )]
    #[test_case(
        12,
        0b0000_0000_0000_0001_0000_0000_0000_0001_0000_0000_0000_0001_0000_0000_0000_0001
    )]
    #[test_case(
        11,
        0b0000_0000_0000_0011_0000_0000_0000_0011_0000_0000_0000_0011_0000_0000_0000_0011
    )]
    #[test_case(
        10,
        0b0000_0000_0000_0111_0000_0000_0000_0111_0000_0000_0000_0111_0000_0000_0000_0111
    )]
    #[test_case(1, 0b0000_1111_1111_1111_0000_1111_1111_1111_0000_1111_1111_1111_0000_1111_1111_1111)]
    fn n_cards_per_suit(n: usize, expected: u64) {
        let tracker = CardTracker::for_n_cards_per_suit(n);

        assert_eq!(tracker, CardTracker::from_u64(expected));
    }

    #[test_case("H:AQ,C:AQJ", &Suit::Hearts)]
    #[test_case("H:AKQ,D:AT", &Suit::Diamonds)]
    #[test_case("H:AKQT,S:T", &Suit::Spades)]
    fn contained_cards_in_suit(hand_str: &str, suit: &Suit) {
        let hand = Hand::<5>::from_str(hand_str).unwrap();
        let tracker = CardTracker::from_hand(hand);
        assert_eq!(tracker.all_contained_cards(), hand.cards().copied().collect_vec());
        assert_eq!(
            tracker.contained_cards_in_suit(suit),
            hand.cards_in(*suit).copied().collect_vec()
        )
    }
}
