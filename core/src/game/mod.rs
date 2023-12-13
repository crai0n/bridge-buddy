// pub mod player_queue_map;
pub mod trick_manager;

pub mod game_state;
// mod bid_manager;
pub mod bid_manager;
pub mod hand_manager;

use crate::error::BBError;
use crate::game::game_state::{Bidding, CardPlay, Ended, GameState, OpeningLead, WaitingForDummy};

use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_event::{
    BidEvent, CardEvent, DiscloseHandEvent, DummyUncoveredEvent, GameEvent, NewGameEvent,
};
use crate::primitives::Hand;
use crate::scoring::ScorePoints;

#[derive(Debug, Clone)]
pub enum Game {
    Bidding(GameState<Bidding>),
    OpeningLead(GameState<OpeningLead>),
    WaitingForDummy(GameState<WaitingForDummy>),
    CardPlay(GameState<CardPlay>),
    Ended(GameState<Ended>),
}

impl Game {
    pub fn next_to_play(&self) -> Option<Seat> {
        match &self {
            Game::Bidding(state) => Some(state.next_to_play()),
            Game::OpeningLead(state) => Some(state.next_to_play()),
            Game::WaitingForDummy(state) => Some(state.next_to_play()),
            Game::CardPlay(state) => Some(state.next_to_play()),
            Game::Ended(_) => None,
        }
    }

    pub fn board(&self) -> Board {
        match &self {
            Game::Bidding(state) => state.board(),
            Game::OpeningLead(state) => state.board(),
            Game::WaitingForDummy(state) => state.board(),
            Game::CardPlay(state) => state.board(),
            Game::Ended(state) => state.board(),
        }
    }

    pub fn hand_of(&self, player: Seat) -> Result<Hand, BBError> {
        match &self {
            Game::Bidding(state) => state.hand_of(player),
            Game::OpeningLead(state) => state.hand_of(player),
            Game::WaitingForDummy(state) => state.hand_of(player),
            Game::CardPlay(state) => state.hand_of(player),
            Game::Ended(state) => state.hand_of(player),
        }
    }

    pub fn validate_turn_order(&self, player: Seat) -> Result<(), BBError> {
        match &self {
            Game::Bidding(state) => state.validate_turn_order(player),
            Game::OpeningLead(state) => state.validate_turn_order(player),
            Game::WaitingForDummy(state) => state.validate_turn_order(player),
            Game::CardPlay(state) => state.validate_turn_order(player),
            Game::Ended(_) => Err(BBError::GameHasEnded),
        }
    }

    pub fn process_game_event(&mut self, event: GameEvent) -> Result<(), BBError> {
        match event {
            GameEvent::NewGame(_) => Err(BBError::GameAlreadyStarted),
            GameEvent::DiscloseHand(disclose_hand_event) => self.process_disclose_hand_event(disclose_hand_event),
            GameEvent::Bid(bid_event) => self.process_make_bid_event(bid_event),
            GameEvent::Card(card_event) => self.process_play_card_event(card_event),
            GameEvent::DummyUncovered(dummy_uncovered_event) => {
                self.process_dummy_uncovered_event(dummy_uncovered_event)
            }
            GameEvent::GameEnded(game_ended_event) => {
                let my_score = self.score().unwrap();
                assert_eq!(game_ended_event.score, my_score);
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(event)),
        }
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        match self {
            Game::Bidding(state) => {
                state.process_make_bid_event(bid_event)?;
                if state.bidding_has_ended() {
                    let new_state = state.clone();
                    self.move_from_bidding_to_next_phase_with_state(new_state);
                }
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(GameEvent::Bid(bid_event))),
        }
    }

    fn move_from_bidding_to_next_phase_with_state(&mut self, state: GameState<Bidding>) {
        if let Some(contract) = state.inner.bid_manager.implied_contract() {
            let new_state = state.clone().move_to_opening_lead(contract);

            *self = Game::OpeningLead(new_state);
        } else {
            let new_state = state.clone().move_to_ended_without_card_play();

            *self = Game::Ended(new_state);
        }
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        match self {
            Game::OpeningLead(state) => {
                state.process_play_card_event(card_event)?;

                let state = state.clone();

                let inner = WaitingForDummy {
                    bids: state.inner.bids,
                    trick_manager: state.inner.trick_manager,
                    hand_manager: state.inner.hand_manager,
                    contract: state.inner.contract,
                    board: state.inner.board,
                };

                let new_state = GameState { inner };
                *self = Game::WaitingForDummy(new_state);
                Ok(())
            }
            Game::CardPlay(state) => {
                state.process_play_card_event(card_event)?;
                if state.card_play_has_ended() {
                    let new_state = state.clone().move_from_card_play_to_ended();
                    *self = Game::Ended(new_state);
                }
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(GameEvent::Card(card_event))),
        }
    }

    fn process_dummy_uncovered_event(&mut self, event: DummyUncoveredEvent) -> Result<(), BBError> {
        match self {
            Game::WaitingForDummy(state) => {
                state.process_dummy_uncovered_event(event)?;

                let state = state.clone();

                let new_state = state.move_to_card_play();

                *self = Game::CardPlay(new_state);

                Ok(())
            }
            _ => Err(BBError::InvalidEvent(GameEvent::DummyUncovered(event)))?,
        }
    }

    fn process_disclose_hand_event(&mut self, event: DiscloseHandEvent) -> Result<(), BBError> {
        match self {
            Game::Bidding(state) => state.process_disclose_hand_event(event),
            _ => Err(BBError::InvalidEvent(GameEvent::DiscloseHand(event))),
        }
    }

    pub fn new_from_board(board: Board) -> Self {
        let state = GameState::new(board);
        Game::Bidding(state)
    }

    pub fn from_new_game_event(event: NewGameEvent) -> Self {
        Self::new_from_board(event.board)
    }

    pub fn score(&self) -> Option<ScorePoints> {
        match &self {
            Game::Ended(state) => Some(state.calculate_score()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::Game;
    use crate::primitives::bid::Bid;
    use crate::primitives::deal::Seat;
    use crate::primitives::game_event::{BidEvent, CardEvent, DummyUncoveredEvent, GameEvent};
    use crate::primitives::{Card, Deal};
    use rand::thread_rng;
    use std::str::FromStr;

    #[test]
    fn init() {
        let mut rng = thread_rng();
        let deal = Deal::from_rng(&mut rng);
        let game = Game::new_from_board(deal.board);
        assert!(matches!(game, Game::Bidding(_)))
    }

    #[test]
    fn game_without_card_play() {
        let seed = 9000u64;
        let deal = Deal::from_u64_seed(seed);
        let mut game = Game::new_from_board(deal.board);

        let bids = ["p", "p", "p", "p"];

        for &bid in bids.iter() {
            let bid_event = BidEvent {
                player: game.next_to_play().unwrap(),
                bid: Bid::from_str(bid).unwrap(),
            };
            let game_event = GameEvent::Bid(bid_event);
            game.process_game_event(game_event).unwrap();
        }
        // for event in game.history {
        //     println!("{:?}", event);
        // }
        assert!(matches!(game, Game::Ended(_)));
    }

    #[test]
    fn game_with_card_play() {
        let seed = 9000u64;
        let deal = Deal::from_u64_seed(seed);
        let mut game = Game::new_from_board(deal.board);

        assert_eq!(game.next_to_play(), Some(Seat::West));

        let bids = ["p", "1NT", "p", "2C", "p", "2S", "p", "4S", "p", "p", "p"];

        for &bid in bids.iter() {
            let bid_event = BidEvent {
                player: game.next_to_play().unwrap(),
                bid: Bid::from_str(bid).unwrap(),
            };
            let game_event = GameEvent::Bid(bid_event);
            game.process_game_event(game_event).unwrap();
        }

        assert_eq!(game.next_to_play(), Some(Seat::East));

        let lead = Card::from_str("C2").unwrap();

        let card_event = CardEvent {
            player: game.next_to_play().unwrap(),
            card: lead,
        };
        let game_event = GameEvent::Card(card_event);
        game.process_game_event(game_event).unwrap();

        match &mut game {
            Game::WaitingForDummy(state) => {
                let dummy = state.inner.contract.declarer.partner();
                let dummy_event = DummyUncoveredEvent {
                    dummy: *deal.hand_of(dummy),
                };
                let game_event = GameEvent::DummyUncovered(dummy_event);
                game.process_game_event(game_event).unwrap();
            }
            _ => panic!(),
        }

        assert!(matches!(game, Game::CardPlay(_)));

        assert_eq!(game.next_to_play(), Some(Seat::South));

        let cards = [
            "C7", "CK", "C3", "CJ", "S6", "C4", "C8", "D4", "D6", "D7", "DJ", "C6", "S9", "C5", "C9", "D5", "D8", "D9",
            "D2", "CA", "S2", "ST", "CT", "DT", "D3", "DK", "H2", "H5", "H3", "H7", "H4", "DQ", "DA", "H6", "S3", "S4",
            "SQ", "H8", "S8", "SK", "CQ", "SJ", "SA", "S7", "HJ", "HK", "HA", "S5", "HT", "HQ", "H9",
        ];

        for &card in cards.iter() {
            let card_event = CardEvent {
                player: game.next_to_play().unwrap(),
                card: Card::from_str(card).unwrap(),
            };
            let game_event = GameEvent::Card(card_event);
            game.process_game_event(game_event).unwrap();
        }

        match game {
            Game::Ended(state) => {
                assert_eq!(state.inner.hands.count_played_cards(), 52);
                assert_eq!(state.tricks_won_by_axis(Seat::North), 7);
            }
            _ => panic!(),
        }
    }
}
