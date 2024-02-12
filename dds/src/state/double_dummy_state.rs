use crate::card_manager::card_tracker::CardTracker;
use crate::card_manager::CardManager;
use bridge_buddy_core::game::trick_manager::TrickManager;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::{Card, Hand, Suit};

pub struct DoubleDummyState<const N: usize> {
    trick_manager: TrickManager<N>,
    card_manager: CardManager,
}

#[allow(dead_code)]
impl<const N: usize> DoubleDummyState<N> {
    pub fn new(hands: [Hand<N>; 4], opening_leader: Seat, trumps: Option<Suit>) -> Self {
        Self {
            trick_manager: TrickManager::new(opening_leader, trumps),
            card_manager: CardManager::from_hands(hands),
        }
    }

    pub fn would_win_over_current_winner(&self, card: &Card) -> bool {
        self.trick_manager.would_win_over_current_winner(card)
    }

    pub fn count_cards_in_suit_for_player(&self, suit: Suit, player: Seat) -> usize {
        self.card_manager.count_cards_in_suit_for_player(suit, player)
    }

    pub fn player_is_void_in(&self, suit: Suit, player: Seat) -> bool {
        self.count_cards_in_suit_for_player(suit, player) == 0
    }

    pub fn player_has_singleton_in(&self, suit: Suit, player: Seat) -> bool {
        self.count_cards_in_suit_for_player(suit, player) == 1
    }

    pub fn player_has_doubleton_in(&self, suit: Suit, player: Seat) -> bool {
        self.count_cards_in_suit_for_player(suit, player) == 2
    }

    pub fn count_played_cards(&self) -> usize {
        self.trick_manager.count_played_cards()
    }

    pub fn player_has_higher_cards_in_suit_than_other(&self, player: Seat, suit: Suit, other: Seat) -> bool {
        self.card_manager
            .has_higher_cards_in_suit_than_other(player, suit, other)
    }

    pub fn partner_has_higher_cards_than_opponents(&self, suit: Suit, leader: Seat) -> bool {
        self.player_has_higher_cards_in_suit_than_other(leader + 2, suit, leader + 1)
            && self.player_has_higher_cards_in_suit_than_other(leader + 2, suit, leader + 3)
    }

    pub fn remaining_cards_of(&self, player: Seat) -> impl Iterator<Item = Card> + '_ {
        self.card_manager.remaining_cards_of(player)
    }

    pub fn cards_of(&self, player: Seat) -> &CardTracker {
        self.card_manager.remaining_cards_for_player(player)
    }

    pub fn remaining_cards_of_player_in_suit(&self, player: Seat, suit: Suit) -> impl Iterator<Item = Card> + '_ {
        self.card_manager.remaining_cards_of_player_in_suit(player, suit)
    }

    pub fn current_trick_winner(&self) -> Seat {
        self.trick_manager.current_trick_winner()
    }

    pub fn currently_winning_card(&self) -> Option<Card> {
        self.trick_manager.currently_winning_card()
    }

    pub fn count_trump_cards_for_player(&self, player: Seat) -> usize {
        match self.trump_suit() {
            None => 0,
            Some(trump_suit) => self.card_manager.count_cards_in_suit_for_player(trump_suit, player),
        }
    }

    pub fn count_trump_cards_for_axis(&self, player: Seat) -> usize {
        self.count_trump_cards_for_player(player) + self.count_trump_cards_for_player(player.partner())
    }

    pub fn suit_to_follow(&self) -> Option<Suit> {
        self.trick_manager.suit_to_follow()
    }

    pub fn count_this_sides_trump_cards(&self) -> usize {
        self.count_trump_cards_for_axis(self.next_to_play())
    }

    pub fn count_opponents_trump_cards(&self) -> usize {
        self.count_trump_cards_for_axis(self.next_to_play() + 1)
    }

    pub fn out_of_play_cards(&self) -> &[Card] {
        self.trick_manager.out_of_play_cards()
    }

    pub fn count_cards_in_current_trick(&self) -> usize {
        self.trick_manager.count_cards_in_current_trick()
    }

    pub fn next_to_play(&self) -> Seat {
        self.trick_manager.next_to_play()
    }

    pub fn tricks_left(&self) -> usize {
        self.trick_manager.tricks_left()
    }

    pub fn is_last_trick(&self) -> bool {
        self.trick_manager.tricks_left() == 1
    }

    pub fn tricks_won_by_axis(&self, player: Seat) -> usize {
        self.trick_manager.tricks_won_by_axis(player)
    }

    pub fn last_trick_winner(&self) -> Option<Seat> {
        self.trick_manager.last_trick_winner()
    }

    pub fn player_is_leading(&self) -> bool {
        self.trick_manager.suit_to_follow().is_none()
    }

    pub fn trump_suit(&self) -> Option<Suit> {
        self.trick_manager.trump_suit()
    }

    pub fn play(&mut self, card: Card) {
        self.card_manager.play(card, self.next_to_play());
        self.trick_manager.play(card)
    }

    pub fn undo(&mut self) {
        if let Some(card) = self.trick_manager.undo() {
            self.card_manager.unplay(card, self.next_to_play());
        }
    }

    pub fn valid_moves_for(&self, player: Seat) -> Vec<Card> {
        self.cards_of(player).valid_moves(self.suit_to_follow())
    }

    pub fn valid_moves(&self) -> Vec<Card> {
        self.valid_moves_for(self.next_to_play())
    }

    pub fn list_played_cards(&self) -> &[Card] {
        self.trick_manager.played_cards()
    }

    pub fn played_cards(&self) -> CardTracker {
        self.card_manager.played_cards()
    }
}

// #[cfg(test)]
// mod test {
//     use super::DoubleDummyState;
//     use crate::card_manager::card_tracker::CardTracker;
//     use crate::card_manager::CardManager;
//     use bridge_buddy_core::game::trick_manager::TrickManager;
//     use bridge_buddy_core::primitives::card::Rank;
//     use bridge_buddy_core::primitives::deal::Seat;
//     use bridge_buddy_core::primitives::{Card, Suit};
//     use itertools::Itertools;
//     use test_case::test_case;

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
//     let state: DoubleDummyState<13> = DoubleDummyState {
//         trick_manager: TrickManager::new(Seat::North, None),
//         card_manager: CardManager {
//             played_cards: CardTracker::from_cards(&played_cards),
//             remaining_cards: [
//                 CardTracker::from_cards(&my_cards),
//                 CardTracker::empty(),
//                 CardTracker::empty(),
//                 CardTracker::empty(),
//             ],
//         },
//     };
//
//     let moves = state.valid_non_equivalent_moves_for(Seat::North);
//
//     expected.sort_unstable();
//
//     assert_eq!(expected, moves)
// }
// }
