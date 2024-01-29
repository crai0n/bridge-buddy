use bridge_buddy_core::primitives::deal::Seat;
use std::cmp::{max, min};
use std::collections::BTreeMap;

#[allow(dead_code)]
pub struct TranspositionTable {
    inner: BTreeMap<TTKey, TTValue>,
}

#[allow(dead_code)]
impl TranspositionTable {
    pub fn new() -> Self {
        Self { inner: BTreeMap::new() }
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn lookup(&self, key: &TTKey) -> Option<&TTValue> {
        self.inner.get(key)
    }

    pub fn update(&mut self, key: &TTKey, value: TTValue) {
        match self.inner.get(key) {
            None => self.inner.insert(*key, value),
            Some(old) => {
                let highest_lower_bound = max(value.at_least_additional_tricks, old.at_least_additional_tricks);
                let lowest_upper_bound = min(value.at_most_additional_tricks, old.at_most_additional_tricks);
                let new = TTValue {
                    at_least_additional_tricks: highest_lower_bound,
                    at_most_additional_tricks: lowest_upper_bound,
                };
                self.inner.insert(*key, new)
            }
        };
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct TTKey {
    pub depth: usize,
    pub lead: Seat,
    pub remaining_cards: [u32; 4],
}

#[derive(Copy, Clone)]
pub struct TTValue {
    pub at_least_additional_tricks: usize,
    pub at_most_additional_tricks: usize,
}
