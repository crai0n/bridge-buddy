use std::collections::BTreeMap;

use itertools::Itertools;
use rand::{random, thread_rng};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use crate::hand::Hand;
use crate::card::*;


pub struct Deal {
    deal_number: u8,
    vulnerable: Vulnerable,
    cards: BTreeMap<PlayerPosition, Hand>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Vulnerable {
    None,
    NorthSouth,
    EastWest,
    All
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerPosition {
    North,
    East,
    South,
    West,
}


impl Deal {
    pub fn new() -> Deal {
        let deal_number = (random::<u8>() % 32) + 1;
        Self::new_from_number(deal_number)
    }

    pub fn new_from_number(deal_number: u8) -> Deal {
        // calculate vulnerability
        let vulnerable = Self::calculate_vulnerability(deal_number);


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
        let mut cards: BTreeMap<PlayerPosition, Hand>= BTreeMap::new();
        cards.insert(PlayerPosition::North, Hand::new(cards_vec.split_off(39).try_into().unwrap()));
        cards.insert(PlayerPosition::East, Hand::new(cards_vec.split_off(26).try_into().unwrap()));
        cards.insert(PlayerPosition::South, Hand::new(cards_vec.split_off(13).try_into().unwrap()));
        cards.insert(PlayerPosition::West, Hand::new(cards_vec.try_into().unwrap()));

        Deal {deal_number, vulnerable, cards}

    }

    fn calculate_vulnerability(deal_number: u8) -> Vulnerable {
        let v = deal_number - 1;
        let vul = v + v / 4 ;
        match vul % 4 {
            0 => Vulnerable::None,
            1 => Vulnerable::NorthSouth,
            2 => Vulnerable::EastWest,
            _ => Vulnerable::All,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Deal;
    use super::Vulnerable;
    use super::*;

    #[test]
    fn test_vulnerability() {
        // Pattern follows the "BONE"-chart
        assert_eq!(Deal::new_from_number(1).vulnerable, Vulnerable::None);
        assert_eq!(Deal::new_from_number(2).vulnerable, Vulnerable::NorthSouth);
        assert_eq!(Deal::new_from_number(3).vulnerable, Vulnerable::EastWest);
        assert_eq!(Deal::new_from_number(4).vulnerable, Vulnerable::All);

        assert_eq!(Deal::new_from_number(5).vulnerable, Vulnerable::NorthSouth);
        assert_eq!(Deal::new_from_number(6).vulnerable, Vulnerable::EastWest);
        assert_eq!(Deal::new_from_number(7).vulnerable, Vulnerable::All);
        assert_eq!(Deal::new_from_number(8).vulnerable, Vulnerable::None);

        assert_eq!(Deal::new_from_number(9).vulnerable, Vulnerable::EastWest);
        assert_eq!(Deal::new_from_number(10).vulnerable, Vulnerable::All);
        assert_eq!(Deal::new_from_number(11).vulnerable, Vulnerable::None);
        assert_eq!(Deal::new_from_number(12).vulnerable, Vulnerable::NorthSouth);

        assert_eq!(Deal::new_from_number(13).vulnerable, Vulnerable::All);
        assert_eq!(Deal::new_from_number(14).vulnerable, Vulnerable::None);
        assert_eq!(Deal::new_from_number(15).vulnerable, Vulnerable::NorthSouth);
        assert_eq!(Deal::new_from_number(16).vulnerable, Vulnerable::EastWest);
        // Pattern repeats after 16 hands
        assert_eq!(Deal::new_from_number(17).vulnerable, Vulnerable::None);
        assert_eq!(Deal::new_from_number(18).vulnerable, Vulnerable::NorthSouth);
    }

    #[test]
    fn test_deck_integrity() {
        let deal = Deal::new();
        let mut cards: Vec<Card> = Vec::with_capacity(52);

        for hand in deal.cards.values() {
            for &card in hand.cards() {
                cards.push(card)
            }
        }

        cards.sort_unstable();

        assert_eq!(
            cards.iter().nth(1).unwrap(),
            &Card {
                suit: Suit::Clubs,
                denomination: Denomination::Three
            }
        );
        assert_eq!(
            cards.iter().nth(13).unwrap(),
            &Card {
                suit: Suit::Diamonds,
                denomination: Denomination::Two
            }
        );
        assert_eq!(
            cards.iter().nth(17).unwrap(),
            &Card {
                suit: Suit::Diamonds,
                denomination: Denomination::Six
            }
        );
        assert_eq!(
            cards.iter().nth(32).unwrap(),
            &Card {
                suit: Suit::Hearts,
                denomination: Denomination::Eight
            }
        );
        assert_eq!(
            cards.iter().nth(48).unwrap(),
            &Card {
                suit: Suit::Spades,
                denomination: Denomination::Jack
            }
        );

    }

}