mod level;
mod state;
pub mod strain;

use crate::error::BBError;
use std::fmt::Display;
use std::str::FromStr;

use crate::primitives::deal::Seat;
use crate::primitives::Suit;
pub use level::Level;
pub use state::State;
pub use strain::Strain;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Contract {
    pub level: Level,
    pub strain: Strain,
    pub state: State,
    pub declarer: Seat,
}

impl Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{} by {}", self.level, self.strain, self.state, self.declarer)?;
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

        let declarer = match Seat::from_str(&s[..1]) {
            Ok(d) => d,
            Err(_) => return Err(BBError::UnknownContract(s.into())),
        };

        let level = match Level::from_str(&s[1..2]) {
            Ok(l) => l,
            Err(_) => return Err(BBError::UnknownContract(s.into())),
        };

        let count_doubles = s.chars().rev().take_while(|x| *x == 'x' || *x == 'X').count();
        let state = match count_doubles {
            0 => State::Passed,
            1 => State::Doubled,
            2 => State::Redoubled,
            _ => return Err(BBError::UnknownContract(s.into())),
        };

        // rest of the string must be the strain
        let strain_str = &s[2..len - count_doubles];
        let strain = match Strain::from_str(strain_str) {
            Ok(d) => d,
            Err(_) => return Err(BBError::UnknownContract(s.into())),
        };

        Ok(Contract {
            level,
            strain,
            state,
            declarer,
        })
    }
}

impl Contract {
    pub fn expected_tricks(&self) -> usize {
        self.level.expected_tricks()
    }

    pub fn trump_suit(&self) -> Option<Suit> {
        match self.strain {
            Strain::NoTrump => None,
            Strain::Trump(s) => Some(s),
        }
    }
}
#[cfg(test)]
mod test {
    use super::Level::*;
    use super::State::*;
    use super::Strain::*;
    use super::{Contract, Level, State, Strain};
    use crate::primitives::deal::Seat;
    use crate::primitives::deal::Seat::*;
    use crate::primitives::Suit;
    use crate::primitives::Suit::*;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("N1NTx", North, One, NoTrump, Doubled; "No Trump")]
    #[test_case("S2SXx", South, Two, Trump(Spades), Redoubled; "Spades")]
    #[test_case("E3d", East, Three, Trump(Diamonds), Passed; "Diamonds")]
    #[test_case("W4♥X", West, Four, Trump(Hearts), Doubled; "Hearts")]
    fn from_str(str: &str, declarer: Seat, level: Level, strain: Strain, state: State) {
        assert_eq!(
            Contract::from_str(str).unwrap(),
            Contract {
                declarer,
                level,
                strain,
                state
            }
        )
    }

    #[test_case(North, One, Trump(Spades), Passed, "1♠ by N"; "1P")]
    #[test_case(South, Two, Trump(Hearts), Doubled, "2♥X by S"; "2cx")]
    #[test_case(West, Three, NoTrump, Redoubled, "3NTXX by W"; "3ntxx")]
    fn serialize(declarer: Seat, level: Level, strain: Strain, state: State, exp: &str) {
        assert_eq!(
            format!(
                "{}",
                Contract {
                    level,
                    strain,
                    state,
                    declarer
                }
            ),
            exp
        );
    }

    #[test_case("N1S", 7; "One")]
    #[test_case("S2H", 8; "Two")]
    #[test_case("W3D", 9; "Three")]
    #[test_case("E4C", 10; "Four")]
    #[test_case("S5NT", 11; "Five")]
    #[test_case("W6H", 12; "Six")]
    #[test_case("E7C", 13; "Seven")]
    fn expected_tricks(contract_string: &str, expected: usize) {
        let contract = Contract::from_str(contract_string).unwrap();
        assert_eq!(contract.expected_tricks(), expected);
    }

    #[test_case("W1S", Some(Spades); "One")]
    #[test_case("E2H", Some(Hearts); "Two")]
    #[test_case("S3D", Some(Diamonds); "Three")]
    #[test_case("N4C", Some(Clubs); "Four")]
    #[test_case("E5NT", None; "Five")]
    #[test_case("W6H", Some(Hearts); "Six")]
    #[test_case("S7C", Some(Clubs); "Seven")]
    fn trump_suit(contract_string: &str, expected: Option<Suit>) {
        let contract = Contract::from_str(contract_string).unwrap();
        assert_eq!(contract.trump_suit(), expected);
    }
}
