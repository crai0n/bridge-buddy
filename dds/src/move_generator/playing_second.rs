use crate::move_generator::MoveGenerator;
use crate::primitives::DdsMove;
use crate::state::VirtualState;

impl MoveGenerator {
    pub fn calc_priority_playing_second<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        if moves.first().unwrap().card.suit != state.suit_to_follow().unwrap() {
            Self::calc_priority_nt_discard(moves, state);
        } else {
            for dds_move in moves.iter_mut() {
                dds_move.priority -= dds_move.card.rank as isize;
            }
        }
    }
}
