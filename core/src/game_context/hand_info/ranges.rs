use std::fmt::Display;
use std::ops::RangeInclusive;

#[derive(PartialEq)]
pub struct LengthRange(pub RangeInclusive<usize>);

#[derive(PartialEq)]
pub struct PointRange(pub RangeInclusive<f64>);
#[derive(PartialEq)]
pub struct WinnerRange(pub RangeInclusive<f64>);
#[derive(PartialEq)]
pub struct LoserRange(pub RangeInclusive<f64>);

impl Display for LengthRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {} cards", self.0.start(), self.0.end())
    }
}

impl Display for PointRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {} points", self.0.start(), self.0.end())
    }
}

impl Display for WinnerRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {} winners", self.0.start(), self.0.end())
    }
}
impl Display for LoserRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {} losers", self.0.start(), self.0.end())
    }
}
