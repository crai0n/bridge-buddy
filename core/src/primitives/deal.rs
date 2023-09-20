use crate::primitives::{Card, Denomination, Hand, Suit};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::{random, thread_rng};
use std::ops;
use strum::IntoEnumIterator;

pub struct Deal {
    pub deal_number: u8,
    pub vulnerable: Vulnerable,
    pub dealer: PlayerPosition,
    pub hands: [Hand; 4],
}

#[derive(Debug, PartialEq, Eq)]
pub enum Vulnerable {
    None,
    NorthSouth,
    EastWest,
    All,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PlayerPosition {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl ops::Add<usize> for PlayerPosition {
    type Output = PlayerPosition;

    fn add(self, rhs: usize) -> PlayerPosition {
        match (self as usize + rhs) % 4 {
            0 => PlayerPosition::North,
            1 => PlayerPosition::East,
            2 => PlayerPosition::South,
            _ => PlayerPosition::West,
        }
    }
}

impl Deal {
    pub fn new() -> Deal {
        let deal_number = (random::<u8>() % 32) + 1;
        Self::new_from_number(deal_number)
    }

    pub fn new_from_number(deal_number: u8) -> Deal {
        // calculate vulnerability
        let vulnerable = Self::calculate_vulnerability(deal_number);
        let dealer = Self::calculate_dealer(deal_number);

        // create the cards for playing
        let mut cards_vec = Vec::<Card>::from_iter(
            Suit::iter()
                .cartesian_product(Denomination::iter())
                .map(|(suit, denomination)| Card { suit, denomination }),
        );
        assert_eq!(cards_vec.len(), 52);

        // shuffle cards
        let mut rng = thread_rng();
        cards_vec.shuffle(&mut rng);

        //distribute cards
        let hands_vec = vec![
            Hand::from_cards(&cards_vec.split_off(39)).unwrap(),
            Hand::from_cards(&cards_vec.split_off(26)).unwrap(),
            Hand::from_cards(&cards_vec.split_off(13)).unwrap(),
            Hand::from_cards(&cards_vec).unwrap(),
        ];

        Deal {
            deal_number,
            vulnerable,
            dealer,
            hands: hands_vec.try_into().unwrap(),
        }
    }

    fn calculate_vulnerability(deal_number: u8) -> Vulnerable {
        let v = deal_number - 1;
        let vul = v + v / 4;
        match vul % 4 {
            0 => Vulnerable::None,
            1 => Vulnerable::NorthSouth,
            2 => Vulnerable::EastWest,
            _ => Vulnerable::All,
        }
    }

    fn calculate_dealer(deal_number: u8) -> PlayerPosition {
        match (deal_number - 1) % 4 {
            0 => PlayerPosition::North,
            1 => PlayerPosition::East,
            2 => PlayerPosition::South,
            _ => PlayerPosition::West,
        }
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
    use super::Vulnerable;
    use super::*;
    use crate::primitives::*;
    use test_case::test_case;

    #[test_case( 1, Vulnerable::None, PlayerPosition::North ; "Deal construction 1")]
    #[test_case( 2, Vulnerable::NorthSouth, PlayerPosition::East ; "Deal construction 2 ")]
    #[test_case( 3, Vulnerable::EastWest, PlayerPosition::South ; "Deal construction 3")]
    #[test_case( 4, Vulnerable::All, PlayerPosition::West; "Deal construction 4")]
    #[test_case( 5, Vulnerable::NorthSouth, PlayerPosition::North ; "Deal construction 5")]
    #[test_case( 6, Vulnerable::EastWest, PlayerPosition::East ; "Deal construction 6")]
    #[test_case( 7, Vulnerable::All, PlayerPosition::South ; "Deal construction 7")]
    #[test_case( 8, Vulnerable::None, PlayerPosition::West; "Deal construction 8")]
    #[test_case( 9, Vulnerable::EastWest, PlayerPosition::North ; "Deal construction 9")]
    #[test_case(10, Vulnerable::All, PlayerPosition::East ; "Deal construction 10")]
    #[test_case(11, Vulnerable::None, PlayerPosition::South ; "Deal construction 11")]
    #[test_case(12, Vulnerable::NorthSouth, PlayerPosition::West ; "Deal construction 12")]
    #[test_case(13, Vulnerable::All, PlayerPosition::North ; "Deal construction 13")]
    #[test_case(14, Vulnerable::None, PlayerPosition::East ; "Deal construction 14")]
    #[test_case(15, Vulnerable::NorthSouth, PlayerPosition::South ; "Deal construction 15")]
    #[test_case(16, Vulnerable::EastWest, PlayerPosition::West ; "Deal construction 16")]
    #[test_case(17, Vulnerable::None, PlayerPosition::North ; "Deal construction 17")]
    #[test_case(18, Vulnerable::NorthSouth, PlayerPosition::East ; "Deal construction 18")]

    fn test_deal_construction(deal_number: u8, vulnerable: Vulnerable, dealer: PlayerPosition) {
        let deal = Deal::new_from_number(deal_number);
        assert_eq!(deal.dealer, dealer);
        assert_eq!(deal.vulnerable, vulnerable);
        assert_eq!(deal.hands[0].cards().count(), 13);
    }

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
