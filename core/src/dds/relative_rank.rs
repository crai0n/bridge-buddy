use strum::EnumIter;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, EnumIter)]
pub enum RelativeRank {
    Thirteenth = 0,
    Twelveth,
    Eleventh,
    Tenth,
    Ninth,
    Eigth,
    Seventh,
    Sixth,
    Fifth,
    Fourth,
    Third,
    Second,
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
