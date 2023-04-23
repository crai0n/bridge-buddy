use crate::card::*;

pub struct Hand {
    cards: [Card; 13],
}

impl Hand {
    pub fn new(cards: [Card; 13]) -> Self {
        let hand = Hand { cards };
        hand.validate();
        hand
    }

    pub fn cards(&self) -> &[Card; 13] {
        &self.cards
    }

    pub fn cards_in(&self, suit: Suit) -> impl Iterator<Item = &Card> {
        self.cards.iter().filter(move |&x| x.suit == Some(suit))
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    pub fn high_card_points(&self) -> u8 {
        self.cards.iter().fold(0, |acc, card| match &card.denomination {
            Denomination::Ace => acc + 4,
            Denomination::King => acc + 3,
            Denomination::Queen => acc + 2,
            Denomination::Jack => acc + 1,
            _ => acc,
        })
    }

    fn validate(&self) {
        for i in 0..13 {
            for j in i + 1..13 {
                assert_ne!(self.cards[i], self.cards[j], "invalid hand - duplicate cards")
            }
        }
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
                suit: Some(Clubs),
                denomination: Ace,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Ace,
            },
            Card {
                suit: Some(Hearts),
                denomination: Ace,
            },
            Card {
                suit: Some(Spades),
                denomination: Ace,
            },
            Card {
                suit: Some(Spades),
                denomination: King,
            },
            Card {
                suit: Some(Spades),
                denomination: Queen,
            },
            Card {
                suit: Some(Spades),
                denomination: Jack,
            },
            Card {
                suit: Some(Spades),
                denomination: Ten,
            },
            Card {
                suit: Some(Spades),
                denomination: Nine,
            },
            Card {
                suit: Some(Spades),
                denomination: Eight,
            },
            Card {
                suit: Some(Spades),
                denomination: Seven,
            },
            Card {
                suit: Some(Spades),
                denomination: Six,
            },
            Card {
                suit: Some(Spades),
                denomination: Two,
            },
        ]);
        assert_eq!(
            hand.cards()[1],
            Card {
                suit: Some(Diamonds),
                denomination: Ace,
            }
        );
        assert_eq!(hand.cards_in(Spades).count(), 10);
        assert!(!hand.contains(&Card {
            suit: Some(Diamonds),
            denomination: King
        }));
        assert!(hand.contains(&Card {
            suit: Some(Diamonds),
            denomination: Ace
        }));
        assert_eq!(hand.high_card_points(), 22);
    }

    #[test]
    #[should_panic(expected = "invalid hand - duplicate cards")]
    fn test_hand_validation() {
        Hand::new([
            Card {
                suit: Some(Diamonds),
                denomination: Two,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Three,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Four,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Five,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Six,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Seven,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Eight,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Nine,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Ten,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Jack,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Queen,
            },
            Card {
                suit: Some(Diamonds),
                denomination: King,
            },
            Card {
                suit: Some(Diamonds),
                denomination: Two,
            },
        ]);
    }
}
