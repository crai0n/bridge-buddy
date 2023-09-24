use crate::error::BBError;
use crate::primitives::Suit;
use crate::util;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum ContractDenomination {
    Trump(Suit),
    NoTrump,
}

impl std::fmt::Display for ContractDenomination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractDenomination::Trump(s) => {
                write!(f, "{}", s)
            }
            ContractDenomination::NoTrump => {
                write!(f, "NT")
            }
        }
    }
}

impl std::str::FromStr for ContractDenomination {
    type Err = BBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() == 1 {
            let char = util::single_char_from_str(s)?;
            match Suit::from_char(char) {
                Ok(s) => Ok(ContractDenomination::Trump(s)),
                Err(e) => Err(e),
            }
        } else {
            match s {
                "SA" | "NT" => Ok(ContractDenomination::NoTrump),
                _ => Err(BBError::ParseError(s.into(), "unknown contract")),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::ContractDenomination;
    use super::ContractDenomination::*;
    use crate::primitives::Suit::*;
    use std::cmp::Ordering;
    use std::cmp::Ordering::*;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("NT", NoTrump; "No Trump")]
    #[test_case("S", Trump(Spades); "Spades")]
    #[test_case("d", Trump(Diamonds); "Diamonds")]
    #[test_case("♥", Trump(Hearts); "Hearts")]
    fn from_str(str: &str, den: ContractDenomination) {
        assert_eq!(ContractDenomination::from_str(str).unwrap(), den)
    }

    #[test_case(NoTrump, "NT"; "No Trump")]
    fn serialize(contract_den: ContractDenomination, expected: &str) {
        let contract_str = format!("{}", contract_den);
        assert_eq!(&contract_str, expected);
    }

    #[test_case(NoTrump, Trump(Spades), Greater; "No Trump is higher than Spades")]
    #[test_case(Trump(Spades), Trump(Hearts), Greater; "Spades is higher than Hearts")]
    #[test_case(Trump(Hearts), Trump(Diamonds), Greater; "Hearts is higher than Diamonds")]
    #[test_case(Trump(Diamonds), Trump(Clubs), Greater; "Diamonds is higher than Clubs")]
    #[test_case(Trump(Hearts), Trump(Hearts), Equal; "Hearts is equal to Hearts")]
    fn ordering(one: ContractDenomination, other: ContractDenomination, expected: Ordering) {
        let ord = one.cmp(&other);
        assert_eq!(ord, expected);
    }
}
