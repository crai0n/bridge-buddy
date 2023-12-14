use crate::primitives::bid::*;

use itertools::Itertools;

use crate::error::BBError;
use crate::game::bid_manager::BidManager;
use crate::primitives::deal::Seat;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BidLine {
    bids: Vec<Bid>,
}

impl Display for BidLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bid_iter = self.bids.iter().map(|x| format!("{}", x)).join("-");
        write!(f, "{}", bid_iter)?;
        Ok(())
    }
}

impl BidLine {
    pub fn bids(&self) -> &[Bid] {
        &self.bids
    }

    pub fn bid(&mut self, bid: Bid) {
        self.bids.push(bid)
    }

    pub fn new() -> Self {
        let bids = vec![];
        BidLine { bids }
    }

    pub fn len(&self) -> usize {
        self.bids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl std::str::FromStr for BidLine {
    type Err = BBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bids = s
            .trim()
            .trim_matches('-')
            .split('-')
            .map(Bid::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let mut bid_manager = BidManager::new(Seat::North);
        for bid in bids {
            bid_manager.bid(bid)?;
        }
        Ok(bid_manager.bid_line().clone())
    }
}

impl Default for BidLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::primitives::bid_line::BidLine;
    use std::str::FromStr;

    use crate::error::BBError;
    use crate::primitives::bid::Bid;
    use test_case::test_case;

    #[test_case("P-1NT", &["P", "1NT"]; "Pass then 1NT")]
    #[test_case("p-1NT-P-2C-Pass-2D-", &["P", "1NT", "P", "2C", "P", "2D"]; "Two Diamonds")]
    #[test_case("P-1NT-X-Pass-p-xX", &["P", "1NT", "X", "P", "P", "XX"]; "Redoubled 1NT")]
    #[test_case("P-1NT-P-P-x", &["P", "1NT", "P", "P", "X"]; "Doubled 1NT")]
    fn from_str(input: &str, expect_line: &[&str]) {
        let input_line = BidLine::from_str(input).unwrap();
        let expected = expect_line.iter().map(|x| Bid::from_str(x).unwrap()).collect();
        assert_eq!(input_line, BidLine { bids: expected });
    }

    #[test_case("P-1NT", "Pass-1NT"; "Pass then 1NT")]
    #[test_case("p-1NT-P-2C-Pass-2D-", "Pass-1NT-Pass-2♣-Pass-2♦"; "Two Diamonds")]
    #[test_case("P-1NT-X-Pass-p-xX", "Pass-1NT-X-Pass-Pass-XX"; "Redoubled 1NT")]
    #[test_case("P-1NT-P-P-x", "Pass-1NT-Pass-Pass-X"; "Doubled 1NT")]
    fn round_trip(input: &str, expected: &str) {
        let input_line = BidLine::from_str(input).unwrap();
        let string = format!("{}", input_line);
        assert_eq!(string, expected);
    }

    #[test_case("P-X", "X"; "Double without Contract")]
    #[test_case("1NT-1S", "1S"; "1 Spades after 1NT")]
    #[test_case("1NT-P-X", "X"; "Double partners contract")]
    #[test_case("1NT-X-P-XX", "XX"; "Redouble partners double")]
    #[test_case("1NT-X-1H", "1H"; "Hearts after doubled No-Trump")]
    fn invalid_bid(input: &str, invalid: &str) {
        let bid_line = BidLine::from_str(input);
        let invalid_bid = Bid::from_str(invalid).unwrap();
        assert_eq!(bid_line, Err(BBError::InvalidBid(invalid_bid)))
    }
}
