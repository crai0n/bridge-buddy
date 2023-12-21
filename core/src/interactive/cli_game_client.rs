use crate::actors::game_client::GameClient;
use crate::engine::{Move, SelectMove};
use crate::error::BBError;
use crate::game::Game;
use crate::interactive::cli_move_selector::CliMoveSelector;
use crate::interactive::cli_presenter::CliPresenter;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::{BidEvent, CardEvent, GameEvent};
use crate::primitives::player_event::PlayerEvent;

#[allow(dead_code)]
pub struct CliGameClient {
    seat: Seat,
    game: Option<Game>,
    move_selector: CliMoveSelector,
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

impl CliGameClient {
    fn get_move(&self, game: &Game) -> Result<PlayerEvent, BBError> {
        let engine_move = self.move_selector.select_move(game)?;
        let player = game.next_to_play().unwrap();
        match engine_move {
            Move::Bid(bid) => Ok(PlayerEvent::Bid(BidEvent { player, bid })),
            Move::Card(card) => Ok(PlayerEvent::Card(CardEvent { player, card })),
        }
    }

    pub fn new(seat: Seat) -> Self {
        CliGameClient {
            seat,
            game: None,
            move_selector: CliMoveSelector::new(seat),
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
}

#[cfg(test)]
mod test {
    use crate::actors::game_client::GameClient;

    use crate::engine::bidding_engine::SelectBid;
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
            Game::Bidding(state) => player.move_selector.select_bid(state),
            _ => panic!(),
        };
    }
}
