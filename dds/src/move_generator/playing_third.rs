use crate::move_generator::MoveGenerator;
use crate::primitives::DdsMove;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::Suit;
use std::cmp::max;

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
        let me = state.next_to_play();
        let lead_suit = state.suit_to_follow().unwrap();
        let currently_winning_card = state.currently_winning_card().unwrap();

        let partner_is_winning = state.current_trick_winner() == me.partner();
        let rho_is_winning = !partner_is_winning;

        let my_cards = state.cards_of(me);
        let my_highest_card = my_cards.highest_card_in(lead_suit).unwrap();

        let lhos_cards = state.cards_of(me + 1);
        let rhos_cards = state.cards_of(me + 3);
        let lhos_highest_card = lhos_cards.highest_card_in(lead_suit);
        let lhos_lowest_card = lhos_cards.lowest_card_in(lead_suit);

        let lho_can_still_win = match lhos_highest_card {
            None => false,
            Some(card) => card > currently_winning_card,
        };

        let my_cards_do_not_matter = match lhos_lowest_card {
            None => my_highest_card < currently_winning_card,
            Some(lowest_card) => my_highest_card < currently_winning_card && my_highest_card < lowest_card,
        };

        if my_cards_do_not_matter {
            // play low
            for candidate in moves {
                candidate.priority -= candidate.card.rank as isize;
            }
        } else if lho_can_still_win {
            let mut already_beaten_cards = 0;
            for candidate in moves {
                let beats = lhos_cards.count_cards_lower_than(candidate.card);
                if beats > already_beaten_cards {
                    // give a bonus to every card that beats more of lho's cards,
                    // so we will try winning, then forcing the ace, etc.
                    // otherwise play as cheap as possible
                    candidate.priority += 10 * beats as isize - candidate.card.rank as isize;
                    already_beaten_cards = beats;
                } else {
                    candidate.priority -= candidate.card.rank as isize;
                }
            }
        } else if rho_is_winning {
            let mut win_bonus = 20;
            for candidate in moves {
                if candidate.card > currently_winning_card {
                    // win as cheaply as possible
                    candidate.priority += win_bonus - candidate.card.rank as isize;
                    win_bonus = 0;
                } else {
                    // if we can't beat RHO, play low
                    candidate.priority -= candidate.card.rank as isize;
                }
            }
        } else {
            // partner will beat LHO anyways
            // it might be worth it to overtake if we can run the suit afterwards

            let my_high_card_count = my_cards.count_high_cards_per_suit()[lead_suit as usize];
            let opponents_length = max(
                lhos_cards.count_cards_in(lead_suit),
                rhos_cards.count_cards_in(lead_suit) + 1,
            );

            if my_high_card_count >= opponents_length {
                // play high!
                for candidate in moves {
                    candidate.priority += candidate.card.rank as isize;
                }
            } else {
                // play low
                for candidate in moves {
                    candidate.priority -= candidate.card.rank as isize;
                }
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
        // let lead_suit = state.suit_to_follow().unwrap();
        // let me = state.next_to_play();
        //
        // let my_cards = state.cards_of(me);
        // let lhos_cards = state.cards_of(me + 1);
        //
        // let partner_is_winning = state.current_trick_winner() == me.partner();
        // let rho_is_winning = !partner_is_winning;

        // let lhos_highest_card = lhos_cards.highest_card_in(lead_suit);
        // let lhos_lowest_card = lhos_cards.lowest_card_in(lead_suit);
        // let my_highest_card = my_cards.highest_card_in(lead_suit).unwrap();
        // let _lhos_highest_trump_card = lhos_cards.highest_card_in(trump_suit);
        //
        // let lho_can_ruff = lhos_cards.is_void_in(lead_suit) && !lhos_cards.is_void_in(trump_suit);

        // let currently_winning_card = state.currently_winning_card().unwrap();

        // let rho_has_ruffed = lead_suit != trump_suit && currently_winning_card.suit == trump_suit;

        // let lho_can_still_win = match lhos_highest_card {
        //     Some(high_card) => high_card > currently_winning_card,
        //     None => lho_can_ruff
        // };
        //
        // let my_cards_do_not_matter = match lhos_lowest_card {
        //     None => my_highest_card < currently_winning_card,
        //     Some(lowest_card) => my_highest_card < currently_winning_card && my_highest_card < lowest_card,
        // };

        // if lead_suit == trump_suit {
        //     if my_cards_do_not_matter {
        //         // just play low
        //         for candidate in moves {
        //             candidate.priority -= candidate.card.rank as isize;
        //         }
        //     } else if rho_is_winning {
        //
        //
        //     } else if lho_can_still_win {
        //
        //     }
        // } else {
        //     if rho_has_ruffed || my_cards_do_not_matter {
        //         // just play low
        //         for candidate in moves {
        //             candidate.priority -= candidate.card.rank as isize;
        //         }
        //     } else if rho_is_winning {
        //
        //     }
        // }

        for candidate in moves {
            if candidate.card > state.currently_winning_card().unwrap() {
                candidate.priority += candidate.card.rank as isize;
            }
        }
    }
}