use crate::error::BBError;
use std::cmp::Ordering;
use strum::Display;

pub const N_VIRTUAL_RANKS: usize = 13;

pub const VIRTUAL_RANK_ARRAY: [VirtualRank; N_VIRTUAL_RANKS] = [
    VirtualRank::Two,
    VirtualRank::Three,
    VirtualRank::Four,
    VirtualRank::Five,
    VirtualRank::Six,
    VirtualRank::Seven,
    VirtualRank::Eight,
    VirtualRank::Nine,
    VirtualRank::Ten,
    VirtualRank::Jack,
    VirtualRank::Queen,
    VirtualRank::King,
    VirtualRank::Ace,
];

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
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
}

impl From<u16> for VirtualRank {
    fn from(value: u16) -> VirtualRank {
        match value {
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
    pub fn from_char(char: char) -> Result<VirtualRank, BBError> {
        match char {
            'A' => Ok(VirtualRank::Ace),
            'a' => Ok(VirtualRank::Ace),
            'K' => Ok(VirtualRank::King),
            'k' => Ok(VirtualRank::King),
            'Q' => Ok(VirtualRank::Queen),
            'q' => Ok(VirtualRank::Queen),
            'J' => Ok(VirtualRank::Jack),
            'j' => Ok(VirtualRank::Jack),
            'T' => Ok(VirtualRank::Ten),
            't' => Ok(VirtualRank::Ten),
            '9' => Ok(VirtualRank::Nine),
            '8' => Ok(VirtualRank::Eight),
            '7' => Ok(VirtualRank::Seven),
            '6' => Ok(VirtualRank::Six),
            '5' => Ok(VirtualRank::Five),
            '4' => Ok(VirtualRank::Four),
            '3' => Ok(VirtualRank::Three),
            '2' => Ok(VirtualRank::Two),
            c => Err(BBError::UnknownRank(c.into())),
        }
    }
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

impl std::str::FromStr for VirtualRank {
    type Err = BBError;

    fn from_str(string: &str) -> Result<VirtualRank, BBError> {
        let mut chars = string.trim().chars();
        let char = chars.next().ok_or(BBError::UnknownRank(string.into()))?;
        if chars.next().is_some() {
            return Err(BBError::UnknownRank(string.into()));
        }
        VirtualRank::from_char(char)
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

    #[test_case('a', Ace; "A is Ace")]
    #[test_case('k', King; "k is King")]
    #[test_case('q', Queen; "q is Queen")]
    #[test_case('J', Jack; "J is Jack")]
    #[test_case('t', Ten; "t is Ten")]
    #[test_case('9', Nine; "9 is Nine")]
    #[test_case('7', Seven; "7 is Seven")]
    #[test_case('3', Three; "3 is Three")]
    fn parsing_char(input: char, expected: VirtualRank) {
        assert_eq!(VirtualRank::from_char(input).unwrap(), expected);
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
