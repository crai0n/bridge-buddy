use crate::game::game_state::{Bidding, CardPlay, GameState, OpeningLead};
use crate::primitives::bid::{AuxiliaryBid, Bid};
use crate::primitives::deal::PlayerPosition;
use crate::primitives::{Card, Suit};

pub enum SubjectiveVulnerability {
    None,
    Us,
    Them,
    All,
}

pub struct MockBiddingEngine {}

pub struct MockCardPlayEngine {
    seat: PlayerPosition,
}

impl MockBiddingEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find_bid(&self, state: &GameState<Bidding>) -> Bid {
        match state.inner.bid_manager.lowest_available_contract_bid() {
            Some(bid) => Bid::Contract(bid),
            None => Bid::Auxiliary(AuxiliaryBid::Pass),
        }
    }
}

impl Default for MockBiddingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MockCardPlayEngine {
    pub fn new(seat: PlayerPosition) -> Self {
        Self { seat }
    }

    pub fn pick_opening_lead(&self, state: &GameState<OpeningLead>) -> Card {
        let hand = state.inner.hand_manager.known_remaining_cards_of(self.seat);
        let card = hand.first().unwrap();
        *card
    }

    pub fn pick_card(&self, state: &GameState<CardPlay>) -> Card {
        match state.inner.trick_manager.suit_to_follow() {
            None => self.pick_lead(state),
            Some(suit) => self.pick_discard(suit, state),
        }
    }

    fn pick_lead(&self, state: &GameState<CardPlay>) -> Card {
        let remaining_cards = state.inner.hand_manager.known_remaining_cards_of(self.seat);
        let card = remaining_cards.first().unwrap();
        *card
    }

    fn pick_discard(&self, suit: Suit, state: &GameState<CardPlay>) -> Card {
        let remaining_cards = state.inner.hand_manager.known_remaining_cards_of(self.seat);
        if let Some(card) = remaining_cards.iter().find(|x| x.suit == suit) {
            *card
        } else {
            *remaining_cards.first().unwrap()
        }
    }
}

impl Default for MockCardPlayEngine {
    fn default() -> Self {
        Self::new(PlayerPosition::South)
    }
}
