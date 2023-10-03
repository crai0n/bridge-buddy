use crate::primitives::card::Denomination;
use crate::primitives::card::Denomination::*;
use std::fmt::Display;
use std::ops::Add;

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct HcpValue(pub f64);

impl Display for HcpValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} HCP", self.0)
    }
}

impl From<f64> for HcpValue {
    fn from(num: f64) -> Self {
        HcpValue(num)
    }
}

impl Add for HcpValue {
    type Output = HcpValue;

    fn add(self, rhs: Self) -> Self::Output {
        HcpValue(self.0 + rhs.0)
    }
}

impl Add<f64> for HcpValue {
    type Output = HcpValue;

    fn add(self, rhs: f64) -> Self::Output {
        HcpValue(self.0 + rhs)
    }
}

impl Add<HcpValue> for f64 {
    type Output = HcpValue;

    fn add(self, rhs: HcpValue) -> Self::Output {
        rhs.add(self)
    }
}

impl From<Denomination> for HcpValue {
    fn from(denomination: Denomination) -> Self {
        match denomination {
            Ace => 4.0,
            King => 3.0,
            Queen => 2.0,
            Jack => 1.0,
            _ => 0.0,
        }
        .into()
    }
}
