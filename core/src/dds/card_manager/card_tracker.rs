use crate::dds::card_manager::relative_tracker::RelativeTracker;
use crate::primitives::card::Denomination;
use crate::primitives::{Card, Hand, Suit};
use itertools::Itertools;
use std::fmt::Debug;
use strum::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct CardTracker([u16; 4]);

pub const SUIT_ARRAY: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

fn element_wise<const N: usize, T: Copy + Clone + Debug, F>(a: [T; N], b: [T; N], function: F) -> [T; N]
where
    F: Fn(&T, &T) -> T,
{
    a.iter()
        .zip(b.iter())
        .map(|(i, j)| function(i, j))
        .collect_vec()
        .try_into()
        .unwrap()
}

impl CardTracker {
    pub fn suit_state(&self, suit: Suit) -> &u16 {
        &self.0[suit as usize]
    }

    pub fn suit_state_mut(&mut self, suit: Suit) -> &mut u16 {
        &mut self.0[suit as usize]
    }

    pub fn empty() -> Self {
        Self([0u16; 4])
    }

    pub fn for_n_cards_per_suit(n: usize) -> Self {
        let mask = (1u16 << (13 - n)) - 1;
        Self([mask; 4])
    }

    pub fn relative_cards_given_played_cards(self, played: &CardTracker) -> RelativeTracker {
        let fields = SUIT_ARRAY.map(|suit| {
            let absolute = *self.suit_state(suit);
            let played = *played.suit_state(suit);

            Self::relative_ranks_given_played_denominations(absolute, played)
        });

        RelativeTracker::from_u16s(fields)
    }

    fn relative_ranks_given_played_denominations(absolute: u16, played: u16) -> u16 {
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

        ranks
    }

    #[allow(dead_code)]
    pub fn from_u64(val: u64) -> Self {
        let field = [val as u16, (val >> 16) as u16, (val >> 32) as u16, (val >> 48) as u16];
        Self(field)
    }

    #[allow(dead_code)]
    pub fn from_u16s(val: [u16; 4]) -> Self {
        Self(val)
    }

    #[allow(dead_code)]
    pub fn union(&self, other: &Self) -> Self {
        let new = element_wise(self.0, other.0, |a, b| a | b);

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
    pub fn count_cards_in_suit(&self, suit: Suit) -> u8 {
        self.suit_state(suit).count_ones() as u8
    }

    pub fn cards_per_suit(&self) -> [u8; 4] {
        self.0.map(|field| field.count_ones() as u8)
    }

    const fn u64_from_card(card: Card) -> u64 {
        1 << (card.denomination as usize + 16 * card.suit as usize)
    }

    const fn u16_from_card(card: Card) -> u16 {
        1 << (card.denomination as usize)
    }

    pub fn add_card(&mut self, card: Card) {
        *self.suit_state_mut(card.suit) |= Self::u16_from_card(card)
    }

    pub fn remove_card(&mut self, card: Card) {
        *self.suit_state_mut(card.suit) &= !Self::u16_from_card(card)
    }

    #[allow(dead_code)]
    pub fn contains_card(&self, card: Card) -> bool {
        self.field() & Self::u64_from_card(card) != 0
    }

    #[allow(dead_code)]
    pub fn contained_cards(&self) -> Vec<Card> {
        let mut vec = vec![];
        for suit in Suit::iter() {
            for denomination in Denomination::iter() {
                let interesting_bit = 1 << (denomination as usize);
                if self.suit_state(suit) & interesting_bit != 0 {
                    vec.push(Card { denomination, suit });
                }
            }
        }
        vec
    }

    pub fn all_contained_cards(&self) -> Vec<Card> {
        let mut vec = vec![];

        for suit in Suit::iter() {
            let mut tracking_field = *self.suit_state(suit);

            while tracking_field != 0 {
                let lowest_bit = tracking_field & (!tracking_field + 1);
                tracking_field &= !lowest_bit;
                let index = lowest_bit.ilog2();
                let denomination = Denomination::from((index % 16) as u16);
                vec.push(Card { suit, denomination })
            }
        }

        vec
    }

    pub fn contained_cards_in_suit(&self, suit: Suit) -> Vec<Card> {
        let mut vec = vec![];

        let mut tracking_field = *self.suit_state(suit);

        while tracking_field != 0 {
            let lowest_bit = tracking_field & (!tracking_field + 1);
            tracking_field &= !lowest_bit;
            let index = lowest_bit.ilog2();
            let denomination = Denomination::from((index % 16) as u16);
            vec.push(Card { suit, denomination })
        }

        vec
    }

    pub fn field(&self) -> u64 {
        SUIT_ARRAY.iter().fold(0u64, |total, suit| {
            total | (*self.suit_state(*suit) as u64) << (*suit as usize * 16)
        })
    }

    pub fn only_tops_of_sequences(self) -> Self {
        let tops = self.0.map(|field| !(field >> 1) & field);

        CardTracker::from_u16s(tops)
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
        assert_eq!(tracker.0, [1, 0, 0, 0]);
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

        assert_eq!(tracker.field(), expected);
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
