#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Card {
    pub denomination: Denomination,
    pub suit: Suit,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Denomination {
    fn from_char(char: &char) -> Result<Denomination, ()> {
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

impl Suit {
    fn from_char(char: &char) -> Result<Suit, ()> {
        match char {
            'S' => Ok(Suit::Spades),
            'H' => Ok(Suit::Hearts),
            'D' => Ok(Suit::Diamonds),
            'C' => Ok(Suit::Clubs),
            _ => Err(()),
        }
    }
}

impl Card {
    fn from_str(string: &str) -> Result<Card, ()> {
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
}
