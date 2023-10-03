use super::hcp_value::HcpValue;
use std::fmt::Display;
use std::ops::RangeInclusive;

#[derive(PartialEq, Debug)]
pub struct HcpRange(pub RangeInclusive<f64>);

impl Display for HcpRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {} hcp", self.0.start(), self.0.end())
    }
}

impl From<HcpValue> for HcpRange {
    fn from(value: HcpValue) -> Self {
        HcpRange(value.0..=value.0)
    }
}
