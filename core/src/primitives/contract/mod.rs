pub mod contract_denomination;
mod contract_level;
mod contract_state;

use crate::error::BBError;
use std::fmt::Display;
use std::str::FromStr;

pub use contract_denomination::ContractDenomination;
pub use contract_level::ContractLevel;
pub use contract_state::ContractState;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Contract {
    pub level: ContractLevel,
    pub denomination: ContractDenomination,
    pub state: ContractState,
}

impl Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.level, self.denomination, self.state)?;
        Ok(())
    }
}

impl FromStr for Contract {
    type Err = BBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();

        if len < 2 {
            return Err(BBError::UnknownContract(s.into()));
        }

        let level = match ContractLevel::from_str(&s[..1]) {
            Ok(l) => l,
            Err(_) => return Err(BBError::UnknownContract(s.into())),
        };

        let count_doubles = s.chars().rev().take_while(|x| *x == 'x' || *x == 'X').count();
        let state = match count_doubles {
            0 => ContractState::Passed,
            1 => ContractState::Doubled,
            2 => ContractState::Redoubled,
            _ => return Err(BBError::UnknownContract(s.into())),
        };

        // rest of the string must be the denomination
        let den_str = &s[1..len - count_doubles];
        let denomination = match ContractDenomination::from_str(den_str) {
            Ok(d) => d,
            Err(_) => return Err(BBError::UnknownContract(s.into())),
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
    use super::ContractDenomination::*;
    use super::ContractLevel::*;
    use super::ContractState::*;
    use super::{Contract, ContractDenomination, ContractLevel, ContractState};
    use crate::primitives::Suit::*;
    use std::cmp::Ordering::*;
    use std::{cmp::Ordering, str::FromStr};
    use test_case::test_case;

    #[test_case("1NTx", One, NoTrump, Doubled; "No Trump")]
    #[test_case("2SXx", Two, Trump(Spades), Redoubled; "Spades")]
    #[test_case("3d", Three, Trump(Diamonds), Passed; "Diamonds")]
    #[test_case("4♥X", Four, Trump(Hearts), Doubled; "Hearts")]
    fn from_str(str: &str, level: ContractLevel, denomination: ContractDenomination, state: ContractState) {
        assert_eq!(
            Contract::from_str(str).unwrap(),
            Contract {
                level,
                denomination,
                state
            }
        )
    }

    #[test_case(One, Trump(Spades), Passed, "1♠"; "1P")]
    #[test_case(Two, Trump(Hearts), Doubled, "2♥X"; "2cx")]
    #[test_case(Three, NoTrump, Redoubled, "3NTXX"; "3ntxx")]
    fn serialize(level: ContractLevel, denomination: ContractDenomination, state: ContractState, exp: &str) {
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

    #[test_case("1SX", "1NT", Less; "Even doubled 1S is less than 1NT")]
    #[test_case("1S", "1H", Greater; "1S is more than 1H")]
    #[test_case("2D", "1H", Greater; "2D is more than 1H")]
    #[test_case("2D", "2DX", Less; "Doubling is worth more")]
    fn ordering(one: &str, other: &str, expected: Ordering) {
        let c1 = Contract::from_str(one).unwrap();
        let c2 = Contract::from_str(other).unwrap();
        assert_eq!(c1.cmp(&c2), expected)
    }
}
