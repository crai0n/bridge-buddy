use crate::primitives::Suit;
pub use board::Board;
pub use deck::Deck;
pub use hand::Hand;
use itertools::Itertools;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
pub use seat::Seat;
use std::cmp::max;
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
pub use vulnerability::Vulnerability;

pub mod axis;
pub mod board;
pub mod deck;
pub mod hand;
pub mod seat;
pub mod turn_rank;
pub mod vulnerability;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Deal<const N: usize> {
    pub board: Board,
    pub hands: [Hand<N>; 4],
}

impl<const N: usize> Deal<N> {
    pub fn from_u64_seed(seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        Self::from_rng(&mut rng)
    }

    pub fn new() -> Deal<N> {
        let mut rng = thread_rng();
        Self::from_rng(&mut rng)
    }

    pub fn from_hands(hands: [Hand<N>; 4]) -> Self {
        let board = Board::from_number(1);
        Deal { board, hands }
    }

    pub fn from_rng(rng: &mut impl Rng) -> Self {
        let board_number = rng.gen_range(1..=Board::MAX_NUMBER);
        Self::from_rng_with_board_number(board_number, rng)
    }
    pub fn new_with_board_number(board_number: usize) -> Self {
        let mut rng = thread_rng();
        Self::from_rng_with_board_number(board_number, &mut rng)
    }

    fn from_rng_with_board_number(board_number: usize, rng: &mut impl Rng) -> Self {
        let board = Board::from_number(board_number);
        let hands = Self::hands_from_rng(rng);

        Deal { board, hands }
    }

    fn hands_from_rng(rng: &mut impl Rng) -> [Hand<N>; 4] {
        let deck = Deck::<N>::shuffled_from_rng(rng);
        deck.deal()
    }

    pub fn vulnerable(&self) -> Vulnerability {
        self.board.vulnerability()
    }

    pub fn dealer(&self) -> Seat {
        self.board.dealer()
    }

    pub fn hand_of(&self, position: Seat) -> &Hand<N> {
        &self.hands[position as usize]
    }
}

impl<const N: usize> Display for Deal<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_lengths: [usize; 4] = Seat::iter()
            .map(|seat| {
                let mut max_length = 0;
                for suit in Suit::iter() {
                    let length = self.hand_of(seat).length_in(suit);
                    if length > max_length {
                        max_length = length
                    }
                }
                max_length as usize
            })
            .collect_vec()
            .try_into()
            .unwrap();
        let west_buffer = format!("{:<1$}", " ", max_lengths[3] + 1);
        let north_buffer = format!("{:<1$}", " ", max(max_lengths[0], max_lengths[2]));

        // north's hand
        for suit in Suit::iter().rev() {
            write!(f, "{}{}", west_buffer, suit)?;
            for card in self.hands[0].cards_in(suit).rev() {
                write!(f, "{}", card.rank)?;
            }
            writeln!(f)?;
        }

        // west and east's hands
        for suit in Suit::iter().rev() {
            write!(f, "{}", suit)?;
            for card in self.hands[3].cards_in(suit).rev() {
                write!(f, "{}", card.rank)?;
            }
            write!(
                f,
                "{:<1$}",
                " ",
                max_lengths[3] - self.hands[3].length_in(suit) as usize + 1
            )?;
            write!(f, "{}{}", north_buffer, suit)?;
            for card in self.hands[1].cards_in(suit).rev() {
                write!(f, "{}", card.rank)?;
            }
            writeln!(f)?;
        }

        // north's hand
        for suit in Suit::iter().rev() {
            write!(f, "{}{}", west_buffer, suit)?;
            for card in self.hands[2].cards_in(suit).rev() {
                write!(f, "{}", card.rank)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<const N: usize> Default for Deal<N> {
    fn default() -> Self {
        Deal::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::{Card, Deal};
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case( 1u64,  20, "C4", "SK"; "Test A")]
    #[test_case( 2u64,  29, "C4", "SQ"; "Test B")]
    #[test_case( 3u64,  11, "C3", "SA"; "Test C")]
    #[test_case( 4u64,  24, "C4", "ST"; "Test D")]
    #[test_case( 5u64,   7, "C5", "SA"; "Test E")]
    fn determinism(seed: u64, board_number: usize, lowest_card: &str, highest_card: &str) {
        // let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let deal = Deal::<13>::from_u64_seed(seed);
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

    #[ignore]
    #[test]
    fn display() {
        let deal = Deal::<13>::new();
        println!("{}", deal)
    }
}
