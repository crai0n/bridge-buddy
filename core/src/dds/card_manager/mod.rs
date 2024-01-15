pub mod card_tracker;
mod relative_tracker;
mod suit_tracker;

use crate::dds::card_manager::card_tracker::CardTracker;
use crate::dds::relative_card::RelativeCard;
use crate::primitives::card::Denomination;
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Hand};
use itertools::Itertools;
use relative_tracker::RelativeTracker;

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
        let ranks = self.relative_cards_for_player(player);

        let tops = ranks.only_tops_of_sequences(); // marks only the highest of a sequence

        let absolute = tops.absolute_cards_given_played_cards(&self.played_cards);

        absolute.all_contained_cards()
    }

    pub fn played_cards(&self) -> CardTracker {
        self.played_cards
    }

    pub fn relative_cards_for_player(&self, player: Seat) -> RelativeTracker {
        self.remaining_cards_for_player(player)
            .relative_cards_given_played_cards(&self.played_cards)
    }

    #[allow(dead_code)]
    pub fn absolute_card(&self, relative_card: RelativeCard) -> Card {
        let rank_discriminant = relative_card.rank as u16;
        let suit_state = *self.played_cards.suit_state(relative_card.suit);

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
    use crate::dds::card_manager::card_tracker::CardTracker;
    use crate::dds::card_manager::relative_tracker::RelativeTracker;
    use crate::dds::card_manager::CardManager;
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
