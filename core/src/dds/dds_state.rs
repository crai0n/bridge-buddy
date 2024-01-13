use crate::dds::card_tracker::{CardTracker, RelativeTracker};
use crate::dds::dds_trick_manager::DdsTrickManager;
use crate::dds::relative_card::RelativeCard;
use crate::dds::relative_rank::RelativeRank;
use crate::primitives::card::Denomination;
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Hand, Suit};
use itertools::Itertools;

pub struct DdsState<const N: usize> {
    trick_manager: DdsTrickManager<N>,
    played_cards: CardTracker,
    remaining_cards: [CardTracker; 4],
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
            played_cards: CardTracker::empty(),
            remaining_cards: remaining_cards_tracker,
        }
    }

    pub fn play(&mut self, card: Card) {
        // println!("{} played {}", self.next_to_play(), card);
        self.remaining_cards[self.next_to_play() as usize].remove_card(card);
        self.played_cards.add_card(card);
        self.trick_manager.play(card)
    }

    pub fn relative_card(&self, card: Card) -> RelativeCard {
        // this is only correct if CardTracker is a played_card_tracker
        let rank_discriminant = card.denomination as u16;
        let suit_state = self.played_cards.only_suit(card.suit).field();

        let suit_state = suit_state >> (16 * card.suit as usize);

        let only_bits_above = suit_state >> (rank_discriminant + 1);
        let rank = RelativeRank::from(rank_discriminant + only_bits_above.count_ones() as u16);
        RelativeCard { rank, suit: card.suit }
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
            self.played_cards.remove_card(card);
            self.remaining_cards[self.next_to_play() as usize].add_card(card);
        }
    }

    pub fn remaining_cards_of(&self, player: Seat) -> Vec<Card> {
        self.remaining_cards[player as usize].all_contained_cards()
    }

    pub fn available_cards_of(&self, player: Seat) -> Vec<Card> {
        match self.trick_manager.suit_to_follow() {
            None => self.remaining_cards_of(player),
            Some(suit) => {
                let suit_cards = self.remaining_cards[player as usize].contained_cards_in_suit(suit);
                if suit_cards.is_empty() {
                    self.remaining_cards_of(player)
                } else {
                    suit_cards
                }
            }
        }
    }

    pub fn available_indistinguishable_moves_for(&self, player: Seat) -> Vec<Card> {
        let vec = self.indistinguishable_moves_for(player);
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

    pub fn indistinguishable_moves_for(&self, player: Seat) -> Vec<Card> {
        let rank_field = self.relative_cards_for_player(player).field();

        let mut tracking_field = !(rank_field >> 1) & rank_field; // marks only the highest of a sequence

        let mut vec = vec![];

        while tracking_field != 0 {
            let lowest_bit = tracking_field & (!tracking_field + 1);
            tracking_field &= !lowest_bit;
            let index = lowest_bit.ilog2();
            let suit = Suit::from((index / 16) as u16);
            let rank = RelativeRank::from((index % 16) as u16);
            let rel_card = RelativeCard { rank, suit };
            let card = self.absolute_card(rel_card);
            vec.push(card)
        }

        vec
    }

    pub fn played_cards(&self) -> Vec<Card> {
        self.played_cards.all_contained_cards()
    }

    #[allow(dead_code)]
    fn relative_suit_state_field_for_player(&self, player: Seat, suit: Suit) -> u16 {
        let mut ranks = 0u16;

        let mut absolute_field = self.remaining_cards[player as usize].only_suit(suit).field() as u16;
        let played_field = self.played_cards.only_suit(suit).field() as u16;

        while absolute_field != 0 {
            let cursor = absolute_field & (!absolute_field + 1); // isolates lowest bit
            let index = cursor.ilog2(); // position of cursor

            let pop_count = (played_field >> index).count_ones();
            let rank_index = index + pop_count;
            ranks |= 1 << rank_index; // add this card's relative rank
            absolute_field &= !cursor; // delete lowest bit
        }

        // there is an alternative algorithm using addition:
        let absolute_field = self.remaining_cards[player as usize].only_suit(suit).field();

        ranks = absolute_field as u16;
        let mut mask = 0u16;
        for index in 0..16 {
            let cursor = 1 << index;
            mask |= cursor;
            if played_field & cursor != 0 {
                let adder = ranks & mask;
                ranks += adder;
            }
        }

        // or with a while loop:
        ranks = absolute_field as u16;
        let mut search_field = played_field;
        while search_field != 0 {
            let cursor = search_field & (!search_field + 1); // isolate lowest bit
            search_field &= !cursor; // deletes lowest bit
            let mask = (cursor << 1) - 1; // masks out all higher bits
            let adder = ranks & mask;
            ranks += adder;
        }

        //TODO: Decide on an algorithm, if possible rewrite it so that it can be adapted to create an inverse operation

        ranks
    }

    fn relative_cards_for_player(&self, player: Seat) -> RelativeTracker {
        let mut ranks = 0u64;

        let my_field = self.remaining_cards[player as usize].field();
        let played_field = self.played_cards.field();

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
        RelativeTracker::from_u64(ranks)
    }

    pub fn absolute_card(&self, relative_card: RelativeCard) -> Card {
        let rank_discriminant = relative_card.rank as u16;
        let suit_state = self.played_cards.only_suit(relative_card.suit).field();

        let suit_state = suit_state >> (16 * relative_card.suit as usize) as u16;

        let zeros = rank_discriminant - suit_state.count_ones() as u16;

        let mut indicator = suit_state;

        for _ in 0..zeros {
            indicator |= 1 << indicator.trailing_ones();
        }

        let denomination_discriminant = indicator.trailing_ones() as u16;

        Card {
            suit: relative_card.suit,
            denomination: Denomination::from(denomination_discriminant),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::dds::card_tracker::{CardTracker, RelativeTracker};
    use crate::dds::dds_state::DdsState;
    use crate::dds::dds_trick_manager::DdsTrickManager;
    use crate::dds::relative_card::RelativeCard;
    use crate::dds::relative_rank::RelativeRank;
    use crate::primitives::card::Denomination;
    use crate::primitives::deal::Seat;
    use crate::primitives::{Card, Suit};
    use itertools::Itertools;
    use std::str::FromStr;
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
            played_cards: CardTracker::from_cards(&played_cards),
            remaining_cards: [
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
        let state: DdsState<13> = DdsState {
            trick_manager: DdsTrickManager::new(Seat::North, None),
            played_cards: CardTracker::from_u64(played_field as u64),
            remaining_cards: [
                CardTracker::from_u64(my_field as u64),
                CardTracker::empty(),
                CardTracker::empty(),
                CardTracker::empty(),
            ],
        };

        let expected = RelativeTracker::from_u64(expected as u64);

        assert_eq!(state.relative_cards_for_player(Seat::North), expected)
    }

    #[test_case(
        0b0000_0011_1001_0110_0000_0011_0000_1001_0000_0011_0000_1000,
        0b0001_1000_0000_1001_0000_1100_0110_0110_0000_1100_0110_0110,
        0b0000_1110_0111_0000_0000_1100_1100_0000_0000_1100_1000_0000
    )]

    fn rank_field64(my_field: u64, played_field: u64, expected: u64) {
        let state: DdsState<13> = DdsState {
            trick_manager: DdsTrickManager::new(Seat::North, None),
            played_cards: CardTracker::from_u64(played_field),
            remaining_cards: [
                CardTracker::from_u64(my_field),
                CardTracker::empty(),
                CardTracker::empty(),
                CardTracker::empty(),
            ],
        };

        let expected = RelativeTracker::from_u64(expected);

        assert_eq!(state.relative_cards_for_player(Seat::North), expected)
    }

    #[test_case("D2", &[], RelativeCard { rank: RelativeRank::Thirteenth, suit: Suit::Diamonds})]
    #[test_case("S2", &["S3", "S5"], RelativeCard { rank: RelativeRank::Eleventh, suit: Suit::Spades})]
    #[test_case("D2", &["C3"], RelativeCard { rank: RelativeRank::Thirteenth, suit: Suit::Diamonds})]
    #[test_case("S3", &["D3", "S4", "S5", "S6", "D7", "D9", "C8"], RelativeCard { rank: RelativeRank::Ninth, suit: Suit::Spades})]
    #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "DT", "DK", "DA"], RelativeCard { rank: RelativeRank::Fourth, suit: Suit::Diamonds})]
    #[test_case("D8", &["D3", "D4", "D5", "D6", "D7", "D9", "DT", "DK", "DA"], RelativeCard { rank: RelativeRank::Third, suit: Suit::Diamonds})]
    fn relative_rank(card: &str, cards: &[&str], expected: RelativeCard) {
        let cards = cards.iter().map(|str| Card::from_str(str).unwrap()).collect_vec();

        let state: DdsState<13> = DdsState {
            trick_manager: DdsTrickManager::new(Seat::North, None),
            played_cards: CardTracker::from_cards(&cards),
            remaining_cards: [
                CardTracker::empty(),
                CardTracker::empty(),
                CardTracker::empty(),
                CardTracker::empty(),
            ],
        };

        let test_card = Card::from_str(card).unwrap();

        assert_eq!(state.relative_card(test_card), expected);
    }

    #[test_case("D2", &[])]
    #[test_case("S2", &["S3"])]
    #[test_case("D2", &["C3"])]
    #[test_case("S3", &["D3", "S4", "S5", "S6", "D7", "D9", "C8"])]
    #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "C8"])]
    #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "DT", "DK", "DA"])]
    #[test_case("D8", &["D3", "D4", "D5", "D6", "D7", "D9", "DT", "DK", "DA"])]
    fn absolute_card(card: &str, cards: &[&str]) {
        let cards = cards.iter().map(|str| Card::from_str(str).unwrap()).collect_vec();

        let state: DdsState<13> = DdsState {
            trick_manager: DdsTrickManager::new(Seat::North, None),
            played_cards: CardTracker::from_cards(&cards),
            remaining_cards: [
                CardTracker::empty(),
                CardTracker::empty(),
                CardTracker::empty(),
                CardTracker::empty(),
            ],
        };

        let test_card = Card::from_str(card).unwrap();

        let relative_card = state.relative_card(test_card);

        assert_eq!(state.absolute_card(relative_card), test_card);
    }
}
