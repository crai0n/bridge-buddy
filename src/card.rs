#[derive(Debug, PartialEq)]
pub struct Card {
    pub denomination: Denomination,
    pub suit: Suit,
}

#[derive(Debug, PartialEq, PartialOrd)]
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

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
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
