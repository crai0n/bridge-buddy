use crate::error::BBError;
use crate::primitives::Suit;

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
            let char = s.chars().next().unwrap();
            match Suit::from_char(char) {
                Ok(s) => Ok(ContractDenomination::Trump(s)),
                Err(e) => Err(e),
            }
        } else {
            match s.to_uppercase().as_ref() {
                "SA" | "NT" => Ok(ContractDenomination::NoTrump),
                _ => Err(BBError::UnknownContractDenomination(s.into())),
            }
        }
    }
}

impl ContractDenomination {
    pub const fn next(&self) -> Self {
        match self {
            ContractDenomination::Trump(suit) => match suit {
                Suit::Spades => ContractDenomination::NoTrump,
                s => ContractDenomination::Trump(s.next()),
            },
            ContractDenomination::NoTrump => ContractDenomination::Trump(Suit::Clubs),
        }
    }

    pub const fn previous(&self) -> Self {
        match self {
            ContractDenomination::Trump(suit) => match suit {
                Suit::Clubs => ContractDenomination::NoTrump,
                s => ContractDenomination::Trump(s.previous()),
            },
            ContractDenomination::NoTrump => ContractDenomination::Trump(Suit::Spades),
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

    #[test_case("NP")]
    #[test_case("PT")]
    #[test_case("SS")]
    #[test_case("K")]
    #[test_case("i")]
    fn from_str_fails(str: &str) {
        assert!(ContractDenomination::from_str(str).is_err());
    }

    #[test_case(NoTrump, Trump(Clubs))]
    #[test_case(Trump(Clubs), Trump(Diamonds))]
    #[test_case(Trump(Spades), NoTrump)]
    fn next(input: ContractDenomination, expected: ContractDenomination) {
        assert_eq!(input.next(), expected);
    }

    #[test_case(NoTrump, Trump(Clubs))]
    #[test_case(Trump(Clubs), Trump(Diamonds))]
    #[test_case(Trump(Spades), NoTrump)]
    fn previous(expected: ContractDenomination, input: ContractDenomination) {
        assert_eq!(input.previous(), expected);
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

    #[test]
    fn is_clone() {
        let mut one = NoTrump;
        let two = one;

        assert_eq!(one, two);

        one = Trump(Clubs);

        assert_eq!(one, Trump(Clubs));
        assert_eq!(two, NoTrump);
    }
}
