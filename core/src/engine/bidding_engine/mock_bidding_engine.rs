use crate::engine::bidding_engine::SelectBid;
use crate::game::game_state::{Bidding, GameState};
use crate::primitives::bid::{AuxiliaryBid, Bid, ContractBid};
use crate::primitives::contract::{ContractDenomination, ContractLevel};
use crate::primitives::Suit;
use rand::Rng;
use std::cmp::Ordering;

pub struct MockBiddingEngine {}

impl MockBiddingEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn select_random_contract_bid_as_target() -> ContractBid {
        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(0..35);
        let level = match x / 5 {
            0 => ContractLevel::One,
            1 => ContractLevel::Two,
            2 => ContractLevel::Three,
            3 => ContractLevel::Four,
            4 => ContractLevel::Five,
            5 => ContractLevel::Six,
            6 => ContractLevel::Seven,
            _ => unreachable!(),
        };
        let denomination = match x % 5 {
            0 => ContractDenomination::Trump(Suit::Clubs),
            1 => ContractDenomination::Trump(Suit::Diamonds),
            2 => ContractDenomination::Trump(Suit::Hearts),
            3 => ContractDenomination::Trump(Suit::Spades),
            4 => ContractDenomination::NoTrump,
            _ => unreachable!(),
        };
        ContractBid { level, denomination }
    }

    fn find_bid(&self, state: &GameState<Bidding>) -> Bid {
        let my_target_bid = Self::select_random_contract_bid_as_target();

        match state.inner.bid_manager.lowest_available_contract_bid() {
            Some(bid) => match my_target_bid.cmp(&bid) {
                Ordering::Less => Bid::Auxiliary(AuxiliaryBid::Pass),
                _ => Bid::Contract(bid),
            },
            None => Bid::Auxiliary(AuxiliaryBid::Pass),
        }
    }
}

impl Default for MockBiddingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectBid for MockBiddingEngine {
    fn select_bid(&self, state: &GameState<Bidding>) -> Bid {
        self.find_bid(state)
    }
}
