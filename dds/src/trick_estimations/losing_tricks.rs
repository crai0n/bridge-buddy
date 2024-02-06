use super::quick_tricks::*;
use crate::primitives::VirtualCard;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::Suit;
use std::cmp::{max, min, Ordering};
use strum::IntoEnumIterator;

pub fn losing_tricks_for_leader<const N: usize>(state: &VirtualState<N>) -> usize {
    match state.trumps() {
        None => nt_losing_tricks(state),
        Some(trump_suit) => trump_losing_tricks(state, trump_suit),
    }
}

fn trump_losing_tricks<const N: usize>(state: &VirtualState<N>, trump_suit: Suit) -> usize {
    // this routine is inspired heavily by Bo Haglund's Double Dummy Solver
    let player = state.next_to_play();
    let players = [player, player + 1, player + 2, player + 3];
    let cards = players.map(|x| state.remaining_cards_for_player(x));

    let [my_cards, lhos_cards, partners_cards, rhos_cards] = &cards;

    let my_card_count = count_cards_per_suit(my_cards);
    let partners_card_count = count_cards_per_suit(partners_cards);
    let lhos_card_count = count_cards_per_suit(lhos_cards);
    let rhos_card_count = count_cards_per_suit(rhos_cards);

    let my_simple_high_card_count = count_high_cards_per_suit(my_cards);
    let partners_simple_high_card_count = count_high_cards_per_suit(partners_cards);
    let lhos_simple_high_card_count = count_high_cards_per_suit(lhos_cards);
    let rhos_simple_high_card_count = count_high_cards_per_suit(rhos_cards);

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
        if lhos_card_count[trump_suit as usize] >= 2
            && lhos_cards.contains(&VirtualCard {
                suit: trump_suit,
                rank: VirtualRank::King,
            })
        {
            return 1;
        }
    } else if partners_simple_high_card_count[trump_suit as usize] == 1 {
        // We cannot drive out the trump King
        if rhos_card_count[trump_suit as usize] >= 2
            && rhos_cards.contains(&VirtualCard {
                suit: trump_suit,
                rank: VirtualRank::King,
            })
        {
            return 1;
        }
    }
    0
}

fn nt_losing_tricks<const N: usize>(state: &VirtualState<N>) -> usize {
    // this routine is inspired heavily by Bo Haglund's Double Dummy Solver
    let player = state.next_to_play();
    let players = [player, player + 1, player + 2, player + 3];
    let cards = players.map(|x| state.remaining_cards_for_player(x));

    let [my_cards, lhos_cards, partners_cards, rhos_cards] = &cards;

    let my_card_count = count_cards_per_suit(my_cards);
    let partners_card_count = count_cards_per_suit(partners_cards);

    let lhos_simple_high_card_count = count_high_cards_per_suit(lhos_cards);
    let rhos_simple_high_card_count = count_high_cards_per_suit(rhos_cards);

    let mut partners_suits = 0;
    let mut lhos_suits = 0;
    let mut rhos_suits = 0;

    for suit in Suit::iter() {
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

    for suit in Suit::iter() {
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
