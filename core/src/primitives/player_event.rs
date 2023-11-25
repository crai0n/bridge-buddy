use crate::primitives::bid::Bid;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::Card;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerEvent {
    Bid(BidEvent),
    Card(CardEvent),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BidEvent {
    pub player: PlayerPosition,
    pub bid: Bid,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CardEvent {
    pub player: PlayerPosition,
    pub card: Card,
}
