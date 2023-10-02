use crate::bid_analyzer::bidding_situation::BiddingSituation;
use crate::bid_analyzer::hand_info::HandInfo;
use crate::primitives::bid::Bid;

#[derive(PartialEq)]
pub struct BidRule {
    situation: BiddingSituation,
    precedence: usize,
    conditions: Vec<HandInfo>,
    bid: Bid,
}

// "Answer1Major; 1 ; Length(Spades, LengthRange(3..=13), TotalPoints(PointRange(6-10)); 2S"
