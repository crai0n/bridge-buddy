use bridge_buddy_core::primitives::card::suit::SUIT_ARRAY;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::Suit;
use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Default)]
pub struct TranspositionTable {
    inner: HashMap<TTKey, TTValue>,
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

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct TTKey {
    pub tricks_left: usize,
    pub trumps: Option<Suit>,
    pub lead: Seat,
    pub remaining_cards: [u32; 4],
}

#[allow(dead_code)]
impl TTKey {
    pub fn new(tricks_left: usize, trumps: Option<Suit>, lead: Seat, remaining_cards: [u32; 4]) -> Self {
        Self {
            tricks_left,
            trumps,
            lead,
            remaining_cards,
        }
    }
    pub fn calc_id(&self) -> [u32; 4] {
        // use a single u32 to store:
        // the number of tricks left (N:1..=13), 13 values in 4 bits
        // the trump suit (T:0..=4), 5 values in 3 bits
        // the leader(L:0..=3), 4 values in 2 bits
        // the number of cards left in play for three suits (n:0..=13), 14^3=2744 values in 4*3 = 12 bits
        // and the position of the Ace for each suit (xA:0..=3), 4^4 values = 256 values in 8 bits
        // so in total 4 + 3 + 2 + 12 + 8 = 29 bits
        // layout is
        // 0bNNNN_00LL_0TTT_SSSS_sAHH_HHhA_DDDD_dAcA

        let mut pre_id = 0u32;

        pre_id |= (self.tricks_left as u32) << 28; // 0bNNNN
        pre_id |= (self.lead as u32) << 24; // 0b00LL

        if let Some(trumps) = self.trumps {
            pre_id |= (trumps as u32 + 1) << 20 // 0b0TTT
        }

        let trans_field = Self::mutate(self.remaining_cards);

        for suit in SUIT_ARRAY {
            let (suit_id, index) = if suit == Suit::Clubs {
                (trans_field[0][suit as usize] as u32 & 3u32, 0)
            } else {
                (trans_field[0][suit as usize] as u32, (suit as u32 * 6) - 4)
            };
            pre_id |= suit_id << index;
        }

        // output[0] will be pre_id, the rest will be
        // 3 more u32s for the position of the remaining cards:
        // 0bcKcQ_cJcT_dKdQ_dJdT_hKhQ_hJhT_sKsQ_sJsT
        // 9..=6
        // 5..=2

        let mut output = trans_field.map(u32::from_be_bytes);

        output[0] = pre_id;

        output
    }

    pub fn mutate(fields: [u32; 4]) -> [[u8; 4]; 4] {
        // output contains the first byte of every suit, then the second byte of every suit, etc.
        let mut output = [[0u8; 4]; 4];

        for suit in SUIT_ARRAY {
            for (index, byte) in fields[suit as usize].to_be_bytes().iter().enumerate() {
                output[index][suit as usize] = *byte;
            }
        }

        output
    }
}

#[derive(Clone)]
pub struct TTValue {
    pub at_least: usize,
    pub at_most: usize,
}

#[cfg(test)]
mod test {
    use crate::transposition_table::TTKey;
    use bridge_buddy_core::primitives::deal::Seat;
    use bridge_buddy_core::primitives::Suit;

    #[test]
    fn mutate() {
        let fields = [
            0b0001_0100_1101_1000_0000_0000_0000_0000,
            0b0001_0101_0011_1010_0000_0000_0000_0000,
            0b0001_0110_0100_1110_0000_0000_0000_0000,
            0b0001_0111_1001_0011_0000_0000_0000_0000,
        ];
        let mutated = super::TTKey::mutate(fields);

        for field in mutated {
            for byte in field {
                print!("{:8b}", byte)
            }
            println!()
        }

        let expected = [
            [0b0001_0100, 0b0001_0101, 0b0001_0110, 0b0001_0111],
            [0b1101_1000, 0b0011_1010, 0b0100_1110, 0b1001_0011],
            [0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000],
            [0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000],
        ];

        for field in expected {
            for byte in field {
                print!("{:8b}", byte)
            }
            println!()
        }

        assert_eq!(mutated, expected)
    }

    #[test]
    fn calc_id() {
        let tt_key = TTKey::new(
            5,
            Some(Suit::Clubs),
            Seat::South,
            [
                0b0001_0100_1101_1000_0000_0000_0000_0000,
                0b0001_0101_0011_1010_0000_0000_0000_0000,
                0b0001_0110_0100_1110_0000_0000_0000_0000,
                0b0001_0111_1001_0011_0000_0000_0000_0000,
            ],
        );

        let id = tt_key.calc_id();

        println!("NNNN00LL0TTTSSSSsAHHHHhADDDDdAcA");

        println!("{:032b}", id[0]);

        // 0bNNNN_00LL_0TTT_SSSS_sAHH_HHhA_DDDD_dAcA
        let expected = [
            0b0101_0010_0001_0101_1101_0110_0101_0100,
            0b1101_1000_0011_1010_0100_1110_1001_0011,
            0,
            0,
        ];

        println!("{:032b}", expected[0]);

        println!("{:032b}", id[1]);
        println!("{:032b}", expected[1]);

        assert_eq!(id, expected);
    }
}
