#[derive(Clone)]
pub struct DdsConfig {
    pub move_ordering: bool,
    pub check_quick_tricks: bool,
    pub quick_tricks_in_second_hand: bool,
    pub use_transposition_table: bool,
    pub check_losing_tricks: bool,
    pub pre_estimate: bool,
    pub multi_threading: bool,
}

impl Default for DdsConfig {
    fn default() -> Self {
        Self {
            move_ordering: true,
            check_quick_tricks: true,
            quick_tricks_in_second_hand: true,
            use_transposition_table: true,
            check_losing_tricks: true,
            pre_estimate: false,
            multi_threading: true,
        }
    }
}
