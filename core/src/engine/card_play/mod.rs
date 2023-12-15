use crate::primitives::Card;

pub mod mock;

pub trait Carder {
    fn get_card() -> Card;
}
