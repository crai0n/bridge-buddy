use crate::primitives::VirtualCard;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::Suit;
use itertools::Itertools;
use std::cmp::{max, min};
use strum::IntoEnumIterator;

pub fn nt_quick_tricks_for_player<const N: usize>(state: &VirtualState<N>, player: Seat) -> usize {
    // Quick tricks are the tricks that an axis can take without losing the lead.
    // For this, we need to look at both hands combined
    let players = [player, player + 1, player + 2, player + 3];
    let cards = players.map(|x| state.remaining_cards_for_player(x));

    let [my_cards, _lhos_cards, partners_cards, _rhos_cards] = &cards;

    let high_card_tricks = nt_high_card_tricks_per_suit(my_cards, partners_cards);

    let my_cards_per_suit = count_cards_per_suit(my_cards);
    // let partners_cards_per_suit = count_cards_per_suit(partners_cards);
    // let lhos_cards_per_suit = count_cards_per_suit(lhos_cards);
    // let rhos_cards_per_suit = count_cards_per_suit(rhos_cards);

    // let mut quick_tricks = [0; 4];

    // for i in 0..4 {
    //     if high_card_tricks[i] > max(lhos_cards_per_suit[i], rhos_cards_per_suit[i]) {
    //         // after our high cards, opponents have nothing left, so we can run our low cards,
    //         // making one trick for each card of the longer hand
    //         quick_tricks[i] = max(my_cards_per_suit[i], partners_cards_per_suit[i])
    //     } else {
    //         // just count the high card tricks
    //         quick_tricks[i] = high_card_tricks[i]
    //     }
    // }

    // if we have a lot of quick tricks, they might collide. we can never make more tricks than we have cards
    min(high_card_tricks.iter().sum(), my_cards_per_suit.iter().sum())
}

pub fn trump_quick_tricks_for_player<const N: usize>(
    _state: &VirtualState<N>,
    _player: Seat,
    _trump_suit: Suit,
) -> usize {
    // there is a trump suit.
    // To reach this maximum number of quick tricks we need to make sure that opponents cannot ruff.

    // let lhos_trump_cards = lhos_cards_per_suit[trump_suit as usize];
    // let rhos_trump_cards = rhos_cards_per_suit[trump_suit as usize];
    //
    // let opponents_trump_cards = [lhos_trump_cards, rhos_trump_cards];
    //
    // let our_trump_high_cards = high_card_tricks[trump_suit as usize];
    //
    // // let opponents_max_cards_per_suit =
    // //     [0, 1, 2, 3].map(|i| max(lhos_cards_per_suit[i], rhos_cards_per_suit[i]));
    //
    // match opponents_trump_cards.map(|num| num.cmp(&our_trump_high_cards)) {
    //     [Ordering::Greater, Ordering::Greater] => {
    //         // println!("Both opponents have trumps left");
    //         // both opponents can ruff
    //         // only count high cards until one is void
    //         let quick_tricks = [0, 1, 2, 3].map(|i| min(rhos_cards_per_suit[i], high_card_tricks[i]));
    //         let mut quick_tricks = [0, 1, 2, 3].map(|i| min(lhos_cards_per_suit[i], quick_tricks[i]));
    //
    //         // correct count for trump suit
    //         quick_tricks[trump_suit as usize] = our_trump_high_cards;
    //         quick_tricks.iter().sum()
    //     }
    //     [Ordering::Greater, _] => {
    //         // LHO can ruff
    //         // only count high cards until LHO is void
    //         let mut quick_tricks = [0, 1, 2, 3].map(|i| min(lhos_cards_per_suit[i], high_card_tricks[i]));
    //
    //         // correct count for trump suit
    //         quick_tricks[trump_suit as usize] = our_trump_high_cards;
    //         quick_tricks.iter().sum()
    //     }
    //     [_, Ordering::Greater] => {
    //         // RHO can ruff
    //         // only count high cards until RHO is void
    //         let mut quick_tricks = [0, 1, 2, 3].map(|i| min(rhos_cards_per_suit[i], high_card_tricks[i]));
    //
    //         // correct count for trump suit
    //         quick_tricks[trump_suit as usize] = our_trump_high_cards;
    //         quick_tricks.iter().sum()
    //     }
    //     [_, _] => {
    //         // opponents won't have trumps left
    //         high_card_tricks.iter().sum()
    //     }
    // }
    //
    // let max_cards = my_cards_per_suit.iter().sum();
    //
    // min(final_quick_tricks, max_cards)
    0
}
pub fn quick_tricks_for_player<const N: usize>(state: &VirtualState<N>, player: Seat) -> usize {
    match state.trumps() {
        None => nt_quick_tricks_for_player(state, player),
        Some(trump_suit) => trump_quick_tricks_for_player(state, player, trump_suit),
    }
}
#[allow(dead_code)]
fn trump_high_card_tricks_per_suit(
    _my_cards: &[VirtualCard],
    _partners_cards: &[VirtualCard],
    _trump_suit: Suit,
) -> [usize; 4] {
    // let (quick_tricks, partners_blocked_quick_tricks, my_blocked_quick_tricks, can_move_to_partner, can_move_back) =
    //     high_card_tricks_per_suit(my_cards, partners_cards);

    // We might have "stuck tricks" left over. Counting these takes a bit more caution than in NT, because we need to
    // be careful about ruffing from opponents. In the best case, we can just run all our trump tricks first, and then

    // check trump suit first:
    // if partners_blocked_quick_tricks[trump_suit as usize] != 0 {
    //     // there are blocked tricks in partners hand in trump suit
    // } else if {
    //     my_blocked_quick_tricks[trump_suit as usize] != 0 {
    //         // there are blocked tricks in my hand in trump suit
    //     }
    // } else {
    //     // we can run all trump tricks
    // }

    [0; 4]
}

fn nt_high_card_tricks_per_suit(my_cards: &[VirtualCard], partners_cards: &[VirtualCard]) -> [usize; 4] {
    let (mut quick_tricks, partners_blocked_quick_tricks, my_blocked_quick_tricks, can_move_to_partner, can_move_back) =
        high_card_tricks_per_suit(my_cards, partners_cards);

    // We might have "stuck tricks" left over. These can be counted in the following situations:
    // Partners blocked tricks: We can always run the "blocked" suit on our side first, so any move to partner is
    // enough to touch the "blocked tricks" afterwards.
    if can_move_to_partner.contains(&true) {
        for i in 0..4 {
            quick_tricks[i] += partners_blocked_quick_tricks[i];
        }
    }

    // Our own blocked quick tricks: After we unblock the suits from our hand, we use the suit that forces us into partners
    // hand to move over and run partners stuck tricks.
    // We can count our own stuck tricks if we have a suit where we can move back
    if can_move_back.contains(&true) {
        for i in 0..4 {
            quick_tricks[i] += my_blocked_quick_tricks[i];
        }
    }

    // println!("reporting {:?} to caller", quick_tricks);
    quick_tricks
}
#[allow(clippy::type_complexity)]
fn high_card_tricks_per_suit(
    my_cards: &[VirtualCard],
    partners_cards: &[VirtualCard],
) -> ([usize; 4], [usize; 4], [usize; 4], [bool; 4], [bool; 4]) {
    let my_card_count = count_cards_per_suit(my_cards);
    let partners_card_count = count_cards_per_suit(partners_cards);

    let my_simple_high_card_count = count_high_cards_per_suit(my_cards);
    let partners_simple_high_card_count = count_high_cards_per_suit(partners_cards);

    let [my_extended_high_card_count, partners_extended_high_card_count] =
        count_combined_high_cards_per_suit(my_cards, partners_cards);

    let mut quick_tricks = [0; 4];
    let mut partners_blocked_quick_tricks = [0; 4];
    let mut my_blocked_quick_tricks = [0; 4];
    let mut can_move_to_partner = [false; 4];
    let mut can_move_back = [false; 4];
    let mut long_side_remaining_cards = [0, 1, 2, 3].map(|i| max(partners_card_count[i], my_card_count[i]));

    for i in 0..4 {
        if partners_card_count[i] >= my_card_count[i] {
            // I have the short suit, play my high cards first
            if my_card_count[i] > my_extended_high_card_count[i] {
                // after playing all my high cards, I have at least one low card left to play towards partner
                if my_extended_high_card_count[i] > 0 {
                    // I can use this suit to get back into my own hand
                    can_move_back[i] = true;
                }
                quick_tricks[i] += my_extended_high_card_count[i];
                long_side_remaining_cards[i] -= my_extended_high_card_count[i];
                // partners is trying to hold on to his highest card
                let long_side_remaining_high_cards =
                    min(long_side_remaining_cards[i], partners_extended_high_card_count[i]);
                if long_side_remaining_high_cards > 0 {
                    // now i play my lowest card and partner takes over with his highest card to run the rest
                    quick_tricks[i] += long_side_remaining_high_cards;
                    can_move_to_partner[i] = true;
                }
            } else if my_extended_high_card_count[i] > my_simple_high_card_count[i] {
                // partner has at least one high card higher than my lowest high card
                // so I run all but my lowest high card
                if my_extended_high_card_count[i] > 1 {
                    // I can use this suit to get back into my own hand
                    can_move_back[i] = true;
                }
                quick_tricks[i] += my_extended_high_card_count[i] - 1;
                long_side_remaining_cards[i] -= my_extended_high_card_count[i] - 1;
                // partners is trying to hold on to his highest card
                let long_side_remaining_high_cards =
                    min(long_side_remaining_cards[i], partners_extended_high_card_count[i]);
                if long_side_remaining_high_cards > 0 {
                    // now i play my lowest card and partner takes over with his highest card to run the rest
                    quick_tricks[i] += long_side_remaining_high_cards;
                    can_move_to_partner[i] = true;
                } else {
                    unreachable!("long side should always have high cards left here")
                }
            } else {
                if my_extended_high_card_count[i] > 0 {
                    // I can use this suit to get back into my own hand
                    can_move_back[i] = true;
                }
                // I have only winners in this suit, I cannot move to partner, so i have to unblock
                quick_tricks[i] += my_simple_high_card_count[i];
                // partner might have high cards left over
                long_side_remaining_cards[i] -= my_extended_high_card_count[i];
                // partners is trying to hold on to his highest card
                let long_side_remaining_high_cards =
                    min(long_side_remaining_cards[i], partners_extended_high_card_count[i]);
                partners_blocked_quick_tricks[i] += long_side_remaining_high_cards;
            }
        } else {
            // My suit is longer (at least 1 card)
            if partners_extended_high_card_count[i] == 0 {
                // partner has no high cards that need to be played, just count mine
                quick_tricks[i] += my_simple_high_card_count[i];
            } else if my_extended_high_card_count[i] == my_card_count[i]
                && my_simple_high_card_count[i] >= partners_extended_high_card_count[i]
            {
                // I can run the whole suit without partner
                quick_tricks[i] += my_extended_high_card_count[i];
            } else {
                // partner has high cards
                if my_card_count[i] > my_extended_high_card_count[i]
                    || my_extended_high_card_count[i] > my_simple_high_card_count[i]
                {
                    // i use my lowest card to play to partners high cards.
                    if partners_extended_high_card_count[i] > 0 {
                        can_move_to_partner[i] = true;
                    }
                    if partners_card_count[i] > partners_extended_high_card_count[i] {
                        // partner has a low card to play back to me after running his high cards
                        quick_tricks[i] += partners_extended_high_card_count[i];
                        // i am trying to hold on to my highest card
                        long_side_remaining_cards[i] -= partners_extended_high_card_count[i];
                        let long_side_remaining_high_cards =
                            min(long_side_remaining_cards[i], my_extended_high_card_count[i]);
                        if long_side_remaining_high_cards > 0 {
                            can_move_back[i] = true;
                            quick_tricks[i] += long_side_remaining_high_cards;
                        }
                    } else if partners_extended_high_card_count[i] > partners_simple_high_card_count[i] {
                        // partner will need to sacrifice his lowest high card to play back to me
                        quick_tricks[i] += max(partners_extended_high_card_count[i] - 1, 1);
                        // i am trying to hold on to my highest card
                        long_side_remaining_cards[i] -= max(partners_extended_high_card_count[i] - 1, 1);
                        let long_side_remaining_high_cards =
                            min(long_side_remaining_cards[i], my_extended_high_card_count[i]);
                        if long_side_remaining_high_cards > 0 && partners_card_count[i] > 1 {
                            can_move_back[i] = true;
                            quick_tricks[i] += long_side_remaining_high_cards;
                        } else {
                            my_blocked_quick_tricks[i] = long_side_remaining_high_cards;
                        }
                    } else {
                        // partner has only all the highest cards, once we touch this suit we are stuck in partner
                        quick_tricks[i] += partners_simple_high_card_count[i];

                        // I am trying to hold on to my highest cards
                        long_side_remaining_cards[i] -= partners_simple_high_card_count[i];
                        let long_side_remaining_high_cards =
                            min(long_side_remaining_cards[i], my_extended_high_card_count[i]);
                        my_blocked_quick_tricks[i] = long_side_remaining_high_cards
                    }
                } else {
                    // If I Hold AK and partner holds Q, i have only high cards and the longer suit,
                    // just run them, partner will have nothing left.
                    quick_tricks[i] += my_simple_high_card_count[i];
                }
            }
        }
    }
    // println!("immediate quick tricks: {:?}", quick_tricks);
    // println!("p.blocked quick tricks: {:?}", partners_blocked_quick_tricks);
    // println!("m.blocked quick tricks: {:?}", my_blocked_quick_tricks);
    // println!("   can move to partner: {:?}", can_move_to_partner);
    // println!("         can move back: {:?}", can_move_back);
    (
        quick_tricks,
        partners_blocked_quick_tricks,
        my_blocked_quick_tricks,
        can_move_to_partner,
        can_move_back,
    )
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
    let mut sorted = cards.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let sorted = sorted;

    let mut result = [0usize; 4];

    for (suit, cards) in &sorted.iter().group_by(|card| card.suit) {
        for _card in cards {
            result[suit as usize] += 1;
        }
    }

    result
}

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
    #[test_case(&["CA", "SA", "C7", "DA", "HK", "HT", "HA"],[1, 1, 2, 1] )]
    fn count_high_cards_per_suit(cards: &[&str], expected: [usize; 4]) {
        let cards = cards
            .iter()
            .map(|&input| VirtualCard::from_str(input).unwrap())
            .collect_vec();
        assert_eq!(super::count_high_cards_per_suit(&cards), expected);
    }

    #[test_case(&["CA", "CK", "CT", "DQ", "HA", "HT", "S7"], &["CQ", "CJ", "C7", "DA", "HK", "HT", "SA"], [[3, 0, 1, 0],[2,1,1,1]] )]
    #[test_case(&["CA", "CJ"], &["CK", "CQ", "C2"], [[2, 0, 0, 0],[2,0,0,0]] )]
    #[test_case(&["CA"], &["CK", "C2"], [[1, 0, 0, 0],[1,0,0,0]] )]
    #[test_case(&["SA", "SJ", "DQ", "CQ", "CJ"], &["SQ", "HA", "HK", "HQ", "HT"], [[0, 0, 0, 1], [0, 0, 3, 0]] )]
    #[test_case(&["SK", "DK", "DT", "CA", "CT"], &["ST", "HJ", "DA", "DJ", "CK"], [[1, 1, 0, 0], [1, 1, 0, 0]] )]
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

    #[test_case(&["CA", "CK"], &["C2", "H2"], [2,0,0,0])]
    #[test_case(&["CA", "H2"], &["C2", "CK"], [1,0,0,0])]
    #[test_case(&["CA", "H2", "H7"], &["CK", "C2", "HA"], [2,0,1,0])]
    #[test_case(&["SA", "SQ", "SJ"], &["SK", "ST", "S9", "S8"], [0,0,0,4])]
    #[test_case(&["SA", "SJ", "DQ", "CQ", "CJ"], &["SQ", "HA", "HK", "HQ", "HT"], [0, 0, 0, 1] )]
    #[test_case(&["SQ", "HA", "HK", "HQ", "HT"], &["SA", "SJ", "DQ", "CQ", "CJ"], [0, 0, 3, 1] )]
    #[test_case(&["SK", "DK", "DT", "CA", "CT"], &["ST", "HJ", "DA", "DJ", "CK"], [2, 2, 0, 0] )]
    fn nt_high_card_tricks(my_cards: &[&str], partners_cards: &[&str], expected: [usize; 4]) {
        let my_cards = my_cards
            .iter()
            .map(|&input| VirtualCard::from_str(input).unwrap())
            .collect_vec();
        let partners_cards = partners_cards
            .iter()
            .map(|&input| VirtualCard::from_str(input).unwrap())
            .collect_vec();
        assert_eq!(
            super::nt_high_card_tricks_per_suit(&my_cards, &partners_cards),
            expected
        );
    }
}
