use crate::game::game_state::{Bidding, GameState};
use crate::primitives::bid::Bid;

pub mod mock_bidding_engine;

pub trait SelectBid {
    fn select_bid(&self, state: &GameState<Bidding>) -> Bid;
}
