#[derive(Eq, PartialEq, Debug, Clone, Copy, Ord, PartialOrd)]
pub enum GamePhase {
    Setup,
    Bidding,
    CardPlay,
    Ended,
}
