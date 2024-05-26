use crate::error::BBError;
use crate::game::game_phase_states::{CardPlayState, NextToPlay};
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::game::GamePhaseState;

use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_event::DummyUncoveredEvent;
use crate::primitives::{Contract, Hand};

#[derive(Debug, Clone)]
pub struct WaitingForDummyState {
    pub bids: BidLine,
    pub trick_manager: TrickManager<13>,
    pub hand_manager: HandManager,
    pub contract: Contract,
    pub board: Board,
}

impl GamePhaseState for WaitingForDummyState {
    fn implied_contract(&self) -> Option<Contract> {
        Some(self.contract)
    }
}

impl NextToPlay for WaitingForDummyState {
    fn next_to_play(&self) -> Seat {
        self.trick_manager.next_to_play()
    }
}

impl WaitingForDummyState {
    pub fn board(&self) -> Board {
        self.board
    }

    pub fn declarer(&self) -> Seat {
        self.contract.declarer
    }

    pub fn dummy(&self) -> Seat {
        self.declarer().partner()
    }

    pub fn hand_of(&self, player: Seat) -> Result<Hand<13>, BBError> {
        self.hand_manager.hand_of(player)
    }

    pub fn process_dummy_uncovered_event(&mut self, event: DummyUncoveredEvent) -> Result<(), BBError> {
        self.hand_manager
            .register_known_hand(event.dummy, self.contract.declarer.partner())?;

        Ok(())
    }

    pub fn move_to_card_play(self) -> CardPlayState {
        CardPlayState {
            bids: self.bids,
            trick_manager: self.trick_manager,
            hand_manager: self.hand_manager,
            contract: self.contract,
            board: self.board,
        }
    }
}
