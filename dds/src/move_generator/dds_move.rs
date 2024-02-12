use crate::state::virtual_card::VirtualCard;

#[derive(Debug, Clone, Copy)]
pub struct DdsMove {
    pub card: VirtualCard,
    pub sequence_length: usize,
    pub priority: isize,
}

impl DdsMove {
    pub fn new(card: VirtualCard) -> Self {
        Self {
            card,
            sequence_length: 1,
            priority: 0,
        }
    }
}
