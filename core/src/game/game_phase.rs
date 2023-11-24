use crate::error::BBError;
use crate::game::game_state::{Bidding, CardPlay, Ended, GameState, OpeningLead, WaitingForDummy};
use crate::primitives::deal::PlayerPosition;
use crate::primitives::game_event::{BidEvent, CardEvent, DiscloseHandEvent, DummyUncoveredEvent, GameEvent};

#[derive(Debug, Clone)]
pub enum GamePhase {
    Bidding(GameState<Bidding>),
    OpeningLead(GameState<OpeningLead>),
    WaitingForDummy(GameState<WaitingForDummy>),
    CardPlay(GameState<CardPlay>),
    Ended(GameState<Ended>),
}

impl GamePhase {
    pub fn next_to_play(&self) -> Option<PlayerPosition> {
        match &self {
            GamePhase::Bidding(state) => state.next_to_play(),
            GamePhase::OpeningLead(state) => state.next_to_play(),
            GamePhase::WaitingForDummy(state) => state.next_to_play(),
            GamePhase::CardPlay(state) => state.next_to_play(),
            GamePhase::Ended(_) => None,
        }
    }

    pub fn validate_turn_order(&self, player: PlayerPosition) -> Result<(), BBError> {
        match &self {
            GamePhase::Bidding(state) => state.validate_turn_order(player),
            GamePhase::OpeningLead(state) => state.validate_turn_order(player),
            GamePhase::WaitingForDummy(state) => state.validate_turn_order(player),
            GamePhase::CardPlay(state) => state.validate_turn_order(player),
            GamePhase::Ended(_) => Err(BBError::GameHasEnded),
        }
    }

    pub fn process_event(&mut self, event: GameEvent) -> Result<(), BBError> {
        match event {
            GameEvent::NewGame(_) => Err(BBError::GameAlreadyStarted),
            GameEvent::DiscloseHand(disclose_hand_event) => self.process_disclose_hand_event(disclose_hand_event),
            GameEvent::Bid(bid_event) => self.process_make_bid_event(bid_event),
            GameEvent::Card(card_event) => self.process_play_card_event(card_event),
            GameEvent::DummyUncovered(dummy_uncovered_event) => {
                self.process_dummy_uncovered_event(dummy_uncovered_event)
            }
            _ => Err(BBError::InvalidEvent(event)),
        }
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        match self {
            GamePhase::Bidding(state) => {
                state.process_make_bid_event(bid_event)?;
                if state.bidding_has_ended() {
                    let new_state = state.clone();
                    self.move_from_bidding_to_next_phase_with_state(new_state);
                }
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(GameEvent::Bid(bid_event))),
        }
    }

    fn move_from_bidding_to_next_phase_with_state(&mut self, state: GameState<Bidding>) {
        if let Some(contract) = state.inner.bid_manager.implied_contract() {
            let new_state = state.clone().move_to_opening_lead(contract);

            *self = GamePhase::OpeningLead(new_state);
        } else {
            let new_state = state.clone().move_to_ended_without_card_play();

            *self = GamePhase::Ended(new_state);
        }
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        match self {
            GamePhase::OpeningLead(state) => {
                state.process_play_card_event(card_event)?;

                let state = state.clone();

                let inner = WaitingForDummy {
                    bids: state.inner.bids,
                    trick_manager: state.inner.trick_manager,
                    hand_manager: state.inner.hand_manager,
                    contract: state.inner.contract,
                };

                let new_state = GameState { inner };
                *self = GamePhase::WaitingForDummy(new_state);
                Ok(())
            }
            GamePhase::CardPlay(state) => {
                state.process_play_card_event(card_event)?;
                if state.card_play_has_ended() {
                    let new_state = state.clone().move_from_card_play_to_ended();
                    *self = GamePhase::Ended(new_state);
                }
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(GameEvent::Card(card_event))),
        }
    }

    fn process_dummy_uncovered_event(&mut self, event: DummyUncoveredEvent) -> Result<(), BBError> {
        match self {
            GamePhase::WaitingForDummy(state) => {
                state.process_dummy_uncovered_event(event)?;

                let state = state.clone();

                let new_state = state.move_to_card_play();

                *self = GamePhase::CardPlay(new_state);

                Ok(())
            }
            _ => Err(BBError::InvalidEvent(GameEvent::DummyUncovered(event)))?,
        }
    }

    fn process_disclose_hand_event(&mut self, event: DiscloseHandEvent) -> Result<(), BBError> {
        match self {
            GamePhase::Bidding(state) => state.process_disclose_hand_event(event),
            _ => Err(BBError::InvalidEvent(GameEvent::DiscloseHand(event))),
        }
    }
}
