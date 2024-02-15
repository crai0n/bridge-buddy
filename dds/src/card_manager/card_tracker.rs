use super::suit_field::SuitField;

use bridge_buddy_core::primitives::{Card, Hand, Suit};

use bridge_buddy_core::primitives::card::suit::SUIT_ARRAY;
use bridge_buddy_core::primitives::card::Rank;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CardTracker([SuitField; 4]);

impl CardTracker {
    pub fn suit_state(&self, suit: Suit) -> &SuitField {
        &self.0[suit as usize]
    }

    pub fn has_higher_cards_in_suit_than(&self, suit: Suit, other: &Self) -> bool {
        self.suit_state(suit)
            .has_higher_ranks_than_other(other.suit_state(suit))
    }

    pub fn suit_state_mut(&mut self, suit: Suit) -> &mut SuitField {
        &mut self.0[suit as usize]
    }

    pub fn empty() -> Self {
        Self([SuitField::empty(); 4])
    }

    #[allow(dead_code)]
    pub fn from_u16s(val: [u16; 4]) -> Self {
        let inner = val.map(SuitField::from_u16);
        Self(inner)
    }

    #[allow(dead_code)]
    pub fn from_suit_fields(fields: [SuitField; 4]) -> Self {
        Self(fields)
    }

    #[allow(dead_code)]
    pub fn from_u64(val: u64) -> Self {
        let field = [val as u16, (val >> 16) as u16, (val >> 32) as u16, (val >> 48) as u16];
        Self::from_u16s(field)
    }

    pub fn for_n_cards_per_suit(n: usize) -> Self {
        Self([SuitField::for_n_cards_per_suit(n); 4])
    }

    #[allow(dead_code)]
    pub fn union(&self, other: &Self) -> Self {
        let new = SUIT_ARRAY.map(|suit| self.suit_state(suit).union(other.suit_state(suit)));
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

    pub fn count_cards_in(&self, suit: Suit) -> usize {
        self.suit_state(suit).count_cards()
    }

    #[allow(dead_code)]
    pub fn count_cards(&self) -> usize {
        SUIT_ARRAY
            .into_iter()
            .fold(0, |result, suit| self.suit_state(suit).count_cards() + result)
    }

    pub fn is_void_in(&self, suit: Suit) -> bool {
        self.suit_state(suit).is_void()
    }

    pub fn has_cards_in(&self, suit: Suit) -> bool {
        !self.is_void_in(suit)
    }

    pub fn has_singleton_in(&self, suit: Suit) -> bool {
        self.count_cards_in(suit) == 1
    }

    pub fn has_doubleton_in(&self, suit: Suit) -> bool {
        self.count_cards_in(suit) == 2
    }

    pub fn count_cards_per_suit(&self) -> [usize; 4] {
        self.0.map(|suit| suit.count_cards())
    }

    pub fn add_card(&mut self, card: Card) {
        self.suit_state_mut(card.suit).add_rank(card.rank)
    }

    pub fn remove_card(&mut self, card: Card) {
        self.suit_state_mut(card.suit).remove_rank(card.rank)
    }

    #[allow(dead_code)]
    pub fn contains(&self, card: &Card) -> bool {
        self.suit_state(card.suit).contains_rank(&card.rank)
    }

    #[allow(dead_code)]
    pub fn contains_in(&self, rank: &Rank, suit: Suit) -> bool {
        self.suit_state(suit).contains_rank(rank)
    }

    pub fn all_cards(&self) -> impl DoubleEndedIterator<Item = Card> + '_ {
        SUIT_ARRAY
            .into_iter()
            .flat_map(|suit| self.suit_state(suit).iter().map(move |rank| Card { suit, rank }))
    }

    pub fn cards_in(&self, suit: Suit) -> impl DoubleEndedIterator<Item = Card> + '_ {
        self.ranks_in(suit).map(move |rank| Card { suit, rank })
    }

    pub fn ranks_in(&self, suit: Suit) -> impl DoubleEndedIterator<Item = Rank> + '_ {
        self.suit_state(suit).into_iter()
    }

    pub fn highest_card_in(&self, suit: Suit) -> Option<Card> {
        self.highest_rank_in(suit).map(|rank| Card { suit, rank })
    }

    pub fn highest_rank_in(&self, suit: Suit) -> Option<Rank> {
        self.suit_state(suit).highest_rank()
    }

    #[allow(dead_code)]
    pub fn lowest_card_in(&self, suit: Suit) -> Option<Card> {
        self.lowest_rank_in(suit).map(|rank| Card { suit, rank })
    }

    #[allow(dead_code)]
    pub fn lowest_rank_in(&self, suit: Suit) -> Option<Rank> {
        self.suit_state(suit).lowest_rank()
    }

    #[allow(dead_code)]
    pub fn count_cards_lower_than(&self, card: Card) -> usize {
        self.suit_state(card.suit).cards_lower_than(card.rank).count_cards()
    }

    #[allow(dead_code)]
    pub fn count_cards_higher_than(&self, card: Card) -> usize {
        self.suit_state(card.suit).cards_higher_than(card.rank).count_cards()
    }
}

#[cfg(test)]
mod test {
    use super::CardTracker;

    use bridge_buddy_core::primitives::{Card, Hand, Suit};
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

    #[test_case("H:AQ,C:AQJ", Suit::Hearts)]
    #[test_case("H:AKQ,D:AT", Suit::Diamonds)]
    #[test_case("H:AKQT,S:T", Suit::Spades)]
    fn contained_cards_in_suit(hand_str: &str, suit: Suit) {
        let hand = Hand::<5>::from_str(hand_str).unwrap();
        let tracker = CardTracker::from_hand(hand);
        assert_eq!(tracker.all_cards().collect_vec(), hand.cards().copied().collect_vec());
        assert_eq!(
            tracker.cards_in(suit).collect_vec(),
            hand.cards_in(suit).copied().collect_vec()
        )
    }
}
