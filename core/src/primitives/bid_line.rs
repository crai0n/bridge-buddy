use crate::primitives::bid::*;
use crate::primitives::contract::*;

use itertools::Itertools;

use crate::error::BBError;
use crate::primitives::contract::Contract;
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
        if let Some(last_contract_bid) = self.last_contract_bid() {
            let state = self.calculate_contract_state();

            Some(Contract {
                level: last_contract_bid.level,
                denomination: last_contract_bid.denomination,
                state,
            })
        } else {
            None
        }
    }

    fn calculate_contract_state(&self) -> ContractState {
        match self.bids.iter().rev().map_while(Self::access_auxiliary).max() {
            Some(AuxiliaryBid::Pass) => ContractState::Passed,
            Some(AuxiliaryBid::Double) => ContractState::Doubled,
            Some(AuxiliaryBid::Redouble) => ContractState::Redoubled,
            _ => ContractState::Passed,
        }
    }

    fn access_auxiliary(bid: &Bid) -> Option<&AuxiliaryBid> {
        match bid {
            Bid::Contract(_) => None,
            Bid::Auxiliary(ab) => Some(ab),
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

    pub fn bidding_has_ended(&self) -> bool {
        self.bids.len() > 3 // every player has bid at least once
            && self
                .bids
                .iter()
                .rev()
                .take(3)
                .all(|x| x == &Bid::Auxiliary(AuxiliaryBid::Pass))
    }

    fn can_pass(&self) -> bool {
        !self.bidding_has_ended()
    }

    fn can_bid_contract(&self, new: &ContractBid) -> bool {
        !self.bidding_has_ended()
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
        !self.bidding_has_ended()
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
        !self.bidding_has_ended()
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

    pub fn implied_declarer_position(&self) -> Option<usize> {
        let auction_winner = self.bids.iter().rposition(|x| matches!(x, Bid::Contract(_)));

        match auction_winner {
            None => None,
            Some(offset) => {
                let denomination = self.implied_contract().unwrap().denomination;
                self.first_occurence_of_contract_denomination(denomination, offset)
            }
        }
    }

    fn first_occurence_of_contract_denomination(
        &self,
        denomination: ContractDenomination,
        offset: usize,
    ) -> Option<usize> {
        self.bids
            .iter()
            .enumerate()
            .find(|(i, x)| Self::bid_matches_denomination(x, denomination) && Self::same_axis(i, offset))
            .map(|(i, _)| i)
    }

    fn same_axis(position: &usize, other_position: usize) -> bool {
        (position + other_position) % 2 == 0
    }

    fn bid_matches_denomination(bid: &&Bid, denomination: ContractDenomination) -> bool {
        matches!(bid, Bid::Contract(y) if y.denomination == denomination)
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

    use crate::error::BBError;
    use crate::primitives::bid::Bid;
    use crate::primitives::contract::Contract;
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

    #[test_case("P-P", ""; "No Contract implied")]
    #[test_case("1NT-2S-P-P", "2S"; "2 Spades")]
    #[test_case("1NT-X-P", "1NTX"; "Doubled 1NT")]
    #[test_case("1NT-X-XX-P", "1NTXX"; "Redoubled 1NT")]
    #[test_case("2D", "2D"; "Two Diamonds")]
    #[test_case("1NT-X-2H-P-P-P", "2H"; "Two Hearts")]
    fn implied_contract(input: &str, implied: &str) {
        let bid_line = BidLine::from_str(input).unwrap();
        let implied_contract = Contract::from_str(implied).ok();
        assert_eq!(bid_line.implied_contract(), implied_contract)
    }

    #[test_case("1NT-2NT-P-3NT-P-P-P", 1)]
    #[test_case("1NT-2H-P-3NT-P-P-P", 3)]
    fn implied_declarer_position(input: &str, expected: usize) {
        let bid_line = BidLine::from_str(input).unwrap();
        assert_eq!(bid_line.implied_declarer_position().unwrap(), expected);
    }

    #[test_case("P-P-P", false; "Third player passes")]
    #[test_case("P-P-P-P", true; "All pass")]
    #[test_case("1NT-X-P", false; "Doubled 1NT")]
    #[test_case("1NT-X-2H-P-P-P", true; "Two Hearts")]
    fn contract_is_final(input: &str, expected: bool) {
        let bid_line = BidLine::from_str(input).unwrap();
        assert_eq!(bid_line.bidding_has_ended(), expected);
    }
}
