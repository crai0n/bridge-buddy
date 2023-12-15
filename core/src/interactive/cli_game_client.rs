use crate::actors::game_client::GameClient;
use crate::engine::MoveFinder;
use crate::error::BBError;
use crate::game::Game;
use crate::interactive::cli_move_finder::CliMoveFinder;
use crate::interactive::cli_presenter::CliPresenter;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::GameEvent;
use crate::primitives::player_event::PlayerEvent;

#[allow(dead_code)]
pub struct CliGameClient {
    seat: Seat,
    game: Option<Game>,
    presenter: CliPresenter,
    move_finder: CliMoveFinder,
}

impl GameClient for CliGameClient {
    fn process_game_event(&mut self, event: GameEvent) -> Result<(), BBError> {
        match event {
            GameEvent::NewGame(new_game_event) => {
                self.game = Some(Game::from_new_game_event(new_game_event));
                CliPresenter::print_game_event_to_console(GameEvent::NewGame(new_game_event));
                Ok(())
            }
            _ => match &mut self.game {
                None => Err(BBError::GameHasNotStarted)?,
                Some(game) => {
                    game.process_game_event(event)?;
                    CliPresenter::print_game_event_to_console(event);
                    Ok(())
                }
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

impl CliGameClient {
    fn get_move_for(&self, seat: Seat) -> Result<PlayerEvent, BBError> {
        self.move_finder.find_move_for(self.game.as_ref().unwrap(), seat)
    }

    pub fn new(seat: Seat) -> Self {
        CliGameClient {
            seat,
            game: None,
            presenter: CliPresenter {},
            move_finder: CliMoveFinder::new(seat),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::actors::game_client::GameClient;
    use crate::engine::MoveFinder;
    use crate::game::Game;
    use crate::interactive::cli_game_client::CliGameClient;
    use crate::primitives::deal::Board;
    use crate::primitives::game_event::GameEvent::DiscloseHand;
    use crate::primitives::game_event::{DiscloseHandEvent, GameEvent, NewGameEvent};
    use crate::primitives::Hand;
    use std::str::FromStr;

    #[allow(dead_code)]
    fn display_hand() {
        let hand = Hand::from_str("S:AKQ,H:AKQ,D:AKQ,C:AKQJ").unwrap();
        let board = Board::from_number(5);

        let seat = board.dealer();

        let mut player = CliGameClient::new(seat);

        let ng_event = NewGameEvent { board };
        let event = GameEvent::NewGame(ng_event);

        player.process_game_event(event).unwrap();

        let hand_event = DiscloseHand(DiscloseHandEvent { seat, hand });

        player.process_game_event(hand_event).unwrap();

        let _bid = match player.game.as_ref().unwrap() {
            Game::Bidding(state) => player.move_finder.find_bid(state),
            _ => panic!(),
        };
    }
}
