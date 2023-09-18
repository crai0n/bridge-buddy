use crate::card::Card;
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
    NotUnique(Card),
    UnknownSuit(String),
    UnknownDenomination(String),
}

impl Display for BBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            BBError::ParseError(cause, description) => writeln!(f, "{}: {}", description, cause),
            BBError::NotUnique(card) => writeln!(f, "card not unique: {}", card),
            BBError::UnknownDenomination(d) => writeln!(f, "unknown denomination: {}", d),
            BBError::UnknownSuit(s) => writeln!(f, "unknown suit: {}", s),
        }
    }
}
