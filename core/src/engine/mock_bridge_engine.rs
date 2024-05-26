use crate::engine::bidding_engine::mock_bidding_engine::MockBiddingEngine;
use crate::engine::bidding_engine::SelectBid;
use crate::engine::card_play_engine::mock_card_play_engine::MockCardPlayEngine;
use crate::engine::card_play_engine::SelectCard;
use crate::engine::engine_state::EngineState;
use crate::engine::subjective_game_view::{SubjectiveGamePhaseStateView, SubjectiveGameStateView};
use crate::engine::SelectMove;
use crate::error::BBError;
use crate::game::game_phase_states::{BiddingState, CardPlayState, OpeningLeadState};

use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::{
    BidEvent, BiddingEndedEvent, CardEvent, DiscloseHandEvent, DummyUncoveredEvent, GameEndedEvent, GameEvent,
    NewGameEvent,
};
use crate::primitives::Card;

pub struct MockBridgeEngine {
    bidding_engine: MockBiddingEngine,
    card_play_engine: MockCardPlayEngine,
    _engine_state: EngineState,
}

impl MockBridgeEngine {
    pub fn new(seat: Seat) -> Self {
        Self {
            bidding_engine: MockBiddingEngine::new(),
            card_play_engine: MockCardPlayEngine::new(seat),
            _engine_state: EngineState::empty(),
        }
    }
}

impl SelectBid for MockBridgeEngine {
    fn select_bid(&self, state: SubjectiveGamePhaseStateView<BiddingState>) -> Bid {
        self.bidding_engine.select_bid(state)
    }
}

impl SelectCard for MockBridgeEngine {
    fn select_card(&self, state: SubjectiveGamePhaseStateView<CardPlayState>) -> Card {
        self.card_play_engine.select_card(state)
    }

    fn select_opening_lead(&self, state: SubjectiveGamePhaseStateView<OpeningLeadState>) -> Card {
        self.card_play_engine.select_opening_lead(state)
    }
}

impl SelectMove for MockBridgeEngine {
    fn process_game_event(&mut self, event: GameEvent, game_state: SubjectiveGameStateView) -> Result<(), BBError> {
        match event {
            GameEvent::NewGame(event) => self.process_new_game_event(event, game_state),
            GameEvent::DiscloseHand(event) => self.process_disclose_hand_event(event, game_state),
            GameEvent::Bid(event) => self.process_bid_event(event, game_state),
            GameEvent::BiddingEnded(event) => self.process_bidding_ended_event(event, game_state),
            GameEvent::Card(event) => self.process_card_event(event, game_state),
            GameEvent::DummyUncovered(event) => self.process_dummy_uncovered_event(event, game_state),
            GameEvent::GameEnded(event) => self.process_game_ended_event(event, game_state),
        }
    }
}

impl MockBridgeEngine {
    fn process_new_game_event(
        &mut self,
        _event: NewGameEvent,
        _game_state: SubjectiveGameStateView,
    ) -> Result<(), BBError> {
        Err(BBError::GameAlreadyStarted)
    }

    fn process_disclose_hand_event(
        &mut self,
        _event: DiscloseHandEvent,
        _game_state: SubjectiveGameStateView,
    ) -> Result<(), BBError> {
        Ok(())
    }

    fn process_bid_event(&mut self, event: BidEvent, game_state: SubjectiveGameStateView) -> Result<(), BBError> {
        match game_state {
            SubjectiveGameStateView::Bidding(state) => self.interpret_bid(event, state),
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::Bid(event)))),
        }
    }

    fn interpret_bid(
        &mut self,
        _event: BidEvent,
        _data: SubjectiveGamePhaseStateView<BiddingState>,
    ) -> Result<(), BBError> {
        // let player = data.next_to_play();
        Ok(())
    }

    fn process_bidding_ended_event(
        &mut self,
        _event: BiddingEndedEvent,
        _game_state: SubjectiveGameStateView,
    ) -> Result<(), BBError> {
        Ok(())
    }

    fn process_card_event(&mut self, event: CardEvent, game_state: SubjectiveGameStateView) -> Result<(), BBError> {
        match game_state {
            SubjectiveGameStateView::OpeningLead(state) => self.interpret_opening_lead(event, state),
            SubjectiveGameStateView::CardPlay(state) => self.interpret_card(event, state),
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::Card(event)))),
        }
    }

    fn interpret_opening_lead(
        &mut self,
        _event: CardEvent,
        _data: SubjectiveGamePhaseStateView<OpeningLeadState>,
    ) -> Result<(), BBError> {
        Ok(())
    }

    fn interpret_card(
        &mut self,
        _event: CardEvent,
        _data: SubjectiveGamePhaseStateView<CardPlayState>,
    ) -> Result<(), BBError> {
        Ok(())
    }

    fn process_dummy_uncovered_event(
        &mut self,
        _event: DummyUncoveredEvent,
        _state: SubjectiveGameStateView,
    ) -> Result<(), BBError> {
        Ok(())
    }

    fn process_game_ended_event(
        &mut self,
        _event: GameEndedEvent,
        _game_state: SubjectiveGameStateView,
    ) -> Result<(), BBError> {
        Ok(())
    }
}
