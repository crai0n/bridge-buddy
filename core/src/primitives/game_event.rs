use crate::primitives::deal::{Board, Seat};
use crate::primitives::player_event::PlayerEvent;
use crate::primitives::{Contract, Deal, Hand};
use crate::scoring::ScorePoints;

pub use crate::primitives::player_event::{BidEvent, CardEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEvent {
    NewGame(NewGameEvent),
    DiscloseHand(DiscloseHandEvent),
    Bid(BidEvent),
    MoveToCardPlay(MoveToCardPlayEvent),
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
    pub hand: Hand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToCardPlayEvent {
    pub final_contract: Contract,
    pub declarer: Seat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DummyUncoveredEvent {
    pub dummy: Hand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameEndedEvent {
    pub deal: Deal,
    pub score: ScorePoints,
}

impl From<PlayerEvent> for GameEvent {
    fn from(player_event: PlayerEvent) -> Self {
        match player_event {
            PlayerEvent::Bid(event) => GameEvent::Bid(BidEvent {
                player: event.player,
                bid: event.bid,
            }),
            PlayerEvent::Card(event) => GameEvent::Card(CardEvent {
                player: event.player,
                card: event.card,
            }),
        }
    }
}
