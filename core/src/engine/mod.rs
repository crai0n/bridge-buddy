use crate::engine::bidding_engine::SelectBid;
use crate::engine::card_play_engine::SelectCard;
use crate::error::BBError;
use crate::game::Game;
use crate::primitives::bid::Bid;
use crate::primitives::Card;

pub mod bidding_engine;
pub mod card_play_engine;
pub mod hand_evaluation;
pub mod mock_bridge_engine;

pub enum Move {
    Bid(Bid),
    Card(Card),
}

pub trait SelectMove: SelectCard + SelectBid {
    fn select_move(&self, game: &Game) -> Result<Move, BBError> {
        match game {
            Game::Bidding(state) => {
                let bid = self.select_bid(state);
                Ok(Move::Bid(bid))
            }
            Game::OpeningLead(state) => {
                let card = self.select_opening_lead(state);
                Ok(Move::Card(card))
            }
            Game::CardPlay(state) => {
                let card = self.select_card(state);
                Ok(Move::Card(card))
            }
            Game::WaitingForDummy(_) => Err(BBError::OutOfTurn(None)),
            Game::Ended(_) => Err(BBError::GameHasEnded),
        }
    }
}
