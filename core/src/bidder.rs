use crate::error::ParseError;
use strum::{Display, EnumString};

use crate::contract::*;

use crate::deal::PlayerPosition;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractBid {
    level: ContractLevel,
    denomination: ContractDenomination,
}

impl std::fmt::Display for ContractBid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.level, self.denomination)?;
        Ok(())
    }
}

impl std::str::FromStr for ContractBid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let level = match ContractLevel::from_str(&s[..1]) {
            Ok(l) => l,
            Err(_) => {
                return Err(ParseError {
                    cause: s.into(),
                    description: "unknown level",
                })
            }
        };

        let den_str = &s[1..];

        let denomination = match ContractDenomination::from_str(den_str) {
            Ok(d) => d,
            Err(_) => {
                return Err(ParseError {
                    cause: s.into(),
                    description: "unknown contract denomination",
                })
            }
        };

        Ok(ContractBid { level, denomination })
    }
}

#[derive(Debug, Display, EnumString, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuxiliaryBid {
    #[strum(serialize = "p")]
    #[strum(serialize = "P")]
    #[strum(to_string = "Pass")]
    Pass,
    #[strum(serialize = "x")]
    #[strum(to_string = "X")]
    Double,
    #[strum(serialize = "xx")]
    #[strum(serialize = "Xx")]
    #[strum(serialize = "xX")]
    #[strum(to_string = "XX")]
    Redouble,
}

// impl std::fmt::Display for AuxiliaryBid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             AuxiliaryBid::Pass => write!(f, "Pass")?
//             AuxiliaryBid::Double => write!(f, "X")?
//             AuxiliaryBid::Redouble => write!(f, "XX")?
//         }
//         Ok(())
//     }
// }

#[derive(Debug, PartialEq, Eq)]
pub enum Bid {
    Contract(ContractBid),
    Auxiliary(AuxiliaryBid),
}

impl std::fmt::Display for Bid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bid::Contract(c) => write!(f, "{}", c)?,
            Bid::Auxiliary(a) => write!(f, "{}", a)?,
        }
        Ok(())
    }
}

impl std::str::FromStr for Bid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(c) = ContractBid::from_str(s) {
            Ok(Bid::Contract(c))
        } else if let Ok(a) = AuxiliaryBid::from_str(s) {
            Ok(Bid::Auxiliary(a))
        } else {
            Err(ParseError {
                cause: s.into(),
                description: "cannot parse bid",
            })
        }
    }
}

pub struct BidLine {
    bids: Vec<Bid>,
    dealer: PlayerPosition,
}

#[cfg(test)]
mod test {
    use crate::bidder::*;
    use crate::card::Suit;
    use std::str::FromStr;
    // use std::{cmp::Ordering, str::FromStr};
    use test_case::test_case;

    #[test_case("1NT", ContractLevel::One, ContractDenomination::NoTrump; "No Trump")]
    #[test_case("2S", ContractLevel::Two, ContractDenomination::Trump(Suit::Spades); "Spades")]
    #[test_case("3d", ContractLevel::Three, ContractDenomination::Trump(Suit::Diamonds); "Diamonds")]
    #[test_case("4♥",ContractLevel::Four, ContractDenomination::Trump(Suit::Hearts); "Hearts")]
    fn contract_bid_from_str(str: &str, level: ContractLevel, denomination: ContractDenomination) {
        assert_eq!(ContractBid::from_str(str).unwrap(), ContractBid { level, denomination })
    }

    #[test_case("1NT", Bid::Contract(ContractBid{ level: ContractLevel::One, denomination: ContractDenomination::NoTrump}); "No Trump")]
    #[test_case("2S", Bid::Contract(ContractBid{ level: ContractLevel::Two, denomination: ContractDenomination::Trump(Suit::Spades)}); "Spades")]
    #[test_case("7d", Bid::Contract(ContractBid{ level: ContractLevel::Seven, denomination: ContractDenomination::Trump(Suit::Diamonds)}); "Diamonds")]
    #[test_case("4♥",Bid::Contract(ContractBid{ level: ContractLevel::Four, denomination: ContractDenomination::Trump(Suit::Hearts)}); "Hearts")]
    #[test_case("p",Bid::Auxiliary(AuxiliaryBid::Pass); "pass")]
    #[test_case("x",Bid::Auxiliary(AuxiliaryBid::Double); "double")]
    #[test_case("Xx",Bid::Auxiliary(AuxiliaryBid::Redouble); "redouble")]
    fn bid_from_str(str: &str, bid: Bid) {
        assert_eq!(Bid::from_str(str).unwrap(), bid)
    }
}
