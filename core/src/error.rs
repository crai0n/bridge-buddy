use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::GameEvent;
use crate::primitives::{Card, Suit};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum BBError {
    ParseError(String, &'static str),
    Duplicate(Card),
    CardCount,
    UnknownCard(String),
    UnknownSuit(String),
    UnknownRank(String),
    UnknownContract(String),
    UnknownStrain(String),
    UnknownBid(String),
    InvalidBid(Bid),
    GameAlreadyStarted,
    GameHasNotStarted,
    GameHasEnded,
    OutOfTurn(Option<Seat>),
    PlayerUnreachable(Seat),
    SeatTaken(Seat),
    InvalidEvent(Box<GameEvent>),
    InvalidCard(Card),
    NotAuthorized(Seat),
    InsufficientInfo,
    InvalidHandInfo,
    InvalidContract,
    NoGame,
    GameStuck,
    CannotPlayFor(Seat),
    FollowSuit(Suit),
    NotYourCard(Card),
    AlreadyPlayed(Card),
    WrongBidType(Bid),
}

impl Display for BBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            BBError::ParseError(cause, description) => writeln!(f, "{}: {}", description, cause),
            BBError::Duplicate(card) => writeln!(f, "card not unique: {}", card),
            BBError::UnknownCard(c) => writeln!(f, "unknown card: {}", c),
            BBError::UnknownRank(d) => writeln!(f, "unknown rank: {}", d),
            BBError::UnknownSuit(s) => writeln!(f, "unknown suit: {}", s),
            BBError::CardCount => writeln!(f, "wrong number of cards"),
            BBError::UnknownContract(c) => writeln!(f, "unknown contract: {}", c),
            BBError::UnknownStrain(c) => writeln!(f, "unknown contract strain: {}", c),
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
            BBError::InvalidContract => writeln!(f, "This is an impossible Contract"),
            BBError::NoGame => writeln!(f, "There is no game to start"),
            BBError::GameStuck => writeln!(f, "It seems the game is stuck"),
            BBError::CannotPlayFor(seat) => writeln!(f, "You cannot play for {}.", seat),
            BBError::FollowSuit(suit) => writeln!(f, "You have to follow suit: {}.", suit),
            BBError::NotYourCard(card) => writeln!(f, "Card {} belongs to another player.", card),
            BBError::AlreadyPlayed(card) => writeln!(f, "Card {} has already been played.", card),
            BBError::WrongBidType(bid) => writeln!(f, "Bid has wrong type: {}", bid),
        }
    }
}
