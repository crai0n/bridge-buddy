use super::DdsMove;
use super::MoveGenerator;
use crate::card_manager::card_tracker::SUIT_ARRAY;
use crate::state::virtual_card::VirtualCard;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use std::cmp::max;

impl MoveGenerator {
    pub fn calc_priority_leading<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        let suit_weights = Self::suit_weights_for_leading(state);
        for candidate in moves {
            let suit = candidate.card.suit;
            let suit_weight = suit_weights[suit as usize];

            // favor leading from a sequence
            // candidate.priority +=
            //     suit_weight + 2 * (candidate.sequence_length - 1) as isize + candidate.card.rank as isize;

            match state.trump_suit() {
                Some(_trump_suit) => {
                    candidate.priority += if candidate.sequence_length >= 2 {
                        suit_weight + 50 + candidate.card.rank as isize
                    } else {
                        suit_weight + candidate.card.rank as isize
                    }
                }
                None => {
                    candidate.priority += if candidate.sequence_length >= 3 {
                        suit_weight + 50 + candidate.card.rank as isize
                    } else {
                        suit_weight + candidate.card.rank as isize
                    }
                }
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

            let partner_dominates_suit = state.partner_has_higher_cards_than_opponents(suit, player);

            if partner_dominates_suit {
                bonus += 25;
            }

            if rhos_hand.contains_winning_rank_in(suit) {
                bonus -= 18;
            }

            if lhos_hand.contains_runner_up_in(suit) && partners_hand.contains_winning_rank_in(suit) {
                bonus += 30;
            }

            if lhos_hand.contains_winning_rank_in(suit) && partners_hand.contains_runner_up_in(suit) {
                bonus += 12;
            }

            if state.owner_of_runner_up_in(suit) == Some(partner) || state.owner_of_runner_up_in(suit) == Some(player) {
                bonus += 12;
            } else if state.owner_of_runner_up_in(suit) == Some(lho) && lhos_hand.has_singleton_in(suit) {
                bonus += 11;
            } else if state.owner_of_runner_up_in(suit) == Some(rho) && rhos_hand.has_singleton_in(suit) {
                bonus += 15;
            }

            let partner_owns_both_2nd_and_3rd = state.owner_of_runner_up_in(suit) == Some(partner)
                && state.owner_of(VirtualCard {
                    suit,
                    rank: VirtualRank::Queen,
                }) == Some(partner);

            let partner_owns_2nd_and_we_own_3rd =
                state.owner_of_runner_up_in(suit) == Some(partner) && state.owner_of_runner_up_in(suit) == Some(player);

            let we_own_2nd_and_partner_owns_3rd =
                state.owner_of_runner_up_in(suit) == Some(player) && state.owner_of_runner_up_in(suit) == Some(partner);

            if partner_owns_both_2nd_and_3rd {
                bonus += 26;
            } else if partners_hand.count_cards_in(suit) >= 2
                && (partner_owns_2nd_and_we_own_3rd || we_own_2nd_and_partner_owns_3rd)
            {
                bonus += 25;
            }

            if let Some(trump_suit) = state.trump_suit() {
                // trump game
                if suit == trump_suit {
                    // trump_suit
                    let our_trump_length = max(
                        my_hand.count_cards_in(trump_suit),
                        partners_hand.count_cards_in(trump_suit),
                    );
                    let opponents_trump_length = max(
                        lhos_hand.count_cards_in(trump_suit),
                        rhos_hand.count_cards_in(trump_suit),
                    );
                    if our_trump_length > opponents_trump_length {
                        bonus += 5;
                    }
                } else {
                    // side suit
                    let lho_can_ruff = lhos_hand.is_void_in(suit) && lhos_hand.has_cards_in(trump_suit);
                    let rho_can_ruff = rhos_hand.is_void_in(suit) && rhos_hand.has_cards_in(trump_suit);
                    let partner_can_ruff = partners_hand.is_void_in(suit) && partners_hand.has_cards_in(trump_suit);
                    let i_can_ruff_partners_return = suit != trump_suit
                        && my_hand.has_singleton_in(suit)
                        && my_hand.has_cards_in(trump_suit)
                        && partners_hand.count_cards_in(suit) >= 2;

                    if lho_can_ruff || rho_can_ruff {
                        let partner_dominates_trump = partner_can_ruff
                            && match (lho_can_ruff, rho_can_ruff) {
                                (true, true) => {
                                    partners_hand.highest_card_in(trump_suit) > lhos_hand.highest_card_in(trump_suit)
                                        && partners_hand.highest_card_in(trump_suit)
                                            > rhos_hand.highest_card_in(trump_suit)
                                }
                                (true, false) => {
                                    partners_hand.highest_card_in(trump_suit)
                                        > partners_hand.highest_card_in(trump_suit)
                                }
                                (false, true) => {
                                    partners_hand.highest_card_in(trump_suit) > rhos_hand.highest_card_in(trump_suit)
                                }
                                (false, false) => true,
                            };
                        if partner_dominates_trump {
                            bonus += 30;
                        } else {
                            bonus -= 30;
                        }
                    } else if partner_can_ruff {
                        bonus += 20;
                    } else if i_can_ruff_partners_return {
                        bonus += 16;
                    }
                }
            } else {
                // no trump game
                if (rhos_hand.has_singleton_winner_in(suit) || lhos_hand.has_singleton_winner_in(suit))
                    && (state.owner_of_runner_up_in(suit) == Some(partner)
                        || state.owner_of_runner_up_in(suit) == Some(player))
                {
                    bonus += 20;
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

        [0, 1, 2, 3].map(|i| suit_bonus[i] - ((count_lho[i] + count_rho[i]) * 16) / 15)
    }
}
