mod leading;
mod playing_last;
mod playing_second;
mod playing_third;

use crate::card_manager::card_tracker::SUIT_ARRAY;
use crate::primitives::DdsMove;
use crate::state::VirtualState;
use rand::Rng;

pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn generate_moves<const N: usize>(state: &VirtualState<N>, move_ordering: bool) -> Vec<DdsMove> {
        let valid_moves = state.valid_moves().map(DdsMove::new);
        let mut unique_moves = Self::select_one_move_per_sequence(valid_moves);
        if move_ordering {
            Self::calc_priority(&mut unique_moves, state);
            Self::sort_moves_by_priority_descending(&mut unique_moves);
        } else {
            Self::randomize_priority(&mut unique_moves);
        }
        unique_moves
    }

    fn randomize_priority(moves: &mut [DdsMove]) {
        let mut rng = rand::thread_rng();
        for candidate in moves {
            candidate.priority = rng.gen();
        }
    }

    fn calc_priority<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        match state.count_cards_in_current_trick() {
            0 => Self::calc_priority_leading(moves, state),
            1 => Self::calc_priority_playing_second(moves, state),
            2 => Self::calc_priority_playing_third(moves, state),
            3 => Self::calc_priority_playing_last(moves, state),
            _ => unreachable!(),
        };
    }

    fn suit_weights_for_discarding<const N: usize>(state: &VirtualState<N>) -> [isize; 4] {
        // Taken from Bo Haglund's Double Dummy Solver
        let player = state.next_to_play();
        let my_hand = state.cards_of(player);
        SUIT_ARRAY.map(|suit| {
            if my_hand.has_doubleton_runner_up_in(suit) {
                1
            } else if my_hand.has_singleton_winner_in(suit) {
                0
            } else {
                my_hand.count_cards_in(suit) as isize * 64 / 36
            }
        })
    }

    pub fn calc_priority_nt_void<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        let suit_weights = Self::suit_weights_for_discarding(state);

        for candidate in moves.iter_mut() {
            let suit = candidate.card.suit;
            let suit_weight = suit_weights[suit as usize];

            candidate.priority += suit_weight - candidate.card.rank as isize;
        }
    }

    fn try_to_win_as_cheaply_as_possible<const N: usize>(state: &VirtualState<N>, candidate: &mut DdsMove) {
        if candidate.card.rank > state.currently_winning_card().unwrap().rank {
            candidate.priority += 50 - candidate.card.rank as isize;
        } else {
            candidate.priority -= candidate.card.rank as isize;
        }
    }

    fn sort_moves_by_priority_descending(moves: &mut [DdsMove]) {
        moves.sort_unstable_by(|a, b| b.priority.cmp(&a.priority));
    }

    fn select_one_move_per_sequence(moves: impl Iterator<Item = DdsMove>) -> Vec<DdsMove> {
        let mut output: Vec<DdsMove> = vec![];
        for candidate_move in moves {
            if let Some(last) = output.last_mut() {
                if candidate_move.card.touches(&last.card) {
                    last.sequence_length += 1;
                } else {
                    output.push(candidate_move);
                }
            } else {
                output.push(candidate_move);
            }
        }
        output
    }
}
