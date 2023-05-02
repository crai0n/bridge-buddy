use std::collections::BTreeMap;

use crate::card::*;
use itertools::Itertools;
use strum::IntoEnumIterator;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 13],
    suit_lengths: BTreeMap<Suit, u8>,
    hand_type: HandType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandType {
    ThreeSuited(Suit, Suit, Suit),
    TwoSuited(Suit, Suit),
    SingleSuited(Suit),
    Balanced(Option<Suit>), // might contain a 5-card suit
}

impl Hand {
    pub fn new(mut cards: [Card; 13]) -> Self {
        cards.sort_unstable();

        for i in 0..12 {
            if cards[i] == cards[i + 1] {
                panic!("invalid hand - duplicate cards");
            }
        }

        let mut sorted_suit_lengths = [
            (Suit::Clubs, 0),
            (Suit::Diamonds, 0),
            (Suit::Hearts, 0),
            (Suit::Spades, 0),
        ];
        for card in &cards {
            match card.suit {
                // suit_lengths is in "intuitive order" of Suits
                Suit::Clubs => sorted_suit_lengths[0].1 += 1,
                Suit::Diamonds => sorted_suit_lengths[1].1 += 1,
                Suit::Hearts => sorted_suit_lengths[2].1 += 1,
                Suit::Spades => sorted_suit_lengths[3].1 += 1,
            };
        }
        sorted_suit_lengths.sort_unstable_by(|x, y| x.1.cmp(&y.1)); //longest suit will be last

        // determine hand-type
        let hand_type: HandType;
        if sorted_suit_lengths[1].1 >= 4 {
            // three suits with at least 4 cards is a three-suiter
            hand_type = HandType::ThreeSuited(
                sorted_suit_lengths[3].0,
                sorted_suit_lengths[2].0,
                sorted_suit_lengths[1].0,
            );
        } else if sorted_suit_lengths[3].1 >= 5 && sorted_suit_lengths[2].1 >= 4 {
            // two-suiter
            hand_type = HandType::TwoSuited(sorted_suit_lengths[3].0, sorted_suit_lengths[2].0);
        } else if sorted_suit_lengths[3].1 >= 6 {
            // one-suiter
            hand_type = HandType::SingleSuited(sorted_suit_lengths[3].0);
        } else {
            // balanced
            match sorted_suit_lengths[3].1 == 5 {
                true => hand_type = HandType::Balanced(Some(sorted_suit_lengths[3].0)),
                false => hand_type = HandType::Balanced(None),
            }
        }

        let suit_lengths = BTreeMap::<Suit, u8>::from(sorted_suit_lengths);

        Hand {
            cards,
            suit_lengths,
            hand_type,
        }
    }

    pub fn from_str(string: &str) -> Result<Hand, ()> {
        let mut cards: Vec<Card> = vec![];
        for cards_in_suit in string.split(['\n', ',']) {
            let (suit, denominations) = cards_in_suit.split_once(':').ok_or(())?;
            for denomination in denominations.trim().chars() {
                let card = Card {
                    denomination: Denomination::from_char(denomination)?,
                    suit: Suit::from_char(suit.trim().chars().next().unwrap())?,
                };
                cards.push(card)
            }
        }
        Ok(Hand::new(cards.try_into().unwrap()))
    }

    pub fn cards(&self) -> impl DoubleEndedIterator<Item = &Card> {
        self.cards.iter()
    }

    pub fn cards_rev(&self) -> impl DoubleEndedIterator<Item = &Card> {
        self.cards().rev()
    }

    pub fn cards_in(&self, suit: Suit) -> impl DoubleEndedIterator<Item = &Card> {
        self.cards.iter().filter(move |&card| card.suit == suit)
    }

    pub fn cards_in_rev(&self, suit: Suit) -> impl DoubleEndedIterator<Item = &Card> {
        self.cards_in(suit).rev()
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    pub fn length_in(&self, suit: Suit) -> u8 {
        self.suit_lengths[&suit]
    }

    pub fn high_card_points(&self) -> u8 {
        self.cards.iter().fold(0, |acc, card| match card.denomination {
            Denomination::Ace => acc + 4,
            Denomination::King => acc + 3,
            Denomination::Queen => acc + 2,
            Denomination::Jack => acc + 1,
            _ => acc,
        })
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for suit in Suit::iter().rev() {
            // Spades, then Hearts, ...
            write!(f, "{}: ", suit)?;
            for card in self.cards_in_rev(suit) {
                write!(f, "{}", card.denomination)?;
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Denomination::*;
    use super::Suit::*;
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_hand_types() {
        let hand = Hand::new([
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
        ]);
        assert_eq!(hand.hand_type, HandType::Balanced(Some(Spades)));
    }

    #[test]
    fn test_methods() {
        let hand = Hand::new([
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
        ]);
        assert_eq!(
            hand.cards().nth(1).unwrap(),
            &Card {
                suit: Diamonds,
                denomination: King,
            }
        );
        assert_eq!(hand.cards_in(Spades).count(), 10);
        assert_eq!(hand.cards_in(Hearts).count(), 0);
        assert_eq!(
            hand.suit_lengths,
            BTreeMap::from([(Spades, 10), (Hearts, 0), (Diamonds, 2), (Clubs, 1)])
        );
        assert!(!hand.contains(&Card {
            suit: Diamonds,
            denomination: Queen
        }));
        assert!(hand.contains(&Card {
            suit: Diamonds,
            denomination: Ace
        }));
        assert_eq!(hand.high_card_points(), 21);
        assert_eq!(hand.hand_type, HandType::SingleSuited(Suit::Spades));
        assert_eq!(format!("{}", hand), "♠: AKQJT98762\n♥: \n♦: AK\n♣: A\n");
        assert_eq!(hand, Hand::from_str("H:, ♠:9J7A2T6K8Q,♦: AK, C: A").unwrap())
    }

    #[test]
    #[should_panic(expected = "invalid hand - duplicate cards")]
    fn test_hand_validation() {
        Hand::new([
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
        ]);
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
        assert_eq!(hand.hand_type, expected_hand_type)
    }
}
