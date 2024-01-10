use crate::primitives::card::Denomination;
use crate::primitives::{Card, Hand, Suit};
use itertools::Itertools;
use rand::prelude::*;
use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;

pub struct Deck<const N: usize> {
    cards: [[Card; N]; 4],
}

impl<const N: usize> Deck<N> {
    pub fn new() -> Self {
        assert!(N <= 13, "Cannot create Decks with more than thirteen cards per suit!");
        let mut cards = Vec::<Card>::from_iter(
            Suit::iter()
                .cartesian_product(Denomination::iter().rev().take(N))
                .map(|(suit, denomination)| Card { suit, denomination }),
        );

        cards.sort_unstable();
        let cards = Self::array_of_arrays_from_vec(cards);

        Deck { cards }
    }

    pub fn shuffled() -> Self {
        let mut deck = Self::new();
        deck.shuffle_with_rng(&mut thread_rng());
        deck
    }

    pub fn shuffled_from_rng(rng: &mut impl Rng) -> Self {
        let mut deck = Self::new();
        deck.shuffle_with_rng(rng);
        deck
    }

    pub fn sort(&mut self) {
        let mut cards = self.cards.iter().flatten().copied().collect_vec();
        cards.sort_unstable();

        self.cards = Self::array_of_arrays_from_vec(cards);
    }

    fn array_of_arrays_from_vec(vec: Vec<Card>) -> [[Card; N]; 4] {
        vec.chunks(N)
            .map(|hand| <[_; N]>::try_from(hand).unwrap())
            .collect_vec()
            .try_into()
            .unwrap()
    }

    fn shuffle_with_rng(&mut self, rng: &mut impl Rng) {
        let mut cards = self.cards.iter().flatten().copied().collect_vec();
        cards.shuffle(rng);

        self.cards = Self::array_of_arrays_from_vec(cards);
    }

    pub fn cards(&self) -> Vec<Card> {
        self.cards.iter().flatten().copied().collect_vec()
    }
}

impl<const N: usize> Deck<N> {
    pub fn deal(&self) -> [Hand<N>; 4] {
        self.cards
            .iter()
            .map(|x| Hand::<N>::from_cards(x).unwrap())
            .collect::<Vec<Hand<N>>>()
            .try_into()
            .unwrap()
    }
}

impl<const N: usize> Default for Deck<N> {
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
        let deck = Deck::<13>::new();
        let cards = deck.cards();
        let first = cards.first().unwrap();
        let last = cards.last().unwrap();
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
        let deck = Deck::<13>::shuffled_from_rng(&mut rng);
        assert_eq!(deck.cards().first().unwrap(), &Card::from_str(first).unwrap());
        assert_eq!(deck.cards().last().unwrap(), &Card::from_str(last).unwrap());
    }
}
