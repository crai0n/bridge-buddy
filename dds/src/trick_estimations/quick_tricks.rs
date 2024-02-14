use crate::state::virtual_card_tracker::VirtualCardTracker;
use crate::state::VirtualState;
use bridge_buddy_core::primitives::Suit;

use std::cmp::{max, min, Ordering};

fn nt_quick_tricks_for_leader<const N: usize>(state: &VirtualState<N>) -> usize {
    // Quick tricks are the tricks that an axis can take without losing the lead.
    // For this, we need to look at both hands combined
    let player = state.next_to_play();
    let players = [player, player + 1, player + 2, player + 3];
    let cards = players.map(|x| state.cards_of(x));

    let [my_cards, _lhos_cards, partners_cards, _rhos_cards] = cards;

    let high_card_tricks = nt_high_card_tricks_per_suit(&my_cards, &partners_cards);

    let my_cards_per_suit = my_cards.count_cards_per_suit();

    min(high_card_tricks.iter().sum(), my_cards_per_suit.iter().sum())
}

fn trump_quick_tricks_for_leader<const N: usize>(state: &VirtualState<N>, trump_suit: Suit) -> usize {
    // Quick tricks are the tricks that an axis can take without losing the lead.
    // For this, we need to look at both hands combined
    let player = state.next_to_play();
    let players = [player, player + 1, player + 2, player + 3];
    let cards = players.map(|x| state.cards_of(x));

    let [my_cards, lhos_cards, partners_cards, rhos_cards] = &cards;

    let high_card_tricks =
        trump_high_card_tricks_per_suit(my_cards, partners_cards, lhos_cards, rhos_cards, trump_suit);

    let my_cards_per_suit = my_cards.count_cards_per_suit();

    min(high_card_tricks.iter().sum(), my_cards_per_suit.iter().sum())
}
pub fn quick_tricks_for_leader<const N: usize>(state: &VirtualState<N>) -> usize {
    match state.trump_suit() {
        None => nt_quick_tricks_for_leader(state),
        Some(trump_suit) => trump_quick_tricks_for_leader(state, trump_suit),
    }
}

pub fn quick_tricks_for_second_hand<const N: usize>(state: &VirtualState<N>) -> usize {
    let lead_suit = state.suit_to_follow().unwrap();

    let player = state.next_to_play();
    let players = [player, player + 1, player + 2, player + 3];
    let cards = players.map(|x| state.cards_of(x));

    let [my_cards, lhos_cards, partners_cards, _rhos_cards] = &cards;

    let my_card_count = my_cards.count_cards_per_suit();
    let partners_card_count = partners_cards.count_cards_per_suit();
    let lhos_card_count = lhos_cards.count_cards_per_suit();

    let my_simple_high_card_count = my_cards.count_high_cards_per_suit();
    let partners_simple_high_card_count = partners_cards.count_high_cards_per_suit();

    match state.trump_suit() {
        None => {
            if my_simple_high_card_count[lead_suit as usize] > 0 {
                my_simple_high_card_count.iter().sum()
            } else if partners_simple_high_card_count[lead_suit as usize] > 0 {
                partners_simple_high_card_count.iter().sum()
            } else {
                0
            }
        }
        Some(trump_suit) => {
            if my_card_count[lead_suit as usize] == 0 && my_card_count[trump_suit as usize] != 0 {
                // I can ruff
                if lhos_card_count[lead_suit as usize] != 0 || lhos_card_count[trump_suit as usize] == 0 {
                    // opponent cannot ruff
                    if my_card_count[trump_suit as usize] > my_simple_high_card_count[trump_suit as usize] {
                        // I can use a small trump for ruffing and then run my high trumps (might be zero)
                        my_simple_high_card_count[trump_suit as usize] + 1
                    } else {
                        // I have only my high trump cards
                        my_simple_high_card_count[trump_suit as usize]
                    }
                } else {
                    // I win this trick with a high trump card if possible
                    my_simple_high_card_count[trump_suit as usize]
                }
            } else if partners_card_count[lead_suit as usize] == 0 && partners_card_count[trump_suit as usize] != 0 {
                // partner can ruff
                if lhos_card_count[lead_suit as usize] != 0 || lhos_card_count[trump_suit as usize] == 0 {
                    // opponent cannot ruff
                    if partners_card_count[trump_suit as usize] > partners_simple_high_card_count[trump_suit as usize] {
                        // partner can use a small trump for ruffing and then run my high trumps (might be zero)
                        partners_simple_high_card_count[trump_suit as usize] + 1
                    } else {
                        // partner has only high trump cards
                        partners_simple_high_card_count[trump_suit as usize]
                    }
                } else {
                    // Partner wins this trick with a high trump card if possible
                    partners_simple_high_card_count[trump_suit as usize]
                }
            } else if lhos_card_count[lead_suit as usize] != 0 || lhos_card_count[trump_suit as usize] == 0 {
                // opponent cannot ruff this round
                let our_high_card_count = max(
                    my_simple_high_card_count[lead_suit as usize],
                    partners_simple_high_card_count[lead_suit as usize],
                );
                if our_high_card_count > 0 {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        }
    }
}

fn trump_high_card_tricks_per_suit(
    my_cards: &VirtualCardTracker,
    partners_cards: &VirtualCardTracker,
    lhos_cards: &VirtualCardTracker,
    rhos_cards: &VirtualCardTracker,
    trump_suit: Suit,
) -> [usize; 4] {
    let (
        high_card_tricks,
        _partners_blocked_high_card_tricks,
        _my_blocked_high_card_tricks,
        _can_move_to_partner,
        mut can_move_back,
    ) = high_card_tricks_per_suit(my_cards, partners_cards);

    let lhos_card_count = lhos_cards.count_cards_per_suit();
    let rhos_card_count = rhos_cards.count_cards_per_suit();

    let opponents_trump_card_count = [
        lhos_card_count[trump_suit as usize],
        rhos_card_count[trump_suit as usize],
    ];

    can_move_back[trump_suit as usize] = false;

    let can_run_trumps_first = can_move_back.contains(&true);

    let drawn_trumps = match can_run_trumps_first {
        true => high_card_tricks[trump_suit as usize],
        false => 0,
    };

    // TODO: There is a problem if we end up in partners hands after playing trumps. We might be stuck there, so we cannot count our quick tricks in other suits.
    // Example: Spades is trumps, we hold Spades King, and AKQ in a side-suit, Partner has Spades Ace and is void in our side-suit.
    // if opponents can ruff, we are not allowed to count side suit tricks

    match opponents_trump_card_count.map(|num| num.cmp(&drawn_trumps)) {
        [Ordering::Greater, Ordering::Greater] => {
            // println!("Both opponents have trumps left");
            // both opponents can ruff
            // only count high cards until one is void

            let quick_tricks = [0, 1, 2, 3].map(|i| min(rhos_card_count[i], high_card_tricks[i]));
            let mut quick_tricks = [0, 1, 2, 3].map(|i| min(lhos_card_count[i], quick_tricks[i]));

            // correct count for trump suit
            quick_tricks[trump_suit as usize] = high_card_tricks[trump_suit as usize];
            quick_tricks
        }
        [Ordering::Greater, _] => {
            // LHO can ruff
            // only count high cards until LHO is void
            let mut quick_tricks = [0, 1, 2, 3].map(|i| min(lhos_card_count[i], high_card_tricks[i]));

            // correct count for trump suit
            quick_tricks[trump_suit as usize] = high_card_tricks[trump_suit as usize];
            quick_tricks
        }
        [_, Ordering::Greater] => {
            // RHO can ruff
            // only count high cards until RHO is void
            let mut quick_tricks = [0, 1, 2, 3].map(|i| min(rhos_card_count[i], high_card_tricks[i]));

            // correct count for trump suit
            quick_tricks[trump_suit as usize] = high_card_tricks[trump_suit as usize];
            quick_tricks
        }
        [_, _] => {
            // opponents don't have trumps left
            high_card_tricks
        }
    }
}

fn nt_high_card_tricks_per_suit(my_cards: &VirtualCardTracker, partners_cards: &VirtualCardTracker) -> [usize; 4] {
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
    my_cards: &VirtualCardTracker,
    partners_cards: &VirtualCardTracker,
) -> ([usize; 4], [usize; 4], [usize; 4], [bool; 4], [bool; 4]) {
    let my_card_count = my_cards.count_cards_per_suit();
    let partners_card_count = partners_cards.count_cards_per_suit();

    let my_simple_high_card_count = my_cards.count_high_cards_per_suit();
    let partners_simple_high_card_count = partners_cards.count_high_cards_per_suit();

    let [my_extended_high_card_count, partners_extended_high_card_count] =
        my_cards.count_combined_high_cards_per_suit(partners_cards);

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
                // TODO: Or I could move to partner if they have more quick tricks in other suits available. How to decide this?
                // Example: I have AQ against partners K, but partner has AKQ in another suit. I'd rather make 1 trick here
                // Idea: set can_move_to_partner to true, but introduce a 'penalty' array, so that we discount tricks if we actually use the
                // entries to partner.
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
    (
        quick_tricks,
        partners_blocked_quick_tricks,
        my_blocked_quick_tricks,
        can_move_to_partner,
        can_move_back,
    )
}

#[cfg(test)]
mod test {
    // use crate::state::virtual_card::VirtualCard;
    // use itertools::Itertools;
    // use std::str::FromStr;
    // use test_case::test_case;

    // #[test_case(&["CA", "CK", "CT", "DQ", "HA", "HT", "S7"], &["CQ", "CJ", "C7", "DA", "HK", "HT", "SA"], [[3, 0, 1, 0],[2,1,1,1]] )]
    // #[test_case(&["CA", "CJ"], &["CK", "CQ", "C2"], [[2, 0, 0, 0],[2,0,0,0]] )]
    // #[test_case(&["CA"], &["CK", "C2"], [[1, 0, 0, 0],[1,0,0,0]] )]
    // #[test_case(&["SA", "SJ", "DQ", "CQ", "CJ"], &["SQ", "HA", "HK", "HQ", "HT"], [[0, 0, 0, 1], [0, 0, 3, 0]] )]
    // #[test_case(&["SK", "DK", "DT", "CA", "CT"], &["ST", "HJ", "DA", "DJ", "CK"], [[1, 1, 0, 0], [1, 1, 0, 0]] )]
    // fn count_combined_high_cards_per_suit(my_cards: &[&str], partners_cards: &[&str], expected: [[usize; 4]; 2]) {
    //     let my_cards = my_cards
    //         .iter()
    //         .map(|&input| VirtualCard::from_str(input).unwrap())
    //         .collect_vec();
    //     let partners_cards = partners_cards
    //         .iter()
    //         .map(|&input| VirtualCard::from_str(input).unwrap())
    //         .collect_vec();
    //     assert_eq!(my_cards.count_combined_high_cards_per_suit(&partners_cards), expected);
    // }

    // #[test_case(&["CA", "CK"], &["C2", "H2"], [2,0,0,0])]
    // #[test_case(&["CA", "H2"], &["C2", "CK"], [1,0,0,0])]
    // #[test_case(&["CA", "H2", "H7"], &["CK", "C2", "HA"], [2,0,1,0])]
    // #[test_case(&["SA", "SQ", "SJ"], &["SK", "ST", "S9", "S8"], [0,0,0,4])]
    // #[test_case(&["SA", "SJ", "DQ", "CQ", "CJ"], &["SQ", "HA", "HK", "HQ", "HT"], [0, 0, 0, 1] )]
    // #[test_case(&["SQ", "HA", "HK", "HQ", "HT"], &["SA", "SJ", "DQ", "CQ", "CJ"], [0, 0, 3, 1] )]
    // #[test_case(&["SK", "DK", "DT", "CA", "CT"], &["ST", "HJ", "DA", "DJ", "CK"], [2, 2, 0, 0] )]
    // #[test_case(&["SA", "SQ", "D5", "D4", "D3"], &["SK", "H7", "H6", "H5", "H4"], [0, 0, 0, 2] )]
    // // #[test_case(&["SA", "SQ", "D5", "D4", "D3"], &["SK", "HA", "HK", "HQ", "HJ"], [0, 0, 4, 1] )] // this is a weird edge case where we should leave tricks on the table in one suit to make more in another
    // fn nt_high_card_tricks(my_cards: &[&str], partners_cards: &[&str], expected: [usize; 4]) {
    //     let my_cards = my_cards
    //         .iter()
    //         .map(|&input| VirtualCard::from_str(input).unwrap())
    //         .collect_vec();
    //     let partners_cards = partners_cards
    //         .iter()
    //         .map(|&input| VirtualCard::from_str(input).unwrap())
    //         .collect_vec();
    //     assert_eq!(
    //         super::nt_high_card_tricks_per_suit(&my_cards, &partners_cards),
    //         expected
    //     );
    // }
}
