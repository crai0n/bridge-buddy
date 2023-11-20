#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
pub enum GamePhase {
    Setup,
    Bidding,
    CardPlay,
    Ended,
}
