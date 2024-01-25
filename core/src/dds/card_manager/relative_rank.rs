use std::cmp::Ordering;
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum RelativeRank {
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

impl From<u16> for RelativeRank {
    fn from(value: u16) -> RelativeRank {
        match value {
            15 => RelativeRank::OutOfPlay,
            0 => RelativeRank::Two,
            1 => RelativeRank::Three,
            2 => RelativeRank::Four,
            3 => RelativeRank::Five,
            4 => RelativeRank::Six,
            5 => RelativeRank::Seven,
            6 => RelativeRank::Eight,
            7 => RelativeRank::Nine,
            8 => RelativeRank::Ten,
            9 => RelativeRank::Jack,
            10 => RelativeRank::Queen,
            11 => RelativeRank::King,
            12 => RelativeRank::Ace,
            _ => panic!("Not a valid relative rank!"),
        }
    }
}

impl RelativeRank {
    #[allow(dead_code)]
    pub fn touches(&self, other: &RelativeRank) -> bool {
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
    use super::RelativeRank;
    use super::RelativeRank::*;
    use test_case::test_case;

    #[test_case(King, Ace; "King and Ace")]
    #[test_case(Ten, Queen; "Ten and Queen")]
    #[test_case(Eight, Jack; "Eight and Jack")]
    #[test_case(Two, Ten; "Two and Ten")]
    fn relative_ranking(lower: RelativeRank, higher: RelativeRank) {
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
    fn display(rank: RelativeRank, expected: &str) {
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
