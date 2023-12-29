use crate::engine::subjective_game_view::SubjectiveGameDataView;
use crate::game::game_data::Bidding;
use crate::primitives::bid::Bid;

pub mod mock_bidding_engine;

pub trait SelectBid {
    fn select_bid(&self, state: SubjectiveGameDataView<Bidding>) -> Bid;
}
