use crate::primitives::deal::PlayerPosition;

#[derive(Copy, Clone)]
pub enum Axis {
    NorthSouth,
    EastWest,
}

impl Axis {
    fn has_player(&self, player: PlayerPosition) -> bool {
        player.is_on_axis(self)
    }
}
