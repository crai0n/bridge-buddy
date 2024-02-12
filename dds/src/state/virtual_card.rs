use bridge_buddy_core::error::BBError;
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

    fn split_string(string: &str) -> Result<[char; 2], BBError> {
        let chars = string.chars().collect::<Vec<char>>();
        chars.try_into().or(Err(BBError::UnknownCard(string.into())))
    }
}

impl std::str::FromStr for VirtualCard {
    type Err = BBError;

    fn from_str(string: &str) -> Result<VirtualCard, Self::Err> {
        let [s, d] = Self::split_string(string)?;
        let suit = Suit::from_char(s)?;
        let rank = VirtualRank::from_char(d)?;
        Ok(VirtualCard { suit, rank })
    }
}
