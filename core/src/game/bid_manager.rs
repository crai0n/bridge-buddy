use crate::error::BBError;
use crate::primitives::bid::{AuxiliaryBid, Bid, ContractBid};
use crate::primitives::bid_line::BidLine;
use crate::primitives::contract::{ContractDenomination, ContractState};
use crate::primitives::deal::axis::Axis;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::Contract;

#[derive(Debug, Clone)]
pub struct BidManager {
    dealer: PlayerPosition,
    bid_line: BidLine,
}

impl BidManager {
    pub fn new(dealer: PlayerPosition) -> Self {
        BidManager {
            dealer,
            bid_line: BidLine::new(),
        }
    }

    pub fn bids(&self) -> &[Bid] {
        self.bid_line.bids()
    }

    pub fn bid_line(&self) -> BidLine {
        self.bid_line.clone()
    }

    pub fn bid(&mut self, bid: Bid) -> Result<(), BBError> {
        self.validate_bid(bid)?;
        self.append_bid(bid);
        Ok(())
    }

    fn append_bid(&mut self, bid: Bid) {
        self.bid_line.bid(bid);
    }

    pub fn validate_bid(&self, bid: Bid) -> Result<(), BBError> {
        if self.is_valid_bid(&bid) {
            Ok(())
        } else {
            Err(BBError::InvalidBid(bid))
        }
    }

    pub fn is_valid_bid(&self, bid: &Bid) -> bool {
        match bid {
            Bid::Auxiliary(AuxiliaryBid::Pass) => self.can_pass(),
            Bid::Auxiliary(AuxiliaryBid::Double) => self.can_double(),
            Bid::Auxiliary(AuxiliaryBid::Redouble) => self.can_redouble(),
            Bid::Contract(c) => self.can_bid_contract(c),
        }
    }

    fn can_pass(&self) -> bool {
        !self.bidding_has_ended()
    }

    pub fn bidding_has_ended(&self) -> bool {
        self.every_player_has_bid_at_least_once() && self.check_last_three_bids(Self::is_three_passes)
    }

    fn every_player_has_bid_at_least_once(&self) -> bool {
        self.bids().len() > 3
    }

    fn is_three_passes(line: &[Bid]) -> bool {
        matches!(
            line,
            [
                Bid::Auxiliary(AuxiliaryBid::Pass),
                Bid::Auxiliary(AuxiliaryBid::Pass),
                Bid::Auxiliary(AuxiliaryBid::Pass)
            ]
        )
    }

    fn can_bid_contract(&self, new: &ContractBid) -> bool {
        !self.bidding_has_ended()
            && match self.last_contract_bid() {
                Some(last) => new > last,
                None => true,
            }
    }

    pub fn last_contract_bid(&self) -> Option<&ContractBid> {
        self.bids()
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
        self.bids().last() == Some(&Bid::Auxiliary(AuxiliaryBid::Double))
    }

    fn last_bid_was_double_from_left_hand_opponent(&self) -> bool {
        self.check_last_three_bids(Self::is_double_pass_pass)
    }

    fn check_last_three_bids(&self, check_pattern: fn(&[Bid]) -> bool) -> bool {
        self.bids().len() >= 3 && check_pattern(&self.bids()[self.bids().len() - 3..])
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
        matches!(self.bids().last(), Some(Bid::Contract(_)))
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

    fn bid_matches_denomination(bid: &Bid, denomination: ContractDenomination) -> bool {
        matches!(bid, Bid::Contract(y) if y.denomination == denomination)
    }

    pub fn implied_contract(&self) -> Option<Contract> {
        if let Some(last_contract_bid) = self.last_contract_bid() {
            let state = self.calculate_contract_state();
            let denomination = last_contract_bid.denomination;
            Some(Contract {
                declarer: self.implied_declarer(denomination),
                level: last_contract_bid.level,
                denomination,
                state,
            })
        } else {
            None
        }
    }
    fn implied_declarer(&self, denomination: ContractDenomination) -> PlayerPosition {
        let axis = self
            .bids()
            .iter()
            .rposition(|x| matches!(x, Bid::Contract(_)))
            .map(|x| (self.dealer + x).axis())
            .unwrap();

        self.first_to_name_denomination_on_axis(denomination, axis)
    }

    fn first_to_name_denomination_on_axis(&self, denomination: ContractDenomination, axis: Axis) -> PlayerPosition {
        self.bids()
            .iter()
            .enumerate()
            .position(|(i, x)| Self::bid_matches_denomination(x, denomination) && axis.has_player(self.dealer + i))
            .map(|x| self.dealer + x)
            .unwrap()
    }

    fn calculate_contract_state(&self) -> ContractState {
        match self.bids().iter().rev().map_while(|x| x.access_auxiliary_bid()).max() {
            Some(AuxiliaryBid::Pass) => ContractState::Passed,
            Some(AuxiliaryBid::Double) => ContractState::Doubled,
            Some(AuxiliaryBid::Redouble) => ContractState::Redoubled,
            _ => ContractState::Passed,
        }
    }

    pub fn next_to_play(&self) -> PlayerPosition {
        self.dealer + self.bid_line.len()
    }
}

impl TryFrom<BidLine> for BidManager {
    type Error = BBError;

    fn try_from(_bid_line: BidLine) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::error::BBError;
    use crate::game::bid_manager::BidManager;
    use crate::primitives::bid::Bid;
    use crate::primitives::bid_line::BidLine;
    use crate::primitives::deal::PlayerPosition;
    use crate::primitives::deal::PlayerPosition::*;
    use crate::primitives::Contract;
    use std::str::FromStr;
    use test_case::test_case;

    #[test]
    fn bidding() {
        let mut bid_manager = BidManager::new(PlayerPosition::North);
        let bid = Bid::from_str("1NT").unwrap();
        bid_manager.bid(bid).unwrap();
        let bid = Bid::from_str("2NT").unwrap();
        bid_manager.bid(bid).unwrap();
        assert_eq!(bid_manager.bids(), BidLine::from_str("1NT-2NT").unwrap().bids());
    }
    #[test]
    fn invalid_bidding() {
        let mut bid_manager = BidManager::new(PlayerPosition::West);
        let bid = Bid::from_str("2NT").unwrap();
        bid_manager.bid(bid).unwrap();
        let bid = Bid::from_str("1NT").unwrap();
        assert_eq!(bid_manager.bid(bid), Err(BBError::InvalidBid(bid)));
    }

    #[test]
    fn next_to_play() {
        let mut bid_manager = BidManager::new(PlayerPosition::West);
        let bid = Bid::from_str("1NT").unwrap();
        bid_manager.bid(bid).unwrap();
        let bid = Bid::from_str("2NT").unwrap();
        bid_manager.bid(bid).unwrap();
        assert_eq!(bid_manager.next_to_play(), PlayerPosition::East);
    }

    #[test_case("P-P", North, ""; "No Contract implied")]
    #[test_case("1NT-2S-P-P", East, "S2S"; "2 Spades")]
    #[test_case("1NT-X-P", South, "S1NTX"; "Doubled 1NT")]
    #[test_case("1NT-X-XX-P", West, "W1NTXX"; "Redoubled 1NT")]
    #[test_case("2D", North, "N2D"; "Two Diamonds")]
    #[test_case("1NT-X-2H-P-P-P", East, "W2H"; "Two Hearts")]
    #[test_case("1NT-2NT-P-3NT-P-P-P", North, "E3NT")]
    #[test_case("1NT-2H-P-3NT-P-P-P", West, "S3NT")]
    fn implied_contract(input: &str, dealer: PlayerPosition, implied: &str) {
        let bid_line = BidLine::from_str(input).unwrap();
        let mut manager = BidManager::new(dealer);
        for &bid in bid_line.bids() {
            manager.bid(bid).unwrap();
        }
        let implied_contract = Contract::from_str(implied).ok();
        assert_eq!(manager.implied_contract(), implied_contract)
    }

    #[test_case("P-P-P", false; "Third player passes")]
    #[test_case("P-P-P-P", true; "All pass")]
    #[test_case("1NT-X-P", false; "Doubled 1NT")]
    #[test_case("1NT-X-2H-P-P-P", true; "Two Hearts")]
    fn bidding_has_ended(input: &str, expected: bool) {
        let bid_line = BidLine::from_str(input).unwrap();
        let mut manager = BidManager::new(PlayerPosition::South);
        for &bid in bid_line.bids() {
            manager.bid(bid).unwrap();
        }
        assert_eq!(manager.bidding_has_ended(), expected);
    }
}
