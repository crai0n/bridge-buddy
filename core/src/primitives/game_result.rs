use crate::primitives::Contract;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameResult {
    Made { contract: Contract, overtricks: usize },
    Failed { contract: Contract, undertricks: usize },
    Unplayed,
}

impl GameResult {
    pub fn calculate_game_result(contract: Contract, actual_tricks: usize) -> GameResult {
        let exp = contract.expected_tricks();
        let act = actual_tricks;
        match exp.cmp(&act) {
            Ordering::Equal => GameResult::Made {
                contract,
                overtricks: 0,
            },
            Ordering::Greater => GameResult::Failed {
                contract,
                undertricks: exp - act,
            },
            Ordering::Less => GameResult::Made {
                contract,
                overtricks: act - exp,
            },
        }
    }
}
