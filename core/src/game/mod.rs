use crate::error::BBError;
use crate::game::card_manager::CardManager;
use crate::game::game_event::{DiscloseHandEvent, DummyUncoveredEvent, GameEndedEvent, GameEvent, MoveToCardPlayEvent};
use crate::game::player_event::{MakeBidEvent, PlayCardEvent, PlayerEvent};
use strum::IntoEnumIterator;

use game_phase::GamePhase;

use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::{Contract, Deal};
use crate::score::{Score, ScorePoints};

pub mod game_event;
pub mod player_event;
// pub mod player_queue_map;
pub mod card_manager;

pub mod game_phase;
pub mod game_state;

pub struct Game {
    deal: Deal,
    phase: GamePhase,
    bid_line: BidLine,
    tricks: Option<CardManager>,
    contract: Option<Contract>,
    declarer: Option<PlayerPosition>,
    history: Vec<GameEvent>,
}

impl Game {
    pub fn new_from_deal(deal: Deal) -> Self {
        Game {
            deal,
            phase: GamePhase::Setup,
            bid_line: BidLine::new(),
            tricks: None,
            contract: None,
            declarer: None,
            history: Vec::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), BBError> {
        match self.phase {
            GamePhase::Setup => {
                self.start_game();
                Ok(())
            }
            _ => Err(BBError::GameAlreadyStarted),
        }
    }

    fn start_game(&mut self) {
        self.phase = GamePhase::Bidding;
        let game_event = GameEvent::GameStarted;
        self.add_event_to_history(game_event);
        self.disclose_hands();
    }

    fn disclose_hands(&mut self) {
        for player in PlayerPosition::iter() {
            self.history.push(GameEvent::DiscloseHand(DiscloseHandEvent {
                board: self.deal.board,
                seat: player,
                hand: self.deal.hand_of(player).clone(),
            }))
        }
    }

    pub fn process_event(&mut self, event: PlayerEvent) -> Result<(), BBError> {
        match (self.phase, event) {
            (GamePhase::Bidding, PlayerEvent::MakeBid(bid_event)) => self.process_make_bid_event(bid_event),
            (GamePhase::CardPlay, PlayerEvent::PlayCard(card_event)) => self.process_play_card_event(card_event),
            (_, PlayerEvent::MakeBid(_)) => Err(BBError::InvalidPlayerEvent(event)),
            (_, PlayerEvent::PlayCard(_)) => Err(BBError::InvalidPlayerEvent(event)),
        }
    }

    fn process_make_bid_event(&mut self, bid_event: MakeBidEvent) -> Result<(), BBError> {
        self.validate_turn_order(bid_event.player)?;
        self.bid_line.bid(bid_event.bid)?;

        let event = GameEvent::from(PlayerEvent::MakeBid(bid_event));

        self.add_event_to_history(event);

        if self.bid_line.bidding_has_ended() {
            self.move_to_next_phase()
        }

        Ok(())
    }

    fn add_event_to_history(&mut self, event: GameEvent) {
        self.history.push(event);
    }

    fn move_to_next_phase(&mut self) {
        if let Some(contract) = self.bid_line.implied_contract() {
            let declarer = self.calculate_declarer_position();
            self.move_to_card_play(contract, declarer);
        } else {
            self.move_to_ended_without_card_play();
        }
    }

    fn calculate_declarer_position(&self) -> PlayerPosition {
        self.deal.dealer() + self.bid_line.implied_declarer_position().unwrap()
    }

    fn move_to_card_play(&mut self, contract: Contract, declarer: PlayerPosition) {
        self.set_up_card_play(contract, declarer);
        self.phase = GamePhase::CardPlay;
        let move_to_card_play_event = MoveToCardPlayEvent {
            final_contract: contract,
            declarer,
        };
        let event = GameEvent::MoveToCardPlay(move_to_card_play_event);

        self.add_event_to_history(event)
    }

    fn set_up_card_play(&mut self, contract: Contract, declarer: PlayerPosition) {
        self.contract = Some(contract);
        self.declarer = Some(declarer);
        self.tricks = Some(CardManager::new_with_deal_info(
            self.declarer.unwrap() + 1,
            contract.trump_suit(),
            &self.deal,
        ));
    }

    fn move_to_ended_without_card_play(&mut self) {
        self.phase = GamePhase::Ended;

        let game_ended_event = GameEndedEvent { score: Score::NO_SCORE };
        let event = GameEvent::GameEnded(game_ended_event);
        self.add_event_to_history(event);
    }

    fn process_play_card_event(&mut self, card_event: PlayCardEvent) -> Result<(), BBError> {
        self.validate_turn_order(card_event.player)?;
        self.tricks.as_mut().unwrap().play(card_event.card)?;

        let event = GameEvent::from(PlayerEvent::PlayCard(card_event));
        self.add_event_to_history(event);

        if self.tricks.as_ref().unwrap().count_played_cards() == 1 {
            self.uncover_dummy();
        } else if self.tricks.as_ref().unwrap().card_play_has_ended() {
            self.move_to_ended();
        }

        Ok(())
    }

    fn move_to_ended(&mut self) {
        self.phase = GamePhase::Ended;
        let event = GameEvent::GameEnded(GameEndedEvent {
            score: self.score().unwrap(),
        });
        self.add_event_to_history(event);
    }

    fn uncover_dummy(&mut self) {
        let dummy = self.declarer.unwrap().partner();
        let dummys_hand = self.deal.hand_of(dummy);

        self.history.push(GameEvent::DummyUncovered(DummyUncoveredEvent {
            dummy: dummys_hand.clone(),
        }));
    }

    fn validate_turn_order(&self, player: PlayerPosition) -> Result<(), BBError> {
        if let Some(current_turn) = self.current_turn() {
            if player == current_turn {
                return Ok(());
            }
        }
        Err(BBError::OutOfTurn(self.current_turn()))
    }

    fn current_turn(&self) -> Option<PlayerPosition> {
        match self.phase {
            GamePhase::Bidding => Some(self.deal.dealer() + self.bid_line.len()),
            GamePhase::CardPlay => Some(self.tricks.as_ref().unwrap().turn()),
            GamePhase::Setup => None,
            GamePhase::Ended => None,
        }
    }

    pub fn score(&self) -> Option<ScorePoints> {
        match self.phase {
            GamePhase::Ended => match (self.declarer, self.contract) {
                (Some(declarer), Some(contract)) => Some(Score::score(
                    contract,
                    self.tricks.as_ref().unwrap().tricks_won_by_axis(declarer),
                    declarer,
                    self.deal.board.is_vulnerable(declarer),
                )),
                _ => Some(Score::NO_SCORE),
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::error::BBError;
    use crate::game::player_event::{MakeBidEvent, PlayCardEvent, PlayerEvent};
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
        let game = Game::new_from_deal(deal);
        assert_eq!(game.phase, GamePhase::Setup)
    }

    #[test]
    fn start_game() {
        let mut rng = thread_rng();
        let deal = Deal::from_rng(&mut rng);
        let mut game = Game::new_from_deal(deal);
        assert_eq!(game.start(), Ok(()));
        assert_eq!(game.start(), Err(BBError::GameAlreadyStarted));
    }

    #[test]
    fn game_without_card_play() {
        let seed = 9000u64;
        let deal = Deal::from_u64_seed(seed);
        let mut game = Game::new_from_deal(deal);

        assert_eq!(game.start(), Ok(()));

        let bids = ["p", "p", "p", "p"];

        for &bid in bids.iter() {
            let bid_event = MakeBidEvent {
                player: game.current_turn().unwrap(),
                bid: Bid::from_str(bid).unwrap(),
            };
            let player_event = PlayerEvent::MakeBid(bid_event);
            game.process_event(player_event).unwrap();
        }
        // for event in game.history {
        //     println!("{:?}", event);
        // }
        assert_eq!(game.phase, GamePhase::Ended);
    }

    #[test]
    fn game_with_card_play() {
        let seed = 9000u64;
        let deal = Deal::from_u64_seed(seed);
        let mut game = Game::new_from_deal(deal.clone());

        // for hand in deal.hands.iter() {
        //     println!("{}", hand);
        // }

        assert_eq!(game.start(), Ok(()));

        let bids = ["p", "1NT", "p", "2C", "p", "2S", "p", "4S", "p", "p", "p"];

        for &bid in bids.iter() {
            let bid_event = MakeBidEvent {
                player: game.current_turn().unwrap(),
                bid: Bid::from_str(bid).unwrap(),
            };
            let player_event = PlayerEvent::MakeBid(bid_event);
            game.process_event(player_event).unwrap();
        }

        assert_eq!(game.phase, GamePhase::CardPlay);
        assert_eq!(game.current_turn(), Some(PlayerPosition::East));

        let cards = [
            "C2", "C7", "CK", "C3", "CJ", "S6", "C4", "C8", "D4", "D6", "D7", "DJ", "C6", "S9", "C5", "C9", "D5", "D8",
            "D9", "D2", "CA", "S2", "ST", "CT", "DT", "D3", "DK", "H2", "H5", "H3", "H7", "H4", "DQ", "DA", "H6", "S3",
            "S4", "SQ", "H8", "S8", "SK", "CQ", "SJ", "SA", "S7", "HJ", "HK", "HA", "S5", "HT", "HQ", "H9",
        ];

        for &card in cards.iter() {
            let card_event = PlayCardEvent {
                player: game.current_turn().unwrap(),
                card: Card::from_str(card).unwrap(),
            };
            let player_event = PlayerEvent::PlayCard(card_event);
            game.process_event(player_event).unwrap();
        }
        assert_eq!(game.phase, GamePhase::Ended);
        assert_eq!(game.tricks.as_ref().unwrap().count_played_cards(), 52);

        // for event in game.history.iter() {
        //     println!("{:?}", event);
        // }
        assert_eq!(
            game.tricks.as_ref().unwrap().tricks_won_by_axis(PlayerPosition::North),
            7
        );
    }
}
