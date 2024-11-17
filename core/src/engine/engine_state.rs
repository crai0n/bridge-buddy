use crate::primitives::hand_info::suit_quality::SuitQuality;

pub struct EngineState {}

impl EngineState {
    pub fn empty() -> Self {
        EngineState {}
    }
}

#[allow(dead_code)]
pub struct SuitHint {
    min_length: usize,
    min_quality: SuitQuality,
}
