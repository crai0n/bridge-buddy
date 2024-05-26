use crate::engine::bidding_engine::SelectBid;
use crate::engine::subjective_game_view::SubjectiveGameDataView;
use crate::game::game_data::BiddingState;
use crate::primitives::bid::{AuxiliaryBid, Bid, ContractBid};
use crate::primitives::contract::{Level, Strain};
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
            0 => Level::One,
            1 => Level::Two,
            2 => Level::Three,
            3 => Level::Four,
            4 => Level::Five,
            5 => Level::Six,
            6 => Level::Seven,
            _ => unreachable!(),
        };
        let strain = match x % 5 {
            0 => Strain::Trump(Suit::Clubs),
            1 => Strain::Trump(Suit::Diamonds),
            2 => Strain::Trump(Suit::Hearts),
            3 => Strain::Trump(Suit::Spades),
            4 => Strain::NoTrump,
            _ => unreachable!(),
        };
        ContractBid { level, strain }
    }

    fn find_bid(&self, data: SubjectiveGameDataView<BiddingState>) -> Bid {
        let my_target_bid = Self::select_random_contract_bid_as_target();

        match data.lowest_available_contract_bid() {
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
    fn select_bid(&self, data: SubjectiveGameDataView<BiddingState>) -> Bid {
        self.find_bid(data)
    }
}
