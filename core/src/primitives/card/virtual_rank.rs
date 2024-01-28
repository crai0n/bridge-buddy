use std::cmp::Ordering;
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum VirtualRank {
    #[strum(serialize = "2")]
    Two = 0,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    #[strum(serialize = "T")]
    Ten,
    #[strum(serialize = "J")]
    Jack,
    #[strum(serialize = "Q")]
    Queen,
    #[strum(serialize = "K")]
    King,
    #[strum(serialize = "A")]
    Ace,
    #[strum(serialize = "OUT")]
    OutOfPlay = 15,
}

impl From<u16> for VirtualRank {
    fn from(value: u16) -> VirtualRank {
        match value {
            15 => VirtualRank::OutOfPlay,
            0 => VirtualRank::Two,
            1 => VirtualRank::Three,
            2 => VirtualRank::Four,
            3 => VirtualRank::Five,
            4 => VirtualRank::Six,
            5 => VirtualRank::Seven,
            6 => VirtualRank::Eight,
            7 => VirtualRank::Nine,
            8 => VirtualRank::Ten,
            9 => VirtualRank::Jack,
            10 => VirtualRank::Queen,
            11 => VirtualRank::King,
            12 => VirtualRank::Ace,
            _ => panic!("Not a valid relative rank!"),
        }
    }
}

impl VirtualRank {
    #[allow(dead_code)]
    pub fn touches(&self, other: &VirtualRank) -> bool {
        // println!("testing {} and {}", self, other);
        match self.cmp(other) {
            Ordering::Less => *other as usize - *self as usize == 1,
            Ordering::Greater => *self as usize - *other as usize == 1,
            Ordering::Equal => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VirtualRank;
    use super::VirtualRank::*;
    use test_case::test_case;

    #[test_case(King, Ace; "King and Ace")]
    #[test_case(Ten, Queen; "Ten and Queen")]
    #[test_case(Eight, Jack; "Eight and Jack")]
    #[test_case(Two, Ten; "Two and Ten")]
    fn relative_ranking(lower: VirtualRank, higher: VirtualRank) {
        assert!(lower < higher);
    }

    #[test_case(Ace, "A")]
    #[test_case(King, "K")]
    #[test_case(Queen, "Q")]
    #[test_case(Jack, "J")]
    #[test_case(Ten, "T")]
    #[test_case(Nine, "9")]
    #[test_case(Eight, "8")]
    #[test_case(Seven, "7")]
    #[test_case(Six, "6")]
    #[test_case(Five, "5")]
    #[test_case(Four, "4")]
    #[test_case(Three, "3")]
    #[test_case(Two, "2")]
    fn display(rank: VirtualRank, expected: &str) {
        assert_eq!(format!("{}", rank), expected);
    }

    #[test]
    fn copy() {
        let mut x = King;
        let y = x;
        x = Queen;
        assert_eq!(x, Queen);
        assert_eq!(y, King);
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Jack), "Jack")
    }
}
