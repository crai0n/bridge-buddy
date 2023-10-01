use crate::primitives::bid::*;
use crate::primitives::contract::*;

use itertools::Itertools;

use crate::error::BBError;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
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
    pub fn implied_contract(&self) -> Option<Contract> {
        let mut bid_iter = self.bids.iter().rev().peekable();
        let state = match bid_iter
            .peeking_take_while(|x| matches!(x, Bid::Auxiliary(_)))
            .map(|x| match x {
                Bid::Auxiliary(aux_bid) => aux_bid,
                _ => unreachable!(),
            })
            .max()
        {
            Some(AuxiliaryBid::Double) => ContractState::Doubled,
            Some(AuxiliaryBid::Redouble) => ContractState::Redoubled,
            _ => ContractState::Passed,
        };

        if let Some(Bid::Contract(contract_bid)) = bid_iter.next() {
            let level = contract_bid.level;
            let denomination = contract_bid.denomination;
            Some(Contract {
                level,
                denomination,
                state,
            })
        } else {
            None // if no one has bid yet, there is no contract
        }
    }

    pub fn new() -> Self {
        let bids = vec![];
        BidLine { bids }
    }

    pub fn bid(&mut self, bid: Bid) -> Result<(), BBError> {
        if self.can_bid(&bid) {
            self.bids.push(bid);
            Ok(())
        } else {
            Err(BBError::InvalidBid(bid))
        }
    }

    pub fn can_bid(&self, bid: &Bid) -> bool {
        match bid {
            Bid::Auxiliary(AuxiliaryBid::Pass) => self.can_pass(),
            Bid::Auxiliary(AuxiliaryBid::Double) => self.can_double(),
            Bid::Auxiliary(AuxiliaryBid::Redouble) => self.can_redouble(),
            Bid::Contract(c) => self.can_bid_contract(c),
        }
    }

    pub fn contract_is_final(&self) -> bool {
        self.three_passes_in_a_row()
    }

    fn three_passes_in_a_row(&self) -> bool {
        self.bids.len() == 3
            && self
                .bids
                .iter()
                .rev()
                .take(3)
                .all(|x| x == &Bid::Auxiliary(AuxiliaryBid::Pass))
    }

    fn can_pass(&self) -> bool {
        !self.contract_is_final()
    }

    fn can_bid_contract(&self, new: &ContractBid) -> bool {
        !self.contract_is_final()
            && match self.last_contract_bid() {
                Some(last) => new > last,
                None => true,
            }
    }

    fn last_contract_bid(&self) -> Option<&ContractBid> {
        self.bids
            .iter()
            .filter_map(|x| match x {
                Bid::Contract(b) => Some(b),
                _ => None,
            })
            .last()
    }

    fn can_redouble(&self) -> bool {
        !self.contract_is_final()
            && (self.last_bid_was_double_from_right_hand_opponent()
                || self.last_bid_was_double_from_left_hand_opponent())
    }

    fn last_bid_was_double_from_right_hand_opponent(&self) -> bool {
        self.bids.last() == Some(&Bid::Auxiliary(AuxiliaryBid::Double))
    }

    fn last_bid_was_double_from_left_hand_opponent(&self) -> bool {
        self.check_last_three_bids(Self::is_double_pass_pass)
    }

    fn check_last_three_bids(&self, check_pattern: fn(&[Bid]) -> bool) -> bool {
        self.bids.len() >= 3 && check_pattern(&self.bids[self.bids.len() - 3..])
    }

    fn is_double_pass_pass(line: &[Bid]) -> bool {
        matches!(
            line,
            [
                Bid::Auxiliary(AuxiliaryBid::Double),
                Bid::Auxiliary(AuxiliaryBid::Pass),
                Bid::Auxiliary(AuxiliaryBid::Pass)
            ]
        )
    }

    fn can_double(&self) -> bool {
        !self.contract_is_final()
            && (self.last_bid_was_contract_bid_from_right_hand_opponent()
                || self.last_bid_was_contract_bid_from_left_hand_opponent())
    }

    fn last_bid_was_contract_bid_from_right_hand_opponent(&self) -> bool {
        matches!(self.bids.last(), Some(Bid::Contract(_)))
    }

    fn last_bid_was_contract_bid_from_left_hand_opponent(&self) -> bool {
        self.check_last_three_bids(Self::is_contract_pass_pass)
    }

    fn is_contract_pass_pass(line: &[Bid]) -> bool {
        matches!(
            line,
            [
                Bid::Contract(_),
                Bid::Auxiliary(AuxiliaryBid::Pass),
                Bid::Auxiliary(AuxiliaryBid::Pass)
            ]
        )
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
        let mut bidline = BidLine::new();
        for bid in bids {
            bidline.bid(bid)?;
        }
        Ok(bidline)
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

    use crate::primitives::bid::Bid;
    use test_case::test_case;

    #[test_case("P-1NT", &["P", "1NT"]; "Pass then 1NT")]
    #[test_case("P-1NT-P-2C-P-2D-", &["P", "1NT", "P", "2C", "P", "2D"]; "Two Diamonds")]
    fn from_str(input: &str, expect_line: &[&str]) {
        let input_line = BidLine::from_str(input).unwrap();
        let expected = expect_line.iter().map(|x| Bid::from_str(x).unwrap()).collect();
        assert_eq!(input_line, BidLine { bids: expected });
    }
}
