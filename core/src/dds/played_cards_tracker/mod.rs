use crate::dds::relative_rank::RelativeRank;
use crate::primitives::card::Denomination;
use crate::primitives::{Card, Suit};

pub struct PlayedCardsTracker {
    tracking_field: u64,
}

impl PlayedCardsTracker {
    pub fn empty() -> Self {
        Self { tracking_field: 0u64 }
    }

    const fn u64_from_card(card: Card) -> u64 {
        1 << (card.denomination as usize + 16 * card.suit as usize)
    }

    pub fn play_card(&mut self, card: Card) {
        self.tracking_field |= Self::u64_from_card(card)
    }

    pub fn unplay_card(&mut self, card: Card) {
        self.tracking_field &= !Self::u64_from_card(card)
    }

    pub fn card_is_played(&self, card: Card) -> bool {
        !self.card_is_unplayed(card)
    }

    pub fn card_is_unplayed(&self, card: Card) -> bool {
        self.tracking_field & Self::u64_from_card(card) == 0
    }

    pub fn suit_state(&self, suit: Suit) -> u16 {
        (self.tracking_field >> (16 * suit as usize)) as u16
    }

    pub const fn u16_from_denomination(denomination: Denomination) -> u16 {
        1 << (denomination as usize)
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
    use crate::dds::played_cards_tracker::PlayedCardsTracker;
    use crate::dds::relative_rank::RelativeRank;
    use crate::primitives::Card;
    use std::str::FromStr;
    use test_case::test_case;

    #[test]
    fn played_cards_tracker() {
        let mut tracker = PlayedCardsTracker::empty();

        tracker.play_card(Card::from_str("C2").unwrap());
        assert_eq!(tracker.tracking_field, 1);
    }

    #[test_case("D2", &[], RelativeRank::Thirteenth)]
    #[test_case("S2", &["S3"], RelativeRank::Twelveth)]
    #[test_case("D2", &["C3"], RelativeRank::Thirteenth)]
    #[test_case("S3", &["D3", "S4", "S5", "S6", "D7", "D9", "C8"], RelativeRank::Ninth)]
    #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "C8"], RelativeRank::Eigth)]
    fn relative_rank(card: &str, cards: &[&str], expected: RelativeRank) {
        let mut tracker = PlayedCardsTracker::empty();

        let test_card = Card::from_str(card).unwrap();

        for card_str in cards {
            let card = Card::from_str(card_str).unwrap();
            tracker.play_card(card);
        }

        assert_eq!(tracker.relative_rank(test_card), expected);
    }

    #[test_case("D2", &[])]
    #[test_case("S2", &["S3"])]
    #[test_case("D2", &["C3"])]
    #[test_case("S3", &["D3", "S4", "S5", "S6", "D7", "D9", "C8"])]
    #[test_case("D2", &["D3", "D4", "D5", "D6", "D7", "D9", "C8"])]
    fn absolute_card(card: &str, cards: &[&str]) {
        let mut tracker = PlayedCardsTracker::empty();

        let test_card = Card::from_str(card).unwrap();

        for card_str in cards {
            let card = Card::from_str(card_str).unwrap();
            tracker.play_card(card);
        }

        let relative_rank = tracker.relative_rank(test_card);
        let suit = test_card.suit;

        assert_eq!(tracker.absolute_card(relative_rank, suit), test_card);
    }
}
