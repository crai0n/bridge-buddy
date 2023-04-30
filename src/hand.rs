use crate::card::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 13],
}

impl Hand {
    pub fn new(mut cards: [Card; 13]) -> Self {
        cards.sort_unstable();
        for i in 0..12 {
            if cards[i] == cards[i + 1] {
                panic!("invalid hand - duplicate cards");
            }
        }
        Hand { cards }
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
        for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            // Don't write a new line for the first suit: spades
            if suit != Suit::Spades {
                writeln!(f)?;
            }
            write!(f, "{}: ", suit)?;
            for card in self.cards_in_rev(suit) {
                write!(f, "{}", card.denomination.clone())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Denomination::*;
    use super::Suit::*;
    use super::*;

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
        assert!(!hand.contains(&Card {
            suit: Diamonds,
            denomination: Queen
        }));
        assert!(hand.contains(&Card {
            suit: Diamonds,
            denomination: Ace
        }));
        assert_eq!(hand.high_card_points(), 21);
        assert_eq!(format!("{}", hand), "♠: AKQJT98762\n♥: \n♦: AK\n♣: A");
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
}
