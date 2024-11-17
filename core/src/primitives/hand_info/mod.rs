pub mod suit_quality;

use crate::primitives::deal::hand::HandType;
use crate::primitives::hand_info::suit_quality::SuitQuality;
use crate::primitives::Suit;
use crate::primitives::Suit::{Clubs, Diamonds, Hearts, Spades};
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use strum::Display;

#[allow(dead_code)]
const ANY_BALANCED: [HandType; 5] = [
    HandType::Balanced(None),
    HandType::Balanced(Some(Spades)),
    HandType::Balanced(Some(Hearts)),
    HandType::Balanced(Some(Diamonds)),
    HandType::Balanced(Some(Clubs)),
];

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum HandInfo {
    HandType(Vec<HandType>),     // One of these HandTypes
    Suit(Vec<SuitInfo>),         // One of these is true
    Strength(Vec<StrengthInfo>), // One of these ranges is true
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SuitInfo {
    suit: Suit,
    at_least: Option<u8>,
    at_most: Option<u8>,
    minimum_quality: Option<SuitQuality>,
}

impl Display for SuitInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lenstr = match (self.at_least, self.at_most) {
            (Some(l), Some(m)) if l == m => format!("exactly {}", l),
            (Some(l), Some(m)) => format!("{}-{}", l, m),
            (None, Some(m)) => format!("at most {}", m),
            (Some(l), None) => format!("at least {}", l),
            _ => "".to_string(),
        };
        if lenstr.is_empty() {
            if let Some(qual) = self.minimum_quality {
                write!(f, "{}s are {}", self.suit, qual)
            } else {
                write!(f, "no info about {}", self.suit)
            }
        } else if let Some(qual) = self.minimum_quality {
            write!(f, "{} {} cards in {}", lenstr, qual, self.suit)
        } else {
            write!(f, "{} cards in {}", lenstr, self.suit)
        }
    }
}

#[derive(Display, Debug, Clone, Copy)]
pub enum StrengthUnit {
    F,
    FL,
    FV,
    Winners,
    Losers,
}

#[derive(Debug, Clone, Copy)]
pub struct StrengthInfo {
    at_least: Option<f64>,
    at_most: Option<f64>,
    unit: StrengthUnit,
}

impl Display for StrengthInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lenstr = match (self.at_least, self.at_most) {
            (Some(l), Some(m)) if l == m => format!("exactly {}", l),
            (Some(l), Some(m)) => format!("{}-{}", l, m),
            (None, Some(m)) => format!("at most {}", m),
            (Some(l), None) => format!("at least {}", l),
            _ => "no info about".to_string(),
        };
        write!(f, "{} {}", lenstr, self.unit)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HandDescription(Vec<HandInfo>); // A HandDescription is true when ALL contained HandInfos are true

#[allow(dead_code)]
impl HandDescription {
    fn default() -> Self {
        HandDescription(vec![])
    }
}

impl Display for HandDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|hi| format!("({})", hi)).join(" and "))
    }
}

impl Display for HandInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HandInfo::HandType(hts) => {
                let desc = hts.iter().map(|ht| format!("{}", ht)).join(" or ");
                write!(f, "{}", desc)
            }
            HandInfo::Strength(ranges) => {
                let desc = ranges.iter().map(|range| format!("{}", range)).join(" or ");
                write!(f, "{}", desc)
            }
            HandInfo::Suit(suit_infos) => {
                let desc = suit_infos.iter().map(|suit| format!("{}", suit)).join(" or ");
                write!(f, "{}", desc)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::HandInfo;
    use super::StrengthInfo;
    use super::StrengthUnit;
    use super::SuitInfo;
    use super::SuitQuality;
    use crate::primitives::deal::hand::HandType;
    use crate::primitives::Suit::*;
    use test_case::test_case;

    #[test_case(HandInfo::HandType(vec![HandType::Balanced(None)]), "balanced without a 5-card suit"; "Balanced Hand")]
    #[test_case(HandInfo::HandType(vec![HandType::Balanced(None), HandType::Balanced(Some(Spades))]), "balanced without a 5-card suit or balanced with 5 cards in ♠"; "Balanced Hand 2")]
    #[test_case(HandInfo::HandType(vec![HandType::Balanced(Some(Spades))]), "balanced with 5 cards in ♠"; "Balanced Hand with 5 card spades")]
    #[test_case(HandInfo::HandType(vec![HandType::ThreeSuited(Spades, Hearts, Clubs)]), "three-suited in ♠, ♥ and ♣"; "Three-suited in  Spades Hearts and Clubs")]
    #[test_case(HandInfo::HandType(vec![HandType::SingleSuited(Diamonds)]), "single-suited in ♦"; "Single-suited Diamonds")]
    #[test_case(HandInfo::Strength(vec![StrengthInfo{at_least: Some(15.0), at_most: Some(19.0), unit: StrengthUnit::F}]), "15-19 F"; "15 to 19 total F")]
    #[test_case(HandInfo::Strength(vec![StrengthInfo{at_least: Some(13.0), at_most: Some(23.0), unit: StrengthUnit::FL}, StrengthInfo{at_least: Some(12.0), at_most: Some(12.0), unit: StrengthUnit::F}]), "13-23 FL or exactly 12 F"; "opening strength")]
    #[test_case(HandInfo::Strength(vec![StrengthInfo{at_least: Some(15.0), at_most: Some(19.0), unit: StrengthUnit::F}, StrengthInfo{at_least: Some(21.0), at_most: Some(22.0), unit: StrengthUnit::F}]), "15-19 F or 21-22 F"; "15 to 22 total F")]
    #[test_case(HandInfo::Suit(vec![SuitInfo{suit: Diamonds, at_least: Some(4), at_most: Some(6), minimum_quality: None}]), "4-6 cards in ♦"; "4 to 6 cards in Diamonds")]
    #[test_case(HandInfo::Suit(vec![SuitInfo{suit: Spades, at_least: None, at_most: None, minimum_quality: Some(SuitQuality::Standing)}]), "♠s are standing"; "Standing Spades-suit")]
    #[test_case(HandInfo::Suit(vec![SuitInfo{suit: Hearts, at_least: None, at_most: None, minimum_quality: Some(SuitQuality::AlmostStanding)}]), "♥s are almost standing"; "Almost standing hearts-suit short")]
    #[test_case(HandInfo::Suit(vec![SuitInfo{suit: Hearts, at_least: Some(5), at_most: None, minimum_quality: Some(SuitQuality::AlmostStanding)}]), "at least 5 almost standing cards in ♥"; "at least 5 cards")]
    #[test_case(HandInfo::Suit(vec![SuitInfo{suit: Hearts, at_least: None, at_most: Some(2), minimum_quality: None}]), "at most 2 cards in ♥"; "short hearts")]
    #[test_case(HandInfo::Suit(vec![SuitInfo{suit: Diamonds, at_least: Some(5), at_most: Some(6), minimum_quality: Some(SuitQuality::Good)}]), "5-6 good cards in ♦"; "specific diamonds")]
    #[test_case(HandInfo::Suit(vec![SuitInfo{suit: Diamonds, at_least: Some(5), at_most: Some(6), minimum_quality: Some(SuitQuality::Good)}, SuitInfo{suit: Hearts, at_least: Some(5), at_most: Some(5), minimum_quality: Some(SuitQuality::Good)}]), "5-6 good cards in ♦ or exactly 5 good cards in ♥"; "specific diamonds and hearts")]
    fn display(input: HandInfo, expected: &str) {
        let string = format!("{}", input);
        assert_eq!(string, expected);
    }
}
