use strum::{Display, EnumString};
#[derive(PartialEq, Debug, EnumString, Clone, Copy, Display)]
pub enum BiddingSituation {
    Unknown,
    OpeningFirstSecond,
    OpeningThirdFourth,
    Answer1NoTrump,
    Answer1Major,
    Answer1Minor,
}
