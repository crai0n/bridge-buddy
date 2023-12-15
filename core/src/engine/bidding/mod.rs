use crate::primitives::bid::Bid;

pub mod mock;

pub trait BidFinder {
    fn find_bid() -> Bid;
}
