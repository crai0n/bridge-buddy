use crate::error::BBError;
use crate::primitives::deal::Seat;
use crate::primitives::Contract;
pub use bidding_state::BiddingState;
pub use card_play_state::CardPlayState;
pub use ended_state::EndedState;
pub use opening_lead_state::OpeningLeadState;
pub use waiting_for_dummy_state::WaitingForDummyState;

mod bidding_state;
mod card_play_state;
mod ended_state;
mod opening_lead_state;
mod waiting_for_dummy_state;

pub trait NextToPlay {
    fn next_to_play(&self) -> Seat;

    fn validate_turn_order(&self, player: Seat) -> Result<(), BBError> {
        let turn = self.next_to_play();
        if player != turn {
            return Err(BBError::OutOfTurn(Some(turn)));
        }
        Ok(())
    }
}

pub trait GamePhaseState {
    fn implied_contract(&self) -> Option<Contract>;
    fn dealer(&self) -> Seat;
}
