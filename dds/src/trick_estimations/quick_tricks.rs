use crate::primitives::VirtualCard;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::deal::Seat;
use itertools::Itertools;
use std::cmp::{max, min, Ordering};
use strum::IntoEnumIterator;

pub fn quick_tricks_for_player<const N: usize>(state: &VirtualState<N>, player: Seat) -> usize {
    // Quick tricks are the tricks that an axis can take without losing the lead.
    // For this, we need to look at both hands combined
    let players = [player, player + 1, player + 2, player + 3];
    let cards = players.map(|x| state.remaining_cards_for_player(x));

    let [my_cards, lhos_cards, partners_cards, rhos_cards] = &cards;

    let my_high_cards = count_high_cards_per_suit(my_cards);
    let _partners_high_cards = count_high_cards_per_suit(partners_cards);

    let mut combined_cards = my_cards.clone();
    combined_cards.extend_from_slice(partners_cards);

    let combined_high_cards = count_high_cards_per_suit(&combined_cards);

    // In a perfect world, we would get to use all our high cards
    // but we can never make more tricks in a suit than we have cards in the longest hand
    let my_cards_per_suit = count_cards_per_suit(my_cards);
    let partners_cards_per_suit = count_cards_per_suit(partners_cards);

    let max_cards_per_suit = [0, 1, 2, 3].map(|i| max(my_cards_per_suit[i], partners_cards_per_suit[i]));
    let _max_quick_tricks = [0, 1, 2, 3].map(|i| min(combined_high_cards[i], max_cards_per_suit[i]));

    // communication is important. To count our combined quick-tricks, we need to make sure that we can
    // move between hands.
    // step 1: check entries for each suit separately:

    //

    let max_quick_tricks = my_high_cards;

    // To reach this maximum number of quick tricks we need to make sure that opponents cannot ruff.
    let final_quick_tricks = match state.trumps() {
        None => {
            // count all quick tricks,
            max_quick_tricks.iter().sum()
        }
        Some(trump_suit) => {
            // there is a trump suit.

            let lhos_cards_per_suit = count_cards_per_suit(lhos_cards);
            let rhos_cards_per_suit = count_cards_per_suit(rhos_cards);

            let lhos_trump_cards = lhos_cards_per_suit[trump_suit as usize];
            let rhos_trump_cards = rhos_cards_per_suit[trump_suit as usize];

            let opponents_trump_cards = [lhos_trump_cards, rhos_trump_cards];

            let our_trump_high_cards = max_quick_tricks[trump_suit as usize];

            // let opponents_max_cards_per_suit =
            //     [0, 1, 2, 3].map(|i| max(lhos_cards_per_suit[i], rhos_cards_per_suit[i]));

            match opponents_trump_cards.map(|num| num.cmp(&our_trump_high_cards)) {
                [Ordering::Greater, Ordering::Greater] => {
                    // both opponents can ruff
                    // only count high cards until one is void
                    let quick_tricks = [0, 1, 2, 3].map(|i| min(rhos_cards_per_suit[i], max_quick_tricks[i]));
                    let mut quick_tricks = [0, 1, 2, 3].map(|i| min(lhos_cards_per_suit[i], quick_tricks[i]));

                    // correct count for trump suit
                    quick_tricks[trump_suit as usize] = our_trump_high_cards;
                    quick_tricks.iter().sum()
                }
                [Ordering::Greater, _] => {
                    // LHO can ruff
                    // only count high cards until LHO is void
                    let mut quick_tricks = [0, 1, 2, 3].map(|i| min(lhos_cards_per_suit[i], max_quick_tricks[i]));

                    // correct count for trump suit
                    quick_tricks[trump_suit as usize] = our_trump_high_cards;
                    quick_tricks.iter().sum()
                }
                [_, Ordering::Greater] => {
                    // RHO can ruff
                    // only count high cards until RHO is void
                    let mut quick_tricks = [0, 1, 2, 3].map(|i| min(rhos_cards_per_suit[i], max_quick_tricks[i]));

                    // correct count for trump suit
                    quick_tricks[trump_suit as usize] = our_trump_high_cards;
                    quick_tricks.iter().sum()
                }
                [_, _] => {
                    // opponents won't have trumps left
                    max_quick_tricks.iter().sum()
                }
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

#[allow(dead_code)]
fn count_combined_high_cards_per_suit(my_cards: &[VirtualCard], partners_cards: &[VirtualCard]) -> [[usize; 4]; 2] {
    let mut sorted = my_cards.iter().map(|card| (card, 0usize)).collect_vec();
    sorted.extend(partners_cards.iter().map(|card| (card, 1usize)));
    sorted.sort_unstable_by(|a, b| b.0.cmp(a.0));
    let sorted = sorted;

    let mut result = [[0usize; 4]; 2];

    for (suit, suit_cards) in &sorted.iter().group_by(|card| card.0.suit) {
        for ((card, owner), rank) in suit_cards.zip(VirtualRank::iter().rev()) {
            if card.rank == rank {
                result[*owner][suit as usize] += 1;
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

// fn check_for_entries_to_partner(my_cards: &[VirtualCard], partners_cards: &[VirtualCard]) -> [bool; 4] {
//     let my_card_count = count_cards_per_suit(my_cards);
//     // let partners_card_count = count_cards_per_suit(partners_cards);
//
//     let mut combined_cards = my_cards.clone().to_vec();
//     combined_cards.extend_from_slice(partners_cards);
//
//     let combined_high_card_count = count_high_cards_per_suit(&combined_cards);
//
//     let [my_extended_high_card_count, partners_extended_high_card_count] =
//         count_combined_high_cards_per_suit(my_cards, partners_cards);
//
//     let my_simple_high_card_count = count_high_cards_per_suit(my_cards);
//     // let partners_high_card_count = count_high_cards_per_suit(partners_cards);
//
//     // we have entries into partners hand if we have a "not high card"
//     // and if partner has additional high cards to ours
//     let i_have_low_cards = [0, 1, 2, 3].map(|i| my_card_count[i] - my_high_card_count[i] > 0);
//     let partner_has_additional_high_cards =
//         [0, 1, 2, 3].map(|i| combined_high_card_count[i] - my_high_card_count[i] > 0);
//     [0, 1, 2, 3].map(|i| i_have_low_cards[i] && partner_has_additional_high_cards[i])
// }

#[cfg(test)]
mod test {
    use crate::primitives::VirtualCard;
    use itertools::Itertools;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case(&["CA", "CK", "C7", "DA", "HK", "HT", "S7"],[3, 1, 2, 1] )]
    fn count_cards_per_suit(cards: &[&str], expected: [usize; 4]) {
        let cards = cards
            .iter()
            .map(|&input| VirtualCard::from_str(input).unwrap())
            .collect_vec();
        assert_eq!(super::count_cards_per_suit(&cards), expected);
    }

    #[test_case(&["CA", "CK", "C7", "DA", "HK", "HT", "S7"],[2, 1, 0, 0] )]
    fn count_high_cards_per_suit(cards: &[&str], expected: [usize; 4]) {
        let cards = cards
            .iter()
            .map(|&input| VirtualCard::from_str(input).unwrap())
            .collect_vec();
        assert_eq!(super::count_high_cards_per_suit(&cards), expected);
    }

    #[test_case(&["CA", "CK", "CT", "DQ", "HA", "HT", "S7"], &["CQ", "CJ", "C7", "DA", "HK", "HT", "SA"], [[3, 0, 1, 0],[2,1,1,1]] )]
    fn count_combined_high_cards_per_suit(my_cards: &[&str], partners_cards: &[&str], expected: [[usize; 4]; 2]) {
        let my_cards = my_cards
            .iter()
            .map(|&input| VirtualCard::from_str(input).unwrap())
            .collect_vec();
        let partners_cards = partners_cards
            .iter()
            .map(|&input| VirtualCard::from_str(input).unwrap())
            .collect_vec();
        assert_eq!(
            super::count_combined_high_cards_per_suit(&my_cards, &partners_cards),
            expected
        );
    }
}
