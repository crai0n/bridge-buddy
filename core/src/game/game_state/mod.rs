use crate::error::BBError;
use crate::primitives::deal::Seat;
pub use bidding::Bidding;
pub use card_play::CardPlay;
pub use ended::Ended;
pub use opening_lead::OpeningLead;
pub use waiting_for_dummy::WaitingForDummy;

mod bidding;
mod card_play;
mod ended;
mod opening_lead;
mod waiting_for_dummy;

#[derive(Debug, Clone)]
pub struct GameState<Phase> {
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
