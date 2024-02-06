pub struct DdsConfig {
    pub move_ordering: bool,
    pub check_quick_tricks: bool,
    pub quick_tricks_in_second_hand: bool,
    pub use_transposition_table: bool,
    pub check_losing_tricks: bool,
    pub pre_estimate: bool,
    // fail_soft: bool,
}

impl Default for DdsConfig {
    fn default() -> Self {
        Self {
            move_ordering: true,
            check_quick_tricks: true,
            quick_tricks_in_second_hand: false,
            use_transposition_table: true,
            check_losing_tricks: true,
            pre_estimate: true,
        }
    }
}
