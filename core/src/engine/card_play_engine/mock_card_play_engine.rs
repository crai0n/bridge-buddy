use crate::engine::card_play_engine::SelectCard;
use crate::game::game_data::{CardPlay, GameData, NextToPlay, OpeningLead};
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Suit};

pub struct MockCardPlayEngine {
    seat: Seat,
}

impl MockCardPlayEngine {
    pub fn new(seat: Seat) -> Self {
        Self { seat }
    }

    fn pick_opening_lead(&self, state: &GameData<OpeningLead>) -> Card {
        let hand = state.inner.hand_manager.known_remaining_cards_of(self.seat);
        let card = hand.first().unwrap();
        *card
    }

    fn pick_card(&self, state: &GameData<CardPlay>) -> Card {
        match state.inner.trick_manager.suit_to_follow() {
            None => self.pick_lead(state),
            Some(suit) => self.pick_discard(suit, state),
        }
    }

    fn pick_lead(&self, state: &GameData<CardPlay>) -> Card {
        let remaining_cards = state.inner.hand_manager.known_remaining_cards_of(state.next_to_play());
        let card = remaining_cards.first().unwrap();
        *card
    }

    fn pick_discard(&self, suit_to_follow: Suit, state: &GameData<CardPlay>) -> Card {
        let remaining_cards = state.inner.hand_manager.known_remaining_cards_of(state.next_to_play());
        if let Some(card) = remaining_cards.iter().find(|x| x.suit == suit_to_follow) {
            *card
        } else {
            *remaining_cards.first().unwrap()
        }
    }
}

impl SelectCard for MockCardPlayEngine {
    fn select_card(&self, state: &GameData<CardPlay>) -> Card {
        self.pick_card(state)
    }

    fn select_opening_lead(&self, state: &GameData<OpeningLead>) -> Card {
        self.pick_opening_lead(state)
    }
}

impl Default for MockCardPlayEngine {
    fn default() -> Self {
        Self::new(Seat::South)
    }
}
