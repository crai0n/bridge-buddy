use crate::primitives::board::vulnerability::Vulnerability;
use crate::primitives::board::Board;
use crate::primitives::board::PlayerPosition;
use crate::primitives::deck::Deck;
use crate::primitives::Hand;
use rand::prelude::*;

pub struct Deal {
    pub board: Board,
    pub hands: [Hand; 4],
}

impl Deal {
    pub fn new() -> Deal {
        let mut rng = thread_rng();
        Self::from_rng(&mut rng)
    }

    pub fn from_rng(rng: &mut impl Rng) -> Self {
        let board_number = rng.gen_range(1..=Board::MAX_NUMBER);
        Self::from_rng_with_board_number(board_number, rng)
    }
    fn new_with_board_number(board_number: usize) -> Self {
        let mut rng = thread_rng();
        Self::from_rng_with_board_number(board_number, &mut rng)
    }

    pub fn from_rng_with_board_number(board_number: usize, rng: &mut impl Rng) -> Self {
        let board = Board::from_number(board_number);
        let hands = Self::hands_from_rng(rng);

        Deal { board, hands }
    }

    fn hands_from_rng(rng: &mut impl Rng) -> [Hand; 4] {
        let mut deck = Deck::new();
        deck.shuffle_with_rng(rng);
        deck.deal()
    }

    pub fn vulnerable(&self) -> Vulnerability {
        self.board.vulnerable()
    }

    pub fn dealer(&self) -> PlayerPosition {
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
    use crate::primitives::card::Denomination;
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
