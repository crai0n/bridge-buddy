use crate::engine::card_play_engine::SelectCard;
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

    fn pick_opening_lead(&self, state: &GameState<OpeningLead>) -> Card {
        let hand = state.inner.hand_manager.known_remaining_cards_of(self.seat);
        let card = hand.first().unwrap();
        *card
    }

    fn pick_card_for(&self, state: &GameState<CardPlay>, seat: Seat) -> Card {
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

impl SelectCard for MockCardPlayEngine {
    fn select_card_for(&self, state: &GameState<CardPlay>, seat: Seat) -> Card {
        self.pick_card_for(state, seat)
    }

    fn select_opening_lead(&self, state: &GameState<OpeningLead>) -> Card {
        self.pick_opening_lead(state)
    }

    fn seat(&self) -> Seat {
        self.seat
    }
}

impl Default for MockCardPlayEngine {
    fn default() -> Self {
        Self::new(Seat::South)
    }
}
