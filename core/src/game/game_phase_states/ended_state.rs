use crate::error::BBError;
use crate::game::game_phase_states::GamePhaseState;
use crate::game::hand_manager::HandManager;

use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_result::GameResult;
use crate::primitives::trick::PlayedTrick;
use crate::primitives::{Contract, Hand};

#[derive(Debug, Clone)]
pub struct EndedState {
    pub bids: BidLine,
    pub tricks: Vec<PlayedTrick>,
    pub hands: HandManager,
    pub result: GameResult,
    pub board: Board,
}

impl GamePhaseState for EndedState {
    fn implied_contract(&self) -> Option<Contract> {
        self.result.played_contract()
    }
    fn dealer(&self) -> Seat {
        self.board.dealer()
    }
}

impl EndedState {
    pub fn hand_of(&self, player: Seat) -> Result<Hand<13>, BBError> {
        self.hands.hand_of(player)
    }

    pub fn declarer(&self) -> Option<Seat> {
        match self.result {
            GameResult::Unplayed => None,
            GameResult::Made {
                contract,
                overtricks: _,
            } => Some(contract.declarer),
            GameResult::Failed {
                contract,
                undertricks: _,
            } => Some(contract.declarer),
        }
    }

    pub fn board(&self) -> Board {
        self.board
    }
}
