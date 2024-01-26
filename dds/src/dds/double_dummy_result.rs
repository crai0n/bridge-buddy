use bridge_buddy_core::primitives::contract::Strain;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::Suit;
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;

pub struct DoubleDummyResult {
    pub max_tricks: [usize; 20],
}

impl DoubleDummyResult {
    pub fn new() -> Self {
        Self { max_tricks: [0; 20] }
    }
    pub fn get_tricks_for_declarer_in_strain(&self, declarer: Seat, strain: Strain) -> usize {
        let i = match declarer {
            Seat::North => 0,
            Seat::East => 1,
            Seat::South => 2,
            Seat::West => 3,
        };
        let j = match strain {
            Strain::Trump(Suit::Clubs) => 0,
            Strain::Trump(Suit::Diamonds) => 1,
            Strain::Trump(Suit::Hearts) => 2,
            Strain::Trump(Suit::Spades) => 3,
            Strain::NoTrump => 4,
        };

        let index = 5 * i + j;

        self.max_tricks[index]
    }

    pub fn set_tricks_for_declarer_in_strain(&mut self, tricks: usize, declarer: Seat, strain: Strain) {
        let i = match declarer {
            Seat::North => 0,
            Seat::East => 1,
            Seat::South => 2,
            Seat::West => 3,
        };

        let j = match strain {
            Strain::Trump(Suit::Clubs) => 0,
            Strain::Trump(Suit::Diamonds) => 1,
            Strain::Trump(Suit::Hearts) => 2,
            Strain::Trump(Suit::Spades) => 3,
            Strain::NoTrump => 4,
        };

        let index = 5 * i + j;

        self.max_tricks[index] = tricks
    }
}

impl Default for DoubleDummyResult {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for DoubleDummyResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  NT ♠S ♥H ♦D ♣C")?;
        for seat in Seat::iter() {
            write!(f, "{} ", seat)?;
            for strain in (0..5).rev() {
                let n_str = format!("{}", self.max_tricks[5 * (seat as usize) + strain]);
                write!(f, "{:>2} ", n_str)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::dds::double_dummy_result::DoubleDummyResult;
    use test_case::test_case;

    #[test_case([0,1,2,3,4,1,2,3,4,5,2,3,4,5,6,3,4,5,6,7], "  NT ♠S ♥H ♦D ♣C\nN  4  3  2  1  0 \nE  5  4  3  2  1 \nS  6  5  4  3  2 \nW  7  6  5  4  3 \n")]
    fn display(max_tricks: [usize; 20], expected: &str) {
        let ddr = DoubleDummyResult { max_tricks };
        let str = format!("{}", ddr);
        assert_eq!(str, expected)
    }
}
