mod auxiliary_bid;
mod contract_bid;

pub use auxiliary_bid::AuxiliaryBid;
pub use contract_bid::ContractBid;

use crate::error::BBError;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Bid {
    Contract(ContractBid),
    Auxiliary(AuxiliaryBid),
}

impl Display for Bid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bid::Contract(c) => write!(f, "{}", c)?,
            Bid::Auxiliary(a) => write!(f, "{}", a)?,
        }
        Ok(())
    }
}

impl FromStr for Bid {
    type Err = BBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(c) = ContractBid::from_str(s) {
            Ok(Bid::Contract(c))
        } else if let Ok(a) = AuxiliaryBid::from_str(s) {
            Ok(Bid::Auxiliary(a))
        } else {
            Err(BBError::UnknownBid(s.into()))
        }
    }
}

impl Bid {
    pub fn access_auxiliary_bid(&self) -> Option<AuxiliaryBid> {
        match self {
            Bid::Auxiliary(auxiliary_bid) => Some(*auxiliary_bid),
            _ => None,
        }
    }

    pub fn access_contract_bid(&self) -> Option<ContractBid> {
        match self {
            Bid::Contract(contract_bid) => Some(*contract_bid),
            _ => None,
        }
    }
}
#[cfg(test)]
mod test {
    use super::AuxiliaryBid::*;
    use super::{AuxiliaryBid, ContractBid};
    use crate::primitives::contract::ContractDenomination::*;
    use crate::primitives::contract::ContractLevel::*;
    use crate::primitives::Suit::*;

    use super::Bid;
    use super::Bid::*;

    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("p", Auxiliary(Pass); "p is Pass")]
    #[test_case("P", Auxiliary(Pass); "P_Pass")]
    #[test_case("X", Auxiliary(Double); "x is Double")]
    #[test_case("x", Auxiliary(Double); "x_Double")]
    #[test_case("XX", Auxiliary(Redouble); "XX_Redouble")]
    #[test_case("1NT", Contract(ContractBid { level: One, denomination: NoTrump}); "No Trump")]
    #[test_case("2S", Contract(ContractBid { level: Two, denomination: Trump(Spades)}); "Two Spades")]
    #[test_case("3d", Contract(ContractBid { level: Three, denomination: Trump(Diamonds)}); "Three Diamonds")]
    #[test_case("4♥", Contract(ContractBid { level: Four, denomination: Trump(Hearts)}); "Four Hearts")]
    #[test_case("7d", Contract(ContractBid{ level: Seven, denomination: Trump(Diamonds)}); "Diamonds")]
    fn from_str(str: &str, bid: Bid) {
        assert_eq!(Bid::from_str(str).unwrap(), bid)
    }

    #[test_case("Y")]
    #[test_case("3G")]
    fn from_str_fails(str: &str) {
        assert!(Bid::from_str(str).is_err())
    }

    #[test_case(Auxiliary(Pass), "Pass"; "Pass")]
    #[test_case(Auxiliary(Double), "X"; "Double")]
    #[test_case(Auxiliary(Redouble), "XX"; "Redouble")]
    #[test_case(Contract(ContractBid { level: One, denomination: NoTrump}), "1NT"; "No Trump")]
    #[test_case(Contract(ContractBid { level: Two, denomination: Trump(Spades)}), "2♠"; "Spades")]
    #[test_case(Contract(ContractBid { level: Three, denomination: Trump(Diamonds)}), "3♦"; "Diamonds")]
    #[test_case(Contract(ContractBid { level: Four, denomination: Trump(Hearts)}), "4♥"; "Hearts")]
    fn serialize(bid: Bid, expected: &str) {
        assert_eq!(format!("{}", bid), expected);
    }

    #[test_case(Auxiliary(Pass), Auxiliary(Pass), true; "Pass is equal to Pass")]
    #[test_case(Auxiliary(Double), Auxiliary(Redouble), false; "Double is less than Redouble")]
    #[test_case(Auxiliary(Redouble), Auxiliary(Pass), false; "Redouble is greater than Pass")]
    #[test_case(Contract(ContractBid { level: Four, denomination: Trump(Hearts)}), Auxiliary(Redouble), false; "4H is not a Redouble")]
    #[test_case(Contract(ContractBid { level: Four, denomination: Trump(Hearts)}), Contract(ContractBid { level: Four, denomination: Trump(Hearts)}), true; "Four hearts is four hearts")]
    fn equality(one: Bid, other: Bid, expected: bool) {
        assert_eq!(one.eq(&other), expected)
    }

    #[test]
    fn is_clone() {
        let one = AuxiliaryBid::Pass;
        let two = one;

        assert_eq!(one, two);
    }

    #[test_case(Auxiliary(Pass), Some(Pass); "Pass is an Auxiliary")]
    #[test_case(Contract(ContractBid { level: Four, denomination: Trump(Hearts)}), None; "4H is not an Auxiliary")]
    fn access_auxiliary(bid: Bid, inner: Option<AuxiliaryBid>) {
        assert_eq!(bid.access_auxiliary_bid(), inner);
    }
    #[test_case(Auxiliary(Pass), None; "Pass is an Auxiliary")]
    #[test_case(Contract(ContractBid { level: Four, denomination: Trump(Hearts)}), Some(ContractBid { level: Four, denomination: Trump(Hearts)}); "4H is not a Contract Bid")]
    fn access_contract(bid: Bid, inner: Option<ContractBid>) {
        assert_eq!(bid.access_contract_bid(), inner);
    }
}
