use crate::move_generator::DdsMove;
use crate::move_generator::MoveGenerator;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::Suit;

impl MoveGenerator {
    pub fn calc_priority_playing_last<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        if moves.first().unwrap().card.suit != state.suit_to_follow().unwrap() {
            match state.trump_suit() {
                None => Self::calc_priority_nt_void(moves, state),
                Some(trump_suit) => Self::calc_priority_playing_last_trump_void(moves, state, trump_suit),
            }
        } else {
            match state.trump_suit() {
                None => Self::calc_priority_playing_last_nt_not_void(moves, state),
                Some(trump_suit) => Self::calc_priority_playing_last_trump_not_void(moves, state, trump_suit),
            }
        }
    }

    fn calc_priority_playing_last_nt_not_void<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
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

    fn calc_priority_playing_last_trump_not_void<const N: usize>(
        moves: &mut [DdsMove],
        state: &VirtualState<N>,
        trump_suit: Suit,
    ) {
        let opponents_are_winning = state.current_trick_winner() != state.next_to_play().partner();
        let trick_has_not_been_ruffed = state.currently_winning_card().unwrap().suit != trump_suit;
        for candidate in moves {
            if trick_has_not_been_ruffed && opponents_are_winning {
                Self::try_to_win_as_cheaply_as_possible(state, candidate);
            } else {
                // no way to win, play as low as possible
                candidate.priority -= candidate.card.rank as isize;
            }
        }
    }

    fn calc_priority_playing_last_trump_void<const N: usize>(
        moves: &mut [DdsMove],
        state: &VirtualState<N>,
        trump_suit: Suit,
    ) {
        let suit_weights = Self::suit_weights_for_discarding(state);
        let me = state.next_to_play();

        let partner_is_winning = state.current_trick_winner() == me.partner();
        let opponents_have_ruffed = state.currently_winning_card().unwrap().suit == trump_suit;

        for candidate in moves.iter_mut() {
            let suit = candidate.card.suit;
            let suit_weight = if suit == trump_suit {
                -20 // generally we want to keep our trumps
            } else {
                suit_weights[suit as usize]
            };
            candidate.priority += suit_weight - candidate.card.rank as isize;
        }

        // bonuses for specific ruffing situations
        if opponents_have_ruffed {
            // try cheapest overruff first, then try discards
            for candidate in moves.iter_mut().filter(|candidate| candidate.card.suit == trump_suit) {
                if candidate.card.rank > state.currently_winning_card().unwrap().rank {
                    candidate.priority += 50;
                    break;
                }
            }
        } else if !partner_is_winning {
            // opponents are winning without ruffing, try a cheap ruff, then try discards
            if let Some(candidate) = moves.iter_mut().find(|candidate| candidate.card.suit == trump_suit) {
                candidate.priority += 50;
            }
        }
    }
}
