use crate::primitives::bid::Bid;

pub mod mock;

pub trait Bidder {
    fn get_bid() -> Bid;
}
