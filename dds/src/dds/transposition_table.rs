use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::Suit;
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

    pub fn update_value(&mut self, key: &TTKey, value: TTValue) {
        match self.inner.get(key) {
            None => self.inner.insert(*key, value),
            Some(old) => {
                let highest_lower_bound = max(value.0, old.0);
                let lowest_upper_bound = min(value.1, old.1);
                let new = TTValue(highest_lower_bound, lowest_upper_bound);
                self.inner.insert(*key, new)
            }
        };
    }

    pub fn update_upper_bound(&mut self, key: &TTKey, bound: usize) {
        let new = match self.inner.get(key) {
            None => TTValue(0, bound),
            Some(old) => {
                let highest_lower_bound = old.0;
                let lowest_upper_bound = min(bound, old.1);
                TTValue(highest_lower_bound, lowest_upper_bound)
            }
        };
        self.inner.insert(*key, new);
    }

    pub fn update_lower_bound(&mut self, key: &TTKey, bound: usize) {
        let new = match self.inner.get(key) {
            None => TTValue(bound, key.tricks_left),
            Some(old) => {
                let highest_lower_bound = max(bound, old.0);
                let lowest_upper_bound = old.1;
                TTValue(highest_lower_bound, lowest_upper_bound)
            }
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

#[derive(Copy, Clone)]
pub struct TTValue(pub usize, pub usize);
