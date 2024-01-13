use crate::dds::card_tracker::CardTracker;
use crate::dds::dds_trick_manager::DdsTrickManager;
use crate::dds::relative_rank::RelativeRank;
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Hand, Suit};
use itertools::Itertools;

pub struct DdsState<const N: usize> {
    trick_manager: DdsTrickManager<N>,
    played_cards_tracker: CardTracker,
    remaining_cards_tracker: [CardTracker; 4],
}

impl<const N: usize> DdsState<N> {
    pub fn new(hands: [Hand<N>; 4], opening_leader: Seat, trumps: Option<Suit>) -> Self {
        let remaining_cards_tracker: [CardTracker; 4] = hands
            .iter()
            .map(|hand| CardTracker::from_hand(*hand))
            .collect_vec()
            .try_into()
            .unwrap();

        Self {
            trick_manager: DdsTrickManager::new(opening_leader, trumps),
            played_cards_tracker: CardTracker::empty(),
            remaining_cards_tracker,
        }
    }

    pub fn play(&mut self, card: Card) {
        // println!("{} played {}", self.next_to_play(), card);
        self.remaining_cards_tracker[self.next_to_play() as usize].remove_card(card);
        self.played_cards_tracker.add_card(card);
        self.trick_manager.play(card)
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

    pub fn undo(&mut self) {
        if let Some(card) = self.trick_manager.undo() {
            self.played_cards_tracker.remove_card(card);
            self.remaining_cards_tracker[self.next_to_play() as usize].add_card(card);
        }
    }

    pub fn remaining_cards_of(&self, player: Seat) -> Vec<Card> {
        self.remaining_cards_tracker[player as usize].all_contained_cards()
    }

    pub fn available_cards_of(&self, player: Seat) -> Vec<Card> {
        match self.trick_manager.suit_to_follow() {
            None => self.remaining_cards_of(player),
            Some(suit) => {
                let suit_cards = self.remaining_cards_tracker[player as usize].contained_cards_in_suit(suit);
                if suit_cards.is_empty() {
                    self.remaining_cards_of(player)
                } else {
                    suit_cards
                }
            }
        }
    }

    pub fn indistinguishable_moves_for(&self, player: Seat) -> Vec<Card> {
        let full_tracker = self
            .played_cards_tracker
            .union(&self.remaining_cards_tracker[player as usize]);

        let all_tops_field = full_tracker.tops_of_sequences_field(); // this gives artifacts. it marks cards that I do not own (in case I dont have cards touching a sequence of played cards
        let my_tops_field = self.remaining_cards_tracker[player as usize].tops_of_sequences_field(); // all tops of my sequences
        let pl_tops_field = self.played_cards_tracker.tops_of_sequences_field(); // all tops of played sequences

        let unique_field = (all_tops_field | my_tops_field) & (!pl_tops_field);

        let unique_field2 = my_tops_field & !pl_tops_field; // all tops of my sequences

        assert_eq!(unique_field, unique_field2);

        let unique_tracker = CardTracker::from_field(unique_field);

        let moves = unique_tracker.all_contained_cards();

        println!("new method found the following moves {:?}", moves);
        moves
    }

    pub fn available_indistinguishable_moves_for(&self, player: Seat) -> Vec<Card> {
        let my_field = self.remaining_cards_tracker[player as usize].field();
        let played_field = self.played_cards_tracker.field();
        let rank_field = Self::relative_ranks64(my_field, played_field);

        let rank_tracker = CardTracker::from_field(rank_field);
        let mut tracking_field = rank_tracker.tops_of_sequences_field();

        let mut vec = vec![];

        while tracking_field != 0 {
            let lowest_bit = tracking_field & (!tracking_field + 1);
            tracking_field &= !lowest_bit;
            let index = lowest_bit.ilog2();
            let suit = Suit::from((index / 16) as u16);
            let rank = RelativeRank::from((index % 16) as u16);
            let card = self.played_cards_tracker.absolute_card(rank, suit);
            vec.push(card)
        }

        match self.trick_manager.suit_to_follow() {
            None => vec,
            Some(suit) => {
                let filtered = vec.iter().filter(|x| x.suit == suit).copied().collect_vec();
                if filtered.is_empty() {
                    vec
                } else {
                    filtered
                }
            }
        }
    }

    pub fn all_played_cards(&self) -> Vec<Card> {
        self.played_cards_tracker.all_contained_cards()
    }

    pub fn relative_ranks(my_field: u16, played_field: u16) -> u16 {
        let mut ranks = 0u16;

        for index in 0..16 {
            let cursor = 1 << index;
            if my_field & cursor != 0 {
                let pop_count = (played_field >> index).count_ones();
                let rank_index = index + pop_count;
                ranks |= 1 << rank_index
            }
        }
        ranks
    }

    pub fn relative_ranks64(my_field: u64, played_field: u64) -> u64 {
        let mut ranks = 0u64;

        for suit_index in 0..4 {
            for index in 0..16 {
                let cursor = 1 << index << (suit_index * 16);
                if my_field & cursor != 0 {
                    let played_field = played_field >> (suit_index * 16);
                    let shifted = (played_field as u16) >> index;
                    let pop_count = shifted.count_ones();
                    let rank_index = index + pop_count;
                    ranks |= 1 << rank_index << (suit_index * 16);
                }
            }
        }
        ranks
    }
}

#[cfg(test)]
mod test {
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
            played_cards_tracker: CardTracker::from_cards(&played_cards),
            remaining_cards_tracker: [
                CardTracker::from_cards(&my_cards),
                CardTracker::empty(),
                CardTracker::empty(),
                CardTracker::empty(),
            ],
        };

        let moves = state.available_indistinguishable_moves_for(Seat::North);

        expected.sort_unstable();

        assert_eq!(expected, moves)
    }

    #[test_case(0b0000_0011_0000_1000, 0b0000_1100_0110_0110, 0b0000_1100_1000_0000)]
    #[test_case(0b0000_0011_0000_1001, 0b0000_1100_0110_0110, 0b0000_1100_1100_0000)]
    #[test_case(0b0000_0011_1001_0110, 0b0001_1000_0000_1001, 0b0000_1110_0111_0000)]
    fn rank_field(my_field: u16, played_field: u16, expected: u16) {
        assert_eq!(DdsState::<13>::relative_ranks(my_field, played_field), expected)
    }

    #[test_case(
        0b0000_0011_1001_0110_0000_0011_0000_1001_0000_0011_0000_1000,
        0b0001_1000_0000_1001_0000_1100_0110_0110_0000_1100_0110_0110,
        0b0000_1110_0111_0000_0000_1100_1100_0000_0000_1100_1000_0000
    )]

    fn rank_field64(my_field: u64, played_field: u64, expected: u64) {
        assert_eq!(DdsState::<13>::relative_ranks64(my_field, played_field), expected)
    }
}
