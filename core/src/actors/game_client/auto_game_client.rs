use crate::actors::game_client::GameClient;
use crate::engine::auto_move_finder::AutoMoveFinder;
use crate::engine::MoveFinder;
use crate::error::BBError;
use crate::game::Game;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::GameEvent;
use crate::primitives::player_event::PlayerEvent;

pub struct AutoGameClient {
    seat: Seat,
    game: Option<Game>,
    move_finder: AutoMoveFinder,
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
        self.get_move_for(self.seat)
    }

    fn get_dummy_move(&self) -> Result<PlayerEvent, BBError> {
        self.get_move_for(self.seat.partner())
    }
}

impl AutoGameClient {
    fn get_move_for(&self, seat: Seat) -> Result<PlayerEvent, BBError> {
        self.move_finder.find_move_for(self.game.as_ref().unwrap(), seat)
    }

    pub fn new(seat: Seat) -> Self {
        AutoGameClient {
            seat,
            game: None,
            move_finder: AutoMoveFinder::new(seat),
        }
    }
}
