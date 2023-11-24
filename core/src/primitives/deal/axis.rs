use crate::primitives::deal::PlayerPosition;

#[derive(Copy, Clone)]
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
