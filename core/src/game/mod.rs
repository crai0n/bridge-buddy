mod game_phase;

use crate::error::BBError;
use crate::game::game_phase::{Bidding, CardPlay, Ended, GamePhase, OpeningLead, Setup};
use crate::primitives::bid::Bid;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::{Card, Deal};

// Together, the different versions of game form a state machine that encapsulates the logic of the game and contains
// all information that immediately follow from the rules of the game.

enum Game {
    Setup(GamePhase<Setup>),
    Bidding(GamePhase<Bidding>),
    OpeningLead(GamePhase<OpeningLead>),
    CardPlay(GamePhase<CardPlay>),
    Ended(GamePhase<Ended>),
}

impl Game {
    pub fn new(deal: Deal) -> Self {
        let inner = GamePhase::new(deal);
        Game::Setup(inner)
    }

    pub fn make_bid(&mut self, _bid: Bid, _player: PlayerPosition) -> Result<(), BBError> {
        match self {
            Game::Setup(_) => Err(BBError::CannotBid("Game has not started.".into())),
            Game::Bidding(inner) => {
                inner.bid();
                if inner.contract_is_final() {
                    //*self = Game::OpeningLead(inner.end_bidding());
                }
                Ok(())
            }
            Game::OpeningLead(_) => Err(BBError::CannotBid("Bidding has ended.".into())),
            Game::CardPlay(_) => Err(BBError::CannotBid("Bidding has ended.".into())),
            Game::Ended(_) => Err(BBError::CannotBid("Game has ended.".into())),
        }
    }

    pub fn play_card(&mut self, _card: Card, _player: PlayerPosition) -> Result<(), BBError> {
        match self {
            Game::Setup(_) => Err(BBError::CannotBid("Game has not started.".into())),
            Game::Bidding(_) => Err(BBError::CannotBid("Card play has not started.".into())),
            Game::OpeningLead(_phase) => {
                //phase.check_lead(card, player)?;
                //*self = Game::CardPlay(phase.lead(card));
                Ok(())
            }
            Game::CardPlay(_phase) => {
                //phase.check_play(card, player)?;
                //phase.play_card(&card);
                // if phase.card_play_has_ended() {
                //     *self = Game::Ended(phase.end_play())
                // }
                Ok(())
            }
            Game::Ended(_) => Err(BBError::CannotBid("Game has ended.".into())),
        }
    }

    pub fn get_score(&self) -> Result<usize, BBError> {
        match self {
            Game::Setup(_) => Err(BBError::CannotBid("Game has not started.".into())),
            _ => unimplemented!(),
        }
    }
}

pub enum Move {
    Bid(Bid),
    Card(Card),
}
