use crate::state::VirtualState;
use bridge_buddy_core::primitives::deal::Seat;

pub fn losing_tricks_for_player<const N: usize>(_state: &VirtualState<N>, _player: Seat) -> usize {
    0
}
