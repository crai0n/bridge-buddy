use crate::dds::card_tracker::{CardTracker, RelativeTracker};
use crate::dds::relative_card::RelativeCard;
use crate::dds::relative_rank::RelativeRank;
use crate::primitives::card::Denomination;
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Hand, Suit};
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

    pub fn remaining_cards_for_player(&self, player: Seat) -> CardTracker {
        self.remaining_cards[player as usize]
    }

    pub fn remaining_cards_of(&self, player: Seat) -> Vec<Card> {
        self.remaining_cards_for_player(player).all_contained_cards()
    }

    pub fn non_equivalent_moves_for(&self, player: Seat) -> Vec<Card> {
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

    #[allow(dead_code)]
    pub fn played_cards(&self) -> Vec<Card> {
        self.played_cards.all_contained_cards()
    }

    pub fn relative_cards_for_player(&self, player: Seat) -> RelativeTracker {
        self.remaining_cards_for_player(player)
            .relative_cards_given_played_cards(&self.played_cards)
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

    #[allow(dead_code)]
    fn relative_suit_state_field_for_player(&self, player: Seat, suit: Suit) -> u16 {
        let mut ranks = 0u16;

        let mut absolute_field = self.remaining_cards_for_player(player).only_suit(suit).field() as u16;
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
        let absolute_field = self.remaining_cards_for_player(player).only_suit(suit).field();

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
}

#[cfg(test)]
mod test {
    use crate::dds::card_manager::CardManager;
    use crate::dds::card_tracker::{CardTracker, RelativeTracker};
    use crate::primitives::deal::Seat;
    use test_case::test_case;

    #[test_case(
        0b0000_0011_1001_0110_0000_0011_0000_1001_0000_0011_0000_1000,
        0b0001_1000_0000_1001_0000_1100_0110_0110_0000_1100_0110_0110,
        0b0000_1110_0111_0000_0000_1100_1100_0000_0000_1100_1000_0000
    )]
    fn rank_field64(my_field: u64, played_field: u64, expected: u64) {
        let card_manager = CardManager {
            played_cards: CardTracker::from_u64(played_field),
            remaining_cards: [
                CardTracker::from_u64(my_field),
                CardTracker::empty(),
                CardTracker::empty(),
                CardTracker::empty(),
            ],
        };

        let expected = RelativeTracker::from_u64(expected);

        assert_eq!(card_manager.relative_cards_for_player(Seat::North), expected)
    }

    #[test_case(0b0000_0011_0000_1000, 0b0000_1100_0110_0110, 0b0000_1100_1000_0000)]
    #[test_case(0b0000_0011_0000_1001, 0b0000_1100_0110_0110, 0b0000_1100_1100_0000)]
    #[test_case(0b0000_0011_1001_0110, 0b0001_1000_0000_1001, 0b0000_1110_0111_0000)]
    fn rank_field(my_field: u16, played_field: u16, expected: u16) {
        let card_manager = CardManager {
            played_cards: CardTracker::from_u64(played_field as u64),
            remaining_cards: [
                CardTracker::from_u64(my_field as u64),
                CardTracker::empty(),
                CardTracker::empty(),
                CardTracker::empty(),
            ],
        };

        let expected = RelativeTracker::from_u64(expected as u64);

        assert_eq!(card_manager.relative_cards_for_player(Seat::North), expected)
    }
}
