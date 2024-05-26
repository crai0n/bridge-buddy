use crate::engine::subjective_game_view::SubjectiveGamePhaseStateView;
use crate::game::game_phase_states::BiddingState;
use crate::primitives::bid::Bid;

pub mod mock_bidding_engine;

pub trait SelectBid {
    fn select_bid(&self, state: SubjectiveGamePhaseStateView<BiddingState>) -> Bid;
}
