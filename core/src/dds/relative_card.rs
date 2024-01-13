use crate::dds::relative_rank::RelativeRank;
use crate::primitives::Suit;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelativeCard {
    pub suit: Suit,
    pub rank: RelativeRank,
}

impl std::fmt::Display for RelativeCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

impl RelativeCard {
    pub fn touches(&self, other: &RelativeCard) -> bool {
        self.suit == other.suit && self.rank.touches(&other.rank)
    }
}
