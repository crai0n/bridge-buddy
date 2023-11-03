#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Copy, Clone)]
pub enum TurnRank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
}

impl From<usize> for TurnRank {
    fn from(value: usize) -> Self {
        match value % 4 {
            0 => TurnRank::First,
            1 => TurnRank::Second,
            2 => TurnRank::Third,
            3 => TurnRank::Fourth,
            _ => unreachable!(),
        }
    }
}

impl TurnRank {
    pub const fn same_axis(&self, other: &TurnRank) -> bool {
        (*self as usize + *other as usize) % 2 == 0
    }
}
