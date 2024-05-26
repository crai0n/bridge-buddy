use crate::error::BBError;
use crate::primitives::deal::Seat;
pub use bidding::BiddingState;
pub use card_play::CardPlayState;
pub use ended::EndedState;
pub use opening_lead::OpeningLeadState;
pub use waiting_for_dummy::WaitingForDummyState;

mod bidding;
mod card_play;
mod ended;
mod opening_lead;
mod waiting_for_dummy;

#[derive(Debug, Clone)]
pub struct GameData<Phase> {
    pub inner: Phase,
}

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
