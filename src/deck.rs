use crate::card::*;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let cards = Vec::<Card>::from_iter(
            Suit::iter()
                .cartesian_product(Denomination::iter())
                .map(|(suit, denomination)| Card { suit, denomination }),
        );
        assert_eq!(cards.len(), 52);
        Deck { cards }
    }

    fn shuffle(&mut self) -> () {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn shuffled(mut self) -> Self {
        self.shuffle();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, Denomination, Suit};
    use crate::deck::*;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(
            deck.cards.iter().nth(1).unwrap(),
            &Card {
                suit: Suit::Clubs,
                denomination: Denomination::Three
            }
        );
        assert_eq!(
            deck.cards.iter().nth(13).unwrap(),
            &Card {
                suit: Suit::Diamonds,
                denomination: Denomination::Two
            }
        );
        assert_eq!(
            deck.cards.iter().nth(17).unwrap(),
            &Card {
                suit: Suit::Diamonds,
                denomination: Denomination::Six
            }
        );
        assert_eq!(
            deck.cards.iter().nth(32).unwrap(),
            &Card {
                suit: Suit::Hearts,
                denomination: Denomination::Eight
            }
        );
        assert_eq!(
            deck.cards.iter().nth(48).unwrap(),
            &Card {
                suit: Suit::Spades,
                denomination: Denomination::Jack
            }
        );
    }
    #[test]
    fn test_deck_shuffle() {
        let mut deck = Deck::new();
        deck.shuffle();
        assert_ne!(
            deck.cards.iter().nth(48).unwrap(),
            &Card {
                suit: Suit::Spades,
                denomination: Denomination::Jack // this fails randomly, consider adding a preseed to rng
            }
        );
    }
}
