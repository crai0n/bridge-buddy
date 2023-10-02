use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ContractLevel {
    #[strum(to_string = "1")]
    One,
    #[strum(to_string = "2")]
    Two,
    #[strum(to_string = "3")]
    Three,
    #[strum(to_string = "4")]
    Four,
    #[strum(to_string = "5")]
    Five,
    #[strum(to_string = "6")]
    Six,
    #[strum(to_string = "7")]
    Seven,
}

#[cfg(test)]
mod test {
    use super::ContractLevel;
    use super::ContractLevel::*;
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
    fn from_string(input: &str, expected: ContractLevel) {
        let contract = ContractLevel::from_str(input).unwrap();
        assert_eq!(contract, expected);
    }

    #[test_case(One, "1"; "One_1")]
    #[test_case(Two, "2"; "Two_2")]
    #[test_case(Three, "3"; "Three_3")]
    #[test_case(Four, "4"; "Four_4")]
    #[test_case(Five, "5"; "Five_5")]
    #[test_case(Six, "6"; "Six_6")]
    #[test_case(Seven, "7"; "Seven_7")]
    fn serialize(contract_level: ContractLevel, expected: &str) {
        let contract_str = format!("{}", contract_level);
        assert_eq!(&contract_str, expected);
    }

    #[test_case(One, Two, Less; "One is less than Two")]
    #[test_case(Two, Five, Less; "Two is less than Five")]
    #[test_case(Three, Three, Equal; "Three is equal to Three")]
    #[test_case(Five, One, Greater; "Five is more than Two")]
    #[test_case(Seven, Six, Greater; "Seven is more than Six")]
    fn ordering(one: ContractLevel, other: ContractLevel, expected: Ordering) {
        let ord = one.cmp(&other);
        assert_eq!(ord, expected);
    }
}
