use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::Card;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerEvent {
    Bid(BidEvent),
    Card(CardEvent),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BidEvent {
    pub player: Seat,
    pub bid: Bid,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CardEvent {
    pub player: Seat,
    pub card: Card,
}
