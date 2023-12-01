use crate::primitives::bid::Bid;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::game_event::GameEvent;
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
    GameAlreadyStarted,
    GameHasNotStarted,
    GameHasEnded,
    OutOfTurn(Option<PlayerPosition>),
    PlayerUnreachable(PlayerPosition),
    SeatTaken(PlayerPosition),
    InvalidEvent(GameEvent),
    InvalidCard(Card),
    NotAuthorized(PlayerPosition),
    InsufficientInfo,
    InvalidHandInfo,
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
            BBError::InvalidBid(bid) => writeln!(f, "invalid bid: {}", bid),
            BBError::GameAlreadyStarted => writeln!(f, "Game has already started!"),
            BBError::PlayerUnreachable(player) => writeln!(f, "No queue found for player at {}", player),
            BBError::OutOfTurn(correct_player) => match correct_player {
                Some(correct_player) => writeln!(f, "It's not your turn! {} ", correct_player),
                None => writeln!(f, "It's not your turn!"),
            },
            BBError::SeatTaken(seat) => writeln!(f, "There is already a player at {}", seat),
            BBError::InvalidEvent(game_event) => {
                writeln!(f, "This event is not valid: {:?}", game_event)
            }

            BBError::InvalidCard(card) => writeln!(f, "You cannot play {}!", card),
            BBError::NotAuthorized(player_position) => {
                writeln!(f, "You are not allowed to play for {}!", player_position)
            }
            BBError::GameHasEnded => writeln!(f, "The game has ended!"),
            BBError::GameHasNotStarted => writeln!(f, "The game has not started!"),
            BBError::InsufficientInfo => writeln!(f, "Not enough information to calculate this."),
            BBError::InvalidHandInfo => writeln!(f, "Hands are not valid for a bridge game."),
        }
    }
}
