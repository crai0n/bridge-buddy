use crate::primitives::VirtualCard;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::deal::Seat;
use itertools::Itertools;
use std::cmp::{max, min};
use strum::IntoEnumIterator;

pub fn quick_tricks_for_player<const N: usize>(state: &VirtualState<N>, player: Seat) -> usize {
    // Quick tricks are the tricks that an axis can take without losing the lead.
    // For this, we need to look at both hands combined
    let players = [player, player.partner(), player + 1, player + 3];
    let [my_cards, partners_cards, lhos_cards, rhos_cards] = players.map(|x| state.remaining_cards_for_player(x));

    let my_quick_tricks = count_high_cards_per_suit(&my_cards);
    let _partners_quick_tricks = count_high_cards_per_suit(&partners_cards);

    let mut combined_cards = my_cards.clone();
    combined_cards.extend_from_slice(&partners_cards);

    let combined_quick_tricks = count_high_cards_per_suit(&combined_cards);

    // In a perfect world, we would get to use all our high cards
    let max_quick_tricks = combined_quick_tricks;

    // but we can never make more tricks in a suit than we have cards in the longest hand
    let my_cards_per_suit = count_cards_per_suit(&my_cards);
    let partners_cards_per_suit = count_cards_per_suit(&partners_cards);

    let max_cards_per_suit = [0, 1, 2, 3].map(|i| max(my_cards_per_suit[i], partners_cards_per_suit[i]));
    let _higher_bounds = [0, 1, 2, 3].map(|i| min(max_quick_tricks[i], max_cards_per_suit[i]));

    let higher_bounds = my_quick_tricks;

    // To reach this maximum number of quick tricks we need to make sure that opponents cannot ruff.
    let final_quick_tricks = match state.trumps() {
        None => {
            // count all quick tricks,
            higher_bounds.iter().sum()
        }
        Some(trump_suit) => {
            // there is a trump suit.

            let lhos_cards_per_suit = count_cards_per_suit(&lhos_cards);
            let rhos_cards_per_suit = count_cards_per_suit(&rhos_cards);

            let opponents_max_cards_per_suit =
                [0, 1, 2, 3].map(|i| max(lhos_cards_per_suit[i], rhos_cards_per_suit[i]));

            if opponents_max_cards_per_suit[trump_suit as usize] == 0 {
                // count all quick tricks,
                higher_bounds.iter().sum()
            } else {
                // count only trump quick tricks,
                higher_bounds[trump_suit as usize]
            }
        }
    };

    final_quick_tricks
}

fn count_high_cards_per_suit(cards: &[VirtualCard]) -> [usize; 4] {
    let mut sorted = cards.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let sorted = sorted;

    let mut result = [0usize; 4];

    for (suit, suit_cards) in &sorted.iter().group_by(|card| card.suit) {
        for (card, rank) in suit_cards.zip(VirtualRank::iter().rev()) {
            if card.rank == rank {
                result[suit as usize] += 1;
            }
        }
    }

    result
}

fn count_cards_per_suit(cards: &[VirtualCard]) -> [usize; 4] {
    let mut result = [0usize; 4];

    for (suit, cards) in &cards.iter().group_by(|card| card.suit) {
        for _card in cards {
            result[suit as usize] += 1;
        }
    }

    result
}
