use crate::primitives::hand_info::ranges::{HcpRange, LengthRange, PointRange};
use crate::primitives::hand_info::suit_quality::SuitQuality;

pub struct EngineState {}

impl EngineState {
    pub fn empty() -> Self {
        EngineState {}
    }
}

#[allow(dead_code)]
pub struct HandDescription {
    suit_lengths: [LengthRange; 4],
    hcp: HcpRange,
    total_points: PointRange,
}

#[allow(dead_code)]
pub struct SuitHint {
    min_length: usize,
    min_quality: SuitQuality,
}
