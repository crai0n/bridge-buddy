use crate::primitives::bid::Bid;
use crate::primitives::Card;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub(crate) cause: String,
    pub(crate) description: &'static str,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.description, self.cause)
    }
}

#[derive(Debug, PartialEq)]
pub enum BBError {
    ParseError(String, &'static str),
    Duplicate(Card),
    CardCount,
    UnknownCard(String),
    UnknownSuit(String),
    UnknownDenomination(String),
    UnknownContract(String),
    UnknownContractDenomination(String),
    UnknownBid(String),
    InvalidBid(Bid),
}

impl Display for BBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            BBError::ParseError(cause, description) => writeln!(f, "{}: {}", description, cause),
            BBError::Duplicate(card) => writeln!(f, "card not unique: {}", card),
            BBError::UnknownCard(c) => writeln!(f, "unknown card: {}", c),
            BBError::UnknownDenomination(d) => writeln!(f, "unknown denomination: {}", d),
            BBError::UnknownSuit(s) => writeln!(f, "unknown suit: {}", s),
            BBError::CardCount => writeln!(f, "wrong number of cards"),
            BBError::UnknownContract(c) => writeln!(f, "unknown contract: {}", c),
            BBError::UnknownContractDenomination(c) => writeln!(f, "unknown contract denomination: {}", c),
            BBError::UnknownBid(c) => writeln!(f, "unknown bid: {}", c),
            BBError::InvalidBid(b) => writeln!(f, "invalid bid: {}", b),
        }
    }
}
