use crate::engine::subjective_game_view::SubjectiveSeat;
use crate::primitives::Card;

pub struct SubjectiveTrick {
    lead: SubjectiveSeat,
    cards: Vec<Card>,
}

impl SubjectiveTrick {
    pub fn new(lead: SubjectiveSeat) -> Self {
        SubjectiveTrick {
            lead,
            cards: Vec::new(),
        }
    }

    pub fn with_cards(lead: SubjectiveSeat, cards: &[Card]) -> Self {
        SubjectiveTrick {
            lead,
            cards: cards.to_vec(),
        }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn lead(&self) -> SubjectiveSeat {
        self.lead
    }
}
