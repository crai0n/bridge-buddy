use super::virtual_card::VirtualCard;
use crate::card_manager::suit_field::SuitField;
use bridge_buddy_core::primitives::card::rank::RANK_ARRAY;
use bridge_buddy_core::primitives::card::virtual_rank::{VirtualRank, VIRTUAL_RANK_ARRAY};
use bridge_buddy_core::primitives::card::Rank;
use bridge_buddy_core::primitives::Card;
use lazy_static::lazy_static;

pub struct Virtualizer {
    played: [SuitField; 4],
}

lazy_static! {
    static ref TO_VIRTUAL_GIVEN_OUT_OF_PLAY: [[Option<VirtualRank>; 13]; 8192] = {
        let mut virt_rank = [[None; 13]; 8192];
        for rank in RANK_ARRAY {
            for out_of_play in 0u16..8192 {
                virt_rank[out_of_play as usize][rank as usize] = try_virtual_from_absolute_rank(rank, out_of_play)
            }
        }
        virt_rank
    };
    static ref TO_ABSOLUTE_GIVEN_OUT_OF_PLAY: [[Option<Rank>; 13]; 8192] = {
        let mut abs_rank = [[None; 13]; 8192];
        for virt_rank in VIRTUAL_RANK_ARRAY {
            for out_of_play in 0u16..8192 {
                abs_rank[out_of_play as usize][virt_rank as usize] =
                    try_absolute_from_virtual_rank(virt_rank, out_of_play)
            }
        }
        abs_rank
    };
}

fn try_absolute_from_virtual_rank(virtual_rank: VirtualRank, played: u16) -> Option<Rank> {
    let rel_index = virtual_rank as u32;
    let mut index = 0;

    while index <= rel_index {
        if played & (1 << index) == 0 {
            let shifted = played >> index;
            let pop_count = shifted.count_ones();

            if rel_index == index + pop_count {
                return Some(Rank::try_from(index).unwrap());
            }
        }
        index += 1;
    }
    None
}

fn try_virtual_from_absolute_rank(rank: Rank, played: u16) -> Option<VirtualRank> {
    let relative = 1u16 << rank as usize;

    if relative & played != 0 {
        return None;
    }

    let index = rank as u16;

    let shifted = played >> index;
    let pop_count = shifted.count_ones() as u16;

    Some(VirtualRank::from(index + pop_count))
}

impl Virtualizer {
    pub fn new(played: [SuitField; 4]) -> Self {
        Self { played }
    }

    pub fn virtual_to_absolute(&self, virtual_card: VirtualCard) -> Option<Card> {
        let suit = virtual_card.suit;
        let out_of_play = self.played[suit as usize];
        let absolute_rank = TO_ABSOLUTE_GIVEN_OUT_OF_PLAY[out_of_play.0 as usize][virtual_card.rank as usize];
        absolute_rank.map(|rank| Card { rank, suit })
    }

    pub fn absolute_to_virtual(&self, card: Card) -> Option<VirtualCard> {
        let suit = card.suit;
        let out_of_play = self.played[suit as usize];
        let virtual_rank = TO_VIRTUAL_GIVEN_OUT_OF_PLAY[out_of_play.0 as usize][card.rank as usize];
        virtual_rank.map(|rank| VirtualCard { rank, suit })
    }
}

#[cfg(test)]
mod test {
    use crate::card_manager::suit_field::SuitField;
    use crate::state::virtual_card::VirtualCard;
    use crate::state::virtualizer::Virtualizer;
    use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
    use bridge_buddy_core::primitives::card::Rank;
    use bridge_buddy_core::primitives::{Card, Suit};
    use test_case::test_case;

    #[test_case(Rank::Two, 0b0000_0011_0000_1000, Some(VirtualRank::Five))]
    #[test_case(Rank::Two, 0b0000_0011_0100_1000, Some(VirtualRank::Six))]
    #[test_case(Rank::Two, 0b0000_0011_0100_1001, None)]
    fn virtual_given_played(rank: Rank, played: u16, expected: Option<VirtualRank>) {
        let played = SuitField::from_u16(played);
        let array = [played; 4];
        let virtualizer = Virtualizer::new(array);
        let suit = Suit::Clubs;
        let expected = expected.map(|rank| VirtualCard { rank, suit });
        let relative = virtualizer.absolute_to_virtual(Card { suit, rank });
        assert_eq!(relative, expected)
    }

    #[test_case(VirtualRank::Five, 0b0000_0011_0000_1000, Some(Rank::Two))]
    #[test_case(VirtualRank::Six, 0b0000_0011_0100_1000, Some(Rank::Two))]
    #[test_case(VirtualRank::Jack, 0b0000_0011_0100_1001, Some(Rank::Nine))]
    fn absolute_given_played(rank: VirtualRank, played: u16, expected: Option<Rank>) {
        let played = SuitField::from_u16(played);
        let array = [played; 4];
        let virtualizer = Virtualizer::new(array);
        let suit = Suit::Clubs;
        let expected = expected.map(|rank| Card { rank, suit });
        let absolute = virtualizer.virtual_to_absolute(VirtualCard { suit, rank });
        assert_eq!(absolute, expected)
    }
}
