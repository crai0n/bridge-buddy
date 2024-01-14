use crate::primitives::deal::Seat;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Axis {
    NorthSouth,
    EastWest,
}

impl Axis {
    pub fn has_player(&self, player: Seat) -> bool {
        player.is_on_axis(self)
    }

    pub fn players(&self) -> [Seat; 2] {
        match self {
            Axis::NorthSouth => [Seat::North, Seat::South],
            Axis::EastWest => [Seat::East, Seat::West],
        }
    }
}

impl From<Seat> for Axis {
    fn from(player: Seat) -> Self {
        match player {
            Seat::North => Axis::NorthSouth,
            Seat::East => Axis::EastWest,
            Seat::South => Axis::NorthSouth,
            Seat::West => Axis::EastWest,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::primitives::deal::axis::Axis;
    use crate::primitives::deal::axis::Axis::*;
    use crate::primitives::deal::Seat;
    use crate::primitives::deal::Seat::*;
    use test_case::test_case;

    #[test_case(North, NorthSouth)]
    #[test_case(South, NorthSouth)]
    #[test_case(East, EastWest)]
    #[test_case(West, EastWest)]
    fn from_player(player: Seat, expected: Axis) {
        assert_eq!(Axis::from(player), expected);
    }
}
