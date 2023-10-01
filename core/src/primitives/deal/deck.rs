use crate::primitives::card::{Card, Denomination, Suit};
use crate::primitives::Hand;
use itertools::Itertools;
use rand::prelude::*;
use strum::IntoEnumIterator;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    const NUM_CARDS: usize = 52;
    pub fn new() -> Self {
        let cards = Vec::<Card>::from_iter(
            Suit::iter()
                .cartesian_product(Denomination::iter())
                .map(|(suit, denomination)| Card { suit, denomination }),
        );
        assert_eq!(cards.len(), Self::NUM_CARDS);
        Deck { cards }
    }

    pub fn shuffled() -> Self {
        let mut deck = Self::new();
        deck.shuffle();
        deck
    }

    pub fn shuffled_from_rng(rng: &mut impl Rng) -> Self {
        let mut deck = Self::new();
        deck.shuffle_with_rng(rng);
        deck
    }

    pub fn sort(&mut self) {
        self.cards.sort_unstable()
    }

    pub fn shuffle(&mut self) {
        self.shuffle_with_rng(&mut thread_rng())
    }

    pub fn shuffle_with_rng(&mut self, rng: &mut impl Rng) {
        self.cards.shuffle(rng)
    }

    pub fn deal(&self) -> [Hand; 4] {
        self.cards
            .chunks(Self::NUM_CARDS / 4)
            .map(|x| Hand::from_cards(x).unwrap())
            .collect::<Vec<Hand>>()
            .try_into()
            .unwrap()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck::new()
    }
}

#[cfg(test)]
mod test {
    use super::Deck;
    use crate::primitives::Card;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;
    use std::str::FromStr;
    use test_case::test_case;

    #[test]
    fn sorted() {
        let deck = Deck::default();
        let first = deck.cards.first().unwrap();
        let last = deck.cards.last().unwrap();
        assert_eq!(first, &Card::from_str("C2").unwrap());
        assert_eq!(last, &Card::from_str("SA").unwrap());
    }

    #[test_case(  1u64, "S4", "H4"; "Four of Spades and Four of Hearts")]
    #[test_case(  2u64, "D9", "CQ"; "Nine of Diamonds and Queen of Clubs")]
    #[test_case(  3u64, "SK", "C7"; "King of Spades and Seven of Clubs")]
    #[test_case(  4u64, "S2", "SA"; "Two of Spades and Ace of Spades")]
    #[test_case(  5u64, "H6", "CQ"; "Six of Hearts and Queen of Clubs")]
    fn determinism(seed: u64, first: &str, last: &str) {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let deck = Deck::shuffled_from_rng(&mut rng);
        assert_eq!(deck.cards.first().unwrap(), &Card::from_str(first).unwrap());
        assert_eq!(deck.cards.last().unwrap(), &Card::from_str(last).unwrap());
    }
}
