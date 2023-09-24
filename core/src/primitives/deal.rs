use crate::primitives::board::vulnerability::Vulnerability;
use crate::primitives::board::Board;
use crate::primitives::board::PlayerPosition;
use crate::primitives::{card::Denomination, Card, Hand, Suit};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::{random, thread_rng};
use strum::IntoEnumIterator;

pub struct Deal {
    pub board: Board,
    pub hands: [Hand; 4],
}

impl Deal {
    pub fn new() -> Deal {
        let board_number = (random::<usize>() % 32) + 1;
        Self::new_with_board_number(board_number)
    }

    pub fn new_with_board_number(board_number: usize) -> Deal {
        let mut deck = Deal::shuffled_deck();

        let hands_vec = vec![
            Hand::from_cards(&deck.split_off(39)).unwrap(),
            Hand::from_cards(&deck.split_off(26)).unwrap(),
            Hand::from_cards(&deck.split_off(13)).unwrap(),
            Hand::from_cards(&deck).unwrap(),
        ];

        Deal {
            board: Board::from_number(board_number),
            hands: hands_vec.try_into().unwrap(),
        }
    }

    fn sorted_deck() -> Vec<Card> {
        let deck = Vec::<Card>::from_iter(
            Suit::iter()
                .cartesian_product(Denomination::iter())
                .map(|(suit, denomination)| Card { suit, denomination }),
        );
        assert_eq!(deck.len(), 52);
        deck
    }

    fn shuffled_deck() -> Vec<Card> {
        let mut deck = Deal::sorted_deck();
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        deck
    }

    fn vulnerable(&self) -> Vulnerability {
        self.board.vulnerable()
    }

    fn dealer(&self) -> PlayerPosition {
        self.board.dealer()
    }
}

impl Default for Deal {
    fn default() -> Self {
        Deal::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Deal;
    use super::*;
    use crate::primitives::*;

    #[test]
    fn test_deck_integrity() {
        let deal = Deal::new();
        let mut cards: Vec<Card> = Vec::with_capacity(52);

        for hand in deal.hands {
            for &card in hand.cards() {
                cards.push(card)
            }
        }

        cards.sort_unstable();

        assert_eq!(
            cards.get(1).unwrap(),
            &Card {
                suit: Suit::Clubs,
                denomination: Denomination::Three
            }
        );
        assert_eq!(
            cards.get(13).unwrap(),
            &Card {
                suit: Suit::Diamonds,
                denomination: Denomination::Two
            }
        );
        assert_eq!(
            cards.get(17).unwrap(),
            &Card {
                suit: Suit::Diamonds,
                denomination: Denomination::Six
            }
        );
        assert_eq!(
            cards.get(32).unwrap(),
            &Card {
                suit: Suit::Hearts,
                denomination: Denomination::Eight
            }
        );
        assert_eq!(
            cards.get(48).unwrap(),
            &Card {
                suit: Suit::Spades,
                denomination: Denomination::Jack
            }
        );
    }
}
