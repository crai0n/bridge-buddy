use crate::deal::PlayerPosition;

use crate::card::Suit;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum ContractLevel {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum ContractState {
    Passed,
    Doubled,
    Redoubled,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum ContractDenomination {
    Trump(Suit),
    NoTrump,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Contract {
    level: ContractLevel,
    denomination: ContractDenomination,
    state: ContractState,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractBid {
    level: ContractLevel,
    denomination: ContractDenomination,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AuxiliaryBid {
    Pass,
    Double,
    Redouble,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Bid {
    Contract(ContractBid),
    Auxiliary(AuxiliaryBid),
}

pub struct BidLine {
    bids: Vec<Bid>,
    dealer: PlayerPosition,
}

impl BidLine {
    pub fn implied_contract(&self) -> Option<Contract> {
        let mut state = ContractState::Passed;
        for bid in self.bids.iter().rev() {
            match bid {
                Bid::Auxiliary(AuxiliaryBid::Pass) => continue,
                Bid::Auxiliary(AuxiliaryBid::Double) => {
                    if state == ContractState::Passed {
                        state = ContractState::Doubled;
                    } // FIXME!
                    continue;
                }
                Bid::Auxiliary(AuxiliaryBid::Redouble) => {
                    state = ContractState::Redoubled;
                    continue;
                }
                Bid::Contract(a) => {
                    let level = a.level;
                    let denomination = a.denomination;
                    return Some(Contract {
                        level,
                        denomination,
                        state,
                    });
                }
            }
        }
        None // if all players pass, there is no contract
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
                .map(|n| self.dealer + n)
        } else {
            None // if all players pass, there is no declarer
        }
    }

    pub fn is_final(&self) -> bool {
        self.bids
            .iter()
            .rev()
            .take(3)
            .all(|x| x == &Bid::Auxiliary(AuxiliaryBid::Pass))
    }

    pub fn can_bid(&self, bid: Bid) -> bool {
        if self.is_final() {
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
                    return true;
                }

                if self.bids.len() >= 3 {
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
                    return true;
                }
                if self.bids.len() >= 3 {
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
                // a contract bid is possible if it is higher than the currently implied contract
                if let Some(implied_contract) = self.implied_contract() {
                    // there is an implied contract, so compare
                    b.level > implied_contract.level || b.denomination > implied_contract.denomination
                } else {
                    // no contract implied yet, you can start the bidding
                    true
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bidder::*;
    use std::cmp::Ordering;

    #[test]
    fn contract_ordering_spades_notrump() {
        let level = ContractLevel::One;
        let denomination = ContractDenomination::Trump(Suit::Spades);
        let state = ContractState::Doubled;
        let c1 = Contract {
            level,
            denomination,
            state,
        };
        let level = ContractLevel::One;
        let denomination = ContractDenomination::NoTrump;
        let state = ContractState::Passed;
        let c2 = Contract {
            level,
            denomination,
            state,
        };
        assert_eq!(c1.cmp(&c2), Ordering::Less)
    }

    #[test]
    fn contract_ordering_hearts_spades() {
        let level = ContractLevel::One;
        let denomination = ContractDenomination::Trump(Suit::Hearts);
        let state = ContractState::Doubled;
        let c1 = Contract {
            level,
            denomination,
            state,
        };
        let level = ContractLevel::One;
        let denomination = ContractDenomination::Trump(Suit::Spades);
        let state = ContractState::Passed;
        let c2 = Contract {
            level,
            denomination,
            state,
        };
        assert_eq!(c1.cmp(&c2), Ordering::Less)
    }
}
