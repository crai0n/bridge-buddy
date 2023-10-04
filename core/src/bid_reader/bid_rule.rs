use crate::bid_reader::bidding_situation::BiddingSituation;
use crate::game_context::hand_description::HandDescription;
use crate::primitives::bid::Bid;

#[derive(PartialEq)]
pub struct BidRule {
    situation: BiddingSituation,
    precedence: usize,
    bid: Bid,
    conditions: Vec<HandDescription>,
    artificial: bool,
    meaning: String,
}

// "Answer1Major; 1 ; Length(Spades, LengthRange(3..=13), TotalPoints(PointRange(6-10)); 2S"
