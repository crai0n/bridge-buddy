use crate::game::game_state::{CardPlay, GameState, OpeningLead};
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Suit};

pub struct MockCardPlayEngine {
    seat: Seat,
}

impl MockCardPlayEngine {
    pub fn new(seat: Seat) -> Self {
        Self { seat }
    }

    pub fn pick_opening_lead(&self, state: &GameState<OpeningLead>) -> Card {
        let hand = state.inner.hand_manager.known_remaining_cards_of(self.seat);
        let card = hand.first().unwrap();
        *card
    }

    pub fn pick_card_for(&self, state: &GameState<CardPlay>, seat: Seat) -> Card {
        match state.inner.trick_manager.suit_to_follow() {
            None => self.pick_lead_for(state, seat),
            Some(suit) => self.pick_discard_for(suit, state, seat),
        }
    }

    fn pick_lead_for(&self, state: &GameState<CardPlay>, seat: Seat) -> Card {
        let remaining_cards = state.inner.hand_manager.known_remaining_cards_of(seat);
        let card = remaining_cards.first().unwrap();
        *card
    }

    fn pick_discard_for(&self, suit: Suit, state: &GameState<CardPlay>, seat: Seat) -> Card {
        let remaining_cards = state.inner.hand_manager.known_remaining_cards_of(seat);
        if let Some(card) = remaining_cards.iter().find(|x| x.suit == suit) {
            *card
        } else {
            *remaining_cards.first().unwrap()
        }
    }
}

impl Default for MockCardPlayEngine {
    fn default() -> Self {
        Self::new(Seat::South)
    }
}
