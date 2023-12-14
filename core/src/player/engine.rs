use crate::game::game_state::{Bidding, CardPlay, GameState, OpeningLead};
use crate::primitives::bid::{AuxiliaryBid, Bid, ContractBid};
use crate::primitives::contract::{ContractDenomination, ContractLevel};
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Suit};
use rand::Rng;
use std::cmp::Ordering;

pub enum SubjectiveVulnerability {
    None,
    Us,
    Them,
    All,
}

pub struct MockBiddingEngine {}

pub struct MockCardPlayEngine {
    seat: Seat,
}

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

    pub fn find_bid(&self, state: &GameState<Bidding>) -> Bid {
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

impl MockCardPlayEngine {
    pub fn new(seat: Seat) -> Self {
        Self { seat }
    }

    pub fn pick_opening_lead(&self, state: &GameState<OpeningLead>) -> Card {
        let hand = state.inner.hand_manager.known_remaining_cards_of(self.seat);
        let card = hand.first().unwrap();
        *card
    }

    pub fn pick_card_for(&self, state: &GameState<CardPlay>, seat: Seat) -> Card {
        match state.inner.trick_manager.suit_to_follow() {
            None => self.pick_lead_for(state, seat),
            Some(suit) => self.pick_discard_for(suit, state, seat),
        }
    }

    fn pick_lead_for(&self, state: &GameState<CardPlay>, seat: Seat) -> Card {
        let remaining_cards = state.inner.hand_manager.known_remaining_cards_of(seat);
        let card = remaining_cards.first().unwrap();
        *card
    }

    fn pick_discard_for(&self, suit: Suit, state: &GameState<CardPlay>, seat: Seat) -> Card {
        let remaining_cards = state.inner.hand_manager.known_remaining_cards_of(seat);
        if let Some(card) = remaining_cards.iter().find(|x| x.suit == suit) {
            *card
        } else {
            *remaining_cards.first().unwrap()
        }
    }
}

impl Default for MockCardPlayEngine {
    fn default() -> Self {
        Self::new(Seat::South)
    }
}
