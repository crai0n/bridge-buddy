use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::Suit;
use std::cmp::{max, min};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct TranspositionTable {
    inner: BTreeMap<TTKey, TTValue>,
}

impl TranspositionTable {
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn lookup(&self, key: &TTKey) -> Option<&TTValue> {
        self.inner.get(key)
    }

    pub fn update_upper_bound(&mut self, key: &TTKey, bound: usize) {
        let new = match self.inner.get(key) {
            None => TTValue {
                at_least: 0,
                at_most: bound,
            },
            Some(old) => TTValue {
                at_least: old.at_least,
                at_most: min(bound, old.at_most),
            },
        };
        self.inner.insert(*key, new);
    }

    pub fn update_lower_bound(&mut self, key: &TTKey, bound: usize) {
        let new = match self.inner.get(key) {
            None => TTValue {
                at_least: bound,
                at_most: key.tricks_left,
            },
            Some(old) => TTValue {
                at_least: max(bound, old.at_least),
                at_most: old.at_most,
            },
        };
        self.inner.insert(*key, new);
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct TTKey {
    pub tricks_left: usize,
    pub trumps: Option<Suit>,
    pub lead: Seat,
    pub remaining_cards: [u32; 4],
}

#[derive(Clone, Copy)]
pub struct TTValue {
    pub at_least: usize,
    pub at_most: usize,
}
