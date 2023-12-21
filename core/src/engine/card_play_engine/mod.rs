use crate::game::game_state::{CardPlay, GameState, OpeningLead};
use crate::primitives::Card;

pub mod mock_card_play_engine;

pub trait SelectCard {
    fn select_card(&self, state: &GameState<CardPlay>) -> Card;

    fn select_opening_lead(&self, state: &GameState<OpeningLead>) -> Card;
}
