use crate::primitives::bid::*;
use crate::primitives::contract::*;

use crate::primitives::deal::PlayerPosition;
use itertools::Itertools;

use std::fmt::Display;

pub struct BidLine {
    bids: Vec<Bid>,
    first_bid: PlayerPosition,
}

impl Display for BidLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Headline
        writeln!(f, " North  East South  West")?;

        let bid_iter = self.bids.iter().map(|x| format!("{:>6}", x));
        //first line
        let mut line_str = String::with_capacity(24);
        let mut i;
        match self.first_bid {
            PlayerPosition::North => {
                i = 0;
            }
            PlayerPosition::East => {
                line_str += "      ";
                i = 1;
            }
            PlayerPosition::South => {
                line_str += "            ";
                i = 2;
            }
            PlayerPosition::West => {
                line_str += "                  ";
                i = 3;
            }
        }

        for bid_str in bid_iter {
            i += 1;
            line_str += &bid_str;
            if i % 4 == 0 {
                writeln!(f, "{}", line_str)?;
            }
        }
        if i % 4 != 0 {
            writeln!(f, "{}", line_str)?;
        }

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

    pub fn implied_declarer(&self) -> Option<PlayerPosition> {
        // find the last ContractBid
        if let Some(Bid::Contract(last)) = self.bids.iter().filter(|x| matches!(x, Bid::Contract(_))).last() {
            // find the position of the first bid with that denomination
            self.bids
                .iter()
                .position(|x| match x {
                    Bid::Auxiliary(_) => false,
                    Bid::Contract(b) => b.denomination == last.denomination,
                })
                .map(|n| self.first_bid + n)
        } else {
            None // if no one has bid yet, there is no declarer
        }
    }

    pub fn contract_is_final(&self) -> bool {
        self.bids
            .iter()
            .rev()
            .take(3)
            .all(|x| x == &Bid::Auxiliary(AuxiliaryBid::Pass))
    }

    pub fn can_bid(&self, bid: Bid) -> bool {
        if self.contract_is_final() {
            return false;
        }; // after three passes, no one can bid
        match bid {
            Bid::Auxiliary(AuxiliaryBid::Pass) => {
                // everyone can pass as long as the bidding is open
                true
            }
            Bid::Auxiliary(AuxiliaryBid::Double) => {
                // double is possible immediately after a contract bid or after a contract bid and two passes.
                if let Some(Bid::Contract(_)) = self.bids.last() {
                    true
                } else if self.bids.len() >= 3 {
                    let index = self.bids.len() - 3;
                    matches!(
                        &self.bids[index..],
                        [
                            Bid::Contract(_),
                            Bid::Auxiliary(AuxiliaryBid::Pass),
                            Bid::Auxiliary(AuxiliaryBid::Pass)
                        ]
                    )
                } else {
                    false
                }
            }
            Bid::Auxiliary(AuxiliaryBid::Redouble) => {
                // Redouble is possible immediately after a double or after a double and two passes
                if self.bids.last() == Some(&Bid::Auxiliary(AuxiliaryBid::Double)) {
                    true
                } else if self.bids.len() >= 3 {
                    let index = self.bids.len() - 3;
                    matches!(
                        &self.bids[index..],
                        [
                            Bid::Auxiliary(AuxiliaryBid::Double),
                            Bid::Auxiliary(AuxiliaryBid::Pass),
                            Bid::Auxiliary(AuxiliaryBid::Pass)
                        ]
                    )
                } else {
                    false
                }
            }
            Bid::Contract(b) => {
                if let Some(Bid::Contract(last)) = self.bids.iter().filter(|x| matches!(x, Bid::Contract(_))).last() {
                    // a contract bid is possible if it is higher than the last contract bid
                    b > *last
                } else {
                    // no contract implied yet, you can start the bidding
                    true
                }
            }
        }
    }

    // pub fn bids_are_valid(bids: &Bid[]) -> bool {

    //     let checker = BidLine {bids: vec![]};

    // }
}

#[cfg(test)]
mod test {
    use crate::bidder::*;
    use crate::primitives::Suit;

    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("1NT", Bid::Contract(ContractBid{ level: ContractLevel::One, denomination: ContractDenomination::NoTrump}); "No Trump")]
    #[test_case("2S", Bid::Contract(ContractBid{ level: ContractLevel::Two, denomination: ContractDenomination::Trump(Suit::Spades)}); "Spades")]
    #[test_case("7d", Bid::Contract(ContractBid{ level: ContractLevel::Seven, denomination: ContractDenomination::Trump(Suit::Diamonds)}); "Diamonds")]
    #[test_case("4♥",Bid::Contract(ContractBid{ level: ContractLevel::Four, denomination: ContractDenomination::Trump(Suit::Hearts)}); "Hearts")]
    #[test_case("p",Bid::Auxiliary(AuxiliaryBid::Pass); "pass")]
    #[test_case("x",Bid::Auxiliary(AuxiliaryBid::Double); "double")]
    #[test_case("Xx",Bid::Auxiliary(AuxiliaryBid::Redouble); "redouble")]
    fn bid_from_str(str: &str, bid: Bid) {
        assert_eq!(Bid::from_str(str).unwrap(), bid)
    }
}
