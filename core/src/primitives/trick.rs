use crate::primitives::deal::PlayerPosition;
use crate::primitives::Card;

trait Trick {
    fn lead(&self) -> PlayerPosition;
    fn cards(&self) -> &[Card];
}

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
}

impl Trick for ActiveTrick {
    fn lead(&self) -> PlayerPosition {
        self.lead
    }
    fn cards(&self) -> &[Card] {
        &self.cards
    }
}

#[derive(Clone)]
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
