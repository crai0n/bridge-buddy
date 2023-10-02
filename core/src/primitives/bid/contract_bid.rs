use crate::error::BBError;
use crate::primitives::contract::ContractDenomination;
use crate::primitives::contract::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct ContractBid {
    pub level: ContractLevel,
    pub denomination: ContractDenomination,
}

impl std::str::FromStr for ContractBid {
    type Err = BBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(BBError::UnknownBid(s.into()));
        }

        let level = match ContractLevel::from_str(&s[..1]) {
            Ok(l) => l,
            Err(_) => return Err(BBError::UnknownBid(s.into())),
        };

        let den_str = &s[1..];

        let denomination = match ContractDenomination::from_str(den_str) {
            Ok(d) => d,
            Err(_) => return Err(BBError::UnknownBid(s.into())),
        };

        Ok(ContractBid { level, denomination })
    }
}

impl std::fmt::Display for ContractBid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.level, self.denomination)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::ContractBid;
    use crate::primitives::contract::ContractDenomination::*;
    use crate::primitives::contract::ContractLevel::*;
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
    fn from_str(str: &str, level: ContractLevel, denomination: ContractDenomination) {
        assert_eq!(ContractBid::from_str(str).unwrap(), ContractBid { level, denomination })
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
    fn serialize(level: ContractLevel, denomination: ContractDenomination, expected: &str) {
        let bid = ContractBid { level, denomination };
        assert_eq!(format!("{}", bid), expected);
    }

    #[test_case("1S", "1NT", Less; "1S is less than 1NT")]
    #[test_case("1S", "1H", Greater; "1S is more than 1H")]
    #[test_case("2D", "1H", Greater; "2D is more than 1H")]
    #[test_case("7D", "4H", Greater; "7D is more than 4H")]
    #[test_case("7NT", "7C", Greater; "7NT is more than 7C")]
    #[test_case("2D", "2D", Equal; "2D is equal to itself")]
    fn ordering(one: &str, other: &str, expected: Ordering) {
        let c1 = Contract::from_str(one).unwrap();
        let c2 = Contract::from_str(other).unwrap();
        assert_eq!(c1.cmp(&c2), expected)
    }
}
