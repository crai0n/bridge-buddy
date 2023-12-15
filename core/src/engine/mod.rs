use crate::primitives::player_event::PlayerEvent;

pub mod bidding;
pub mod card_play;
pub mod evaluator;

pub trait MoveFinder {
    fn find_move() -> PlayerEvent;
}
