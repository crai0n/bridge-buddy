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
    fn from_string(input: &str, expected: ContractState) {
        let contract = ContractState::from_str(input).unwrap();
        assert_eq!(contract, expected);
    }
    #[test_case(Doubled, "X"; "Doubled_X")]
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
