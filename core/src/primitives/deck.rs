use crate::primitives::card::{Card, Denomination, Suit};
use crate::primitives::Hand;
use itertools::Itertools;
use rand::prelude::*;
use strum::IntoEnumIterator;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let cards = Vec::<Card>::from_iter(
            Suit::iter()
                .cartesian_product(Denomination::iter())
                .map(|(suit, denomination)| Card { suit, denomination }),
        );
        assert_eq!(cards.len(), 52);
        Deck { cards }
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

    pub fn deal(mut self) -> [Hand; 4] {
        let hands_vec = (0..4).map(|_| self.deal_single_hand()).collect::<Vec<Hand>>();
        assert!(self.cards.is_empty());
        hands_vec.try_into().unwrap()
    }

    fn deal_single_hand(&mut self) -> Hand {
        let len = self.cards.len();
        let cards = self.cards.split_off(len - 13);
        Hand::from_cards(&cards).unwrap()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck::new()
    }
}
