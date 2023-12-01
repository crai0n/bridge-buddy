use crate::primitives::deal::PlayerPosition;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Axis {
    NorthSouth,
    EastWest,
}

impl Axis {
    pub fn has_player(&self, player: PlayerPosition) -> bool {
        player.is_on_axis(self)
    }
}

impl From<PlayerPosition> for Axis {
    fn from(player: PlayerPosition) -> Self {
        match player {
            PlayerPosition::North => Axis::NorthSouth,
            PlayerPosition::East => Axis::EastWest,
            PlayerPosition::South => Axis::NorthSouth,
            PlayerPosition::West => Axis::EastWest,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::primitives::deal::axis::Axis;
    use crate::primitives::deal::axis::Axis::*;
    use crate::primitives::deal::PlayerPosition;
    use crate::primitives::deal::PlayerPosition::*;
    use test_case::test_case;

    #[test_case(North, NorthSouth)]
    #[test_case(South, NorthSouth)]
    #[test_case(East, EastWest)]
    #[test_case(West, EastWest)]
    fn from_player(player: PlayerPosition, expected: Axis) {
        assert_eq!(Axis::from(player), expected);
    }
}
