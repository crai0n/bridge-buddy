use crate::error::BBError;
use crate::game::game_state::{CardPlay, GameState};
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_event::DummyUncoveredEvent;
use crate::primitives::{Contract, Hand};

#[derive(Debug, Clone)]
pub struct WaitingForDummy {
    pub bids: BidLine,
    pub trick_manager: TrickManager,
    pub hand_manager: HandManager,
    pub contract: Contract,
    pub board: Board,
}

impl GameState<WaitingForDummy> {
    pub fn next_to_play(&self) -> Seat {
        self.inner.trick_manager.next_to_play()
    }

    pub fn board(&self) -> Board {
        self.inner.board
    }

    pub fn validate_turn_order(&self, player: Seat) -> Result<(), BBError> {
        let turn = self.next_to_play();
        if player != turn {
            return Err(BBError::OutOfTurn(Some(turn)));
        }
        Ok(())
    }

    pub fn hand_of(&self, player: Seat) -> Result<Hand, BBError> {
        self.inner.hand_manager.hand_of(player)
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
            board: self.inner.board,
        };

        GameState { inner }
    }
}
