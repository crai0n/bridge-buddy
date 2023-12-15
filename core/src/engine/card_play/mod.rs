use crate::primitives::Card;

pub mod mock;

pub trait CardFinder {
    fn find_card() -> Card;
}
