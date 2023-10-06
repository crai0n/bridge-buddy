use crate::evaluator::hcp::HcpRange;
use crate::primitives::deal::hand::HandType;
use crate::primitives::Suit;
use ranges::{LengthRange, PointRange};
pub use suit_quality::SuitQuality;

mod ranges;
pub mod suit_quality;

#[derive(PartialEq)]
pub enum HandDescription {
    HandType(HandType),
    SuitLength(Suit, LengthRange),
    SuitQuality(Suit, SuitQuality),
    Hcp(HcpRange),
    TotalPoints(PointRange),
}

impl std::fmt::Display for HandDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HandDescription::HandType(ht) => write!(f, "Hand is {}.", ht),
            HandDescription::SuitLength(suit, range) => write!(f, "Hand has {} in {}.", range, suit),
            HandDescription::SuitQuality(suit, quality) => write!(f, "Hand's {} are {}.", suit, quality),
            HandDescription::Hcp(range) => write!(f, "Hand has {}.", range),
            HandDescription::TotalPoints(range) => write!(f, "Hand has {}.", range),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ranges::LengthRange;
    use super::ranges::PointRange;
    use super::HandDescription;
    use crate::engine_context::hand_description::SuitQuality;
    use crate::evaluator::hcp::HcpRange;
    use crate::primitives::deal::hand::HandType;
    use crate::primitives::Suit::*;
    use test_case::test_case;

    #[test_case(HandDescription::HandType(HandType::Balanced(None)), "Hand is balanced."; "Balanced Hand")]
    #[test_case(HandDescription::HandType(HandType::Balanced(Some(Spades))), "Hand is balanced with 5 cards in ♠."; "Balanced Hand with 5 card spades")]
    #[test_case(HandDescription::HandType(HandType::ThreeSuited(Spades, Hearts, Clubs)), "Hand is three-suited: ♠, ♥ and ♣."; "Three-suited in  Spades Hearts and Clubs")]
    #[test_case(HandDescription::HandType(HandType::SingleSuited(Diamonds)), "Hand is single-suited: ♦."; "Single-suited Diamonds")]
    #[test_case(HandDescription::SuitLength(Diamonds, LengthRange(4..=6) ), "Hand has 4 to 6 cards in ♦."; "4 to 6 cards in Diamonds")]
    #[test_case(HandDescription::Hcp(HcpRange(12.0..=16.0)), "Hand has 12 to 16 hcp."; "12 to 16 hcp")]
    #[test_case(HandDescription::TotalPoints(PointRange(15.0..=19.0)), "Hand has 15 to 19 points."; "15 to 19 total points")]
    #[test_case(HandDescription::SuitQuality(Spades, SuitQuality::Standing), "Hand's ♠ are standing."; "Standing Spades-suit")]
    #[test_case(HandDescription::SuitQuality(Hearts, SuitQuality::AlmostStanding), "Hand's ♥ are almost standing."; "Almost standing hearts-suit")]
    fn display(input: HandDescription, expected: &str) {
        let string = format!("{}", input);
        assert_eq!(string, expected);
    }
}
