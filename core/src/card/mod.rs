use crate::error::ParseError;
pub use denomination::Denomination;
pub use suit::Suit;

pub mod denomination;
pub mod suit;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub suit: Suit,
    pub denomination: Denomination,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.denomination)
    }
}

impl std::str::FromStr for Card {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Card, Self::Err> {
        let [s, d] = Self::split_string(string)?;
        let suit = Suit::from_char(s)?;
        let denomination = Denomination::from_char(d)?;
        Ok(Card { suit, denomination })
    }
}

impl Card {
    fn split_string(string: &str) -> Result<[char; 2], ParseError> {
        let chars = string.chars().collect::<Vec<char>>();
        match chars.try_into() {
            Ok(c) => Ok(c),
            _ => Err(ParseError {
                cause: string.into(),
                description: "cards consist of two characters",
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Card;
    use super::Denomination::*;
    use super::Suit::*;
    use super::*;
    use std::cmp::Ordering;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("HQ", Hearts, Queen)]
    #[test_case("SK", Spades, King)]
    #[test_case("DA", Diamonds, Ace)]
    #[test_case("C9", Clubs, Nine)]
    #[test_case("h7", Hearts, Seven)]
    #[test_case("c5", Clubs, Five)]
    #[test_case("dJ", Diamonds, Jack)]
    #[test_case("Hj", Hearts, Jack)]
    fn parsing(input: &str, suit: Suit, denomination: Denomination) {
        assert_eq!(Card::from_str(input).unwrap(), Card { suit, denomination });
    }

    #[test_case("h7", "hA", Ordering::Less)]
    #[test_case("s7", "hK", Ordering::Greater)]
    #[test_case("s7", "s7", Ordering::Equal)]
    fn total_ordering(left: &str, right: &str, expected: Ordering) {
        let left_card = Card::from_str(left).unwrap();
        let right_card = Card::from_str(right).unwrap();
        assert_eq!(left_card.cmp(&right_card), expected);
    }

    #[test_case(Nine, Clubs, "♣9")]
    #[test_case(Five, Diamonds, "♦5")]
    #[test_case(Queen, Hearts, "♥Q")]
    #[test_case(Ace, Spades, "♠A")]
    fn display(denomination: Denomination, suit: Suit, expected: &str) {
        let card = Card { suit, denomination };
        assert_eq!(format!("{}", card), expected);
    }

    #[test_case(Spades, Ace)]
    #[test_case(Hearts, King)]
    #[test_case(Diamonds, Queen)]
    #[test_case(Clubs, Jack)]
    #[test_case(Hearts, Ten)]
    #[test_case(Diamonds, Nine)]
    #[test_case(Clubs, Eight)]
    #[test_case(Spades, Seven)]
    #[test_case(Diamonds, Six)]
    #[test_case(Clubs, Five)]
    #[test_case(Spades, Four)]
    #[test_case(Hearts, Three)]
    #[test_case(Clubs, Two)]
    fn round_trip(suit: Suit, denomination: Denomination) {
        let card = Card { suit, denomination };
        let string = format!("{}", card);
        let new_card = Card::from_str(&string).unwrap();
        assert_eq!(card, new_card);
    }
}
