use crate::engine::subjective_game_view::SubjectiveGameDataView;
use crate::game::game_data::{CardPlay, OpeningLead};
use crate::primitives::Card;

pub mod mock_card_play_engine;

pub trait SelectCard {
    fn select_card(&self, state: SubjectiveGameDataView<CardPlay>) -> Card;

    fn select_opening_lead(&self, state: SubjectiveGameDataView<OpeningLead>) -> Card;
}
