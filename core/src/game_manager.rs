use crate::error::BBError;
use crate::game::game_event::{DiscloseHandEvent, DummyUncoveredEvent, GameEndedEvent, GameEvent, NewGameEvent};
use crate::game::game_phase::GamePhase;
use crate::game::Game;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::Deal;
use strum::IntoEnumIterator;

pub struct GameManager {
    deal: Deal,
    game: Option<Game>,
    history: Vec<GameEvent>,
}

impl GameManager {
    pub fn new_from_deal(deal: Deal) -> Self {
        GameManager {
            deal,
            game: None,
            history: Vec::new(),
        }
    }

    pub fn new() -> Self {
        let deal = Deal::new();
        Self::new_from_deal(deal)
    }

    pub fn start_game(&mut self) -> Result<(), BBError> {
        match self.game {
            Some(_) => Err(BBError::GameAlreadyStarted),
            None => {
                let new_game_event = NewGameEvent { board: self.deal.board };
                let game_event = GameEvent::NewGame(new_game_event);
                self.add_event_to_history(game_event);
                self.game = Some(Game::from_new_game_event(new_game_event));
                self.disclose_hands();
                Ok(())
            }
        }
    }

    pub fn process_event(&mut self, event: GameEvent) -> Result<(), BBError> {
        match &mut self.game {
            None => Err(BBError::GameHasNotStarted)?,
            Some(game) => match event {
                GameEvent::Bid(_) => {
                    game.process_event(event)?;
                }
                GameEvent::Card(_) => {
                    game.process_event(event)?;
                }
                _ => return Err(BBError::InvalidEvent(event)),
            },
        }

        self.react_to_new_game_state();
        Ok(())
    }

    fn react_to_new_game_state(&mut self) {
        match &mut self.game.as_mut().unwrap().game_phase() {
            GamePhase::WaitingForDummy(state) => {
                let dummy = state.inner.contract.declarer.partner();
                self.disclose_dummy(dummy);
            }
            GamePhase::Ended(_) => self.finalize_result(),
            _ => (),
        }
    }

    fn disclose_hands(&mut self) {
        for player in PlayerPosition::iter() {
            let game_event = GameEvent::DiscloseHand(DiscloseHandEvent {
                seat: player,
                hand: *self.deal.hand_of(player),
            });
            self.add_event_to_history(game_event);
            self.game.as_mut().unwrap().process_event(game_event).unwrap();
        }
    }

    fn disclose_dummy(&mut self, dummy: PlayerPosition) {
        let dummy_uncovered_event = DummyUncoveredEvent {
            dummy: *self.deal.hand_of(dummy),
        };
        let game_event = GameEvent::DummyUncovered(dummy_uncovered_event);

        self.add_event_to_history(game_event);
        self.game.as_mut().unwrap().process_event(game_event).unwrap();
    }

    fn finalize_result(&mut self) {
        let game_ended_event = GameEndedEvent {
            score: self.game.as_mut().unwrap().score().unwrap(),
        };
        let event = GameEvent::GameEnded(game_ended_event);
        self.add_event_to_history(event);
    }

    fn add_event_to_history(&mut self, event: GameEvent) {
        self.history.push(event);
    }
}
impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}
