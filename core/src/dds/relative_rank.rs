use std::cmp::Ordering;
use strum::{Display, EnumIter};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, EnumIter, Display)]
pub enum RelativeRank {
    #[strum(serialize = "13th")]
    Thirteenth = 0,
    #[strum(serialize = "12th")]
    Twelveth,
    #[strum(serialize = "11th")]
    Eleventh,
    #[strum(serialize = "10th")]
    Tenth,
    #[strum(serialize = "9th")]
    Ninth,
    #[strum(serialize = "8th")]
    Eigth,
    #[strum(serialize = "7th")]
    Seventh,
    #[strum(serialize = "6th")]
    Sixth,
    #[strum(serialize = "5th")]
    Fifth,
    #[strum(serialize = "4th")]
    Fourth,
    #[strum(serialize = "3rd")]
    Third,
    #[strum(serialize = "2nd")]
    Second,
    #[strum(serialize = "Highest")]
    Highest,
}

impl From<u16> for RelativeRank {
    fn from(value: u16) -> Self {
        match value {
            0 => RelativeRank::Thirteenth,
            1 => RelativeRank::Twelveth,
            2 => RelativeRank::Eleventh,
            3 => RelativeRank::Tenth,
            4 => RelativeRank::Ninth,
            5 => RelativeRank::Eigth,
            6 => RelativeRank::Seventh,
            7 => RelativeRank::Sixth,
            8 => RelativeRank::Fifth,
            9 => RelativeRank::Fourth,
            10 => RelativeRank::Third,
            11 => RelativeRank::Second,
            12.. => RelativeRank::Highest,
        }
    }
}

impl RelativeRank {
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
mod test {
    use crate::dds::relative_rank::RelativeRank;
    use test_case::test_case;

    #[test_case(RelativeRank::Highest, RelativeRank::Second, true)]
    #[test_case(RelativeRank::Second, RelativeRank::Highest, true)]
    #[test_case(RelativeRank::Third, RelativeRank::Fourth, true)]
    #[test_case(RelativeRank::Second, RelativeRank::Fourth, false)]
    fn touches(one: RelativeRank, other: RelativeRank, expected: bool) {
        assert_eq!(one.touches(&other), expected);
    }
}
