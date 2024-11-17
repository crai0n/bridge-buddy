use crate::primitives::bid_line::BidLine;
use crate::primitives::hand_info::HandDescription;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub struct ForumDPlusBidInterpreter {
    bid_map: BTreeMap<BidLine, BidInterpretation>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BidInterpretation(Vec<HandDescription>); // A BidInterpretation is ANY of the contained HandDescriptions

impl Display for BidInterpretation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            write!(f, "No information")
        } else {
            write!(f, "{}", self.0.iter().map(|i| format!("{}", i)).join(", OR "))
        }
    }
}

#[allow(dead_code)]
pub trait BidInterpreter {
    fn interpret(&self, bid_line: BidLine) -> Option<BidInterpretation>;
}
#[allow(dead_code)]
impl BidInterpreter for ForumDPlusBidInterpreter {
    fn interpret(&self, bid_line: BidLine) -> Option<BidInterpretation> {
        self.bid_map.get(&bid_line).cloned()
    }
}
#[allow(dead_code)]
impl ForumDPlusBidInterpreter {
    fn new() -> Self {
        unimplemented!()
    }
}
