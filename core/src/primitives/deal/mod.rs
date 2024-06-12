use crate::primitives::card::suit::SUIT_ARRAY;
use crate::primitives::deal::seat::SEAT_ARRAY;

pub use board::Board;
pub use deck::Deck;
pub use hand::Hand;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
pub use seat::Seat;
use std::cmp::max;
use std::fmt::{Display, Formatter};

use crate::primitives::card::Rank;
use crate::primitives::{Card, Suit};
pub use vulnerability::Vulnerability;

pub mod axis;
pub mod board;
mod choose;
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
        Self::random_from_rng(&mut rng)
    }

    pub fn random() -> Deal<N> {
        let mut rng = thread_rng();
        Self::random_from_rng(&mut rng)
    }

    pub fn from_hands(hands: [Hand<N>; 4]) -> Self {
        let board = Board::from_number(1);
        Deal { board, hands }
    }

    pub fn random_from_rng(rng: &mut impl Rng) -> Self {
        let board_number = rng.gen_range(1..=Board::MAX_NUMBER);
        Self::random_from_rng_with_board_number(board_number, rng)
    }
    pub fn random_with_board_number(board_number: usize) -> Self {
        let mut rng = thread_rng();
        Self::random_from_rng_with_board_number(board_number, &mut rng)
    }

    fn random_from_rng_with_board_number(board_number: usize, rng: &mut impl Rng) -> Self {
        let board = Board::from_number(board_number);
        let hands = Self::hands_from_rng(rng);

        Deal { board, hands }
    }

    fn hands_from_rng(rng: &mut impl Rng) -> [Hand<N>; 4] {
        let deck = Deck::<N>::new().shuffled_with_rng(rng);
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

impl Deal<13> {
    pub fn from_andrews_page(_page: u128) -> Self {
        // This calculates the deal according to Thomas Andrews' Algorithm
        // https://bridge.thomasoandrews.com/bridge/impossible/algorithm.html

        // const N_PAGES: u128 = 53644737765488792839237440000;
        // const NORTH_MAX: u128 = 635013559600; // 52 choose 13
        const EAST_MAX: u128 = 8122425444; // 39 choose 13
        const SOUTH_MAX: u128 = 10400600; // 26 choose 13

        let s_index = _page % SOUTH_MAX;

        let temp = _page / SOUTH_MAX;
        let e_index = temp % EAST_MAX;

        let n_index = temp / EAST_MAX;

        let n_seq = choose::index_to_sequence(n_index);

        let e_seq = choose::index_to_sequence(e_index);

        let s_seq = choose::index_to_sequence(s_index);

        let mut result = [Seat::West; 52];

        let [mut index_n, mut count_n, mut index_e, mut count_e, mut index_s, mut count_s] = [0; 6];

        for res in &mut result {
            index_n += 1;
            if count_n < 13 && n_seq[count_n] == index_n as u8 - 1 {
                *res = Seat::North;
                count_n += 1;
                continue;
            }

            index_e += 1;
            if count_e < 13 && e_seq[count_e] == index_e as u8 - 1 {
                *res = Seat::East;
                count_e += 1;
                continue;
            }

            index_s += 1;
            if count_s < 13 && s_seq[count_s] == index_s as u8 - 1 {
                *res = Seat::South;
                count_s += 1;
                continue;
            }
        }

        let n_cards: [Card; 13] = result
            .iter()
            .enumerate()
            .filter(|(_, seat)| **seat == Seat::North)
            .map(|(i, _)| Self::u8_to_card(i as u8))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let s_cards: [Card; 13] = result
            .iter()
            .enumerate()
            .filter(|(_, seat)| **seat == Seat::South)
            .map(|(i, _)| Self::u8_to_card(i as u8))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let e_cards: [Card; 13] = result
            .iter()
            .enumerate()
            .filter(|(_, seat)| **seat == Seat::East)
            .map(|(i, _)| Self::u8_to_card(i as u8))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let w_cards: [Card; 13] = result
            .iter()
            .enumerate()
            .filter(|(_, seat)| **seat == Seat::West)
            .map(|(i, _)| Self::u8_to_card(i as u8))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let n_hand = Hand::<13>::from_cards(&n_cards).unwrap();
        let s_hand = Hand::<13>::from_cards(&s_cards).unwrap();
        let e_hand = Hand::<13>::from_cards(&e_cards).unwrap();
        let w_hand = Hand::<13>::from_cards(&w_cards).unwrap();

        Deal::<13>::from_hands([n_hand, e_hand, s_hand, w_hand])
    }

    fn u8_to_card(n: u8) -> Card {
        let suit = Suit::from(3 - (n as u16 / 13));
        let rank = Rank::try_from(12 - (n as u32 % 13)).unwrap();
        Card { suit, rank }
    }

    pub fn from_pavlicek_page(_page: u128) -> Option<Self> {
        // This calculates the deal according to Richard Pavlicek's Algorithm
        // http://www.rpbridge.net/7z68.htm

        unimplemented!()
    }
}

impl<const N: usize> Display for Deal<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_lengths: [usize; 4] = SEAT_ARRAY.map(|seat| {
            let mut max_length = 0;
            for suit in SUIT_ARRAY {
                let length = self.hand_of(seat).length_in(suit);
                if length > max_length {
                    max_length = length
                }
            }
            max_length as usize
        });
        let west_buffer = format!("{:<1$}", " ", max_lengths[3] + 1);
        let north_buffer = format!("{:<1$}", " ", max(max_lengths[0], max_lengths[2]));

        // north's hand
        for &suit in SUIT_ARRAY.iter().rev() {
            write!(f, "{}{}", west_buffer, suit)?;
            for card in self.hands[0].cards_in(suit).rev() {
                write!(f, "{}", card.rank)?;
            }
            writeln!(f)?;
        }

        // west and east's hands
        for &suit in SUIT_ARRAY.iter().rev() {
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
        for &suit in SUIT_ARRAY.iter().rev() {
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
        Deal::random()
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::{Card, Deal, Hand};
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
        let deal = Deal::<13>::random();
        println!("{}", deal)
    }

    #[test_case(1, "S:AKQJT98765432", "H:AKQJT98765432", "D:AKQJT98765432", "C:AKQJT98765432")]
    #[test_case(
        10,
        "S:AKQJT98765432",
        "H:AKQJT98765432",
        "D:AKQJ98765432, C:A",
        "C:KQJT98765432, D:T"
    )]
    #[test_case(
        1000000000000000000000000000,
        "♠:KT862,♥:J62,♦:9632,♣:A",
        "♠:A93,♥:K94,♦:J4,♣:K9765",
        "♠:4,♥:A5,♦:AKT75,♣:QJT43",
        "♠:QJ75,♥:QT873,♦:Q8,♣:82"
    )]
    fn from_andrews(page: u128, north: &str, east: &str, south: &str, west: &str) {
        let north = Hand::<13>::from_str(north).unwrap();
        let east = Hand::<13>::from_str(east).unwrap();
        let south = Hand::<13>::from_str(south).unwrap();
        let west = Hand::<13>::from_str(west).unwrap();

        let expected = Deal::from_hands([north, east, south, west]);

        let deal = Deal::from_andrews_page(page);

        assert_eq!(deal, expected)
    }
}
