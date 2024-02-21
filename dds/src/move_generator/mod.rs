mod dds_move;
mod leading;
mod playing_last;
mod playing_second;
mod playing_third;

use crate::state::virtual_card::VirtualCard;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::card::suit::SUIT_ARRAY;
pub use dds_move::DdsMove;
use itertools::Itertools;
use rand::Rng;

pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn generate_moves<const N: usize>(state: &VirtualState<N>, move_ordering: bool) -> Vec<DdsMove> {
        let player = state.next_to_play();
        let mut unique_moves = match state.suit_to_follow() {
            Some(lead_suit) => {
                let card_tracker = state.cards_of(player);
                if card_tracker.is_void_in(lead_suit) {
                    Self::select_one_move_per_sequence(card_tracker.all_cards())
                } else {
                    Self::select_one_move_per_sequence(card_tracker.all_cards_in(lead_suit))
                }
            }
            None => {
                let card_tracker = state.cards_of(player);
                Self::select_one_move_per_sequence(card_tracker.all_cards())
            }
        };
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

    fn select_one_move_per_sequence(moves: impl Iterator<Item = VirtualCard>) -> Vec<DdsMove> {
        moves
            .peekable()
            .batching(|peekable| match peekable.next() {
                None => None,
                Some(x) => {
                    let mut mve = DdsMove::new(x);
                    while let Some(y) = peekable.next_if(|y| mve.card.touches(y)) {
                        mve.card = y;
                        mve.sequence_length += 1;
                    }
                    Some(mve)
                }
            })
            .collect_vec()
    }
}
