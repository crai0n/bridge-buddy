use crate::game::card_manager::CardManager;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::Contract;

pub struct GameState<Phase> {
    state: Phase,
}

pub struct Bidding {
    bid_line: BidLine,
}

pub struct CardPlay {
    bid_line: BidLine,
    tricks: CardManager,
    contract: Contract,
    declarer: PlayerPosition,
}

pub struct Ended {
    bid_line: BidLine,
    tricks: CardManager,
    contract: Option<Contract>,
    declarer: Option<PlayerPosition>,
}
