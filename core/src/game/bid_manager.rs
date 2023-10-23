use crate::error::BBError;
use crate::primitives::bid::Bid;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::PlayerPosition;

pub struct BidManager {
    bids: BidLine,
    declarer: PlayerPosition,
    turn: PlayerPosition,
}

impl BidManager {
    pub fn new(declarer: PlayerPosition) -> BidManager {
        BidManager {
            bids: BidLine::default(),
            declarer,
            turn: declarer,
        }
    }

    pub fn bid(&mut self, bid: Bid) -> Result<(), BBError> {
        self.bids.bid(bid)?;
        self.turn = self.turn + 1;
        Ok(())
    }

    pub fn turn(&self) -> PlayerPosition {
        self.turn
    }
}
