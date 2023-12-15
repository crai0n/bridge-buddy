use crate::primitives::deal::Seat;

#[allow(dead_code)]
pub struct CliBidFinder {
    seat: Seat,
}

impl CliBidFinder {
    pub fn new(seat: Seat) -> Self {
        CliBidFinder { seat }
    }
}
