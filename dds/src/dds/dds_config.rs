pub struct DdsConfig {
    pub move_ordering: bool,
    pub check_quick_tricks: bool,
}

impl Default for DdsConfig {
    fn default() -> Self {
        Self {
            move_ordering: true,
            check_quick_tricks: true,
        }
    }
}
