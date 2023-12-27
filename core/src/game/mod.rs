// pub mod player_queue_map;
pub mod trick_manager;

pub mod game_state;
// mod bid_manager;
pub mod bid_manager;
pub mod hand_manager;
pub mod scoring;

use crate::error::BBError;
use crate::game::game_state::{Bidding, CardPlay, Ended, GameState, NextToPlay, OpeningLead, WaitingForDummy};

use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_event::{
    BidEvent, BiddingEndedEvent, CardEvent, DiscloseHandEvent, DummyUncoveredEvent, GameEndedEvent, GameEvent,
    NewGameEvent,
};
use crate::primitives::game_result::GameResult;
use crate::primitives::Hand;

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
            GameEvent::BiddingEnded(bidding_ended_event) => self.process_bidding_ended_event(bidding_ended_event),
            GameEvent::Card(card_event) => self.process_play_card_event(card_event),
            GameEvent::DummyUncovered(dummy_uncovered_event) => {
                self.process_dummy_uncovered_event(dummy_uncovered_event)
            }
            GameEvent::GameEnded(game_ended_event) => self.process_game_ended_event(game_ended_event),
        }
    }

    pub fn process_bidding_ended_event(&mut self, bidding_ended_event: BiddingEndedEvent) -> Result<(), BBError> {
        match self {
            Game::Bidding(state) => {
                if state.inner.bid_manager.implied_contract() != Some(bidding_ended_event.final_contract)
                    || !state.bidding_has_ended()
                {
                    Err(BBError::InvalidEvent(Box::new(GameEvent::BiddingEnded(
                        bidding_ended_event,
                    ))))?
                }
                let new_state = state.clone().move_to_opening_lead(bidding_ended_event.final_contract);
                *self = Game::OpeningLead(new_state);
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::BiddingEnded(
                bidding_ended_event,
            )))),
        }
    }

    pub fn process_game_ended_event(&mut self, game_ended_event: GameEndedEvent) -> Result<(), BBError> {
        match self {
            Game::Bidding(state) => {
                assert_eq!(game_ended_event.result, GameResult::Unplayed);
                let new_state = state.clone().move_to_ended_without_card_play();
                *self = Game::Ended(new_state);
            }
            Game::CardPlay(state) => {
                assert_eq!(state.calculate_game_result(), game_ended_event.result);
                let new_state = state.clone().move_from_card_play_to_ended();
                *self = Game::Ended(new_state);
            }
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::GameEnded(game_ended_event))))?,
        };
        Ok(())
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        match self {
            Game::Bidding(state) => state.process_make_bid_event(bid_event),
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::Bid(bid_event)))),
        }
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        match self {
            Game::OpeningLead(state) => {
                state.process_play_card_event(card_event)?;
                let new_state = state.clone().move_to_waiting_for_dummy();
                *self = Game::WaitingForDummy(new_state);
                Ok(())
            }
            Game::CardPlay(state) => {
                state.process_play_card_event(card_event)?;
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::Card(card_event)))),
        }
    }

    fn process_dummy_uncovered_event(&mut self, event: DummyUncoveredEvent) -> Result<(), BBError> {
        match self {
            Game::WaitingForDummy(state) => {
                state.process_dummy_uncovered_event(event)?;

                let new_state = state.clone().move_to_card_play();
                *self = Game::CardPlay(new_state);
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::DummyUncovered(event))))?,
        }
    }

    fn process_disclose_hand_event(&mut self, event: DiscloseHandEvent) -> Result<(), BBError> {
        match self {
            Game::Bidding(state) => state.process_disclose_hand_event(event),
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::DiscloseHand(event)))),
        }
    }

    pub fn new_from_board(board: Board) -> Self {
        let state = GameState::new(board);
        Game::Bidding(state)
    }

    pub fn from_new_game_event(event: NewGameEvent) -> Self {
        Self::new_from_board(event.board)
    }

    pub fn declarer(&self) -> Option<Seat> {
        match &self {
            Game::Bidding(_) => None,
            Game::OpeningLead(state) => Some(state.declarer()),
            Game::WaitingForDummy(state) => Some(state.declarer()),
            Game::CardPlay(state) => Some(state.declarer()),
            Game::Ended(state) => state.declarer(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::scoring::ScoreCalculator;
    use crate::game::Game;
    use crate::primitives::bid::Bid;
    use crate::primitives::deal::Seat;
    use crate::primitives::game_event::{
        BidEvent, BiddingEndedEvent, CardEvent, DummyUncoveredEvent, GameEndedEvent, GameEvent,
    };
    use crate::primitives::game_result::GameResult;
    use crate::primitives::{Card, Contract, Deal};
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
        match game {
            Game::Bidding(state) => {
                assert!(state.bidding_has_ended());
                assert_eq!(state.inner.bid_manager.implied_contract(), None);
            }
            _ => panic!(),
        }
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

        let final_contract = match &game {
            Game::Bidding(state) => {
                assert!(state.inner.bid_manager.bidding_has_ended());
                state.inner.bid_manager.implied_contract().unwrap()
            }
            _ => panic!(),
        };

        let game_event = GameEvent::BiddingEnded(BiddingEndedEvent { final_contract });

        game.process_game_event(game_event).unwrap();

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

        let result = match &game {
            Game::CardPlay(state) => state.calculate_game_result(),
            _ => panic!(),
        };
        let score = ScoreCalculator::score_result(result, deal.vulnerable());

        let game_event = GameEvent::GameEnded(GameEndedEvent { result, deal, score });
        game.process_game_event(game_event).unwrap();

        match game {
            Game::Ended(state) => {
                assert_eq!(state.inner.hands.count_played_cards(), 52);
                assert_eq!(
                    state.inner.result,
                    GameResult::Failed {
                        contract: Contract::from_str("N4S").unwrap(),
                        undertricks: 3
                    }
                );
            }
            _ => panic!(),
        }
    }
}
