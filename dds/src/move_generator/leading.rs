use super::MoveGenerator;
use crate::card_manager::card_tracker::SUIT_ARRAY;
use crate::primitives::DdsMove;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::Suit;

impl MoveGenerator {
    pub fn calc_priority_leading<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        match state.trump_suit() {
            None => Self::calc_priority_leading_nt(moves, state),
            Some(trump_suit) => Self::calc_priority_leading_trump(moves, state, trump_suit),
        }
    }

    #[allow(clippy::collapsible_if)]
    fn calc_priority_leading_nt<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        let suit_weights = Self::suit_weights_for_leading(state);
        let player = state.next_to_play();
        let _partner = player + 2;
        let _lho = player + 1;
        let rho = player + 3;
        for candidate in moves {
            let we_can_win_trick_by_force = candidate.card.rank == VirtualRank::Ace
                || state.partner_has_higher_cards_than_opponent(candidate.card.suit, player);
            let suit_weight = suit_weights[candidate.card.suit as usize];
            let suit = candidate.card.suit;

            if we_can_win_trick_by_force {
                if state.owner_of_runner_up_in(suit) == Some(rho) {
                    if state.cards_of(rho).has_singleton_in(suit) {
                        // encourage, because we can catch runner-up
                        candidate.priority += suit_weight + 13;
                    } else {
                        // discourage, because we cannot catch runner-up
                        candidate.priority += suit_weight - 13;
                    }
                }
            }

            if candidate.sequence_length >= 3 {
                candidate.priority += 50 + suit_weight;
            }
        }
    }

    fn calc_priority_leading_trump<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>, trump_suit: Suit) {
        let suit_weights = Self::suit_weights_for_leading(state);
        let player = state.next_to_play();
        for candidate in moves {
            let _is_winning_move = candidate.card.rank == VirtualRank::Ace
                || state.partner_has_higher_cards_than_opponent(candidate.card.suit, player);
            let suit_weight = suit_weights[candidate.card.suit as usize];
            if candidate.sequence_length >= 2 {
                candidate.priority += 50 + suit_weight;
            }

            let our_trump_count = state.count_this_sides_trump_cards();
            let opponents_trump_count = state.count_opponents_trump_cards();

            if our_trump_count >= opponents_trump_count && candidate.card.suit == trump_suit {
                candidate.priority += 100 + suit_weight;
            }
        }
    }

    fn suit_weights_for_leading<const N: usize>(state: &VirtualState<N>) -> [isize; 4] {
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

            if rhos_hand.contains_winning_rank_in(suit) || rhos_hand.contains_runner_up_in(suit) {
                bonus -= 18;
            }
            if let Some(trump_suit) = state.trump_suit() {
                let lho_can_ruff = lhos_hand.is_void_in(suit) && lhos_hand.has_cards_in(trump_suit);
                if lho_can_ruff {
                    bonus -= 10;
                }
                let i_can_ruff_partners_return = suit != trump_suit
                    && my_hand.has_singleton_in(suit)
                    && my_hand.has_cards_in(trump_suit)
                    && partners_hand.count_cards_in(suit) >= 2;
                if i_can_ruff_partners_return {
                    bonus += 16
                }
            }
            bonus
        });

        let [count_lho, count_rho] = [lho, rho].map(|opponent| {
            SUIT_ARRAY.map(|suit| {
                let count = state.cards_of(opponent).count_cards_in(suit);
                match count {
                    0 => state.count_played_cards() as isize + 4,
                    _ => count as isize * 4,
                }
            })
        });

        [0, 1, 2, 3].map(|i| suit_bonus[i] - ((count_lho[i] + count_rho[i]) * 32) / 15)
    }
}
