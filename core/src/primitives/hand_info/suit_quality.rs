use strum::Display;

#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum SuitQuality {
    #[strum(serialize = "weak")]
    Weak,
    #[strum(serialize = "acceptable")]
    Acceptable,
    #[strum(serialize = "good")]
    Good,
    #[strum(serialize = "very good")]
    VeryGood,
    #[strum(serialize = "almost standing")]
    AlmostStanding,
    #[strum(serialize = "standing")]
    Standing,
}
