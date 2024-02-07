use crate::card_manager::suit_field::SuitField;
use crate::primitives::VirtualCard;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::card::Rank;
use bridge_buddy_core::primitives::{Card, Suit};

include!(concat!(env!("OUT_DIR"), "/relative_map.rs"));
include!(concat!(env!("OUT_DIR"), "/absolute_map.rs"));

pub struct Virtualizer {
    played: [SuitField; 4],
}

impl Virtualizer {
    pub fn new(played: [SuitField; 4]) -> Self {
        Self { played }
    }
    pub fn virtual_to_absolute(&self, virtual_card: VirtualCard) -> Option<Card> {
        let suit = virtual_card.suit;
        let absolute_rank = self.try_find_absolute(suit, virtual_card.rank);
        absolute_rank.map(|rank| Card { rank, suit })
    }

    pub fn absolute_to_virtual(&self, card: Card) -> Option<VirtualCard> {
        let suit = card.suit;
        let virtual_rank = self.try_find_virtual(suit, card.rank);
        virtual_rank.map(|rank| VirtualCard { rank, suit })
    }

    fn try_find_absolute(&self, suit: Suit, relative: VirtualRank) -> Option<Rank> {
        let field = 1u32 << relative as usize;
        let key = field << 16 | self.played[suit as usize].0 as u32;
        *ABSOLUTE.get(&key).unwrap()
    }

    fn try_find_virtual(&self, suit: Suit, absolute: Rank) -> Option<VirtualRank> {
        let field = 1u32 << absolute as usize;
        let key = field << 16 | self.played[suit as usize].0 as u32;
        *VIRTUAL.get(&key).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::card_manager::suit_field::SuitField;
    use crate::state::virtualizer::Virtualizer;
    use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
    use bridge_buddy_core::primitives::card::Rank;
    use bridge_buddy_core::primitives::Suit;
    use test_case::test_case;

    #[test_case(Rank::Two, 0b0000_0011_0000_1000, Some(VirtualRank::Five))]
    #[test_case(Rank::Two, 0b0000_0011_0100_1000, Some(VirtualRank::Six))]
    #[test_case(Rank::Two, 0b0000_0011_0100_1001, None)]
    fn virtual_given_played(rank: Rank, played: u16, expected: Option<VirtualRank>) {
        let played = SuitField::from_u16(played);
        let array = [played; 4];
        let virtualizer = Virtualizer::new(array);
        let relative = virtualizer.try_find_virtual(Suit::Clubs, rank);
        assert_eq!(relative, expected)
    }

    #[test_case(VirtualRank::Five, 0b0000_0011_0000_1000, Some(Rank::Two))]
    #[test_case(VirtualRank::Six, 0b0000_0011_0100_1000, Some(Rank::Two))]
    #[test_case(VirtualRank::Jack, 0b0000_0011_0100_1001, Some(Rank::Nine))]
    fn absolute_given_played(rank: VirtualRank, played: u16, expected: Option<Rank>) {
        let played = SuitField::from_u16(played);
        let array = [played; 4];
        let virtualizer = Virtualizer::new(array);
        let absolute = virtualizer.try_find_absolute(Suit::Clubs, rank);
        assert_eq!(absolute, expected)
    }
}
