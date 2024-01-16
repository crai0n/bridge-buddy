use super::turn_rank::TurnRank;
use crate::primitives::deal::axis::Axis;
use crate::primitives::deal::{Board, Vulnerability};
use crate::primitives::Deal;
use std::ops;
use strum::{Display, EnumIter, EnumString};

pub const N_SEATS: usize = 4;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Display, EnumIter, EnumString, Ord, PartialOrd)]
pub enum Seat {
    #[strum(serialize = "n")]
    #[strum(to_string = "N")]
    North = 0,
    #[strum(serialize = "e")]
    #[strum(to_string = "E")]
    East = 1,
    #[strum(serialize = "s")]
    #[strum(to_string = "S")]
    South = 2,
    #[strum(serialize = "w")]
    #[strum(to_string = "W")]
    West = 3,
}

impl ops::Add<TurnRank> for Seat {
    type Output = Seat;

    fn add(self, rhs: TurnRank) -> Seat {
        match (self as usize + rhs as usize) % 4 {
            0 => Seat::North,
            1 => Seat::East,
            2 => Seat::South,
            _ => Seat::West,
        }
    }
}

impl ops::Add<usize> for Seat {
    type Output = Seat;

    fn add(self, rhs: usize) -> Seat {
        match (self as usize + rhs) % 4 {
            0 => Seat::North,
            1 => Seat::East,
            2 => Seat::South,
            _ => Seat::West,
        }
    }
}

impl Seat {
    pub fn turn_rank_on_deal<const N: usize>(&self, deal: &Deal<N>) -> TurnRank {
        self.turn_rank_on_board(&deal.board)
    }

    pub fn turn_rank_on_board(&self, board: &Board) -> TurnRank {
        self.turn_rank_relative_to(&board.dealer())
    }

    pub fn turn_rank_relative_to(&self, other: &Seat) -> TurnRank {
        TurnRank::from(4 - *other as usize + *self as usize)
    }

    pub const fn partner(&self) -> Self {
        match self {
            Seat::North => Seat::South,
            Seat::East => Seat::West,
            Seat::South => Seat::North,
            Seat::West => Seat::East,
        }
    }

    pub const fn same_axis(&self, other: &Seat) -> bool {
        (*self as usize + *other as usize) % 2 == 0
    }

    pub const fn axis(&self) -> Axis {
        match self {
            Seat::North | Seat::South => Axis::NorthSouth,
            Seat::East | Seat::West => Axis::EastWest,
        }
    }

    pub const fn is_on_axis(&self, axis: &Axis) -> bool {
        match axis {
            Axis::NorthSouth => matches!(self, Seat::North | Seat::South),
            Axis::EastWest => matches!(self, Seat::East | Seat::West),
        }
    }

    pub const fn is_vulnerable(&self, vulnerability: Vulnerability) -> bool {
        vulnerability.player_is_vulnerable(self)
    }
}

#[cfg(test)]
mod test {
    use super::Seat;
    use super::Seat::*;

    use crate::primitives::deal::turn_rank::TurnRank;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("n", North; "North")]
    #[test_case("N", North; "North2")]
    #[test_case("s", South; "South")]
    #[test_case("S", South; "South2")]
    #[test_case("e", East; "East")]
    #[test_case("E", East; "East2")]
    #[test_case("w", West; "West")]
    #[test_case("W", West; "West2")]
    fn from_str(input: &str, expected: Seat) {
        let player_pos = Seat::from_str(input).unwrap();
        assert_eq!(player_pos, expected);
    }

    #[test_case(North, "N"; "North")]
    #[test_case(South, "S"; "South")]
    #[test_case(East, "E"; "East")]
    #[test_case(West, "W"; "West")]
    fn display(input: Seat, expected: &str) {
        let str = format!("{}", input);
        assert_eq!(str, expected);
    }

    #[test_case(North, 2, South; "North2")]
    #[test_case(South, 4, South; "South4")]
    #[test_case(East, 3, North; "East3")]
    #[test_case(West, 5, North; "West5")]
    #[test_case(East, 2, West; "East2")]
    #[test_case(West, 2, East; "West2")]
    fn add(start: Seat, add: usize, expected: Seat) {
        assert_eq!(start + add, expected)
    }

    #[test_case(North, North, TurnRank::First)]
    #[test_case(East, East, TurnRank::First)]
    #[test_case(East, North, TurnRank::Second)]
    #[test_case(South, North, TurnRank::Third)]
    #[test_case(West, South, TurnRank::Second)]
    #[test_case(North, West, TurnRank::Second)]
    #[test_case(East, West, TurnRank::Third)]
    fn turn_rank_relative_to(player: Seat, other: Seat, expected: TurnRank) {
        assert_eq!(player.turn_rank_relative_to(&other), expected);
    }

    #[test_case(North, South, false; "North")]
    #[test_case(South, South, true; "South")]
    #[test_case(East, North, false; "East")]
    #[test_case(West, North, false; "West")]
    fn equality(one: Seat, other: Seat, expected: bool) {
        assert_eq!(one.eq(&other), expected)
    }

    #[test_case(North, South; "North's partner is South")]
    #[test_case(South, North; "South's partner is North")]
    #[test_case(West, East; "West's partner is East")]
    #[test_case(East, West; "East's partner is West")]
    fn partner(player: Seat, expected: Seat) {
        assert_eq!(player.partner(), expected)
    }
}
