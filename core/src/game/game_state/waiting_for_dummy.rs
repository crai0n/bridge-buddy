use crate::error::BBError;
use crate::game::game_state::{CardPlay, GameState};
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::game_event::DummyUncoveredEvent;
use crate::primitives::Contract;

#[derive(Debug, Clone)]
pub struct WaitingForDummy {
    pub bids: BidLine,
    pub trick_manager: TrickManager,
    pub hand_manager: HandManager,
    pub contract: Contract,
}

impl GameState<WaitingForDummy> {
    pub fn next_to_play(&self) -> Option<PlayerPosition> {
        Some(self.inner.trick_manager.next_to_play())
    }

    pub fn validate_turn_order(&self, player: PlayerPosition) -> Result<(), BBError> {
        if let Some(next_to_play) = self.next_to_play() {
            if player == next_to_play {
                return Ok(());
            }
        }
        Err(BBError::OutOfTurn(self.next_to_play()))
    }

    pub fn process_dummy_uncovered_event(&mut self, event: DummyUncoveredEvent) -> Result<(), BBError> {
        self.inner
            .hand_manager
            .register_known_hand(event.dummy, self.inner.contract.declarer.partner())?;

        Ok(())
    }

    pub fn move_to_card_play(self) -> GameState<CardPlay> {
        let inner = CardPlay {
            bids: self.inner.bids,
            trick_manager: self.inner.trick_manager,
            hand_manager: self.inner.hand_manager,
            contract: self.inner.contract,
        };

        GameState { inner }
    }
}
