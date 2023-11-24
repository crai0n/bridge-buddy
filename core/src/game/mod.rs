use crate::error::BBError;
use crate::game::game_event::{BidEvent, CardEvent, GameEvent, NewGameEvent};

use crate::game::bid_manager::BidManager;
use crate::game::game_state::GameState;
use game_phase::GamePhase;

use crate::primitives::deal::{Board, PlayerPosition};
use crate::primitives::Contract;
use crate::score::{Score, ScorePoints};

pub mod game_event;
// pub mod player_queue_map;
pub mod trick_manager;

pub mod game_phase;
pub mod game_state;
// mod bid_manager;
pub mod bid_manager;
pub mod game_manager;
pub mod hand_manager;

pub struct Game {
    board: Board,
    game_phase: GamePhase,
}

impl Game {
    pub fn new_from_board(board: Board) -> Self {
        let state = GameState {
            bid_manager: BidManager::new(board.dealer()),
            tricks: None,
            hands: None,
            contract: None,
            declarer: None,
        };
        Game {
            board,
            game_phase: GamePhase::Bidding(state),
        }
    }

    pub fn from_new_game_event(event: NewGameEvent) -> Self {
        Self::new_from_board(event.board)
    }

    pub fn process_event(&mut self, event: GameEvent) -> Result<(), BBError> {
        match (&mut self.game_phase, event) {
            (_, GameEvent::NewGame(_)) => Err(BBError::GameAlreadyStarted),
            (GamePhase::Bidding(state), GameEvent::DiscloseHand(disclose_hand_event)) => {
                state.process_disclose_hand_event(disclose_hand_event)
            }
            (GamePhase::Bidding(_state), GameEvent::Bid(bid_event)) => {
                self.game_phase.process_make_bid_event(bid_event)
            }
            (GamePhase::OpeningLead(state), GameEvent::Card(card_event)) => {
                state.process_play_card_event(card_event)?;
                self.game_phase = GamePhase::WaitingForDummy(state.clone());
                Ok(())
            }
            (GamePhase::WaitingForDummy(state), GameEvent::DummyUncovered(dummy_uncovered_event)) => {
                state.process_dummy_uncovered_event(dummy_uncovered_event)?;
                self.game_phase = GamePhase::CardPlay(state.clone());
                Ok(())
            }
            (GamePhase::CardPlay(_state), GameEvent::Card(card_event)) => {
                self.game_phase.process_play_card_event(card_event)
            }
            (_, GameEvent::Bid(_)) => Err(BBError::InvalidEvent(event)),
            (_, GameEvent::Card(_)) => Err(BBError::InvalidEvent(event)),
            _ => Err(BBError::InvalidEvent(event)),
        }
    }

    fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        self.game_phase.process_make_bid_event(bid_event)?;

        if self.game_phase.bidding_has_ended() {
            self.move_from_bidding_to_next_phase();
        }

        Ok(())
    }

    fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        self.game_phase.process_play_card_event(card_event)?;

        if self.game_phase.card_play_has_ended() {
            self.move_to_ended();
        }

        Ok(())
    }

    fn move_from_bidding_to_next_phase(&mut self) {
        match &self.game_phase {
            GamePhase::Bidding(state) => {
                if let Some(contract) = state.bid_manager.implied_contract() {
                    let declarer = state.bid_manager.implied_declarer().unwrap();
                    self.move_to_card_play(contract, declarer);
                } else {
                    self.move_to_ended_without_card_play();
                }
            }
            _ => panic!(),
        }
    }

    fn move_to_card_play(&mut self, contract: Contract, declarer: PlayerPosition) {
        self.game_phase.set_up_card_play(contract, declarer);

        if let GamePhase::Bidding(state) = &self.game_phase {
            self.game_phase = GamePhase::OpeningLead(state.clone());
        } else {
            unreachable!()
        }
    }

    fn move_to_ended_without_card_play(&mut self) {
        if let GamePhase::Bidding(state) = &self.game_phase {
            self.game_phase = GamePhase::Ended(state.clone());
        } else {
            unreachable!()
        }
    }

    fn move_to_ended(&mut self) {
        if let GamePhase::CardPlay(state) = &mut self.game_phase {
            self.game_phase = GamePhase::Ended(state.clone());
        }
    }

    fn validate_turn_order(&self, player: PlayerPosition) -> Result<(), BBError> {
        self.game_phase.validate_turn_order(player)
    }

    fn next_to_play(&self) -> Option<PlayerPosition> {
        self.game_phase.next_to_play()
    }

    pub fn score(&self) -> Option<ScorePoints> {
        match &self.game_phase {
            GamePhase::Ended(state) => match (state.declarer, state.contract) {
                (Some(declarer), Some(contract)) => Some(Score::score(
                    contract,
                    state.tricks_won_by_axis(declarer),
                    declarer,
                    self.board.is_vulnerable(declarer),
                )),
                _ => Some(Score::NO_SCORE),
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::game_event::{BidEvent, CardEvent, DummyUncoveredEvent, GameEvent};
    use crate::game::{game_phase::GamePhase, Game};
    use crate::primitives::bid::Bid;
    use crate::primitives::deal::PlayerPosition;
    use crate::primitives::{Card, Deal};
    use rand::thread_rng;
    use std::str::FromStr;

    #[test]
    fn init() {
        let mut rng = thread_rng();
        let deal = Deal::from_rng(&mut rng);
        let game = Game::new_from_board(deal.board);
        assert!(matches!(game.game_phase, GamePhase::Bidding(_)))
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
            game.process_event(game_event).unwrap();
        }
        // for event in game.history {
        //     println!("{:?}", event);
        // }
        assert!(matches!(game.game_phase, GamePhase::Ended(_)));
    }

    #[test]
    fn game_with_card_play() {
        let seed = 9000u64;
        let deal = Deal::from_u64_seed(seed);
        let mut game = Game::new_from_board(deal.board);

        let bids = ["p", "1NT", "p", "2C", "p", "2S", "p", "4S", "p", "p", "p"];

        for &bid in bids.iter() {
            let bid_event = BidEvent {
                player: game.next_to_play().unwrap(),
                bid: Bid::from_str(bid).unwrap(),
            };
            let game_event = GameEvent::Bid(bid_event);
            game.process_event(game_event).unwrap();
        }

        assert_eq!(game.next_to_play(), Some(PlayerPosition::East));

        let lead = Card::from_str("C2").unwrap();

        let card_event = CardEvent {
            player: game.next_to_play().unwrap(),
            card: lead,
        };
        let game_event = GameEvent::Card(card_event);
        game.process_event(game_event).unwrap();

        match &mut game.game_phase {
            GamePhase::WaitingForDummy(state) => {
                let dummy = state.declarer.unwrap().partner();
                let dummy_event = DummyUncoveredEvent {
                    dummy: *deal.hand_of(dummy),
                };
                let game_event = GameEvent::DummyUncovered(dummy_event);
                game.process_event(game_event).unwrap();
            }
            _ => panic!(),
        }

        assert!(matches!(game.game_phase, GamePhase::CardPlay(_)));

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
            game.process_event(game_event).unwrap();
        }

        match game.game_phase {
            GamePhase::Ended(state) => {
                assert_eq!(state.hands.as_ref().unwrap().count_played_cards(), 52);
                assert_eq!(
                    state.tricks.as_ref().unwrap().tricks_won_by_axis(PlayerPosition::North),
                    7
                );
            }
            _ => panic!(),
        }
    }
}
