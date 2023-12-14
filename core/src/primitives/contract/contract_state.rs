use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum ContractState {
    #[strum(serialize = "p")]
    #[strum(serialize = "P")]
    #[strum(to_string = "")]
    Passed,
    #[strum(serialize = "x")]
    #[strum(to_string = "X")]
    Doubled,
    #[strum(serialize = "xx")]
    #[strum(serialize = "xX")]
    #[strum(serialize = "Xx")]
    #[strum(to_string = "XX")]
    Redoubled,
}

#[cfg(test)]
mod test {
    use super::ContractState;
    use super::ContractState::*;
    use std::cmp::Ordering;
    use std::cmp::Ordering::*;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("p", Passed; "Passed_p")]
    #[test_case("P", Passed; "Passed_capital_p")]
    #[test_case("", Passed; "Passed_nothing")]
    #[test_case("X", Doubled; "Doubled_capital_X")]
    #[test_case("x", Doubled; "Doubled x")]
    #[test_case("xx", Redoubled; "Redoubled")]
    #[test_case("xX", Redoubled; "Redoubled1")]
    #[test_case("Xx", Redoubled; "Redoubled2")]
    #[test_case("XX", Redoubled; "Redoubled3")]
    fn from_string(input: &str, expected: ContractState) {
        let contract = ContractState::from_str(input).unwrap();
        assert_eq!(contract, expected);
    }
    #[test_case(Passed, ""; "Passed_nothing")]
    #[test_case(Doubled, "X"; "Doubled_X")]
    #[test_case(Redoubled, "XX"; "Redoubled XX")]
    fn serialize(contract_state: ContractState, expected: &str) {
        let contract_str = format!("{}", contract_state);
        assert_eq!(&contract_str, expected);
    }

    #[test_case(Doubled, Passed, Greater; "Doubled is higher than passed")]
    #[test_case(Redoubled, Passed, Greater; "Redoubled is higher than passed")]
    #[test_case(Redoubled, Doubled, Greater; "Redoubled is higher than doubled")]
    #[test_case(Redoubled, Redoubled, Equal; "Redoubled is equal to redoubled")]
    fn ordering(one: ContractState, other: ContractState, expected: Ordering) {
        let ord = one.cmp(&other);
        assert_eq!(ord, expected);
    }
}
