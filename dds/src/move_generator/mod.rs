use crate::card_manager::card_tracker::SUIT_ARRAY;
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
        let suit_weights = Self::calculate_suit_weights_for_leading(state);
        for dds_move in moves {
            let suit_weight = suit_weights[dds_move.card.suit as usize];
            match state.trumps() {
                None => {
                    if dds_move.sequence_length >= 3 {
                        dds_move.priority += 50 + suit_weight;
                    }
                }
                Some(trump_suit) => {
                    if dds_move.sequence_length >= 2 {
                        dds_move.priority += 50 + suit_weight;
                    }

                    let our_trump_count = state.count_this_sides_trump_cards();
                    let opponents_trump_count = state.count_opponents_trump_cards();

                    if our_trump_count >= opponents_trump_count && dds_move.card.suit == trump_suit {
                        dds_move.priority += 100 + suit_weight;
                    }
                }
            }
        }
    }

    fn calculate_suit_weights_for_leading<const N: usize>(state: &VirtualState<N>) -> [isize; 4] {
        let player = state.next_to_play();
        let lho = player + 1;
        let partner = player + 2;
        let rho = player + 3;
        let suit_bonus = SUIT_ARRAY.map(|suit| {
            let mut bonus = 0;
            if state.player_can_ruff_suit(suit, lho) {
                bonus -= 10;
            }
            if state.is_owner_of_winning_rank_in(suit, rho) || state.is_owner_of_runner_up_in(suit, rho) {
                bonus -= 18;
            }
            if let Some(trump_suit) = state.trumps() {
                if suit != trump_suit
                    && state.player_has_singleton_in(suit, player)
                    && state.count_trump_cards_for_player(player) > 0
                    && state.count_cards_in_suit_for_player(suit, partner) >= 2
                {
                    bonus += 16
                }
            }
            bonus
        });

        let [count_lho, count_rho] = [lho, rho].map(|opponent| {
            SUIT_ARRAY.map(|suit| {
                let count = state.count_cards_in_suit_for_player(suit, opponent);
                match count {
                    0 => state.count_played_cards() as isize + 4,
                    _ => count as isize * 4,
                }
            })
        });

        [0, 1, 2, 3].map(|i| suit_bonus[i] - ((count_lho[i] + count_rho[i]) * 32) / 15)
    }

    #[allow(dead_code)]
    fn calculate_suit_weights_for_discarding<const N: usize>(state: &VirtualState<N>) -> [isize; 4] {
        let player = state.next_to_play();
        // Taken from Bo Haglund's Double Dummy Solver
        SUIT_ARRAY.map(|suit| {
            if state.count_cards_in_suit_for_player(suit, player) == 2 && state.is_owner_of_runner_up_in(suit, player) {
                1
            } else {
                state.count_cards_in_suit_for_player(suit, player) as isize * 64 / 36
            }
        })
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
            candidate.priority -= candidate.card.rank as isize;
        }
    }

    fn sort_moves_by_priority_descending(moves: &mut [DdsMove]) {
        moves.sort_unstable_by(|a, b| b.priority.cmp(&a.priority));
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
