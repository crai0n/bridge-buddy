use crate::move_generator::MoveGenerator;
use crate::primitives::DdsMove;
use crate::state::VirtualState;

impl MoveGenerator {
    pub fn calc_priority_playing_third<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        if moves.first().unwrap().card.suit != state.suit_to_follow().unwrap() {
            Self::calc_priority_void(moves, state);
        } else {
            for dds_move in moves {
                if dds_move.card > state.currently_winning_card().unwrap() {
                    dds_move.priority += dds_move.card.rank as isize;
                }
            }
        }
    }
}
