use crate::dds::relative_rank::RelativeRank;
use crate::primitives::card::Denomination;
use crate::primitives::{Card, Hand, Suit};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct CardTracker {
    tracking_field: u64,
}

impl CardTracker {
    pub fn empty() -> Self {
        Self { tracking_field: 0u64 }
    }

    pub fn from_field(tracking_field: u64) -> Self {
        Self { tracking_field }
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            tracking_field: self.tracking_field | other.tracking_field,
        }
    }

    pub fn from_cards(cards: &[Card]) -> Self {
        let mut tracker = Self::empty();

        for &card in cards {
            tracker.add_card(card)
        }
        tracker
    }

    pub fn from_hand<const N: usize>(hand: Hand<N>) -> Self {
        let mut tracker = Self::empty();

        for &card in hand.cards() {
            tracker.add_card(card)
        }

        tracker
    }

    const fn u64_from_card(card: Card) -> u64 {
        1 << (card.denomination as usize + 16 * card.suit as usize)
    }

    pub fn add_card(&mut self, card: Card) {
        self.tracking_field |= Self::u64_from_card(card)
    }

    pub fn remove_card(&mut self, card: Card) {
        self.tracking_field &= !Self::u64_from_card(card)
    }

    pub fn contains_card(&self, card: Card) -> bool {
        self.tracking_field & Self::u64_from_card(card) != 0
    }

    pub fn contained_cards(&self) -> Vec<Card> {
        let mut vec = vec![];
        for suit in Suit::iter() {
            for denomination in Denomination::iter() {
                let interesting_bit = 1 << (denomination as usize + 16 * suit as usize);
                if self.tracking_field & interesting_bit != 0 {
                    vec.push(Card { denomination, suit });
                }
            }
        }
        vec
    }

    pub fn all_contained_cards(&self) -> Vec<Card> {
        let mut vec = vec![];
        let mut tracking_field = self.tracking_field;

        while tracking_field != 0 {
            let lowest_bit = tracking_field & (!tracking_field + 1);
            tracking_field &= !lowest_bit;
            let index = lowest_bit.ilog2();
            let suit = Suit::from((index / 16) as u16);
            let denomination = Denomination::from((index % 16) as u16);
            vec.push(Card { suit, denomination })
        }

        vec
    }

    pub fn contained_cards_in_suit(&self, suit: Suit) -> Vec<Card> {
        let mut vec = vec![];
        let field = self.suit_state(suit);
        let mut interesting_bit = 1u16;
        for index in 0u8..16 {
            if field & interesting_bit != 0 {
                let denomination = Denomination::from((index % 16) as u16);
                vec.push(Card { suit, denomination })
            }
            interesting_bit <<= 1;
        }
        vec
    }

    pub fn field(&self) -> u64 {
        self.tracking_field
    }

    pub fn tops_of_sequences_field(&self) -> u64 {
        let field = self.tracking_field;

        !(field >> 1) & field
    }

    pub fn suit_state(&self, suit: Suit) -> u16 {
        (self.tracking_field >> (16 * suit as usize)) as u16
    }

    pub fn relative_rank(&self, card: Card) -> RelativeRank {
        let rank_discriminant = card.denomination as u16;
        let suit_state = self.suit_state(card.suit);
        let only_bits_above = suit_state >> (rank_discriminant + 1);
        RelativeRank::from(rank_discriminant + only_bits_above.count_ones() as u16)
    }

    pub fn absolute_card(&self, relative_rank: RelativeRank, suit: Suit) -> Card {
        let rank_discriminant = relative_rank as u16;
        let suit_state = self.suit_state(suit);

        let zeros = rank_discriminant - suit_state.count_ones() as u16;

        let mut indicator = suit_state;

        for _ in 0..zeros {
            indicator |= 1 << indicator.trailing_ones();
        }

        let denomination_discriminant = indicator.trailing_ones() as u16;

        Card {
            suit,
            denomination: Denomination::from(denomination_discriminant),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::dds::card_tracker::CardTracker;
    use crate::dds::relative_rank::RelativeRank;
    use crate::primitives::{Card, Hand, Suit};
    use itertools::Itertools;
    use std::str::FromStr;
    use test_case::test_case;

    #[test]
    fn card_tracker() {
        let mut tracker = CardTracker::empty();

        tracker.add_card(Card::from_str("C2").unwrap());
        assert_eq!(tracker.tracking_field, 1);
    }

    #[test_case("D2", &[], RelativeRank::Thirteenth)]
    #[test_case("S2", &["S3", "S5"], RelativeRank::Eleventh)]
    #[test_case("D2", &["C3"], RelativeRank::Thirteenth)]
    #[test_case("S3", &["D3", "S4", "S5", "S6", "D7", "D9", "C8"], RelativeRank::Ninth)]
    #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "DT", "DK", "DA"], RelativeRank::Fourth)]
    fn relative_rank(card: &str, cards: &[&str], expected: RelativeRank) {
        let mut tracker = CardTracker::empty();

        let test_card = Card::from_str(card).unwrap();

        for card_str in cards {
            let card = Card::from_str(card_str).unwrap();
            tracker.add_card(card);
        }

        assert_eq!(tracker.relative_rank(test_card), expected);
    }

    #[test_case("D2", &[])]
    #[test_case("S2", &["S3"])]
    #[test_case("D2", &["C3"])]
    #[test_case("S3", &["D3", "S4", "S5", "S6", "D7", "D9", "C8"])]
    #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "C8"])]
    #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "DT", "DK", "DA"])]
    #[test_case("D8", &["D3", "D4", "D5", "D6", "D7", "D9", "DT", "DK", "DA"])]
    fn absolute_card(card: &str, cards: &[&str]) {
        let mut tracker = CardTracker::empty();

        let test_card = Card::from_str(card).unwrap();

        for card_str in cards {
            let card = Card::from_str(card_str).unwrap();
            tracker.add_card(card);
        }

        let relative_rank = tracker.relative_rank(test_card);
        let suit = test_card.suit;

        assert_eq!(tracker.absolute_card(relative_rank, suit), test_card);
    }

    #[test_case("H:AQ,C:AQJ", Suit::Hearts)]
    #[test_case("H:AKQ,D:AT", Suit::Diamonds)]
    #[test_case("H:AKQT,S:T", Suit::Spades)]
    fn contained_cards_in_suit(hand_str: &str, suit: Suit) {
        let hand = Hand::<5>::from_str(hand_str).unwrap();
        let tracker = CardTracker::from_hand(hand);
        assert_eq!(tracker.contained_cards(), hand.cards().copied().collect_vec());
        assert_eq!(
            tracker.contained_cards_in_suit(suit),
            hand.cards_in(suit).copied().collect_vec()
        )
    }
}
