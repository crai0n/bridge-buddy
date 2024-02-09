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
        let opponents_are_winning = state.current_trick_winner() != state.next_to_play().partner();

        let my_cards = state.cards_of(state.next_to_play());
        let my_high_cards = my_cards.count_high_cards_per_suit();
        let i_have_high_cards_to_run = my_high_cards.iter().fold(0, |sum, item| sum + *item) > 0;

        let partners_cards = state.cards_of(state.next_to_play().partner());
        let partners_leadable_suits = partners_cards.count_cards_per_suit().map(|c| c != 0);
        let partner_can_reach_me = partners_leadable_suits
            .iter()
            .zip(my_high_cards)
            .any(|(&can_lead, high_card_count)| can_lead && high_card_count > 0);

        for candidate in moves {
            if opponents_are_winning || i_have_high_cards_to_run && !partner_can_reach_me {
                Self::try_to_win_as_cheaply_as_possible(state, candidate);
            } else {
                candidate.priority -= candidate.card.rank as isize;
            }
        }
    }

    pub fn calc_priority_playing_third_trump_void<const N: usize>(
        moves: &mut [DdsMove],
        state: &VirtualState<N>,
        trump_suit: Suit,
    ) {
        let suit_weights = Self::suit_weights_for_discarding(state);

        let me = state.next_to_play();
        let lead_suit = state.suit_to_follow().unwrap();

        let lhos_cards = state.cards_of(me + 1);

        let currently_winning_card = state.currently_winning_card().unwrap();

        let partner_is_winning = state.current_trick_winner() == me.partner();
        let rho_is_winning = !partner_is_winning;

        let lho_can_ruff = lhos_cards.is_void_in(lead_suit) && !lhos_cards.is_void_in(trump_suit);

        let lho_can_still_win = match lhos_cards.highest_card_in(lead_suit) {
            None => !lhos_cards.is_void_in(trump_suit),
            Some(highest_card) => highest_card > currently_winning_card,
        };

        let rho_has_ruffed = lead_suit != trump_suit && currently_winning_card.suit == trump_suit;

        for candidate in moves {
            let suit = candidate.card.suit;
            if suit == trump_suit {
                // we could ruff
                if lho_can_ruff {
                    let we_can_ruff_higher = candidate.card > lhos_cards.highest_card_in(trump_suit).unwrap();
                    if we_can_ruff_higher {
                        // ruff as high as necessary to win, but don't overspend!
                        candidate.priority += 55 - candidate.card.rank as isize;
                    } else if partner_is_winning {
                        // no sense in sacrificing a trump
                        candidate.priority += -50 + candidate.card.rank as isize;
                    } else {
                        // drive out a high trump
                        candidate.priority += 50 - candidate.card.rank as isize;
                    }
                } else if rho_has_ruffed {
                    if candidate.card.rank > currently_winning_card.rank {
                        // overruffs are good
                        candidate.priority += 50 - candidate.card.rank as isize;
                    } else {
                        // underruffs are bad
                        candidate.priority += -50 - candidate.card.rank as isize;
                    }
                } else if lho_can_still_win || rho_is_winning {
                    // ruff as low as possible
                    candidate.priority += 50 - candidate.card.rank as isize;
                } else {
                    //partner has already won, discourage ruffing
                    candidate.priority += -50 - candidate.card.rank as isize;
                }
            } else {
                // we can only discard
                candidate.priority += suit_weights[suit as usize] - candidate.card.rank as isize;
            }
        }
    }

    pub fn calc_priority_playing_third_trump_not_void<const N: usize>(
        moves: &mut [DdsMove],
        state: &VirtualState<N>,
        _trump_suit: Suit,
    ) {
        for candidate in moves {
            if candidate.card > state.currently_winning_card().unwrap() {
                candidate.priority += candidate.card.rank as isize;
            }
        }
    }
}
