use crate::engine::subjective_game_view::SubjectiveGamePhaseStateView;
use crate::game::game_phase_states::{CardPlayState, OpeningLeadState};
use crate::primitives::Card;

pub mod mock_card_play_engine;

pub trait SelectCard {
    fn select_card(&self, state: SubjectiveGamePhaseStateView<CardPlayState>) -> Card;

    fn select_opening_lead(&self, state: SubjectiveGamePhaseStateView<OpeningLeadState>) -> Card;
}
