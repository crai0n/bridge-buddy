use crate::dds::card_manager::CardManager;
use crate::dds::dds_trick_manager::DdsTrickManager;
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Hand, Suit};
use itertools::Itertools;
#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};

pub struct DdsState<const N: usize> {
    trick_manager: DdsTrickManager<N>,
    card_manager: CardManager,
}

impl<const N: usize> DdsState<N> {
    pub fn new(hands: [Hand<N>; 4], opening_leader: Seat, trumps: Option<Suit>) -> Self {
        Self {
            trick_manager: DdsTrickManager::new(opening_leader, trumps),
            card_manager: CardManager::from_hands(hands),
        }
    }

    pub fn next_to_play(&self) -> Seat {
        self.trick_manager.next_to_play()
    }

    pub fn tricks_left(&self) -> usize {
        self.trick_manager.tricks_left()
    }

    pub fn last_trick_winner(&self) -> Option<Seat> {
        self.trick_manager.last_trick_winner()
    }

    pub fn tricks_won_by_axis(&self, player: Seat) -> usize {
        self.trick_manager.tricks_won_by_axis(player)
    }

    pub fn player_is_leading(&self) -> bool {
        self.trick_manager.suit_to_follow().is_none()
    }

    fn trumps(&self) -> Option<Suit> {
        self.trick_manager.trumps()
    }

    pub fn play(&mut self, card: Card) {
        // println!("{} played {}", self.next_to_play(), card);
        self.card_manager.play(card, self.next_to_play());
        self.trick_manager.play(card)
    }

    pub fn undo(&mut self) {
        if let Some(card) = self.trick_manager.undo() {
            self.card_manager.unplay(card, self.next_to_play());
        }
    }

    pub fn valid_moves_for(&self, player: Seat) -> Vec<Card> {
        let moves = self.card_manager.remaining_cards_of(player);
        self.only_valid(moves)
    }

    pub fn valid_non_equivalent_moves_for(&self, player: Seat) -> Vec<Card> {
        let non_equivalent_moves = self.card_manager.non_equivalent_moves_for(player);
        self.only_valid(non_equivalent_moves)
    }

    pub fn only_valid(&self, moves: Vec<Card>) -> Vec<Card> {
        match self.trick_manager.suit_to_follow() {
            None => moves,
            Some(suit) => {
                let filtered = moves.iter().filter(|x| x.suit == suit).copied().collect_vec();
                if filtered.is_empty() {
                    moves
                } else {
                    filtered
                }
            }
        }
    }

    pub fn quick_tricks_for_player(&self, player: Seat) -> u8 {
        // Quick tricks are the tricks that an axis can take without losing the lead.
        // For this, we need to look at both hands combined
        let players = [player, player.partner()];
        let cards = players.map(|x| self.card_manager.remaining_cards_for_player(x));

        let my_quick_tricks = cards[0]
            .relative_cards_given_played_cards(&self.card_manager.played_cards)
            .count_high_cards_per_suit();

        // TODO: we need to make sure that we have entries into partners hand in any case, trumps or not!
        // check for communication between hands
        // communication exists if we have a card in a suit for which partner has a quick trick
        // assumption: Communication exists, if there is a low card in our hand and a quick trick in partners hand in the same suit
        // let partners_quick_tricks = cards[1]
        //             .relative_cards_given_played_cards(&self.played_cards)
        //             .quick_tricks_per_suit();
        // let communications =
        //
        //
        // let combined_cards = cards[0].union(&cards[1]);
        // // In a perfect world, we would get to use all our high cards
        // let max_quick_tricks = combined_cards
        //     .relative_cards_given_played_cards(&self.played_cards)
        //     .quick_tricks_per_suit();
        // // but we can never make more tricks in a suit than we have cards in the longest hand
        // let cards_per_suit = players.map(|x| self.remaining_cards_for_player(x).cards_per_suit());
        // let max_cards_per_suit = [0, 1, 2, 3].map(|i| max(cards_per_suit[0][i], cards_per_suit[1][i]));
        // let higher_bounds = [0, 1, 2, 3].map(|i| min(max_quick_tricks[i], max_cards_per_suit[i]));
        //

        let higher_bounds = my_quick_tricks;
        // println!("I have at most {:?} quick tricks.", higher_bounds);

        // To reach this maximum number of quick tricks we need to make sure that opponents cannot ruff.
        let final_quick_tricks = match self.trumps() {
            None => {
                // count all quick tricks,
                higher_bounds.iter().sum()
            }
            Some(trump_suit) => {
                // there is a trump suit.

                // first count only trump quick-tricks
                higher_bounds[trump_suit as usize]

                // TODO: We can make more quick tricks if opponents run out of trumps
                // let opponents = [player + 1, player + 3];
                // let opponents_cards_per_suit = opponents.map(|x| self.remaining_cards_for_player(x).cards_per_suit());
                // let opponents_max_trump_cards = [0, 1, 2, 3].map(|i| max(cards_per_suit[0][i], cards_per_suit[1][i]));

                // let trump_quick_tricks = higher_bounds[trump_suit as usize];
                // match opponents_cards_per_suit.map(|array| array[trump_suit as usize].cmp(&trump_quick_tricks)) {
                //     [Ordering::Greater, Ordering::Greater] => {
                //         // both opponents have trumps left
                //
                //         let min_opponents_suit_length =
                //             [0, 1, 2, 3].map(|i| min(opponents_cards_per_suit[0][i], opponents_cards_per_suit[1][i]));
                //
                //         // only count quick tricks until one opponent can ruff
                //         let quick_tricks = [0, 1, 2, 3].map(|i| min(min_opponents_suit_length[i], higher_bounds[i]));
                //         // this never under-counts trump tricks because both opponents have more trumps than we do.
                //
                //         quick_tricks.iter().sum()
                //     }
                //     [Ordering::Greater, _] => {
                //         // Left-Hand Opponent has trumps left
                //         // Right-Hand Opponent cannot ruff anymore
                //         let lho_suit_lengths = opponents_cards_per_suit[0];
                //
                //         // only count quick tricks until one opponent can ruff
                //         let quick_tricks = [0, 1, 2, 3].map(|i| min(lho_suit_lengths[i], higher_bounds[i]));
                //         // this never under-counts trump tricks because lho has more trumps than we do.
                //
                //         quick_tricks.iter().sum()
                //     }
                //     [_, Ordering::Greater] => {
                //         // Left-Hand Opponent cannot ruff anymore
                //         // Right-Hand Opponent has trumps left
                //         let rho_suit_lengths = opponents_cards_per_suit[1];
                //
                //         // only count quick tricks until one opponent can ruff
                //         let quick_tricks = [0, 1, 2, 3].map(|i| min(rho_suit_lengths[i], higher_bounds[i]));
                //         // this never under-counts trump tricks because rho has more trumps than we do.
                //
                //         quick_tricks.iter().sum()
                //     }
                //     _ => {
                //         // Opponents have no trumps left
                //         // we can play our side-suit high cards safely
                //         higher_bounds.iter().sum()
                //     }
                // }
            }
        };

        // println!("I have {} quick tricks.", final_quick_tricks);

        final_quick_tricks
    }
}

#[cfg(test)]
mod test {
    use crate::dds::card_manager::CardManager;
    use crate::dds::card_tracker::CardTracker;
    use crate::dds::dds_state::DdsState;
    use crate::dds::dds_trick_manager::DdsTrickManager;
    use crate::primitives::card::Denomination;
    use crate::primitives::deal::Seat;
    use crate::primitives::{Card, Suit};
    use itertools::Itertools;
    use test_case::test_case;

    #[test_case("JT5", "KQ8743", "J5")] // 0001100001000, 0110001100110, 0001000001000
    #[test_case("JT52", "KQ8743", "J5")] // 0001100001001, 0110001100110, 0001000001000
    #[test_case("JT9643", "AK52", "J6")] // 0001110010110, 1100000001001, 0001000010000
    fn available_moves(my_cards: &str, played_cards: &str, expected: &str) {
        let my_cards = my_cards
            .chars()
            .map(|c| Denomination::from_char(c).unwrap())
            .map(|d| Card {
                denomination: d,
                suit: Suit::Spades,
            })
            .collect_vec();
        let played_cards = played_cards
            .chars()
            .map(|c| Denomination::from_char(c).unwrap())
            .map(|d| Card {
                denomination: d,
                suit: Suit::Spades,
            })
            .collect_vec();
        let mut expected = expected
            .chars()
            .map(|c| Denomination::from_char(c).unwrap())
            .map(|d| Card {
                denomination: d,
                suit: Suit::Spades,
            })
            .collect_vec();

        let state: DdsState<13> = DdsState {
            trick_manager: DdsTrickManager::new(Seat::North, None),
            card_manager: CardManager {
                played_cards: CardTracker::from_cards(&played_cards),
                remaining_cards: [
                    CardTracker::from_cards(&my_cards),
                    CardTracker::empty(),
                    CardTracker::empty(),
                    CardTracker::empty(),
                ],
            },
        };

        let moves = state.valid_non_equivalent_moves_for(Seat::North);

        expected.sort_unstable();

        assert_eq!(expected, moves)
    }

    // #[test_case("D2", &[], RelativeCard { rank: RelativeRank::Thirteenth, suit: Suit::Diamonds})]
    // #[test_case("S2", &["S3", "S5"], RelativeCard { rank: RelativeRank::Eleventh, suit: Suit::Spades})]
    // #[test_case("D2", &["C3"], RelativeCard { rank: RelativeRank::Thirteenth, suit: Suit::Diamonds})]
    // #[test_case("S3", &["D3", "S4", "S5", "S6", "D7", "D9", "C8"], RelativeCard { rank: RelativeRank::Ninth, suit: Suit::Spades})]
    // #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "DT", "DK", "DA"], RelativeCard { rank: RelativeRank::Fourth, suit: Suit::Diamonds})]
    // #[test_case("D8", &["D3", "D4", "D5", "D6", "D7", "D9", "DT", "DK", "DA"], RelativeCard { rank: RelativeRank::Third, suit: Suit::Diamonds})]
    // fn relative_rank(card: &str, cards: &[&str], expected: RelativeCard) {
    //     let cards = cards.iter().map(|str| Card::from_str(str).unwrap()).collect_vec();
    //
    //     let state: DdsState<13> = DdsState {
    //         trick_manager: DdsTrickManager::new(Seat::North, None),
    //         card_manager: CardManager {
    //             played_cards: CardTracker::from_cards(&cards),
    //             remaining_cards: [
    //                 CardTracker::empty(),
    //                 CardTracker::empty(),
    //                 CardTracker::empty(),
    //                 CardTracker::empty(),
    //             ],
    //         },
    //     };
    //
    //     let test_card = Card::from_str(card).unwrap();
    //
    //     assert_eq!(state.relative_card(test_card), expected);
    // }

    // #[test_case("D2", &[])]
    // #[test_case("S2", &["S3"])]
    // #[test_case("D2", &["C3"])]
    // #[test_case("S3", &["D3", "S4", "S5", "S6", "D7", "D9", "C8"])]
    // #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "C8"])]
    // #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "DT", "DK", "DA"])]
    // #[test_case("D8", &["D3", "D4", "D5", "D6", "D7", "D9", "DT", "DK", "DA"])]
    // fn absolute_card(card: &str, cards: &[&str]) {
    //     let cards = cards.iter().map(|str| Card::from_str(str).unwrap()).collect_vec();
    //
    //     let state: DdsState<13> = DdsState {
    //         trick_manager: DdsTrickManager::new(Seat::North, None),
    //         card_manager: CardManager {
    //             played_cards: CardTracker::from_cards(&cards),
    //             remaining_cards: [
    //                 CardTracker::empty(),
    //                 CardTracker::empty(),
    //                 CardTracker::empty(),
    //                 CardTracker::empty(),
    //             ],
    //         },
    //     };
    //
    //     let test_card = Card::from_str(card).unwrap();
    //
    //     let relative_card = state.relative_card(test_card);
    //
    //     assert_eq!(state.absolute_card(relative_card), test_card);
    // }
}
