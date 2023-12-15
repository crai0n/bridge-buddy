use crate::game::game_state::{CardPlay, GameState, OpeningLead};
use crate::primitives::deal::Seat;
use crate::primitives::Card;

pub mod mock;

pub trait CardFinder {
    fn find_card_for(&self, state: &GameState<CardPlay>, seat: Seat) -> Card;

    fn find_opening_lead(&self, state: &GameState<OpeningLead>) -> Card;
}
