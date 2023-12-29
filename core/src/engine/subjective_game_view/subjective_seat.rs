#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubjectiveSeat {
    Myself = 0,
    LeftHandOpponent = 1,
    Partner = 2,
    RightHandOpponent = 3,
}
