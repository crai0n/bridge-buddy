use itertools::Itertools;
use rand::{random, thread_rng};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use crate::hand::Hand;
use crate::card::*;


pub struct Deal {
    deal_number: u8,
    vulnerable: Vulnerable,
    north: Hand,
    east: Hand,
    south: Hand,
    west: Hand
}

#[derive(Debug, PartialEq, Eq)]
pub enum Vulnerable {
    None,
    NorthSouth,
    EastWest,
    All
}


impl Deal {
    pub fn new() -> Deal {
        let deal_number = (random::<u8>() % 32) + 1;
        Self::new_from_number(deal_number)
    }

    pub fn new_from_number(deal_number: u8) -> Deal {
        // calculate vulnerability
        let v = deal_number - 1;
        let vul = v + v / 4 ;
        let vulnerable = match vul % 4 {
            0 => Vulnerable::None,
            1 => Vulnerable::NorthSouth,
            2 => Vulnerable::EastWest,
            _ => Vulnerable::All,
        };

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
        let north = Hand::new(cards_vec.split_off(39).try_into().unwrap());
        let east = Hand::new(cards_vec.split_off(26).try_into().unwrap());
        let south = Hand::new(cards_vec.split_off(13).try_into().unwrap());
        let west = Hand::new(cards_vec.try_into().unwrap());

        Deal {deal_number, vulnerable, north, east, south, west}

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

        for card in deal.north.cards() {
            cards.push(card.clone())
        }
        for card in deal.east.cards() {
            cards.push(card.clone())
        }
        for card in deal.south.cards() {
            cards.push(card.clone())
        }
        for card in deal.west.cards() {
            cards.push(card.clone())
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