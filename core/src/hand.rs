use std::cmp::Ordering;
// use crate::card::suit::SuitIter;
pub use crate::card::{Card, Denomination, Suit};
use crate::error::BBError;
use strum::IntoEnumIterator;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 13],
}

impl Hand {
    pub fn from_cards(cards: &[Card]) -> Result<Self, BBError> {
        let cards = Hand::sanitize_cards(cards)?;
        Ok(Hand { cards })
    }

    fn sanitize_cards(cards: &[Card]) -> Result<[Card; 13], BBError> {
        let mut cards: [Card; 13] = cards.try_into().or(Err(BBError::CardCount))?;
        cards.sort_unstable();
        Hand::check_for_duplicates(&cards)?;
        Ok(cards)
    }

    fn check_for_duplicates(&cards: &[Card; 13]) -> Result<(), BBError> {
        for i in 0..cards.len() - 1 {
            if cards[i] == cards[i + 1] {
                return Err(BBError::Duplicate(cards[i]));
            }
        }
        Ok(())
    }

    pub fn cards(&self) -> impl DoubleEndedIterator<Item = &Card> {
        self.cards.iter()
    }

    pub fn cards_in(&self, suit: Suit) -> impl DoubleEndedIterator<Item = &Card> {
        self.cards.iter().filter(move |&card| card.suit == suit)
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    pub fn length_in(&self, suit: Suit) -> u8 {
        self.cards_in(suit).count() as u8
    }

    pub fn hand_type(&self) -> HandType {
        fn descending_length_and_suit_value(one: &(Suit, u8), other: &(Suit, u8)) -> Ordering {
            match other.1.cmp(&one.1) {
                Ordering::Equal => other.0.cmp(&one.0), //for equal length, order by suit value
                ord => ord,
            }
        }

        let mut suit_lengths = Suit::iter().map(|s| (s, self.length_in(s))).collect::<Vec<_>>();

        suit_lengths.sort_unstable_by(descending_length_and_suit_value);

        match suit_lengths[..] {
            // three suits with at least 4 cards (third cannot have more than four)
            [(s1, _), (s2, _), (s3, 4), _] => HandType::ThreeSuited(s1, s2, s3),
            [(s1, 5..), (s2, 4..), _, _] => HandType::TwoSuited(s1, s2),
            [(s1, 6..), _, _, _] => HandType::SingleSuited(s1),
            [(s1, 5..), _, _, _] => HandType::Balanced(Some(s1)),
            _ => HandType::Balanced(None),
        }
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for suit in Suit::iter().rev() {
            // Spades, then Hearts, ...
            write!(f, "{}: ", suit)?;
            for card in self.cards_in(suit).rev() {
                write!(f, "{}", card.denomination)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Hand {
    type Err = BBError;

    fn from_str(string: &str) -> Result<Hand, Self::Err> {
        fn split_at_colon(string: &str) -> Result<(&str, &str), BBError> {
            string.split_once(':').ok_or(BBError::ParseError(
                string.into(),
                "missing colon between suit and cards",
            ))
        }

        fn create_cards_for_suit(suit_symbol: &str, denominations: &str) -> Result<Vec<Card>, BBError> {
            let mut suit_cards = vec![];
            let suit = Suit::from_str(suit_symbol)?;
            for denomination_char in denominations.trim().chars() {
                let denomination = Denomination::from_char(denomination_char)?;
                suit_cards.push(Card { denomination, suit });
            }
            Ok(suit_cards)
        }

        let mut cards: Vec<Card> = vec![];

        let separate_suits = string.trim().split(['\n', ',']);
        for cards_in_suit in separate_suits {
            let (suit_symbol, denominations) = split_at_colon(cards_in_suit)?;
            let suit_cards = create_cards_for_suit(suit_symbol, denominations)?;
            cards.extend_from_slice(&suit_cards);
        }
        Hand::from_cards(&cards)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum HandType {
    ThreeSuited(Suit, Suit, Suit),
    TwoSuited(Suit, Suit),
    SingleSuited(Suit),
    Balanced(Option<Suit>), // might contain a 5-card suit
}

impl std::fmt::Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ThreeSuited(s1, s2, s3) => write!(f, "three-suited: {}, {} and {}", s1, s2, s3),
            Self::TwoSuited(s1, s2) => write!(f, "two-suited: {} and {}", s1, s2),
            Self::SingleSuited(s) => write!(f, "single-suited: {}", s),
            Self::Balanced(Some(s)) => write!(f, "balanced with five cards in {}", s),
            Self::Balanced(None) => write!(f, "balanced"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Card;
    use super::{Hand, HandType};
    use crate::card::Denomination::*;
    use crate::card::Suit::*;
    use crate::error::BBError;
    use std::str::FromStr;
    use test_case::test_case;

    #[test]
    fn test_hand_types() {
        let hand = Hand::from_cards(&[
            Card {
                suit: Clubs,
                denomination: Ace,
            },
            Card {
                suit: Clubs,
                denomination: King,
            },
            Card {
                suit: Diamonds,
                denomination: King,
            },
            Card {
                suit: Diamonds,
                denomination: Ace,
            },
            Card {
                suit: Diamonds,
                denomination: Queen,
            },
            Card {
                suit: Hearts,
                denomination: Queen,
            },
            Card {
                suit: Hearts,
                denomination: Jack,
            },
            Card {
                suit: Hearts,
                denomination: Ten,
            },
            Card {
                suit: Spades,
                denomination: Nine,
            },
            Card {
                suit: Spades,
                denomination: Eight,
            },
            Card {
                suit: Spades,
                denomination: Seven,
            },
            Card {
                suit: Spades,
                denomination: Six,
            },
            Card {
                suit: Spades,
                denomination: Two,
            },
        ])
        .unwrap();
        assert_eq!(hand.hand_type(), HandType::Balanced(Some(Spades)));
    }

    #[test]
    fn test_methods() {
        let hand = Hand::from_cards(&[
            Card {
                suit: Clubs,
                denomination: Ace,
            },
            Card {
                suit: Diamonds,
                denomination: Ace,
            },
            Card {
                suit: Diamonds,
                denomination: King,
            },
            Card {
                suit: Spades,
                denomination: Ace,
            },
            Card {
                suit: Spades,
                denomination: King,
            },
            Card {
                suit: Spades,
                denomination: Queen,
            },
            Card {
                suit: Spades,
                denomination: Jack,
            },
            Card {
                suit: Spades,
                denomination: Ten,
            },
            Card {
                suit: Spades,
                denomination: Nine,
            },
            Card {
                suit: Spades,
                denomination: Eight,
            },
            Card {
                suit: Spades,
                denomination: Seven,
            },
            Card {
                suit: Spades,
                denomination: Six,
            },
            Card {
                suit: Spades,
                denomination: Two,
            },
        ])
        .unwrap();
        assert_eq!(
            hand.cards().nth(1).unwrap(),
            &Card {
                suit: Diamonds,
                denomination: King,
            }
        );
        assert_eq!(hand.cards_in(Spades).count(), 10);
        assert_eq!(hand.cards_in(Hearts).count(), 0);
        assert_eq!(hand.cards_in(Diamonds).count(), 2);
        assert_eq!(hand.cards_in(Clubs).count(), 1);
        assert!(!hand.contains(&Card {
            suit: Diamonds,
            denomination: Queen
        }));
        assert!(hand.contains(&Card {
            suit: Diamonds,
            denomination: Ace
        }));
        assert_eq!(hand.hand_type(), HandType::SingleSuited(Spades));
        assert_eq!(format!("{}", hand), "♠: AKQJT98762\n♥: \n♦: AK\n♣: A\n");
        assert_eq!(hand, Hand::from_str("H:, ♠:9J7A2T6K8Q,♦: AK, C: A").unwrap())
    }

    #[test]
    fn test_hand_validation() {
        assert_eq!(
            Hand::from_cards(&[
                Card {
                    suit: Diamonds,
                    denomination: Two,
                },
                Card {
                    suit: Diamonds,
                    denomination: Three,
                },
                Card {
                    suit: Diamonds,
                    denomination: Four,
                },
                Card {
                    suit: Diamonds,
                    denomination: Five,
                },
                Card {
                    suit: Diamonds,
                    denomination: Six,
                },
                Card {
                    suit: Diamonds,
                    denomination: Seven,
                },
                Card {
                    suit: Diamonds,
                    denomination: Eight,
                },
                Card {
                    suit: Diamonds,
                    denomination: Nine,
                },
                Card {
                    suit: Diamonds,
                    denomination: Ten,
                },
                Card {
                    suit: Diamonds,
                    denomination: Jack,
                },
                Card {
                    suit: Diamonds,
                    denomination: Queen,
                },
                Card {
                    suit: Diamonds,
                    denomination: King,
                },
                Card {
                    suit: Diamonds,
                    denomination: Two,
                },
            ]),
            Err(BBError::Duplicate(Card {
                suit: Diamonds,
                denomination: Two
            }))
        );
    }

    #[test_case("♠:AKQJT98765432", HandType::SingleSuited(Spades) ; "13-0-0-0")]
    #[test_case("♥:AKQJT98765,♠:432", HandType::SingleSuited(Hearts) ; "10-3-0-0")]
    #[test_case("♦:AKQJT9,♥:876,♠:54,♣:32", HandType::SingleSuited(Diamonds) ; "6-3-2-2")]
    #[test_case("♠:AKQJT9876,♥:5432", HandType::TwoSuited(Spades, Hearts) ; "9-4-0-0")]
    #[test_case("♠:AKQJT,♦:9876,♥:543,♣:2", HandType::TwoSuited(Spades, Diamonds); "5-4-3-1")]
    #[test_case("♦:AKQJT,♣:9876,♠:54,♥:32", HandType::TwoSuited(Diamonds, Clubs); "5-4-2-2")]
    #[test_case("♠:AKQJT,♥:9876,♦:5432", HandType::ThreeSuited(Spades, Hearts, Diamonds); "5-4-4-0")]
    #[test_case("♣:AKQJ,♥:T987,♦:6543,♠:2", HandType::ThreeSuited(Hearts, Diamonds, Clubs); "4-4-4-1")]
    #[test_case("♠:AKQJT,♥:987,♦:654,♣:32", HandType::Balanced(Some(Spades)) ; "5-3-3-2")]
    #[test_case("♠:AKQJ,♥:T98,♦:765,♣:432", HandType::Balanced(None); "4-3-3-3")]
    #[test_case("♠:AKQJ,♥:T987,♦:654,♣:32", HandType::Balanced(None); "4-4-3-2")]
    fn test_hand_type(hand: &str, expected_hand_type: HandType) {
        let hand = Hand::from_str(hand).unwrap();
        assert_eq!(hand.hand_type(), expected_hand_type)
    }
}
