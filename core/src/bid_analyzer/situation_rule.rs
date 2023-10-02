use super::bidding_situation::BiddingSituation;
use crate::error::BBError;
use std::str::FromStr;

use crate::primitives::bid_line::BidLine;

#[derive(PartialEq, Debug)]
pub struct SituationRule {
    pub bid_line: BidLine,
    pub situation: BiddingSituation,
}

impl FromStr for SituationRule {
    type Err = BBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (bid_line_str, situation_str) = Self::split_at_semicolon(s.trim())?;
        match (
            BidLine::from_str(bid_line_str),
            BiddingSituation::from_str(situation_str),
        ) {
            (Ok(bid_line), Ok(situation)) => Ok(SituationRule { bid_line, situation }),
            (Err(e), _) => Err(e),
            (_, Err(_)) => Err(BBError::UnknownBiddingSituation(situation_str.into())),
        }
    }
}

impl SituationRule {
    fn split_at_semicolon(string: &str) -> Result<(&str, &str), BBError> {
        string.split_once(';').ok_or(BBError::ParseError(
            string.into(),
            "missing semi-colon between bid-line and situation",
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SituationRule;
    use crate::bid_analyzer::bidding_situation::BiddingSituation;
    use crate::primitives::bid_line::BidLine;
    // use crate::primitives::contract::ContractDenomination::*;
    // use crate::primitives::contract::ContractLevel::*;
    // use crate::primitives::Suit::*;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("1S;Answer1Major", SituationRule{ bid_line: BidLine::from_str("1S").unwrap(), situation: BiddingSituation::Answer1Major})]
    #[test_case("1NT;Answer1NoTrump", SituationRule{ bid_line: BidLine::from_str("1NT").unwrap(), situation: BiddingSituation::Answer1NoTrump})]
    #[test_case("1D;Answer1Minor", SituationRule{ bid_line: BidLine::from_str("1D").unwrap(), situation: BiddingSituation::Answer1Minor})]
    #[test_case("P-P;OpeningThirdFourth", SituationRule{ bid_line: BidLine::from_str("P-P").unwrap(), situation: BiddingSituation::OpeningThirdFourth})]
    #[test_case(";OpeningFirstSecond", SituationRule{ bid_line: BidLine::from_str("").unwrap(), situation: BiddingSituation::OpeningFirstSecond})]
    #[test_case("2NT;Unknown", SituationRule{ bid_line: BidLine::from_str("2NT").unwrap(), situation: BiddingSituation::Unknown})]
    fn from_str(input: &str, expected: SituationRule) {
        let rule = SituationRule::from_str(input).unwrap();
        assert_eq!(rule, expected);
    }
}
