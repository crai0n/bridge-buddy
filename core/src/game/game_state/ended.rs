use crate::error::BBError;
use crate::game::game_state::GameState;
use crate::game::hand_manager::HandManager;
use crate::game::scoring::{ScoreCalculator, ScorePoints};
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_result::GameResult;
use crate::primitives::trick::PlayedTrick;
use crate::primitives::Hand;

#[derive(Debug, Clone)]
pub struct Ended {
    pub bids: BidLine,
    pub tricks: Vec<PlayedTrick>,
    pub hands: HandManager,
    pub result: GameResult,
    pub board: Board,
}

impl GameState<Ended> {
    pub fn hand_of(&self, player: Seat) -> Result<Hand, BBError> {
        self.inner.hands.hand_of(player)
    }

    pub fn calculate_score(&self) -> ScorePoints {
        ScoreCalculator::score_result(self.inner.result, self.inner.board.vulnerability())
    }

    pub fn declarer(&self) -> Option<Seat> {
        match self.inner.result {
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
        self.inner.board
    }
}
