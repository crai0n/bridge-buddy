use strum::EnumIter;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub suit: Suit,
    pub denomination: Denomination,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
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
    pub fn from_char(char: &char) -> Result<Denomination, ()> {
        match char {
            'A' => Ok(Denomination::Ace),
            'K' => Ok(Denomination::King),
            'Q' => Ok(Denomination::Queen),
            'J' => Ok(Denomination::Jack),
            'T' => Ok(Denomination::Ten),
            '9' => Ok(Denomination::Nine),
            '8' => Ok(Denomination::Eight),
            '7' => Ok(Denomination::Seven),
            '6' => Ok(Denomination::Six),
            '5' => Ok(Denomination::Five),
            '4' => Ok(Denomination::Four),
            '3' => Ok(Denomination::Three),
            '2' => Ok(Denomination::Two),
            _ => Err(()),
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
    pub fn from_char(char: &char) -> Result<Suit, ()> {
        match char {
            'S' => Ok(Suit::Spades),
            '♠' => Ok(Suit::Spades),
            'H' => Ok(Suit::Hearts),
            '♥' => Ok(Suit::Hearts),
            'D' => Ok(Suit::Diamonds),
            '♦' => Ok(Suit::Diamonds),
            'C' => Ok(Suit::Clubs),
            '♣' => Ok(Suit::Clubs),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.denomination)
    }
}

impl Card {
    pub fn from_str(string: &str) -> Result<Card, ()> {
        if string.len() != 2 {
            return Err(());
        }
        match Denomination::from_char(&string.chars().nth(0).unwrap()) {
            Ok(d) => match Suit::from_char(&string.chars().nth(1).unwrap()) {
                Ok(s) => Ok(Card {
                    denomination: d,
                    suit: s,
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
