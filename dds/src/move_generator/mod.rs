use crate::card_manager::card_tracker::SUIT_ARRAY;
use crate::primitives::DdsMove;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::Suit;

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
        match state.trump_suit() {
            None => Self::prioritize_moves_for_leading_no_trump(moves, state),
            Some(trump_suit) => Self::prioritize_moves_for_leading_trump(moves, state, trump_suit),
        }
    }

    #[allow(clippy::collapsible_if)]
    fn prioritize_moves_for_leading_no_trump<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        let suit_weights = Self::calculate_suit_weights_for_leading(state);
        let player = state.next_to_play();
        let _partner = player + 2;
        let _lho = player + 1;
        let rho = player + 3;
        for dds_move in moves {
            let we_can_win_trick_by_force = dds_move.card.rank == VirtualRank::Ace
                || state.partner_has_higher_cards_than_opponent(dds_move.card.suit, player);
            let suit_weight = suit_weights[dds_move.card.suit as usize];
            let suit = dds_move.card.suit;

            if we_can_win_trick_by_force {
                if state.owner_of_runner_up_in(&suit) == Some(rho) {
                    if state.cards_of(rho).has_singleton_in(&suit) {
                        // encourage, because we can catch runner-up
                        dds_move.priority += suit_weight + 13;
                    } else {
                        // discourage, because we cannot catch runner-up
                        dds_move.priority += suit_weight - 13;
                    }
                }
            }

            if dds_move.sequence_length >= 3 {
                dds_move.priority += 50 + suit_weight;
            }
        }
    }

    fn prioritize_moves_for_leading_trump<const N: usize>(
        moves: &mut [DdsMove],
        state: &VirtualState<N>,
        trump_suit: Suit,
    ) {
        let suit_weights = Self::calculate_suit_weights_for_leading(state);
        let player = state.next_to_play();
        for dds_move in moves {
            let _is_winning_move = dds_move.card.rank == VirtualRank::Ace
                || state.partner_has_higher_cards_than_opponent(dds_move.card.suit, player);
            let suit_weight = suit_weights[dds_move.card.suit as usize];
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

    fn calculate_suit_weights_for_leading<const N: usize>(state: &VirtualState<N>) -> [isize; 4] {
        let player = state.next_to_play();
        let lho = player + 1;
        let partner = player + 2;
        let rho = player + 3;
        let rhos_hand = state.cards_of(rho);
        let lhos_hand = state.cards_of(lho);
        let my_hand = state.cards_of(player);
        let partners_hand = state.cards_of(partner);

        let suit_bonus = SUIT_ARRAY.map(|suit| {
            let mut bonus = 0;

            if rhos_hand.contains_winning_rank_in(&suit) || rhos_hand.contains_runner_up_in(&suit) {
                bonus -= 18;
            }
            if let Some(trump_suit) = state.trump_suit() {
                let lho_can_ruff = lhos_hand.is_void_in(&suit) && lhos_hand.has_cards_in(&trump_suit);
                if lho_can_ruff {
                    bonus -= 10;
                }
                let i_can_ruff_partners_return = suit != trump_suit
                    && my_hand.has_singleton_in(&suit)
                    && my_hand.has_cards_in(&trump_suit)
                    && partners_hand.count_cards_in(&suit) >= 2;
                if i_can_ruff_partners_return {
                    bonus += 16
                }
            }
            bonus
        });

        let [count_lho, count_rho] = [lho, rho].map(|opponent| {
            SUIT_ARRAY.map(|suit| {
                let count = state.cards_of(opponent).count_cards_in(&suit);
                match count {
                    0 => state.count_played_cards() as isize + 4,
                    _ => count as isize * 4,
                }
            })
        });

        [0, 1, 2, 3].map(|i| suit_bonus[i] - ((count_lho[i] + count_rho[i]) * 32) / 15)
    }

    fn calculate_suit_weights_for_discarding<const N: usize>(state: &VirtualState<N>) -> [isize; 4] {
        // Taken from Bo Haglund's Double Dummy Solver
        let player = state.next_to_play();
        let my_hand = state.cards_of(player);
        SUIT_ARRAY.map(|suit| {
            if my_hand.has_doubleton_runner_up_in(&suit) {
                1
            } else if my_hand.has_singleton_winner_in(&suit) {
                0
            } else {
                my_hand.count_cards_in(&suit) as isize * 64 / 36
            }
        })
    }

    fn prioritize_moves_for_second_hand<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        if moves.first().unwrap().card.suit != state.suit_to_follow().unwrap() {
            Self::prioritize_cards_if_void(moves, state);
        } else {
            for dds_move in moves.iter_mut() {
                dds_move.priority -= dds_move.card.rank as isize;
            }
        }
    }
    fn prioritize_moves_for_third_hand<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        if moves.first().unwrap().card.suit != state.suit_to_follow().unwrap() {
            Self::prioritize_cards_if_void(moves, state);
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
            Self::prioritize_cards_if_void(moves, state);
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

    fn prioritize_cards_if_void<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        let suit_weights = Self::calculate_suit_weights_for_discarding(state);

        for candidate in moves.iter_mut() {
            let suit = candidate.card.suit;
            if state.trump_suit() == Some(suit) {
                if Some(candidate.card.suit) == state.trump_suit() {
                    candidate.priority += 50;
                }
            } else {
                let suit_weight = suit_weights[candidate.card.suit as usize];
                candidate.priority += suit_weight - candidate.card.rank as isize;
            }
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