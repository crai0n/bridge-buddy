use crate::primitives::bid::Bid;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::Card;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerEvent {
    MakeBid(MakeBidEvent),
    PlayCard(PlayCardEvent),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MakeBidEvent {
    pub player: PlayerPosition,
    pub bid: Bid,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PlayCardEvent {
    pub player: PlayerPosition,
    pub card: Card,
}
