use crate::engine::bidding_engine::SelectBid;
use crate::engine::card_play_engine::SelectCard;
use crate::engine::subjective_game_view::SubjectiveGameStateView;
use crate::error::BBError;
use crate::primitives::bid::Bid;
use crate::primitives::game_event::GameEvent;
use crate::primitives::Card;

pub mod bidding_engine;
pub mod card_play_engine;
mod engine_state;
pub mod hand_evaluation;
pub mod mock_bridge_engine;
pub mod subjective_game_view;

pub enum Move {
    Bid(Bid),
    Card(Card),
}

pub trait SelectMove: SelectCard + SelectBid {
    fn select_move(&self, game: SubjectiveGameStateView) -> Result<Move, BBError> {
        match game {
            SubjectiveGameStateView::Bidding(state) => {
                let bid = self.select_bid(state);
                Ok(Move::Bid(bid))
            }
            SubjectiveGameStateView::OpeningLead(state) => {
                let card = self.select_opening_lead(state);
                Ok(Move::Card(card))
            }
            SubjectiveGameStateView::CardPlay(state) => {
                let card = self.select_card(state);
                Ok(Move::Card(card))
            }
            SubjectiveGameStateView::WaitingForDummy(_) => Err(BBError::OutOfTurn(None)),
            SubjectiveGameStateView::Ended(_) => Err(BBError::GameHasEnded),
        }
    }

    fn process_game_event(&mut self, event: GameEvent, game_state: SubjectiveGameStateView) -> Result<(), BBError>;
}
