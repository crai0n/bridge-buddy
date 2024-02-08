use crate::move_generator::MoveGenerator;
use crate::primitives::DdsMove;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::Suit;

impl MoveGenerator {
    pub fn calc_priority_playing_third<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        if moves.first().unwrap().card.suit != state.suit_to_follow().unwrap() {
            match state.trump_suit() {
                None => Self::calc_priority_nt_discard(moves, state),
                Some(trump_suit) => Self::calc_priority_playing_third_trump_void(moves, state, trump_suit),
            }
        } else {
            match state.trump_suit() {
                None => Self::calc_priority_playing_third_nt_not_void(moves, state),
                Some(trump_suit) => Self::calc_priority_playing_third_trump_not_void(moves, state, trump_suit),
            }
        }
    }

    pub fn calc_priority_playing_third_nt_not_void<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        for dds_move in moves {
            if dds_move.card > state.currently_winning_card().unwrap() {
                dds_move.priority += dds_move.card.rank as isize;
            }
        }
    }

    pub fn calc_priority_playing_third_trump_void<const N: usize>(
        moves: &mut [DdsMove],
        state: &VirtualState<N>,
        _trump_suit: Suit,
    ) {
        for dds_move in moves {
            if dds_move.card > state.currently_winning_card().unwrap() {
                dds_move.priority += dds_move.card.rank as isize;
            }
        }
    }

    pub fn calc_priority_playing_third_trump_not_void<const N: usize>(
        moves: &mut [DdsMove],
        state: &VirtualState<N>,
        _trump_suit: Suit,
    ) {
        for dds_move in moves {
            if dds_move.card > state.currently_winning_card().unwrap() {
                dds_move.priority += dds_move.card.rank as isize;
            }
        }
    }
}
