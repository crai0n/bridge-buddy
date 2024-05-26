// pub mod player_queue_map;
pub mod trick_manager;

pub mod game_phase_states;
// mod bid_manager;
pub mod bid_manager;
pub mod hand_manager;
pub mod scoring;

use crate::error::BBError;
use crate::game::game_phase_states::{
    BiddingState, CardPlayState, EndedState, GamePhaseState, NextToPlay, OpeningLeadState, WaitingForDummyState,
};

use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_event::{
    BidEvent, BiddingEndedEvent, CardEvent, DiscloseHandEvent, DummyUncoveredEvent, GameEndedEvent, GameEvent,
    NewGameEvent,
};
use crate::primitives::game_result::GameResult;
use crate::primitives::Hand;

#[derive(Debug, Clone)]
pub enum GameState {
    Bidding(BiddingState),
    OpeningLead(OpeningLeadState),
    WaitingForDummy(WaitingForDummyState),
    CardPlay(CardPlayState),
    Ended(EndedState),
}

impl GameState {
    pub fn next_to_play(&self) -> Option<Seat> {
        match &self {
            GameState::Bidding(state) => Some(state.next_to_play()),
            GameState::OpeningLead(state) => Some(state.next_to_play()),
            GameState::WaitingForDummy(state) => Some(state.next_to_play()),
            GameState::CardPlay(state) => Some(state.next_to_play()),
            GameState::Ended(_) => None,
        }
    }

    pub fn board(&self) -> Board {
        match &self {
            GameState::Bidding(state) => state.board(),
            GameState::OpeningLead(state) => state.board(),
            GameState::WaitingForDummy(state) => state.board(),
            GameState::CardPlay(state) => state.board(),
            GameState::Ended(state) => state.board(),
        }
    }

    pub fn hand_of(&self, player: Seat) -> Result<Hand<13>, BBError> {
        match &self {
            GameState::Bidding(state) => state.hand_of(player),
            GameState::OpeningLead(state) => state.hand_of(player),
            GameState::WaitingForDummy(state) => state.hand_of(player),
            GameState::CardPlay(state) => state.hand_of(player),
            GameState::Ended(state) => state.hand_of(player),
        }
    }

    pub fn validate_turn_order(&self, player: Seat) -> Result<(), BBError> {
        match &self {
            GameState::Bidding(state) => state.validate_turn_order(player),
            GameState::OpeningLead(state) => state.validate_turn_order(player),
            GameState::WaitingForDummy(state) => state.validate_turn_order(player),
            GameState::CardPlay(state) => state.validate_turn_order(player),
            GameState::Ended(_) => Err(BBError::GameHasEnded),
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
            GameState::Bidding(state) => {
                if state.implied_contract() != Some(bidding_ended_event.final_contract) || !state.bidding_has_ended() {
                    Err(BBError::InvalidEvent(Box::new(GameEvent::BiddingEnded(
                        bidding_ended_event,
                    ))))?
                }
                let new_state = state.clone().move_to_opening_lead(bidding_ended_event.final_contract);
                *self = GameState::OpeningLead(new_state);
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::BiddingEnded(
                bidding_ended_event,
            )))),
        }
    }

    pub fn process_game_ended_event(&mut self, game_ended_event: GameEndedEvent) -> Result<(), BBError> {
        match self {
            GameState::Bidding(state) => {
                assert_eq!(game_ended_event.result, GameResult::Unplayed);
                let new_state = state.clone().move_to_ended_without_card_play();
                *self = GameState::Ended(new_state);
            }
            GameState::CardPlay(state) => {
                assert_eq!(state.calculate_game_result(), game_ended_event.result);
                let new_state = state.clone().move_from_card_play_to_ended();
                *self = GameState::Ended(new_state);
            }
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::GameEnded(game_ended_event))))?,
        };
        Ok(())
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        match self {
            GameState::Bidding(state) => state.process_make_bid_event(bid_event),
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::Bid(bid_event)))),
        }
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        match self {
            GameState::OpeningLead(state) => {
                state.process_play_card_event(card_event)?;
                let new_state = state.clone().move_to_waiting_for_dummy();
                *self = GameState::WaitingForDummy(new_state);
                Ok(())
            }
            GameState::CardPlay(state) => {
                state.process_play_card_event(card_event)?;
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::Card(card_event)))),
        }
    }

    fn process_dummy_uncovered_event(&mut self, event: DummyUncoveredEvent) -> Result<(), BBError> {
        match self {
            GameState::WaitingForDummy(state) => {
                state.process_dummy_uncovered_event(event)?;

                let new_state = state.clone().move_to_card_play();
                *self = GameState::CardPlay(new_state);
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::DummyUncovered(event))))?,
        }
    }

    fn process_disclose_hand_event(&mut self, event: DiscloseHandEvent) -> Result<(), BBError> {
        match self {
            GameState::Bidding(state) => state.process_disclose_hand_event(event),
            _ => Err(BBError::InvalidEvent(Box::new(GameEvent::DiscloseHand(event)))),
        }
    }

    pub fn new_from_board(board: Board) -> Self {
        let state = BiddingState::new(board);
        GameState::Bidding(state)
    }

    pub fn from_new_game_event(event: NewGameEvent) -> Self {
        Self::new_from_board(event.board)
    }

    pub fn declarer(&self) -> Option<Seat> {
        match &self {
            GameState::Bidding(_) => None,
            GameState::OpeningLead(state) => Some(state.declarer()),
            GameState::WaitingForDummy(state) => Some(state.declarer()),
            GameState::CardPlay(state) => Some(state.declarer()),
            GameState::Ended(state) => state.declarer(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::scoring::ScoreCalculator;
    use crate::game::{GamePhaseState, GameState};
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
        let deal: Deal<13> = Deal::random_from_rng(&mut rng);
        let game = GameState::new_from_board(deal.board);
        assert!(matches!(game, GameState::Bidding(_)))
    }

    #[test]
    fn game_without_card_play() {
        let seed = 9000u64;
        let deal: Deal<13> = Deal::from_u64_seed(seed);
        let mut game = GameState::new_from_board(deal.board);

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
            GameState::Bidding(state) => {
                assert!(state.bidding_has_ended());
                assert_eq!(state.implied_contract(), None);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn game_with_card_play() {
        let seed = 9000u64;
        let deal = Deal::from_u64_seed(seed);
        let mut game = GameState::new_from_board(deal.board);

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
            GameState::Bidding(state) => {
                assert!(state.bidding_has_ended());
                state.implied_contract().unwrap()
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
            GameState::WaitingForDummy(state) => {
                let dummy = state.dummy();
                let dummy_event = DummyUncoveredEvent {
                    dummy: *deal.hand_of(dummy),
                };
                let game_event = GameEvent::DummyUncovered(dummy_event);
                game.process_game_event(game_event).unwrap();
            }
            _ => panic!(),
        }

        assert!(matches!(game, GameState::CardPlay(_)));

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
            GameState::CardPlay(state) => state.calculate_game_result(),
            _ => panic!(),
        };
        let score = ScoreCalculator::score_result(result, deal.vulnerable());

        let game_event = GameEvent::GameEnded(GameEndedEvent { result, deal, score });
        game.process_game_event(game_event).unwrap();

        match game {
            GameState::Ended(state) => {
                assert_eq!(state.hands.count_played_cards(), 52);
                assert_eq!(
                    state.result,
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
