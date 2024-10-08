use crate::error::BBError;
use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Level {
    #[strum(to_string = "1")]
    One = 1,
    #[strum(to_string = "2")]
    Two = 2,
    #[strum(to_string = "3")]
    Three = 3,
    #[strum(to_string = "4")]
    Four = 4,
    #[strum(to_string = "5")]
    Five = 5,
    #[strum(to_string = "6")]
    Six = 6,
    #[strum(to_string = "7")]
    Seven = 7,
}

impl Level {
    pub fn expected_tricks(&self) -> usize {
        *self as usize + 6
    }
}

impl Level {
    pub const fn next(&self) -> Result<Self, BBError> {
        match self {
            Level::One => Ok(Level::Two),
            Level::Two => Ok(Level::Three),
            Level::Three => Ok(Level::Four),
            Level::Four => Ok(Level::Five),
            Level::Five => Ok(Level::Six),
            Level::Six => Ok(Level::Seven),
            Level::Seven => Err(BBError::InvalidContract),
        }
    }

    pub const fn previous(&self) -> Result<Self, BBError> {
        match self {
            Level::One => Err(BBError::InvalidContract),
            Level::Two => Ok(Level::One),
            Level::Three => Ok(Level::Two),
            Level::Four => Ok(Level::Three),
            Level::Five => Ok(Level::Four),
            Level::Six => Ok(Level::Five),
            Level::Seven => Ok(Level::Six),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Level;
    use super::Level::*;
    use std::cmp::Ordering;
    use std::cmp::Ordering::*;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("1", One; "One")]
    #[test_case("2", Two; "Two")]
    #[test_case("3", Three; "Three")]
    #[test_case("4", Four; "Four")]
    #[test_case("5", Five; "Five")]
    #[test_case("6", Six; "Six")]
    #[test_case("7", Seven; "Seven")]
    fn from_string(input: &str, expected: Level) {
        let contract = Level::from_str(input).unwrap();
        assert_eq!(contract, expected);
    }

    #[test_case(One, "1"; "One_1")]
    #[test_case(Two, "2"; "Two_2")]
    #[test_case(Three, "3"; "Three_3")]
    #[test_case(Four, "4"; "Four_4")]
    #[test_case(Five, "5"; "Five_5")]
    #[test_case(Six, "6"; "Six_6")]
    #[test_case(Seven, "7"; "Seven_7")]
    fn serialize(level: Level, expected: &str) {
        let contract_str = format!("{}", level);
        assert_eq!(&contract_str, expected);
    }

    #[test_case(One, Two, Less; "One is less than Two")]
    #[test_case(Two, Five, Less; "Two is less than Five")]
    #[test_case(Three, Three, Equal; "Three is equal to Three")]
    #[test_case(Five, One, Greater; "Five is more than Two")]
    #[test_case(Seven, Six, Greater; "Seven is more than Six")]
    fn ordering(one: Level, other: Level, expected: Ordering) {
        let ord = one.cmp(&other);
        assert_eq!(ord, expected);
    }

    #[test_case(One, 7; "One")]
    #[test_case(Two, 8; "Two")]
    #[test_case(Three, 9; "Three")]
    #[test_case(Four, 10; "Four")]
    #[test_case(Five, 11; "Five")]
    #[test_case(Six, 12; "Six")]
    #[test_case(Seven, 13; "Seven")]
    fn expected_tricks(level: Level, expected: usize) {
        assert_eq!(level.expected_tricks(), expected);
    }
}
