use crate::error::BBError;
use crate::game::game_state::GameState;
use crate::game::hand_manager::HandManager;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::{Board, PlayerPosition, Vulnerability};
use crate::primitives::game_result::GameResult;
use crate::primitives::trick::PlayedTrick;
use crate::primitives::Hand;
use crate::score::{Score, ScorePoints};

#[derive(Debug, Clone)]
pub struct Ended {
    pub bids: BidLine,
    pub tricks: Vec<PlayedTrick>,
    pub hands: HandManager,
    pub result: GameResult,
    pub board: Board,
}

impl GameState<Ended> {
    pub fn tricks_won_by_axis(&self, player: PlayerPosition) -> usize {
        self.inner
            .tricks
            .iter()
            .filter(|x| x.winner() == player || x.winner() == player.partner())
            .count()
    }

    pub fn hand_of(&self, player: PlayerPosition) -> Result<Hand, BBError> {
        self.inner.hands.hand_of(player)
    }

    pub fn calculate_score(&self, vulnerability: Vulnerability) -> ScorePoints {
        Score::score_result(self.inner.result, vulnerability)
    }

    pub fn board(&self) -> Board {
        self.inner.board
    }
}
