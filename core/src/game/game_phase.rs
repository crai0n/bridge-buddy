use crate::error::BBError;
use crate::game::game_event::{BidEvent, CardEvent};
use crate::game::game_state::GameState;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::Contract;

#[derive(Debug, Clone)]
pub enum GamePhase {
    Bidding(GameState),
    OpeningLead(GameState),
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

    pub fn set_up_card_play(&mut self, contract: Contract, declarer: PlayerPosition) {
        match self {
            GamePhase::Bidding(state) => state.set_up_card_play(contract, declarer),
            GamePhase::OpeningLead(_) => unreachable!(),
            GamePhase::WaitingForDummy(_) => unreachable!(),
            GamePhase::CardPlay(_) => unreachable!(),
            GamePhase::Ended(_) => unreachable!(),
        }
    }

    pub fn bidding_has_ended(&self) -> bool {
        match &self {
            GamePhase::Bidding(state) => state.bidding_has_ended(),
            GamePhase::OpeningLead(_) => unreachable!(),
            GamePhase::WaitingForDummy(_) => unreachable!(),
            GamePhase::CardPlay(_) => unreachable!(),
            GamePhase::Ended(_) => unreachable!(),
        }
    }

    pub fn card_play_has_ended(&self) -> bool {
        match &self {
            GamePhase::Bidding(_) => unreachable!(),
            GamePhase::OpeningLead(_) => unreachable!(),
            GamePhase::WaitingForDummy(_) => unreachable!(),
            GamePhase::CardPlay(state) => state.card_play_has_ended(),
            GamePhase::Ended(_) => unreachable!(),
        }
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        match self {
            GamePhase::Bidding(state) => {
                state.process_make_bid_event(bid_event)?;
                if state.bidding_has_ended() {
                    if let Some(contract) = state.bid_manager.implied_contract() {
                        let declarer = state.bid_manager.implied_declarer().unwrap();
                        state.set_up_card_play(contract, declarer);
                        *self = GamePhase::OpeningLead(state.clone());
                    } else {
                        *self = GamePhase::Ended(state.clone());
                    }
                }
                Ok(())
            }
            GamePhase::OpeningLead(_) => unreachable!(),
            GamePhase::WaitingForDummy(_) => unreachable!(),
            GamePhase::CardPlay(_) => unreachable!(),
            GamePhase::Ended(_) => unreachable!(),
        }
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        match self {
            GamePhase::Bidding(_) => unreachable!(),
            GamePhase::OpeningLead(state) => state.process_play_card_event(card_event),
            GamePhase::WaitingForDummy(_) => unreachable!(),
            GamePhase::CardPlay(state) => {
                state.process_play_card_event(card_event)?;
                if state.card_play_has_ended() {
                    *self = GamePhase::Ended(state.clone());
                }
                Ok(())
            }
            GamePhase::Ended(_) => unreachable!(),
        }
    }
}
