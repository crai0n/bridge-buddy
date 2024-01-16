use crate::error::BBError;
pub use rank::Rank;
pub use suit::Suit;

pub mod rank;
pub mod suit;

pub const N_CARDS: usize = rank::N_RANKS * suit::N_SUITS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

impl std::str::FromStr for Card {
    type Err = BBError;

    fn from_str(string: &str) -> Result<Card, Self::Err> {
        let [s, d] = Self::split_string(string)?;
        let suit = Suit::from_char(s)?;
        let rank = Rank::from_char(d)?;
        Ok(Card { suit, rank })
    }
}

impl Card {
    fn split_string(string: &str) -> Result<[char; 2], BBError> {
        let chars = string.chars().collect::<Vec<char>>();
        chars.try_into().or(Err(BBError::UnknownCard(string.into())))
    }

    pub fn touches(&self, other: &Card) -> bool {
        self.suit == other.suit && self.rank.touches(&other.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::Card;
    use super::Rank::*;
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
    fn parsing(input: &str, suit: Suit, rank: Rank) {
        assert_eq!(Card::from_str(input).unwrap(), Card { suit, rank });
    }

    #[test_case("oQ")]
    #[test_case("SL")]
    #[test_case("D;")]
    #[test_case("C1")]
    #[test_case(";")]
    fn parsing_fails(input: &str) {
        assert!(Card::from_str(input).is_err());
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
    fn display(rank: Rank, suit: Suit, expected: &str) {
        let card = Card { suit, rank };
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
    fn round_trip(suit: Suit, rank: Rank) {
        let card = Card { suit, rank };
        let string = format!("{}", card);
        let new_card = Card::from_str(&string).unwrap();
        assert_eq!(card, new_card);
    }

    #[test_case(Card { suit: Spades, rank: King }, "Card { suit: Spades, rank: King }")]
    fn debug(input: Card, expected: &str) {
        assert_eq!(format!("{:?}", input), expected)
    }

    #[test]
    fn copy() {
        let mut x = Card {
            suit: Spades,
            rank: King,
        };
        let y = x;
        x = Card {
            suit: Hearts,
            rank: Queen,
        };
        assert_eq!(
            x,
            Card {
                suit: Hearts,
                rank: Queen,
            }
        );
        assert_eq!(
            y,
            Card {
                suit: Spades,
                rank: King,
            }
        );
    }
}
