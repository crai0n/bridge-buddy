use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::Suit;

#[derive(Copy, Clone, Eq, PartialOrd, PartialEq, Ord, Debug)]
pub struct VirtualCard {
    pub suit: Suit,
    pub rank: VirtualRank,
}

impl VirtualCard {
    pub fn touches(&self, other: &Self) -> bool {
        if self.suit == other.suit {
            self.rank.touches(&other.rank)
        } else {
            false
        }
    }
}
