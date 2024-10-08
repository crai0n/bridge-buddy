use crate::error::BBError;
use crate::primitives::bid::{AuxiliaryBid, Bid, ContractBid};
use crate::primitives::bid_line::BidLine;
use crate::primitives::contract::{Level, State, Strain};
use crate::primitives::deal::axis::Axis;
use crate::primitives::deal::Seat;
use crate::primitives::{Contract, Suit};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct BidManager {
    dealer: Seat,
    bid_line: BidLine,
}

impl BidManager {
    pub fn new(dealer: Seat) -> Self {
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

    pub fn lowest_available_contract_bid(&self) -> Option<ContractBid> {
        match self.last_contract_bid() {
            None => Some(ContractBid {
                level: Level::One,
                strain: Strain::Trump(Suit::Clubs),
            }),
            Some(bid) => bid.next().ok(),
        }
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

    fn bid_matches_strain(bid: &Bid, strain: Strain) -> bool {
        matches!(bid, Bid::Contract(y) if y.strain == strain)
    }

    pub fn implied_contract(&self) -> Option<Contract> {
        if let Some(last_contract_bid) = self.last_contract_bid() {
            let state = self.calculate_state();
            let strain = last_contract_bid.strain;
            Some(Contract {
                declarer: self.implied_declarer(strain),
                level: last_contract_bid.level,
                strain,
                state,
            })
        } else {
            None
        }
    }
    fn implied_declarer(&self, strain: Strain) -> Seat {
        let axis = self
            .bids()
            .iter()
            .rposition(|x| matches!(x, Bid::Contract(_)))
            .map(|x| (self.dealer + x).axis())
            .unwrap();

        self.first_to_name_strain_on_axis(strain, axis)
    }

    fn first_to_name_strain_on_axis(&self, strain: Strain, axis: Axis) -> Seat {
        self.bids()
            .iter()
            .enumerate()
            .position(|(i, x)| Self::bid_matches_strain(x, strain) && axis.has_player(self.dealer + i))
            .map(|x| self.dealer + x)
            .unwrap()
    }

    fn calculate_state(&self) -> State {
        match self
            .bids()
            .iter()
            .rev()
            .map_while(|x| x.try_as_auxiliary_bid().ok())
            .max()
        {
            Some(AuxiliaryBid::Pass) => State::Passed,
            Some(AuxiliaryBid::Double) => State::Doubled,
            Some(AuxiliaryBid::Redouble) => State::Redoubled,
            _ => State::Passed,
        }
    }

    pub fn next_to_play(&self) -> Seat {
        self.dealer + self.bid_line.len()
    }
}

impl TryFrom<BidLine> for BidManager {
    type Error = BBError;

    fn try_from(_bid_line: BidLine) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Display for BidManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "North East  South West  ",)?;
        writeln!(f, "------------------------")?;

        let mut x = match self.dealer {
            Seat::North => 0,
            Seat::East => 1,
            Seat::South => 2,
            Seat::West => 3,
        };

        let pad = "      ";

        for _i in 0..x {
            write!(f, "{}", pad)?;
        }

        for bid in self.bids() {
            let str = format!("{}", bid);
            write!(f, "{:<6}", str)?;
            x += 1;
            if x % 4 == 0 {
                writeln!(f)?;
            }
        }
        writeln!(f)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::error::BBError;
    use crate::game::bid_manager::BidManager;
    use crate::primitives::bid::Bid;
    use crate::primitives::bid_line::BidLine;
    use crate::primitives::deal::Seat;
    use crate::primitives::deal::Seat::*;
    use crate::primitives::Contract;
    use std::str::FromStr;
    use test_case::test_case;

    #[test]
    fn bidding() {
        let mut bid_manager = BidManager::new(Seat::North);
        let bid = Bid::from_str("1NT").unwrap();
        bid_manager.bid(bid).unwrap();
        let bid = Bid::from_str("2NT").unwrap();
        bid_manager.bid(bid).unwrap();
        assert_eq!(bid_manager.bids(), BidLine::from_str("1NT-2NT").unwrap().bids());
    }
    #[test]
    fn invalid_bidding() {
        let mut bid_manager = BidManager::new(Seat::West);
        let bid = Bid::from_str("2NT").unwrap();
        bid_manager.bid(bid).unwrap();
        let bid = Bid::from_str("1NT").unwrap();
        assert_eq!(bid_manager.bid(bid), Err(BBError::InvalidBid(bid)));
    }

    #[test]
    fn next_to_play() {
        let mut bid_manager = BidManager::new(Seat::West);
        let bid = Bid::from_str("1NT").unwrap();
        bid_manager.bid(bid).unwrap();
        let bid = Bid::from_str("2NT").unwrap();
        bid_manager.bid(bid).unwrap();
        assert_eq!(bid_manager.next_to_play(), Seat::East);
    }

    #[test_case("P-P", North, ""; "No Contract implied")]
    #[test_case("1NT-2S-P-P", East, "S2S"; "2 Spades")]
    #[test_case("1NT-X-P", South, "S1NTX"; "Doubled 1NT")]
    #[test_case("1NT-X-XX-P", West, "W1NTXX"; "Redoubled 1NT")]
    #[test_case("2D", North, "N2D"; "Two Diamonds")]
    #[test_case("1NT-X-2H-P-P-P", East, "W2H"; "Two Hearts")]
    #[test_case("1NT-2NT-P-3NT-P-P-P", North, "E3NT")]
    #[test_case("1NT-2H-P-3NT-P-P-P", West, "S3NT")]
    fn implied_contract(input: &str, dealer: Seat, implied: &str) {
        let bid_line = BidLine::from_str(input).unwrap();
        let mut manager = BidManager::new(dealer);
        for &bid in bid_line.bids() {
            manager.bid(bid).unwrap();
        }
        let implied_contract = Contract::from_str(implied).ok();
        assert_eq!(manager.implied_contract(), implied_contract)
    }

    #[test_case("1NT-2H-P-3NT-P-P-P", West, "North East  South West  \n------------------------\n                  1NT   \n2♥    Pass  3NT   Pass  \nPass  Pass  \n")]
    fn display(input: &str, dealer: Seat, expected: &str) {
        let bid_line = BidLine::from_str(input).unwrap();
        let mut manager = BidManager::new(dealer);
        for &bid in bid_line.bids() {
            manager.bid(bid).unwrap();
        }
        assert_eq!(format!("{}", manager), expected)
    }

    #[test_case("P-P-P", false; "Third player passes")]
    #[test_case("P-P-P-P", true; "All pass")]
    #[test_case("1NT-X-P", false; "Doubled 1NT")]
    #[test_case("1NT-X-2H-P-P-P", true; "Two Hearts")]
    fn bidding_has_ended(input: &str, expected: bool) {
        let bid_line = BidLine::from_str(input).unwrap();
        let mut manager = BidManager::new(Seat::South);
        for &bid in bid_line.bids() {
            manager.bid(bid).unwrap();
        }
        assert_eq!(manager.bidding_has_ended(), expected);
    }
}
