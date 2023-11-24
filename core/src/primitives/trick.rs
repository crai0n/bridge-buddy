use crate::primitives::deal::PlayerPosition;
use crate::primitives::{Card, Suit};
use std::fmt::{Display, Formatter};

pub trait Trick {
    fn lead(&self) -> PlayerPosition;
    fn cards(&self) -> &[Card];
}

#[derive(Debug, Clone)]
pub struct ActiveTrick {
    lead: PlayerPosition,
    cards: Vec<Card>,
}

impl ActiveTrick {
    pub fn new(lead: PlayerPosition) -> ActiveTrick {
        ActiveTrick {
            lead,
            cards: Vec::with_capacity(4),
        }
    }
    pub fn play(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn suit_to_follow(&self) -> Option<Suit> {
        self.cards.first().map(|x| x.suit)
    }
}

impl Trick for ActiveTrick {
    fn lead(&self) -> PlayerPosition {
        self.lead
    }
    fn cards(&self) -> &[Card] {
        &self.cards
    }
}

impl Display for ActiveTrick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ", self.lead)?;
        for card in self.cards.iter() {
            write!(f, "{}", card)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PlayedTrick {
    lead: PlayerPosition,
    cards: Vec<Card>,
    winner: PlayerPosition,
}

impl PlayedTrick {
    pub fn from_active_trick(active: ActiveTrick, winner: PlayerPosition) -> Self {
        PlayedTrick {
            lead: active.lead(),
            cards: active.cards().into(),
            winner,
        }
    }

    pub fn winner(&self) -> PlayerPosition {
        self.winner
    }

    pub fn is_won_by(&self, player: PlayerPosition) -> bool {
        self.winner == player
    }
}

impl Trick for PlayedTrick {
    fn lead(&self) -> PlayerPosition {
        self.lead
    }
    fn cards(&self) -> &[Card] {
        &self.cards
    }
}
