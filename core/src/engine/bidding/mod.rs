use crate::game::game_state::{Bidding, GameState};
use crate::primitives::bid::Bid;

pub mod mock;

pub trait BidFinder {
    fn find_bid(&self, state: &GameState<Bidding>) -> Bid;
}
