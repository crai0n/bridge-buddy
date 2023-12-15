use crate::engine::bidding::mock::MockBiddingEngine;
use crate::engine::card_play::mock::MockCardPlayEngine;
use crate::error::BBError;
use crate::game::Game;
use crate::player::{Move, Player};
use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::{BidEvent, CardEvent, GameEvent};
use crate::primitives::player_event::PlayerEvent;
use crate::primitives::Card;

pub struct AutoPlayer {
    seat: Seat,
    game: Option<Game>,
    bidding_engine: MockBiddingEngine,
    card_play_engine: MockCardPlayEngine,
}

impl Player for AutoPlayer {
    fn process_game_event(&mut self, event: GameEvent) -> Result<(), BBError> {
        match event {
            GameEvent::NewGame(new_game_event) => {
                self.game = Some(Game::from_new_game_event(new_game_event));
                Ok(())
            }
            _ => match &mut self.game {
                None => Err(BBError::GameHasNotStarted)?,
                Some(game) => game.process_game_event(event),
            },
        }
    }
}

impl Move for AutoPlayer {
    fn get_move(&self) -> Result<PlayerEvent, BBError> {
        self.get_move_for(self.seat)
    }

    fn get_dummy_move(&self) -> Result<PlayerEvent, BBError> {
        self.get_move_for(self.seat.partner())
    }
}

impl AutoPlayer {
    fn get_move_for(&self, seat: Seat) -> Result<PlayerEvent, BBError> {
        match &self.game.as_ref().unwrap() {
            Game::Bidding(state) => {
                if seat != self.seat {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let bid = self.bidding_engine.find_bid(state);
                Ok(self.make_bid(bid))
            }
            Game::OpeningLead(state) => {
                if seat != self.seat {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let card = self.card_play_engine.pick_opening_lead(state);
                Ok(self.play_card_as(card, seat))
            }
            Game::CardPlay(state) => {
                if seat != state.inner.contract.declarer.partner() && seat != self.seat {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let card = self.card_play_engine.pick_card_for(state, seat);
                Ok(self.play_card_as(card, seat))
            }
            Game::WaitingForDummy(_) => Err(BBError::OutOfTurn(None)),
            Game::Ended(_) => Err(BBError::GameHasEnded),
        }
    }

    fn make_bid(&self, bid: Bid) -> PlayerEvent {
        let bid_event = BidEvent { player: self.seat, bid };
        PlayerEvent::Bid(bid_event)
    }

    fn play_card_as(&self, card: Card, seat: Seat) -> PlayerEvent {
        let card_event = CardEvent { player: seat, card };
        PlayerEvent::Card(card_event)
    }

    pub fn new(seat: Seat) -> Self {
        AutoPlayer {
            seat,
            game: None,
            bidding_engine: MockBiddingEngine::new(),
            card_play_engine: MockCardPlayEngine::new(seat),
        }
    }
}
