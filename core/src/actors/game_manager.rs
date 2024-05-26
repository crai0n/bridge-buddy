use crate::error::BBError;
use crate::game::scoring::ScoreCalculator;
use crate::game::{GamePhaseState, GameState};
use crate::primitives::deal::seat::SEAT_ARRAY;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::{
    BiddingEndedEvent, DiscloseHandEvent, DummyUncoveredEvent, GameEndedEvent, GameEvent, NewGameEvent,
};
use crate::primitives::game_result::GameResult;
use crate::primitives::player_event::PlayerEvent;
use crate::primitives::{Contract, Deal};
use itertools::Itertools;

pub struct GameManager {
    deal: Deal<13>,
    game: Option<GameState>,
    history: Vec<GameEvent>,
}

impl GameManager {
    pub fn new_from_deal(deal: Deal<13>) -> Self {
        GameManager {
            deal,
            game: None,
            history: Vec::new(),
        }
    }

    pub fn history(&self) -> Vec<GameEvent> {
        self.history.iter().copied().collect_vec()
    }

    pub fn next_to_play(&self) -> Option<Seat> {
        match &self.game {
            None => None,
            Some(game) => game.next_to_play(),
        }
    }

    pub fn new() -> Self {
        let deal = Deal::random();
        Self::new_from_deal(deal)
    }

    pub fn start_game(&mut self) -> Result<(), BBError> {
        match self.game {
            Some(_) => Err(BBError::GameAlreadyStarted),
            None => {
                let new_game_event = NewGameEvent { board: self.deal.board };
                let game_event = GameEvent::NewGame(new_game_event);
                self.add_event_to_history(game_event);
                self.game = Some(GameState::from_new_game_event(new_game_event));
                self.disclose_hands();
                Ok(())
            }
        }
    }

    pub fn process_player_event(&mut self, event: PlayerEvent) -> Result<(), BBError> {
        match &mut self.game {
            None => Err(BBError::GameHasNotStarted)?,
            Some(game) => {
                let game_event = GameEvent::from(event);
                game.process_game_event(game_event)?;
                self.add_event_to_history(game_event);
            }
        }

        self.react_to_new_game_state();
        Ok(())
    }

    fn react_to_new_game_state(&mut self) {
        match &mut self.game.as_mut().unwrap() {
            GameState::Bidding(state) => {
                if state.bidding_has_ended() {
                    match state.implied_contract() {
                        Some(contract) => {
                            self.end_bidding(contract);
                        }
                        None => {
                            let result = GameResult::Unplayed;
                            self.end_game(result);
                        }
                    }
                }
            }
            GameState::WaitingForDummy(state) => {
                let dummy = state.dummy();
                self.disclose_dummy(dummy);
            }
            GameState::CardPlay(state) => {
                if state.card_play_has_ended() {
                    let result = state.calculate_game_result();
                    self.end_game(result);
                }
            }
            _ => (),
        }
    }

    fn end_bidding(&mut self, contract: Contract) {
        let bidding_ended_event = BiddingEndedEvent {
            final_contract: contract,
        };
        let game_event = GameEvent::BiddingEnded(bidding_ended_event);
        self.add_event_to_history(game_event);
        self.game.as_mut().unwrap().process_game_event(game_event).unwrap();
    }

    fn disclose_hands(&mut self) {
        for player in SEAT_ARRAY {
            let game_event = GameEvent::DiscloseHand(DiscloseHandEvent {
                seat: player,
                hand: *self.deal.hand_of(player),
            });
            self.add_event_to_history(game_event);
            self.game.as_mut().unwrap().process_game_event(game_event).unwrap();
        }
    }

    fn disclose_dummy(&mut self, dummy: Seat) {
        let dummy_uncovered_event = DummyUncoveredEvent {
            dummy: *self.deal.hand_of(dummy),
        };
        let game_event = GameEvent::DummyUncovered(dummy_uncovered_event);

        self.add_event_to_history(game_event);
        self.game.as_mut().unwrap().process_game_event(game_event).unwrap();
    }

    fn end_game(&mut self, result: GameResult) {
        let game_ended_event = GameEndedEvent {
            deal: self.deal,
            result,
            score: ScoreCalculator::score_result(result, self.deal.vulnerable()),
        };
        let event = GameEvent::GameEnded(game_ended_event);
        self.add_event_to_history(event);
        self.game.as_mut().unwrap().process_game_event(event).unwrap();
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
