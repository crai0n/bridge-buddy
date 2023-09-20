use crate::error::BBError;
use strum::{Display, EnumString};

use crate::primitives::Suit;
use crate::util;

#[derive(Debug, Display, EnumString, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Contract {
    pub level: ContractLevel,
    pub denomination: ContractDenomination,
    pub state: ContractState,
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.level, self.denomination, self.state)?;
        Ok(())
    }
}

impl std::str::FromStr for Contract {
    type Err = BBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();

        if len < 2 {
            return Err(BBError::ParseError(s.into(), "string too short"));
        }

        let level = match ContractLevel::from_str(&s[..1]) {
            Ok(l) => l,
            Err(_) => return Err(BBError::ParseError(s.into(), "unknown level")),
        };

        let count_doubles = s.chars().rev().take_while(|x| *x == 'x' || *x == 'X').count();

        let state = match count_doubles {
            0 => ContractState::Passed,
            1 => ContractState::Doubled,
            2 => ContractState::Redoubled,
            _ => return Err(BBError::ParseError(s.into(), "unknown contract state")),
        };

        // rest of the string must be the denomination
        let den_str = &s[1..len - count_doubles];

        let denomination = match ContractDenomination::from_str(den_str) {
            Ok(d) => d,
            Err(_) => return Err(BBError::ParseError(s.into(), "unknown contract denomination")),
        };

        Ok(Contract {
            level,
            denomination,
            state,
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Contract, ContractDenomination, ContractLevel, ContractState, Suit};
    use std::{cmp::Ordering, str::FromStr};
    use test_case::test_case;

    #[test]
    fn contract_ordering_spades_notrump() {
        let level = ContractLevel::One;
        let denomination = ContractDenomination::Trump(Suit::Spades);
        let state = ContractState::Doubled;
        let c1 = Contract {
            level,
            denomination,
            state,
        };
        let level = ContractLevel::One;
        let denomination = ContractDenomination::NoTrump;
        let state = ContractState::Passed;
        let c2 = Contract {
            level,
            denomination,
            state,
        };
        assert_eq!(c1.cmp(&c2), Ordering::Less)
    }

    #[test]
    fn contract_ordering_hearts_spades() {
        let level = ContractLevel::One;
        let denomination = ContractDenomination::Trump(Suit::Hearts);
        let state = ContractState::Doubled;
        let c1 = Contract {
            level,
            denomination,
            state,
        };
        let level = ContractLevel::One;
        let denomination = ContractDenomination::Trump(Suit::Spades);
        let state = ContractState::Passed;
        let c2 = Contract {
            level,
            denomination,
            state,
        };
        assert_eq!(c1.cmp(&c2), Ordering::Less)
    }

    #[test_case(ContractLevel::One, ContractDenomination::Trump(Suit::Spades), ContractState::Passed, "1♠"; "1P")]
    #[test_case(ContractLevel::Two, ContractDenomination::Trump(Suit::Hearts), ContractState::Doubled, "2♥X"; "2cx")]
    #[test_case(ContractLevel::Three, ContractDenomination::NoTrump, ContractState::Redoubled, "3NTXX"; "3ntxx")]
    fn contract_format(level: ContractLevel, denomination: ContractDenomination, state: ContractState, exp: &str) {
        assert_eq!(
            format!(
                "{}",
                Contract {
                    level,
                    denomination,
                    state
                }
            ),
            exp
        );
    }

    #[test_case("NT", ContractDenomination::NoTrump; "No Trump")]
    #[test_case("S", ContractDenomination::Trump(Suit::Spades); "Spades")]
    #[test_case("d", ContractDenomination::Trump(Suit::Diamonds); "Diamonds")]
    #[test_case("♥", ContractDenomination::Trump(Suit::Hearts); "Hearts")]
    fn contract_denomination_from_str(str: &str, den: ContractDenomination) {
        assert_eq!(ContractDenomination::from_str(str).unwrap(), den)
    }

    #[test_case("1NTx", ContractLevel::One, ContractDenomination::NoTrump, ContractState::Doubled; "No Trump")]
    #[test_case("2SXx", ContractLevel::Two, ContractDenomination::Trump(Suit::Spades), ContractState::Redoubled; "Spades")]
    #[test_case("3d", ContractLevel::Three, ContractDenomination::Trump(Suit::Diamonds), ContractState::Passed; "Diamonds")]
    #[test_case("4♥X",ContractLevel::Four, ContractDenomination::Trump(Suit::Hearts), ContractState::Doubled; "Hearts")]
    fn contract_from_str(str: &str, level: ContractLevel, denomination: ContractDenomination, state: ContractState) {
        assert_eq!(
            Contract::from_str(str).unwrap(),
            Contract {
                level,
                denomination,
                state
            }
        )
    }
}
