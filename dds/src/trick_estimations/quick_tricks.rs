use crate::state::VirtualState;
use bridge_buddy_core::primitives::deal::Seat;
pub fn quick_tricks_for_player<const N: usize>(state: &VirtualState<N>, player: Seat) -> usize {
    state.quick_tricks_for_player(player)
}
