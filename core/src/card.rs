use crate::error::ParseError;
use strum::EnumIter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub suit: Suit,
    pub denomination: Denomination,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Denomination {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

impl std::fmt::Display for Denomination {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Denomination::Ace => write!(f, "A"),
            Denomination::King => write!(f, "K"),
            Denomination::Queen => write!(f, "Q"),
            Denomination::Jack => write!(f, "J"),
            Denomination::Ten => write!(f, "T"),
            Denomination::Nine => write!(f, "9"),
            Denomination::Eight => write!(f, "8"),
            Denomination::Seven => write!(f, "7"),
            Denomination::Six => write!(f, "6"),
            Denomination::Five => write!(f, "5"),
            Denomination::Four => write!(f, "4"),
            Denomination::Three => write!(f, "3"),
            Denomination::Two => write!(f, "2"),
        }
    }
}

impl Denomination {
    pub fn from_char(char: char) -> Result<Denomination, ParseError> {
        match char {
            'A' => Ok(Denomination::Ace),
            'a' => Ok(Denomination::Ace),
            'K' => Ok(Denomination::King),
            'k' => Ok(Denomination::King),
            'Q' => Ok(Denomination::Queen),
            'q' => Ok(Denomination::Queen),
            'J' => Ok(Denomination::Jack),
            'j' => Ok(Denomination::Jack),
            'T' => Ok(Denomination::Ten),
            't' => Ok(Denomination::Ten),
            '9' => Ok(Denomination::Nine),
            '8' => Ok(Denomination::Eight),
            '7' => Ok(Denomination::Seven),
            '6' => Ok(Denomination::Six),
            '5' => Ok(Denomination::Five),
            '4' => Ok(Denomination::Four),
            '3' => Ok(Denomination::Three),
            '2' => Ok(Denomination::Two),
            c => Err(ParseError {
                cause: c.into(),
                description: "unknown denomination",
            }),
        }
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Suit::Spades => write!(f, "♠"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
        }
    }
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

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.denomination)
    }
}

impl std::str::FromStr for Card {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Card, Self::Err> {
        if string.len() != 2 {
            return Err(ParseError {
                cause: string.into(),
                description: "cards consist of two characters",
            });
        }
        let mut chars = string.chars();
        match Suit::from_char(chars.next().unwrap()) {
            Ok(s) => match Denomination::from_char(chars.next().unwrap()) {
                Ok(d) => Ok(Card {
                    suit: s,
                    denomination: d,
                }),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
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
