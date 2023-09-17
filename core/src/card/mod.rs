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
    use super::Denomination::*;
    use super::Suit::*;
    use super::*;

    #[test]
    fn test_comparisons() {
        assert_eq!(
            Card {
                denomination: Two,
                suit: Spades
            },
            Card {
                denomination: Two,
                suit: Spades
            }
        );
        assert_ne!(
            Card {
                denomination: Two,
                suit: Diamonds
            },
            Card {
                denomination: Two,
                suit: Spades
            }
        );
        assert!(Clubs < Diamonds);
        assert!(King < Ace);
    }

    #[test]
    fn test_card_display() {
        let nine_of_clubs = Card {
            denomination: Nine,
            suit: Clubs,
        };
        assert_eq!(format!("{}", nine_of_clubs), "♣9");

        let five_of_diamonds = Card {
            denomination: Five,
            suit: Diamonds,
        };
        assert_eq!(format!("{}", five_of_diamonds), "♦5");

        let queen_of_hearts = Card {
            denomination: Queen,
            suit: Hearts,
        };
        assert_eq!(format!("{}", queen_of_hearts), "♥Q");

        let ace_of_spades = Card {
            denomination: Ace,
            suit: Spades,
        };
        assert_eq!(format!("{}", ace_of_spades), "♠A");
    }
}
