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
        let mut interesting_bit = 1u64;
        for index in 0u8..64 {
            if self.tracking_field & interesting_bit != 0 {
                let suit = Suit::from((index / 16) as u16);
                let denomination = Denomination::from((index % 16) as u16);
                vec.push(Card { suit, denomination })
            }
            interesting_bit <<= 1;
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

    pub fn suit_state(&self, suit: Suit) -> u16 {
        (self.tracking_field >> (16 * suit as usize)) as u16
    }

    pub fn relative_rank(&self, card: Card) -> RelativeRank {
        let mut rank_discriminant = card.denomination as u16;
        let suit_state = self.suit_state(card.suit);
        let mut interesting_bit = 1u16 << (rank_discriminant + 1);

        while suit_state & interesting_bit != 0 {
            interesting_bit <<= 1;
            rank_discriminant += 1;
        }
        RelativeRank::from(rank_discriminant)
    }

    pub fn absolute_card(&self, relative_rank: RelativeRank, suit: Suit) -> Card {
        let mut rank_discriminant = relative_rank as u16;
        let suit_state = self.suit_state(suit);
        let mut interesting_bit = 1u16 << rank_discriminant;

        while suit_state & interesting_bit != 0 {
            interesting_bit >>= 1;
            rank_discriminant -= 1;
        }
        Card {
            suit,
            denomination: Denomination::from(rank_discriminant),
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
    #[test_case("S2", &["S3"], RelativeRank::Twelveth)]
    #[test_case("D2", &["C3"], RelativeRank::Thirteenth)]
    #[test_case("S3", &["D3", "S4", "S5", "S6", "D7", "D9", "C8"], RelativeRank::Ninth)]
    #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "C8"], RelativeRank::Eigth)]
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
