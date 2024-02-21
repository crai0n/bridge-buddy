use bridge_buddy_core::primitives::Suit;

use crate::trick_estimations::EstimationState;
use bridge_buddy_core::primitives::card::suit::SUIT_ARRAY;
use std::cmp::{max, min, Ordering};

pub fn losing_tricks_for_leader(estimation_state: EstimationState) -> usize {
    match estimation_state.trump_suit {
        None => nt_losing_tricks(estimation_state),
        Some(trump_suit) => trump_losing_tricks(trump_suit, estimation_state),
    }
}

fn trump_losing_tricks(trump_suit: Suit, estimation_state: EstimationState) -> usize {
    // this routine is inspired heavily by Bo Haglund's Double Dummy Solver
    let player = estimation_state.my_seat;
    let partner = player + 2;
    let lho = player + 1;
    let rho = player + 3;

    let my_card_count = estimation_state.card_counts[player as usize];
    let partners_card_count = estimation_state.card_counts[partner as usize];
    let lhos_card_count = estimation_state.card_counts[lho as usize];
    let rhos_card_count = estimation_state.card_counts[rho as usize];

    let my_simple_high_card_count = estimation_state.high_card_counts[player as usize];
    let partners_simple_high_card_count = estimation_state.high_card_counts[partner as usize];
    let lhos_simple_high_card_count = estimation_state.high_card_counts[lho as usize];
    let rhos_simple_high_card_count = estimation_state.high_card_counts[rho as usize];

    if my_card_count[trump_suit as usize] == 0 && partners_card_count[trump_suit as usize] == 0 {
        // opponents have all the trumps
        return max(
            lhos_card_count[trump_suit as usize],
            rhos_card_count[trump_suit as usize],
        );
    } else if lhos_simple_high_card_count[trump_suit as usize] > 0 {
        // LHO has the highest trump
        return lhos_simple_high_card_count[trump_suit as usize];
    } else if rhos_simple_high_card_count[trump_suit as usize] > 0 {
        // RHO has the highest trump
        return rhos_simple_high_card_count[trump_suit as usize];
    } else if my_simple_high_card_count[trump_suit as usize] == 1 {
        // We cannot drive out the trump King
        if lhos_card_count[trump_suit as usize] >= 2 && estimation_state.king_owners[trump_suit as usize] == Some(lho) {
            return 1;
        }
    } else if partners_simple_high_card_count[trump_suit as usize] == 1 {
        // We cannot drive out the trump King
        if rhos_card_count[trump_suit as usize] >= 2 && estimation_state.king_owners[trump_suit as usize] == Some(rho) {
            return 1;
        }
    }
    0
}

fn nt_losing_tricks(estimation_state: EstimationState) -> usize {
    // this routine is inspired heavily by Bo Haglund's Double Dummy Solver
    let player = estimation_state.my_seat;
    let partner = player + 2;
    let lho = player + 1;
    let rho = player + 3;

    let my_card_count = estimation_state.card_counts[player as usize];
    let partners_card_count = estimation_state.card_counts[partner as usize];
    // let lhos_card_count = estimation_state.card_counts[lho as usize];
    // let rhos_card_count = estimation_state.card_counts[rho as usize];
    //
    // let my_simple_high_card_count = estimation_state.high_card_counts[player as usize];
    // let partners_simple_high_card_count = estimation_state.high_card_counts[partner as usize];
    let lhos_simple_high_card_count = estimation_state.high_card_counts[lho as usize];
    let rhos_simple_high_card_count = estimation_state.high_card_counts[rho as usize];

    let mut partners_suits = 0;
    let mut lhos_suits = 0;
    let mut rhos_suits = 0;

    for suit in SUIT_ARRAY {
        if my_card_count[suit as usize] != 0 {
            if rhos_simple_high_card_count[suit as usize] > 0 {
                rhos_suits += 1;
            } else if lhos_simple_high_card_count[suit as usize] > 0 {
                lhos_suits += 1;
            } else {
                partners_suits += 1;
            }
        }
    }

    if partners_suits == 0 {
        return min(lhos_suits, rhos_suits);
    }

    let mut our_maximum_tricks = 0;

    for suit in SUIT_ARRAY {
        if lhos_simple_high_card_count[suit as usize] == 0 && rhos_simple_high_card_count[suit as usize] == 0 {
            // we have the winner for this suit
            // assume we win with every card
            our_maximum_tricks += max(my_card_count[suit as usize], partners_card_count[suit as usize])
        }
    }

    let tricks_left: usize = my_card_count.iter().sum();

    match tricks_left.cmp(&our_maximum_tricks) {
        Ordering::Greater => 1, //tricks_left - our_maximum_tricks,
        _ => 0,
    }
}
