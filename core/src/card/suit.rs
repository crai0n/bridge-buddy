use crate::error::ParseError;
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

impl Suit {
    pub fn from_char(char: char) -> Result<Suit, ParseError> {
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
            c => Err(ParseError {
                cause: c.into(),
                description: "unknown suit",
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Suit::*;
    use crate::card::Suit;
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

    #[test_case('1')]
    #[test_case('6')]
    #[test_case('u')]
    #[test_case('a')]
    #[test_case('q')]
    #[test_case('K')]
    #[test_case('T')]
    #[test_case('.')]
    #[test_case('o')]
    #[should_panic()]
    fn fail_for_unknown_letters(input: char) {
        Suit::from_char(input).unwrap();
    }
}
