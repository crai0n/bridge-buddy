use crate::game::game_data::{Bidding, GameData};
use crate::primitives::bid::Bid;

pub mod mock_bidding_engine;

pub trait SelectBid {
    fn select_bid(&self, state: &GameData<Bidding>) -> Bid;
}
