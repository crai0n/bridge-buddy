use crate::primitives::contract::Strain;
use crate::primitives::deal::Seat;
use crate::primitives::Suit;
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;

pub struct DoubleDummyResult {
    pub max_tricks: [usize; 20],
}

impl DoubleDummyResult {
    pub fn max_tricks_for_player_in_strain(&self, player: Seat, strain: Strain) -> usize {
        let i = match player {
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
}

impl Display for DoubleDummyResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  NT ♠S ♥H ♦D ♣C")?;
        for seat in Seat::iter() {
            write!(f, "{} ", seat)?;
            for strain in 0..5 {
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

    #[test_case([0,1,2,3,4,1,2,3,4,5,2,3,4,5,6,3,4,5,6,7], "  NT ♠S ♥H ♦D ♣C\nN  0  1  2  3  4 \nE  1  2  3  4  5 \nS  2  3  4  5  6 \nW  3  4  5  6  7 \n")]
    fn display(max_tricks: [usize; 20], expected: &str) {
        let ddr = DoubleDummyResult { max_tricks };
        let str = format!("{}", ddr);
        assert_eq!(str, expected)
    }
}
