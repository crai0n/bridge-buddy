use crate::game::game_data::{CardPlay, GameData, OpeningLead};
use crate::primitives::Card;

pub mod mock_card_play_engine;

pub trait SelectCard {
    fn select_card(&self, state: &GameData<CardPlay>) -> Card;

    fn select_opening_lead(&self, state: &GameData<OpeningLead>) -> Card;
}
