pub mod card_tracker;
pub mod suit_field;

use card_tracker::CardTracker;

use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::{Card, Hand, Suit};
use itertools::Itertools;

pub struct CardManager {
    pub remaining_cards: [CardTracker; 4],
    pub played_cards: CardTracker,
}

impl CardManager {
    pub fn from_hands<const N: usize>(hands: [Hand<N>; 4]) -> CardManager {
        let remaining_cards: [CardTracker; 4] = hands
            .iter()
            .map(|hand| CardTracker::from_hand(*hand))
            .collect_vec()
            .try_into()
            .unwrap();
        Self {
            remaining_cards,
            played_cards: CardTracker::for_n_cards_per_suit(N),
        }
    }

    pub fn play(&mut self, card: Card, player: Seat) {
        // println!("{} played {}", self.next_to_play(), card);
        self.remaining_cards[player as usize].remove_card(card);
        self.played_cards.add_card(card);
    }

    pub fn unplay(&mut self, card: Card, player: Seat) {
        self.played_cards.remove_card(card);
        self.remaining_cards[player as usize].add_card(card);
    }

    pub fn remaining_cards_for_player(&self, player: Seat) -> &CardTracker {
        &self.remaining_cards[player as usize]
    }

    pub fn remaining_cards_of(&self, player: Seat) -> impl Iterator<Item = Card> + '_ {
        self.remaining_cards_for_player(player).all_cards()
    }

    pub fn remaining_cards_of_player_in_suit(&self, player: Seat, suit: Suit) -> impl Iterator<Item = Card> + '_ {
        self.remaining_cards_for_player(player).cards_in(suit)
    }

    pub fn has_higher_cards_in_suit_than_other(&self, player: Seat, suit: Suit, other: Seat) -> bool {
        self.remaining_cards_for_player(player)
            .has_higher_cards_in_suit_than(suit, self.remaining_cards_for_player(other))
    }

    pub fn played_cards(&self) -> &CardTracker {
        &self.played_cards
    }

    pub fn count_cards_in_suit_for_player(&self, suit: Suit, player: Seat) -> usize {
        self.remaining_cards_for_player(player).count_cards_in(suit)
    }
}

// #[cfg(test)]
// mod test {
// use super::card_tracker::CardTracker;
// use super::CardManager;
// use bridge_buddy_core::primitives::card::Rank;
// use bridge_buddy_core::primitives::deal::Seat;
// use bridge_buddy_core::primitives::{Card, Suit};
// use itertools::Itertools;
// use test_case::test_case;

// #[test_case("JT5", "KQ8743", "J5")] // 0001100001000, 0110001100110, 0001000001000
// #[test_case("JT52", "KQ8743", "J5")] // 0001100001001, 0110001100110, 0001000001000
// #[test_case("JT9643", "AK52", "J6")] // 0001110010110, 1100000001001, 0001000010000
// fn available_moves(my_cards: &str, played_cards: &str, expected: &str) {
//     let my_cards = my_cards
//         .chars()
//         .map(|c| Rank::from_char(c).unwrap())
//         .map(|d| Card {
//             rank: d,
//             suit: Suit::Spades,
//         })
//         .collect_vec();
//     let played_cards = played_cards
//         .chars()
//         .map(|c| Rank::from_char(c).unwrap())
//         .map(|d| Card {
//             rank: d,
//             suit: Suit::Spades,
//         })
//         .collect_vec();
//     let mut expected = expected
//         .chars()
//         .map(|c| Rank::from_char(c).unwrap())
//         .map(|d| Card {
//             rank: d,
//             suit: Suit::Spades,
//         })
//         .collect_vec();
//
//     let state = CardManager {
//         played_cards: CardTracker::from_cards(&played_cards),
//         remaining_cards: [
//             CardTracker::from_cards(&my_cards),
//             CardTracker::empty(),
//             CardTracker::empty(),
//             CardTracker::empty(),
//         ],
//     };
//
//     let moves = state.non_equivalent_moves_for(Seat::North);
//
//     expected.sort_unstable();
//
//     assert_eq!(expected, moves);
// }
// }
