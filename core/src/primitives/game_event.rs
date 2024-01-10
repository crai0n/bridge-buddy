use crate::game::scoring::ScorePoints;
use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_result::GameResult;
use crate::primitives::player_event::PlayerEvent;
use crate::primitives::{Contract, Deal, Hand};

pub use crate::primitives::player_event::{BidEvent, CardEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEvent {
    NewGame(NewGameEvent),
    DiscloseHand(DiscloseHandEvent),
    Bid(BidEvent),
    BiddingEnded(BiddingEndedEvent),
    Card(CardEvent),
    DummyUncovered(DummyUncoveredEvent),
    GameEnded(GameEndedEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewGameEvent {
    pub board: Board,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiscloseHandEvent {
    pub seat: Seat,
    pub hand: Hand<13>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BiddingEndedEvent {
    pub final_contract: Contract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DummyUncoveredEvent {
    pub dummy: Hand<13>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameEndedEvent {
    pub deal: Deal<13>,
    pub result: GameResult,
    pub score: ScorePoints,
}

impl From<PlayerEvent> for GameEvent {
    fn from(player_event: PlayerEvent) -> Self {
        match player_event {
            PlayerEvent::Bid(event) => GameEvent::Bid(event),
            PlayerEvent::Card(event) => GameEvent::Card(event),
        }
    }
}
