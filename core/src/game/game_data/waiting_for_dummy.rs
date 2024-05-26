use crate::error::BBError;
use crate::game::game_data::{CardPlayState, GameData, NextToPlay};
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

impl NextToPlay for GameData<WaitingForDummyState> {
    fn next_to_play(&self) -> Seat {
        self.inner.trick_manager.next_to_play()
    }
}

impl GameData<WaitingForDummyState> {
    pub fn board(&self) -> Board {
        self.inner.board()
    }

    pub fn declarer(&self) -> Seat {
        self.inner.declarer()
    }

    pub fn dummy(&self) -> Seat {
        self.inner.dummy()
    }

    pub fn hand_of(&self, player: Seat) -> Result<Hand<13>, BBError> {
        self.inner.hand_of(player)
    }

    pub fn process_dummy_uncovered_event(&mut self, event: DummyUncoveredEvent) -> Result<(), BBError> {
        self.inner.process_dummy_uncovered_event(event)
    }

    pub fn move_to_card_play(self) -> GameData<CardPlayState> {
        self.inner.move_to_card_play()
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

    pub fn move_to_card_play(self) -> GameData<CardPlayState> {
        let inner = CardPlayState {
            bids: self.bids,
            trick_manager: self.trick_manager,
            hand_manager: self.hand_manager,
            contract: self.contract,
            board: self.board,
        };

        GameData { inner }
    }
}
