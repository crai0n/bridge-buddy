use crate::error::BBError;
use crate::primitives::Card;
use strum::Display;

pub const N_SUITS: usize = 4;

pub const SUIT_ARRAY: [Suit; N_SUITS] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {
    #[strum(serialize = "♣")]
    Clubs = 0,
    #[strum(serialize = "♦")]
    Diamonds = 1,
    #[strum(serialize = "♥")]
    Hearts = 2,
    #[strum(serialize = "♠")]
    Spades = 3,
}

impl From<u16> for Suit {
    fn from(value: u16) -> Self {
        match value {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            3.. => Suit::Spades,
        }
    }
}

impl Suit {
    pub fn from_char(char: char) -> Result<Suit, BBError> {
        match char {
            'S' => Ok(Suit::Spades),
            's' => Ok(Suit::Spades),
            '♠' => Ok(Suit::Spades),
            'H' => Ok(Suit::Hearts),
            'h' => Ok(Suit::Hearts),
            '♥' => Ok(Suit::Hearts),
            'D' => Ok(Suit::Diamonds),
            'd' => Ok(Suit::Diamonds),
            '♦' => Ok(Suit::Diamonds),
            'C' => Ok(Suit::Clubs),
            'c' => Ok(Suit::Clubs),
            '♣' => Ok(Suit::Clubs),
            c => Err(BBError::UnknownSuit(c.into())),
        }
    }

    pub fn is_major(&self) -> bool {
        match self {
            Suit::Spades => true,
            Suit::Hearts => true,
            Suit::Diamonds => false,
            Suit::Clubs => false,
        }
    }

    pub fn is_minor(&self) -> bool {
        !self.is_major()
    }

    pub const fn next(&self) -> Self {
        match self {
            Suit::Clubs => Suit::Diamonds,
            Suit::Diamonds => Suit::Hearts,
            Suit::Hearts => Suit::Spades,
            Suit::Spades => Suit::Clubs,
        }
    }

    pub const fn previous(&self) -> Self {
        match self {
            Suit::Clubs => Suit::Spades,
            Suit::Diamonds => Suit::Clubs,
            Suit::Hearts => Suit::Diamonds,
            Suit::Spades => Suit::Hearts,
        }
    }
}

impl std::str::FromStr for Suit {
    type Err = BBError;

    fn from_str(string: &str) -> Result<Suit, BBError> {
        let mut chars = string.trim().chars();
        let char = chars.next().ok_or(BBError::UnknownSuit(string.into()))?;
        if chars.next().is_some() {
            return Err(BBError::UnknownSuit(string.into()));
        }
        Suit::from_char(char)
    }
}

impl From<Card> for Suit {
    fn from(card: Card) -> Suit {
        card.suit
    }
}

#[cfg(test)]
mod tests {
    use super::Suit::*;
    use crate::error::BBError;
    use crate::primitives::card::{Card, Rank};
    use crate::primitives::Suit;
    use std::str::FromStr;
    use test_case::test_case;

    #[test]
    fn relative_ranking_of_suits() {
        assert!(Clubs < Diamonds);
        assert!(Diamonds < Hearts);
        assert!(Hearts < Spades);
        assert!(Spades > Clubs);
    }

    #[test_case('S', Suit::Spades; "uppercase S")]
    #[test_case('H', Suit::Hearts; "uppercase H")]
    #[test_case('D', Suit::Diamonds; "uppercase D")]
    #[test_case('C', Suit::Clubs; "uppercase C")]
    #[test_case('s', Suit::Spades; "lowercase s")]
    #[test_case('h', Suit::Hearts; "lowercase h")]
    #[test_case('d', Suit::Diamonds; "lowercase d")]
    #[test_case('c', Suit::Clubs; "lowercase c")]
    #[test_case('♠', Suit::Spades; "spades symbol")]
    #[test_case('♥', Suit::Hearts; "hearts symbol")]
    #[test_case('♦', Suit::Diamonds; "diamonds symbol")]
    #[test_case('♣', Suit::Clubs; "clubs symbol")]
    fn parse_all_known_symbols(input: char, expected: Suit) {
        assert_eq!(Suit::from_char(input).unwrap(), expected);
    }

    #[test_case("S", Suit::Spades; "uppercase S")]
    #[test_case("H", Suit::Hearts; "uppercase H")]
    #[test_case("d", Suit::Diamonds; "lowercase d")]
    #[test_case("♣", Suit::Clubs; "clubs symbol")]
    fn parse_strings(input: &str, expected: Suit) {
        assert_eq!(Suit::from_str(input).unwrap(), expected);
    }

    #[test_case(""; "Empty string")]
    #[test_case(".c"; "additional char")]
    #[test_case("hd"; "two chars")]
    fn fail_strings(input: &str) {
        assert!(Suit::from_str(input).is_err());
    }

    #[test_case('1')]
    #[test_case('6')]
    #[test_case('u')]
    #[test_case('a')]
    #[test_case('q')]
    #[test_case('K')]
    #[test_case('T')]
    #[test_case('.')]
    #[test_case('o')]
    fn fail_for_unknown_letters(input: char) {
        assert_eq!(Suit::from_char(input).unwrap_err(), BBError::UnknownSuit(input.into()));
    }

    #[test_case(Clubs, "♣")]
    #[test_case(Diamonds, "♦")]
    #[test_case(Hearts, "♥")]
    #[test_case(Spades, "♠")]
    fn display(suit: Suit, expected: &str) {
        assert_eq!(format!("{}", suit), expected)
    }

    #[test_case(Spades)]
    #[test_case(Hearts)]
    #[test_case(Diamonds)]
    #[test_case(Clubs)]
    fn round_trip(suit: Suit) {
        let string = format!("{}", suit);
        let suit_char = string.chars().next().unwrap();
        let new_suit = Suit::from_char(suit_char).unwrap();
        assert_eq!(suit, new_suit);
    }

    #[test_case('.')]
    #[test_case('A')]
    #[test_case('k')]
    #[test_case('j')]
    #[test_case('u')]
    fn fail_misc_characters(input: char) {
        assert_eq!(Suit::from_char(input).unwrap_err(), BBError::UnknownSuit(input.into()))
    }

    #[test]
    fn copy() {
        let mut x = Hearts;
        let y = x;
        x = Spades;
        assert_eq!(x, Spades);
        assert_eq!(y, Hearts);
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Spades), "Spades")
    }

    #[test_case(Spades, true; "Spades is Major")]
    #[test_case(Hearts, true; "Hearts is Major")]
    #[test_case(Diamonds, false; "Diamonds is minor")]
    #[test_case(Clubs, false; "Clubs is minor")]
    fn is_major(suit: Suit, expected: bool) {
        assert_eq!(suit.is_major(), expected)
    }

    #[test_case(Spades, false; "Spades is Major")]
    #[test_case(Hearts, false; "Hearts is Major")]
    #[test_case(Diamonds, true; "Diamonds is Minor")]
    #[test_case(Clubs, true; "Clubs is Minor")]
    fn is_minor(suit: Suit, expected: bool) {
        assert_eq!(suit.is_minor(), expected)
    }

    #[test_case(Card { suit: Suit::Spades, rank: Rank::King}, Suit::Spades; "King of Spades is a Spades")]
    fn from_card(card: Card, expected: Suit) {
        assert_eq!(expected, card.into())
    }
}
