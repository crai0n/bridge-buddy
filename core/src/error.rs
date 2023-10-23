use crate::bid_reader::bidding_situation::BiddingSituation;
use crate::primitives::bid::Bid;
use crate::primitives::bid_line::BidLine;
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

#[derive(Debug)]
pub enum BBError {
    ParseError(String, &'static str),
    Duplicate(Card),
    CardCount,
    UnknownCard(String),
    UnknownSuit(char),
    UnknownDenomination(char),
    UnknownContract(String),
    UnknownContractDenomination(String),
    UnknownBid(String),
    InvalidBid(Bid),
    UnknownBiddingSituation(String),
    DuplicateRule(BidLine, BiddingSituation),
    IoError(std::io::Error),
    TransitionError,
    CannotBid(String),
    OutOfTurn,
    WrongCard,
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
            BBError::UnknownBiddingSituation(s) => writeln!(f, "unknown bidding situation: {}", s),
            BBError::DuplicateRule(bl, sit) => writeln!(f, "line {} is already marked as {}", bl, sit),
            BBError::IoError(err) => writeln!(f, "{}", err),
            BBError::CannotBid(reason) => writeln!(f, "{}", reason),
            BBError::TransitionError => writeln!(f, "unsupported transition between game states"),
            BBError::OutOfTurn => writeln!(f, "It's not your turn to play!"),
            BBError::WrongCard => writeln!(f, "You do not have this card!"),
        }
    }
}

impl From<std::io::Error> for BBError {
    fn from(err: std::io::Error) -> BBError {
        BBError::IoError(err)
    }
}
