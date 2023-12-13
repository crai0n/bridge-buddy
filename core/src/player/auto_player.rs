use crate::error::BBError;
use crate::game::Game;
use crate::player::engine::{MockBiddingEngine, MockCardPlayEngine};
use crate::player::Player;
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

    fn make_move(&self) -> Result<PlayerEvent, BBError> {
        match &self.game.as_ref().unwrap() {
            Game::Bidding(state) => {
                let bid = self.bidding_engine.find_bid(state);
                Ok(self.make_bid(bid))
            }
            Game::CardPlay(state) => {
                let card = self.card_play_engine.pick_card(state);
                Ok(self.play_card(card))
            }
            Game::OpeningLead(state) => {
                let card = self.card_play_engine.pick_opening_lead(state);
                Ok(self.play_card(card))
            }
            Game::WaitingForDummy(_) => Err(BBError::OutOfTurn(None)),
            Game::Ended(_) => Err(BBError::GameHasEnded),
        }
    }
}

impl AutoPlayer {
    fn make_bid(&self, bid: Bid) -> PlayerEvent {
        let bid_event = BidEvent { player: self.seat, bid };
        PlayerEvent::Bid(bid_event)
    }

    fn play_card(&self, card: Card) -> PlayerEvent {
        let card_event = CardEvent {
            player: self.seat,
            card,
        };
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
