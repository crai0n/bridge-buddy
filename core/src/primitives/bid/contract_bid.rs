use crate::error::BBError;
use crate::primitives::contract::Strain;
use crate::primitives::contract::*;
use crate::primitives::Suit;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct ContractBid {
    pub level: Level,
    pub strain: Strain,
}

impl std::str::FromStr for ContractBid {
    type Err = BBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(BBError::UnknownBid(s.into()));
        }

        let level = match Level::from_str(&s[..1]) {
            Ok(l) => l,
            Err(_) => return Err(BBError::UnknownBid(s.into())),
        };

        let strain_str = &s[1..];

        let strain = match Strain::from_str(strain_str) {
            Ok(d) => d,
            Err(_) => return Err(BBError::UnknownBid(s.into())),
        };

        Ok(ContractBid { level, strain })
    }
}

impl std::fmt::Display for ContractBid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.level, self.strain)?;
        Ok(())
    }
}

impl ContractBid {
    pub fn next(&self) -> Result<Self, BBError> {
        match self.strain {
            Strain::NoTrump => Ok(ContractBid {
                level: match self.level.next() {
                    Ok(level) => level,
                    Err(_) => Err(BBError::UnknownBid("-C".into()))?,
                },
                strain: Strain::Trump(Suit::Clubs),
            }),
            Strain::Trump(suit) => Ok(ContractBid {
                level: self.level,
                strain: match suit {
                    Suit::Spades => Strain::NoTrump,
                    s => Strain::Trump(s.next()),
                },
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ContractBid;
    use crate::primitives::contract::Level::*;
    use crate::primitives::contract::Strain::*;
    use crate::primitives::contract::*;
    use crate::primitives::Suit::*;
    use std::cmp::Ordering;
    use std::cmp::Ordering::*;

    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("1NT", One, NoTrump; "No Trump")]
    #[test_case("2S", Two, Trump(Spades); "Spades")]
    #[test_case("3d", Three, Trump(Diamonds); "Diamonds")]
    #[test_case("4♥", Four, Trump(Hearts); "Hearts")]
    fn from_str(str: &str, level: Level, strain: Strain) {
        assert_eq!(ContractBid::from_str(str).unwrap(), ContractBid { level, strain })
    }

    #[test_case(""; "Empty")]
    #[test_case("j"; "single letter")]
    #[test_case("j4"; "letter and number")]
    #[test_case("6k"; "number and letter")]
    fn fails(input: &str) {
        let c1 = Contract::from_str(input);
        assert!(c1.is_err())
    }

    #[test_case(One, NoTrump, "1NT"; "No Trump")]
    #[test_case(Two, Trump(Spades), "2♠"; "Spades")]
    #[test_case(Three, Trump(Diamonds), "3♦"; "Diamonds")]
    #[test_case(Four, Trump(Hearts), "4♥"; "Hearts")]
    fn serialize(level: Level, strain: Strain, expected: &str) {
        let bid = ContractBid { level, strain };
        assert_eq!(format!("{}", bid), expected);
    }

    #[test_case("1S", "1NT", Less; "1S is less than 1NT")]
    #[test_case("1S", "1H", Greater; "1S is more than 1H")]
    #[test_case("2D", "1H", Greater; "2D is more than 1H")]
    #[test_case("7D", "4H", Greater; "7D is more than 4H")]
    #[test_case("7NT", "7C", Greater; "7NT is more than 7C")]
    #[test_case("2D", "2D", Equal; "2D is equal to itself")]
    fn ordering(one: &str, other: &str, expected: Ordering) {
        let c1 = ContractBid::from_str(one).unwrap();
        let c2 = ContractBid::from_str(other).unwrap();
        assert_eq!(c1.cmp(&c2), expected)
    }

    #[test_case("1S", "1NT")]
    #[test_case("2H", "2S")]
    #[test_case("3D", "3H")]
    #[test_case("5C", "5D")]
    #[test_case("5NT", "6C")]
    #[test_case("7NT", "-C")]
    fn next(current: &str, expected: &str) {
        let current = ContractBid::from_str(current).unwrap();
        let expected = ContractBid::from_str(expected);
        assert_eq!(current.next(), expected)
    }
}
