use crate::actors::game_client::GameClient;
use crate::engine::mock_bridge_engine::MockBridgeEngine;
use crate::engine::{Move, SelectMove};
use crate::error::BBError;
use crate::game::Game;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::{CardEvent, GameEvent};
use crate::primitives::player_event::{BidEvent, PlayerEvent};

pub struct AutoGameClient {
    seat: Seat,
    game: Option<Game>,
    move_selector: MockBridgeEngine,
}

impl GameClient for AutoGameClient {
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

    fn get_move(&self) -> Result<PlayerEvent, BBError> {
        match &self.game {
            None => Err(BBError::GameHasNotStarted),
            Some(game) => match game.next_to_play() {
                Some(next_player) if next_player == self.seat => self.get_move(game),
                Some(next_player) if Some(next_player) == self.dummy() && self.can_play_for_dummy() => {
                    self.get_move(game)
                }
                Some(next_player) => Err(BBError::CannotPlayFor(next_player)),
                None => Err(BBError::OutOfTurn(None)),
            },
        }
    }
}

impl AutoGameClient {
    fn get_move(&self, game: &Game) -> Result<PlayerEvent, BBError> {
        let engine_move = self.move_selector.select_move(game)?;
        let player = game.next_to_play().unwrap();
        match engine_move {
            Move::Bid(bid) => Ok(PlayerEvent::Bid(BidEvent { player, bid })),
            Move::Card(card) => Ok(PlayerEvent::Card(CardEvent { player, card })),
        }
    }

    pub fn new(seat: Seat) -> Self {
        AutoGameClient {
            seat,
            game: None,
            move_selector: MockBridgeEngine::new(seat),
        }
    }

    pub fn can_play_for_dummy(&self) -> bool {
        match &self.game {
            Some(Game::CardPlay(state)) => state.declarer() == self.seat,
            _ => false,
        }
    }

    pub fn dummy(&self) -> Option<Seat> {
        match &self.game {
            Some(Game::CardPlay(state)) => Some(state.declarer().partner()),
            _ => None,
        }
    }

    pub fn seat(&self) -> Seat {
        self.seat
    }
}
