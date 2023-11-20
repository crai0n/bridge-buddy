use crate::game::player_event::PlayerEvent;
use crate::primitives::bid::Bid;
use crate::primitives::deal::{Board, PlayerPosition};
use crate::primitives::{Card, Contract, Hand};
use crate::score::ScorePoints;

#[derive(Debug)]
pub enum GameEvent {
    DiscloseHand(DiscloseHandEvent),
    GameStarted,
    BidMade(BidMadeEvent),
    MoveToCardPlay(MoveToCardPlayEvent),
    DummyUncovered(DummyUncoveredEvent),
    CardPlayed(CardPlayedEvent),
    GameEnded(GameEndedEvent),
}

#[derive(Debug)]
pub struct DiscloseHandEvent {
    pub board: Board,
    pub seat: PlayerPosition,
    pub hand: Hand,
}

#[derive(Debug)]
pub struct BidMadeEvent {
    pub player: PlayerPosition,
    pub bid: Bid,
}

#[derive(Debug)]
pub struct MoveToCardPlayEvent {
    pub final_contract: Contract,
    pub declarer: PlayerPosition,
}

#[derive(Debug)]
pub struct CardPlayedEvent {
    pub player: PlayerPosition,
    pub card: Card,
}

#[derive(Debug)]
pub struct DummyUncoveredEvent {
    pub dummy: Hand,
}

#[derive(Debug)]
pub struct GameEndedEvent {
    pub score: ScorePoints,
}

impl From<PlayerEvent> for GameEvent {
    fn from(player_event: PlayerEvent) -> Self {
        match player_event {
            PlayerEvent::MakeBid(event) => GameEvent::BidMade(BidMadeEvent {
                player: event.player,
                bid: event.bid,
            }),
            PlayerEvent::PlayCard(event) => GameEvent::CardPlayed(CardPlayedEvent {
                player: event.player,
                card: event.card,
            }),
        }
    }
}
