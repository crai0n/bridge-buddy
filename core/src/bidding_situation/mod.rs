use strum::{Display, EnumString};

mod situation_finder;
mod situation_rule;

// use crate::primitives::bid_line::BidLine;

#[derive(PartialEq, Debug, EnumString, Clone, Copy, Display)]
pub enum BiddingSituation {
    Unknown,
    OpeningFirstSecond,
    OpeningThirdFourth,
    Answer1NoTrump,
    Answer1Major,
    Answer1Minor,
}
