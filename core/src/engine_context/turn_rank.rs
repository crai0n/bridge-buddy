use crate::primitives::deal::{Board, PlayerPosition};

#[derive(Copy, Clone)]
pub enum TurnRank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
}

impl TurnRank {
    pub fn turn_rank_for_board(board: &Board, my_seat: PlayerPosition) -> TurnRank {
        match my_seat - board.dealer() {
            0 => TurnRank::First,
            1 => TurnRank::Second,
            2 => TurnRank::Third,
            3 => TurnRank::Fourth,
            _ => unreachable!(),
        }
    }
}
