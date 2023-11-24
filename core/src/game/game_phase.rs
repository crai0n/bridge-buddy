use crate::error::BBError;
use crate::game::game_event::{BidEvent, CardEvent, DiscloseHandEvent, DummyUncoveredEvent, GameEvent};
use crate::game::game_state::{Bidding, GameState, NewGameState, OpeningLead};
use crate::primitives::deal::PlayerPosition;

#[derive(Debug, Clone)]
pub enum GamePhase {
    Bidding(NewGameState<Bidding>),
    OpeningLead(NewGameState<OpeningLead>),
    WaitingForDummy(GameState),
    CardPlay(GameState),
    Ended(GameState),
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

    fn move_from_bidding_to_next_phase_with_state(&mut self, state: NewGameState<Bidding>) {
        if let Some(contract) = state.inner.bid_manager.implied_contract() {
            let declarer = state.inner.bid_manager.implied_declarer().unwrap();

            let new_state = state.clone().set_up_card_play(contract, declarer);

            *self = GamePhase::OpeningLead(new_state);
        } else {
            let new_state = GameState {
                bid_manager: state.inner.bid_manager,
                tricks: state.inner.tricks,
                hands: Some(state.inner.hands),
                contract: state.inner.contract,
                declarer: state.inner.declarer,
            };

            *self = GamePhase::Ended(new_state);
        }
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        match self {
            GamePhase::OpeningLead(state) => {
                state.process_play_card_event(card_event)?;

                let state = state.clone();

                let new_state = GameState {
                    bid_manager: state.inner.bid_manager,
                    tricks: Some(state.inner.tricks),
                    hands: Some(state.inner.hands),
                    contract: Some(state.inner.contract),
                    declarer: Some(state.inner.declarer),
                };

                *self = GamePhase::WaitingForDummy(new_state);
                Ok(())
            }
            GamePhase::CardPlay(state) => {
                state.process_play_card_event(card_event)?;
                if state.card_play_has_ended() {
                    *self = GamePhase::Ended(state.clone());
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
                *self = GamePhase::CardPlay(state.clone());
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
