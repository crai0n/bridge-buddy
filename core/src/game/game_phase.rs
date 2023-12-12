use crate::error::BBError;
use crate::game::game_state::{Bidding, CardPlay, Ended, GameState, OpeningLead, WaitingForDummy};

use crate::primitives::deal::{Board, PlayerPosition};
use crate::primitives::game_event::{BidEvent, CardEvent, DiscloseHandEvent, DummyUncoveredEvent, GameEvent, NewGameEvent};
use crate::primitives::Hand;
use crate::score::ScorePoints;

#[derive(Debug, Clone)]
pub enum GamePhase {
    Bidding(GameState<Bidding>),
    OpeningLead(GameState<OpeningLead>),
    WaitingForDummy(GameState<WaitingForDummy>),
    CardPlay(GameState<CardPlay>),
    Ended(GameState<Ended>),
}

impl GamePhase {
    pub fn next_to_play(&self) -> Option<PlayerPosition> {
        match &self {
            GamePhase::Bidding(state) => Some(state.next_to_play()),
            GamePhase::OpeningLead(state) => Some(state.next_to_play()),
            GamePhase::WaitingForDummy(state) => Some(state.next_to_play()),
            GamePhase::CardPlay(state) => Some(state.next_to_play()),
            GamePhase::Ended(_) => None,
        }
    }

    pub fn board(&self) -> Board {
        match &self {
            GamePhase::Bidding(state) => state.board(),
            GamePhase::OpeningLead(state) => state.board(),
            GamePhase::WaitingForDummy(state) => state.board(),
            GamePhase::CardPlay(state) => state.board(),
            GamePhase::Ended(state) => state.board(),
        }
    }

    pub fn hand_of(&self, player: PlayerPosition) -> Result<Hand, BBError> {
        match &self {
            GamePhase::Bidding(state) => state.hand_of(player),
            GamePhase::OpeningLead(state) => state.hand_of(player),
            GamePhase::WaitingForDummy(state) => state.hand_of(player),
            GamePhase::CardPlay(state) => state.hand_of(player),
            GamePhase::Ended(state) => state.hand_of(player),
        }
    }

    pub fn validate_turn_order(&self, player: PlayerPosition) -> Result<(), BBError> {
        match &self {
            GamePhase::Bidding(state) => state.validate_turn_order(player),
            GamePhase::OpeningLead(state) => state.validate_turn_order(player),
            GamePhase::WaitingForDummy(state) => state.validate_turn_order(player),
            GamePhase::CardPlay(state) => state.validate_turn_order(player),
            GamePhase::Ended(_) => Err(BBError::GameHasEnded),
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
            _ => Err(BBError::InvalidEvent(event)),
        }
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        match self {
            GamePhase::Bidding(state) => {
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

            *self = GamePhase::OpeningLead(new_state);
        } else {
            let new_state = state.clone().move_to_ended_without_card_play();

            *self = GamePhase::Ended(new_state);
        }
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        match self {
            GamePhase::OpeningLead(state) => {
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
                *self = GamePhase::WaitingForDummy(new_state);
                Ok(())
            }
            GamePhase::CardPlay(state) => {
                state.process_play_card_event(card_event)?;
                if state.card_play_has_ended() {
                    let new_state = state.clone().move_from_card_play_to_ended();
                    *self = GamePhase::Ended(new_state);
                }
                Ok(())
            }
            _ => Err(BBError::InvalidEvent(GameEvent::Card(card_event))),
        }
    }

    fn process_dummy_uncovered_event(&mut self, event: DummyUncoveredEvent) -> Result<(), BBError> {
        match self {
            GamePhase::WaitingForDummy(state) => {
                state.process_dummy_uncovered_event(event)?;

                let state = state.clone();

                let new_state = state.move_to_card_play();

                *self = GamePhase::CardPlay(new_state);

                Ok(())
            }
            _ => Err(BBError::InvalidEvent(GameEvent::DummyUncovered(event)))?,
        }
    }

    fn process_disclose_hand_event(&mut self, event: DiscloseHandEvent) -> Result<(), BBError> {
        match self {
            GamePhase::Bidding(state) => state.process_disclose_hand_event(event),
            _ => Err(BBError::InvalidEvent(GameEvent::DiscloseHand(event))),
        }
    }

    pub fn new_from_board(board: Board) -> Self {
        let state = GameState::new(board);
        GamePhase::Bidding(state)
    }

    pub fn from_new_game_event(event: NewGameEvent) -> Self {
        Self::new_from_board(event.board)
    }

    pub fn score(&self) -> Option<ScorePoints> {
        match &self {
            GamePhase::Ended(state) => Some(state.calculate_score(self.board().vulnerable())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::game_phase::GamePhase;
    use crate::primitives::bid::Bid;
    use crate::primitives::deal::PlayerPosition;
    use crate::primitives::game_event::{BidEvent, CardEvent, DummyUncoveredEvent, GameEvent};
    use crate::primitives::{Card, Deal};
    use rand::thread_rng;
    use std::str::FromStr;

    #[test]
    fn init() {
        let mut rng = thread_rng();
        let deal = Deal::from_rng(&mut rng);
        let game = GamePhase::new_from_board(deal.board);
        assert!(matches!(game, GamePhase::Bidding(_)))
    }

    #[test]
    fn game_without_card_play() {
        let seed = 9000u64;
        let deal = Deal::from_u64_seed(seed);
        let mut game = GamePhase::new_from_board(deal.board);

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
        assert!(matches!(game, GamePhase::Ended(_)));
    }

    #[test]
    fn game_with_card_play() {
        let seed = 9000u64;
        let deal = Deal::from_u64_seed(seed);
        let mut game = GamePhase::new_from_board(deal.board);

        assert_eq!(game.next_to_play(), Some(PlayerPosition::West));

        let bids = ["p", "1NT", "p", "2C", "p", "2S", "p", "4S", "p", "p", "p"];

        for &bid in bids.iter() {
            let bid_event = BidEvent {
                player: game.next_to_play().unwrap(),
                bid: Bid::from_str(bid).unwrap(),
            };
            let game_event = GameEvent::Bid(bid_event);
            game.process_game_event(game_event).unwrap();
        }

        assert_eq!(game.next_to_play(), Some(PlayerPosition::East));

        let lead = Card::from_str("C2").unwrap();

        let card_event = CardEvent {
            player: game.next_to_play().unwrap(),
            card: lead,
        };
        let game_event = GameEvent::Card(card_event);
        game.process_game_event(game_event).unwrap();

        match &mut game {
            GamePhase::WaitingForDummy(state) => {
                let dummy = state.inner.contract.declarer.partner();
                let dummy_event = DummyUncoveredEvent {
                    dummy: *deal.hand_of(dummy),
                };
                let game_event = GameEvent::DummyUncovered(dummy_event);
                game.process_game_event(game_event).unwrap();
            }
            _ => panic!(),
        }

        assert!(matches!(game, GamePhase::CardPlay(_)));

        assert_eq!(game.next_to_play(), Some(PlayerPosition::South));

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
            GamePhase::Ended(state) => {
                assert_eq!(state.inner.hands.count_played_cards(), 52);
                assert_eq!(state.tricks_won_by_axis(PlayerPosition::North), 7);
            }
            _ => panic!(),
        }
    }
}
