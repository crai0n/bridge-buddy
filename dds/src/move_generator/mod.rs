use crate::primitives::DdsMove;
use crate::state::VirtualState;

pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn generate_moves<const N: usize>(state: &VirtualState<N>, move_ordering: bool) -> Vec<DdsMove> {
        let valid_moves = state.valid_moves();
        let mut unique_moves = Self::select_one_move_per_sequence(&valid_moves);
        if move_ordering {
            Self::prioritize_moves(&mut unique_moves, state);
            Self::sort_moves_by_priority_descending(&mut unique_moves);
        }
        unique_moves
    }

    fn prioritize_moves<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        match state.count_cards_in_current_trick() {
            0 => Self::prioritize_moves_for_leading_hand(moves, state),
            1 => Self::prioritize_moves_for_second_hand(moves, state),
            2 => Self::prioritize_moves_for_third_hand(moves, state),
            3 => Self::prioritize_moves_for_last_hand(moves, state),
            _ => unreachable!(),
        };
    }

    fn prioritize_moves_for_leading_hand<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        for dds_move in moves {
            match state.trumps() {
                None => {
                    if dds_move.sequence_length >= 3 {
                        dds_move.priority += 50;
                    }
                }
                Some(trump_suit) => {
                    if dds_move.sequence_length >= 2 {
                        dds_move.priority += 50;
                    }

                    let our_trump_count = state.count_this_sides_trump_cards();
                    let opponents_trump_count = state.count_opponents_trump_cards();

                    if our_trump_count >= opponents_trump_count && dds_move.card.suit == trump_suit {
                        dds_move.priority += 100;
                    }
                }
            }
        }
    }
    fn prioritize_moves_for_second_hand<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        if moves.first().unwrap().card.suit != state.suit_to_follow().unwrap() {
            Self::prioritize_cards_for_discard(moves, state);
        } else {
            for dds_move in moves.iter_mut() {
                dds_move.priority -= dds_move.card.rank as isize;
            }
        }
    }
    fn prioritize_moves_for_third_hand<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        if moves.first().unwrap().card.suit != state.suit_to_follow().unwrap() {
            Self::prioritize_cards_for_discard(moves, state);
        } else {
            for dds_move in moves {
                if dds_move.card > state.currently_winning_card().unwrap() {
                    dds_move.priority += dds_move.card.rank as isize;
                }
            }
        }
    }
    fn prioritize_moves_for_last_hand<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        if moves.first().unwrap().card.suit != state.suit_to_follow().unwrap() {
            Self::prioritize_cards_for_discard(moves, state);
        } else {
            for dds_move in moves {
                if state.current_trick_winner() == state.next_to_play().partner() {
                    dds_move.priority -= dds_move.card.rank as isize;
                } else {
                    dds_move.priority += dds_move.card.rank as isize;
                }

                if dds_move.card > state.currently_winning_card().unwrap() {
                    break;
                }
            }
        }
    }

    fn prioritize_cards_for_discard<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        for candidate in moves.iter_mut() {
            if Some(candidate.card.suit) == state.trumps() {
                candidate.priority += 50;
            }
            // candidate.priority -= candidate.card.rank as isize;
        }
    }

    fn sort_moves_by_priority_descending(moves: &mut [DdsMove]) {
        moves.sort_unstable_by(|a, b| a.priority.cmp(&b.priority));
    }

    fn select_one_move_per_sequence(moves: &[DdsMove]) -> Vec<DdsMove> {
        let mut output: Vec<DdsMove> = vec![];
        for &candidate_move in moves {
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
