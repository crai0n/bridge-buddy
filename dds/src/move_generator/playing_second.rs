use crate::move_generator::MoveGenerator;
use crate::primitives::DdsMove;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::Suit;

impl MoveGenerator {
    pub fn calc_priority_playing_second<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        if moves.first().unwrap().card.suit != state.suit_to_follow().unwrap() {
            match state.trump_suit() {
                None => Self::calc_priority_nt_void(moves, state),
                Some(trump_suit) => Self::calc_priority_playing_second_trump_void(moves, state, trump_suit),
            }
        } else {
            match state.trump_suit() {
                None => Self::calc_priority_playing_second_nt_not_void(moves, state),
                Some(trump_suit) => Self::calc_priority_playing_second_trump_not_void(moves, state, trump_suit),
            }
        }
    }

    fn calc_priority_playing_second_nt_not_void<const N: usize>(moves: &mut [DdsMove], state: &VirtualState<N>) {
        let me = state.next_to_play();
        let partner = me + 2;
        let lho = me + 1;
        let lhos_cards = state.cards_of(lho);
        let partners_cards = state.cards_of(partner);
        let lead_card = state.currently_winning_card().unwrap();
        let lead_suit = lead_card.suit;

        let partners_highest = partners_cards.highest_card_in(lead_suit);
        let lhos_highest = lhos_cards.highest_card_in(lead_suit);

        let partner_is_void = partners_highest.is_none();
        let lho_is_void = lhos_highest.is_none();

        for candidate in moves.iter_mut() {
            candidate.priority -= candidate.card.rank as isize;
        }

        if partner_is_void && lho_is_void {
            if let Some(beats) = moves.iter_mut().find(|candidate| candidate.card > lead_card) {
                // this card beats leader
                beats.priority += 50;
            } // else we can't do anything
        } else if lho_is_void {
            if partners_highest.unwrap() < lead_card {
                //partner needs our help
                if let Some(candidate) = moves.iter_mut().find(|cand| cand.card > lead_card) {
                    // this card beats leader
                    candidate.priority += 50;
                }
            } // else we just play low
        } else if partner_is_void {
            // partner can't help
            if let Some(candidate) = moves
                .iter_mut()
                .find(|cand| cand.card > lead_card && cand.card > lhos_highest.unwrap())
            {
                // this card beats both opponent
                candidate.priority += 80;
            } else if let Some(candidate) = moves
                .iter_mut()
                .find(|cand| cand.card > lead_card && cand.sequence_length > 1)
            {
                // this card beats lead card, but not rho, and we have one more of these
                candidate.priority += 10;
            } // else we just play low
        } else {
            // both partner and lho have relevant cards
            let partner_cannot_beat_opponents =
                partners_highest.unwrap() < lead_card || partners_highest < lhos_highest;

            if partner_cannot_beat_opponents {
                if let Some(candidate) = moves
                    .iter_mut()
                    .find(|cand| cand.card > lead_card && cand.card > lhos_highest.unwrap())
                {
                    // this card beats both opponent
                    candidate.priority += 80;
                } else if let Some(candidate) = moves
                    .iter_mut()
                    .find(|cand| cand.card > lead_card && cand.sequence_length > 1)
                {
                    // this card beats lead card, but not rho, and we have one more of these
                    candidate.priority += 10;
                } // else we just play low
            }
        }
    }

    fn calc_priority_playing_second_trump_void<const N: usize>(
        moves: &mut [DdsMove],
        state: &VirtualState<N>,
        trump_suit: Suit,
    ) {
        let suit_weights = Self::suit_weights_for_discarding(state);

        let me = state.next_to_play();
        let partner = me + 2;
        let lho = me + 1;
        let lhos_cards = state.cards_of(lho);
        let partners_cards = state.cards_of(partner);
        let lead_card = state.currently_winning_card().unwrap();
        let lead_suit = lead_card.suit;

        let partners_highest = partners_cards.highest_card_in(lead_suit);
        let lhos_highest = lhos_cards.highest_card_in(lead_suit);

        let partner_is_void = partners_highest.is_none();
        let lho_is_void = lhos_highest.is_none();

        let partner_can_ruff = partner_is_void && !partners_cards.is_void_in(trump_suit);
        let lho_can_ruff = lho_is_void && !lhos_cards.is_void_in(trump_suit);

        if lead_suit == trump_suit {
            // we don't make a difference, just pitch from side-suits
            for candidate in moves {
                let suit = candidate.card.suit;
                let suit_weight = suit_weights[suit as usize];
                candidate.priority -= suit_weight - candidate.card.rank as isize;
            }
        } else if lho_is_void && partner_is_void {
            // everyone is void
            if lho_can_ruff && partner_can_ruff {
                // they can both ruff
            } else if lho_can_ruff {
            } else if partner_can_ruff {
            } else {
                // they can't ruff
            }

            for candidate in moves {
                if candidate.card > state.currently_winning_card().unwrap() {
                    candidate.priority += candidate.card.rank as isize;
                }
            }
        } else if lho_is_void {
            if lho_can_ruff {
            } else {
            }
            // lho could ruff
        } else if partner_is_void {
            // partner can ruff
            if partner_can_ruff {
            } else {
            }
        } else {
            // we are the only one void
        }
        //
    }

    fn calc_priority_playing_second_trump_not_void<const N: usize>(
        moves: &mut [DdsMove],
        state: &VirtualState<N>,
        trump_suit: Suit,
    ) {
        let me = state.next_to_play();
        let partner = me + 2;
        let lho = me + 1;
        let lhos_cards = state.cards_of(lho);
        let partners_cards = state.cards_of(partner);
        let lead_card = state.currently_winning_card().unwrap();
        let lead_suit = lead_card.suit;

        let partners_highest = partners_cards.highest_card_in(lead_suit);
        let lhos_highest = lhos_cards.highest_card_in(lead_suit);

        let partner_is_void = partners_highest.is_none();
        let lho_is_void = lhos_highest.is_none();

        if lead_suit == trump_suit {
            // just pitch
            for candidate in moves {
                candidate.priority -= candidate.card.rank as isize;
            }
        } else {
        }

        for candidate in moves {
            if candidate.card > state.currently_winning_card().unwrap() {
                candidate.priority += candidate.card.rank as isize;
            }
        }
    }
}
