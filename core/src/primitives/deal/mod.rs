pub use board::Board;
pub use deck::Deck;
pub use hand::Hand;
pub use player_position::PlayerPosition;
use rand::prelude::*;
pub use vulnerability::Vulnerability;

pub mod board;
pub mod deck;
pub mod hand;
pub mod player_position;
pub mod vulnerability;

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
        let deck = Deck::shuffled_from_rng(rng);
        deck.deal()
    }

    pub fn vulnerable(&self) -> Vulnerability {
        self.board.vulnerable()
    }

    pub fn dealer(&self) -> PlayerPosition {
        self.board.dealer()
    }

    pub fn hand_of(&self, position: PlayerPosition) -> &Hand {
        &self.hands[position as usize]
    }
}

impl Default for Deal {
    fn default() -> Self {
        Deal::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::{Card, Deal};
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case( 1u64,  20, "C4", "SK"; "Test A")]
    #[test_case( 2u64,  29, "C4", "SQ"; "Test B")]
    #[test_case( 3u64,  11, "C3", "SA"; "Test C")]
    #[test_case( 4u64,  24, "C4", "ST"; "Test D")]
    #[test_case( 5u64,   7, "C5", "SA"; "Test E")]
    fn determinism(seed: u64, board_number: usize, lowest_card: &str, highest_card: &str) {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let deal = Deal::from_rng(&mut rng);
        assert_eq!(deal.board.number(), board_number);
        assert_eq!(
            deal.hands.first().unwrap().cards().next().unwrap(),
            &Card::from_str(lowest_card).unwrap()
        );
        assert_eq!(
            deal.hands.first().unwrap().cards().last().unwrap(),
            &Card::from_str(highest_card).unwrap()
        );
    }
}
