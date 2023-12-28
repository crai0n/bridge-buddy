use crate::engine::bidding_engine::SelectBid;
use crate::engine::card_play_engine::SelectCard;
use crate::error::BBError;
use crate::game::GameState;
use crate::primitives::bid::Bid;
use crate::primitives::game_event::GameEvent;
use crate::primitives::Card;

pub mod bidding_engine;
pub mod card_play_engine;
pub mod hand_evaluation;
pub mod mock_bridge_engine;
pub mod subjective_game_view;

pub enum Move {
    Bid(Bid),
    Card(Card),
}

pub trait SelectMove: SelectCard + SelectBid {
    fn select_move(&self, game: &GameState) -> Result<Move, BBError> {
        match game {
            GameState::Bidding(state) => {
                let bid = self.select_bid(state);
                Ok(Move::Bid(bid))
            }
            GameState::OpeningLead(state) => {
                let card = self.select_opening_lead(state);
                Ok(Move::Card(card))
            }
            GameState::CardPlay(state) => {
                let card = self.select_card(state);
                Ok(Move::Card(card))
            }
            GameState::WaitingForDummy(_) => Err(BBError::OutOfTurn(None)),
            GameState::Ended(_) => Err(BBError::GameHasEnded),
        }
    }

    fn process_game_event(&mut self, event: GameEvent) -> Result<(), BBError>;
}
