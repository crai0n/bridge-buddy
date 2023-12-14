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
