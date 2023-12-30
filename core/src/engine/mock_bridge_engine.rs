use crate::engine::bidding_engine::mock_bidding_engine::MockBiddingEngine;
use crate::engine::bidding_engine::SelectBid;
use crate::engine::card_play_engine::mock_card_play_engine::MockCardPlayEngine;
use crate::engine::card_play_engine::SelectCard;
use crate::engine::subjective_game_view::subjectiviser::Subjectiviser;
use crate::engine::subjective_game_view::SubjectiveGameDataView;
use crate::engine::SelectMove;
use crate::error::BBError;
use crate::game::game_data::{Bidding, CardPlay, OpeningLead};

use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::GameEvent;
use crate::primitives::Card;

pub struct MockBridgeEngine {
    bidding_engine: MockBiddingEngine,
    card_play_engine: MockCardPlayEngine,
}

impl MockBridgeEngine {
    pub fn new(seat: Seat) -> Self {
        Self {
            bidding_engine: MockBiddingEngine::new(),
            card_play_engine: MockCardPlayEngine::new(seat),
        }
    }
}

impl SelectBid for MockBridgeEngine {
    fn select_bid(&self, state: SubjectiveGameDataView<Bidding>) -> Bid {
        self.bidding_engine.select_bid(state)
    }
}

impl SelectCard for MockBridgeEngine {
    fn select_card(&self, state: SubjectiveGameDataView<CardPlay>) -> Card {
        self.card_play_engine.select_card(state)
    }

    fn select_opening_lead(&self, state: SubjectiveGameDataView<OpeningLead>) -> Card {
        self.card_play_engine.select_opening_lead(state)
    }
}

impl SelectMove for MockBridgeEngine {
    fn process_game_event(&mut self, _event: GameEvent, _subjectiviser: Subjectiviser) -> Result<(), BBError> {
        Ok(())
    }
}
