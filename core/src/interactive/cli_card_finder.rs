use crate::engine::card_play::CardFinder;
use crate::primitives::deal::Seat;
use crate::primitives::Card;

#[allow(dead_code)]
pub struct CliCardFinder {
    seat: Seat,
}

impl CardFinder for CliCardFinder {
    fn find_card() -> Card {
        todo!()
    }
}

impl CliCardFinder {
    pub fn new(seat: Seat) -> Self {
        CliCardFinder { seat }
    }
}
