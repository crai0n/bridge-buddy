use crate::engine::bidding::mock::MockBiddingEngine;
use crate::engine::bidding::BidFinder;
use crate::engine::card_play::mock::MockCardPlayEngine;
use crate::engine::card_play::CardFinder;
use crate::engine::MoveFinder;
use crate::game::game_state::{Bidding, CardPlay, GameState, OpeningLead};

use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::Card;

pub struct AutoMoveFinder {
    seat: Seat,
    bidding_engine: MockBiddingEngine,
    card_play_engine: MockCardPlayEngine,
}

impl AutoMoveFinder {
    pub fn new(seat: Seat) -> Self {
        Self {
            seat,
            bidding_engine: MockBiddingEngine::new(),
            card_play_engine: MockCardPlayEngine::new(seat),
        }
    }
}

impl MoveFinder for AutoMoveFinder {
    fn find_bid(&self, game_state: &GameState<Bidding>) -> Bid {
        self.bidding_engine.find_bid(game_state)
    }

    fn pick_opening_lead(&self, game_state: &GameState<OpeningLead>) -> Card {
        self.card_play_engine.find_opening_lead(game_state)
    }

    fn pick_card_for(&self, game_state: &GameState<CardPlay>, seat: Seat) -> Card {
        self.card_play_engine.find_card_for(game_state, seat)
    }

    fn seat(&self) -> Seat {
        self.seat
    }
}
