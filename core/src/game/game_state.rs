use crate::error::BBError;
use crate::game::bid_manager::BidManager;
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::{PlayerPosition, Vulnerability};
use crate::primitives::game_event::{BidEvent, CardEvent, DiscloseHandEvent, DummyUncoveredEvent};
use crate::primitives::game_result::GameResult;
use crate::primitives::trick::PlayedTrick;
use crate::primitives::Contract;
use crate::score::{Score, ScorePoints};

#[derive(Debug, Clone)]
pub struct GameState<Phase> {
    pub inner: Phase,
}
#[derive(Debug, Clone)]
pub struct Bidding {
    pub bid_manager: BidManager,
    pub hand_manager: HandManager,
}

#[derive(Debug, Clone)]
pub struct OpeningLead {
    pub bids: BidLine,
    pub trick_manager: TrickManager,
    pub hand_manager: HandManager,
    pub contract: Contract,
}

#[derive(Debug, Clone)]
pub struct WaitingForDummy {
    pub bids: BidLine,
    pub trick_manager: TrickManager,
    pub hand_manager: HandManager,
    pub contract: Contract,
}

#[derive(Debug, Clone)]
pub struct CardPlay {
    pub bids: BidLine,
    pub trick_manager: TrickManager,
    pub hand_manager: HandManager,
    pub contract: Contract,
}

#[derive(Debug, Clone)]
pub struct Ended {
    pub bids: BidLine,
    pub tricks: Vec<PlayedTrick>,
    pub hands: HandManager,
    pub result: GameResult,
}

impl GameState<Bidding> {
    pub fn next_to_play(&self) -> Option<PlayerPosition> {
        Some(self.inner.bid_manager.next_to_play())
    }

    pub fn bidding_has_ended(&self) -> bool {
        self.inner.bid_manager.bidding_has_ended()
    }

    pub fn validate_make_bid_event(&self, bid_event: BidEvent) -> Result<(), BBError> {
        self.validate_turn_order(bid_event.player)?;
        if !self.inner.bid_manager.is_valid_bid(&bid_event.bid) {
            return Err(BBError::InvalidBid(bid_event.bid));
        }
        Ok(())
    }

    pub fn validate_turn_order(&self, player: PlayerPosition) -> Result<(), BBError> {
        if let Some(next_to_play) = self.next_to_play() {
            if player == next_to_play {
                return Ok(());
            }
        }
        Err(BBError::OutOfTurn(self.next_to_play()))
    }

    pub fn process_disclose_hand_event(&mut self, event: DiscloseHandEvent) -> Result<(), BBError> {
        self.inner.hand_manager.register_known_hand(event.hand, event.seat)
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        self.validate_make_bid_event(bid_event)?;
        self.inner.bid_manager.bid(bid_event.bid)?;

        Ok(())
    }

    pub fn move_to_opening_lead(self, contract: Contract) -> GameState<OpeningLead> {
        let inner = OpeningLead {
            bids: self.inner.bid_manager.bid_line().clone(),
            trick_manager: TrickManager::new(contract.declarer + 1, contract.trump_suit()),
            hand_manager: self.inner.hand_manager,
            contract,
        };

        GameState { inner }
    }

    pub fn move_to_ended_without_card_play(self) -> GameState<Ended> {
        let inner = Ended {
            bids: self.inner.bid_manager.bid_line(),
            tricks: Vec::new(),
            hands: self.inner.hand_manager,
            result: GameResult::Unplayed,
        };

        GameState { inner }
    }
}

impl GameState<OpeningLead> {
    pub fn next_to_play(&self) -> Option<PlayerPosition> {
        Some(self.inner.trick_manager.next_to_play())
    }

    pub fn validate_turn_order(&self, player: PlayerPosition) -> Result<(), BBError> {
        if let Some(next_to_play) = self.next_to_play() {
            if player == next_to_play {
                return Ok(());
            }
        }
        Err(BBError::OutOfTurn(self.next_to_play()))
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        self.validate_play_card_event(card_event)?;
        self.inner.trick_manager.play(card_event.card)?;
        self.inner
            .hand_manager
            .process_play_card_event(card_event.card, card_event.player)?;
        Ok(())
    }

    pub fn validate_play_card_event(&self, card_event: CardEvent) -> Result<(), BBError> {
        self.validate_turn_order(card_event.player)?;
        self.inner
            .hand_manager
            .validate_play_card_event(card_event.card, card_event.player)?;

        if let Some(suit) = &self.inner.trick_manager.suit_to_follow() {
            if card_event.card.suit != *suit
                && self
                    .inner
                    .hand_manager
                    .player_is_known_to_have_cards_left_in_suit(card_event.player, *suit)
            {
                return Err(BBError::InvalidCard(card_event.card));
            }
        }
        Ok(())
    }

    pub fn move_to_waiting_for_dummy(self) -> GameState<WaitingForDummy> {
        let inner = WaitingForDummy {
            bids: self.inner.bids,
            trick_manager: self.inner.trick_manager,
            hand_manager: self.inner.hand_manager,
            contract: self.inner.contract,
        };

        GameState { inner }
    }
}

impl GameState<WaitingForDummy> {
    pub fn next_to_play(&self) -> Option<PlayerPosition> {
        Some(self.inner.trick_manager.next_to_play())
    }

    pub fn validate_turn_order(&self, player: PlayerPosition) -> Result<(), BBError> {
        if let Some(next_to_play) = self.next_to_play() {
            if player == next_to_play {
                return Ok(());
            }
        }
        Err(BBError::OutOfTurn(self.next_to_play()))
    }

    pub fn process_dummy_uncovered_event(&mut self, event: DummyUncoveredEvent) -> Result<(), BBError> {
        self.inner
            .hand_manager
            .register_known_hand(event.dummy, self.inner.contract.declarer.partner())?;

        Ok(())
    }

    pub fn move_to_card_play(self) -> GameState<CardPlay> {
        let inner = CardPlay {
            bids: self.inner.bids,
            trick_manager: self.inner.trick_manager,
            hand_manager: self.inner.hand_manager,
            contract: self.inner.contract,
        };

        GameState { inner }
    }
}

impl GameState<CardPlay> {
    pub fn next_to_play(&self) -> Option<PlayerPosition> {
        Some(self.inner.trick_manager.next_to_play())
    }

    pub fn validate_turn_order(&self, player: PlayerPosition) -> Result<(), BBError> {
        if let Some(next_to_play) = self.next_to_play() {
            if player == next_to_play {
                return Ok(());
            }
        }
        Err(BBError::OutOfTurn(self.next_to_play()))
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        self.validate_play_card_event(card_event)?;
        self.inner.trick_manager.play(card_event.card)?;
        self.inner
            .hand_manager
            .process_play_card_event(card_event.card, card_event.player)?;
        Ok(())
    }

    pub fn validate_play_card_event(&self, card_event: CardEvent) -> Result<(), BBError> {
        self.validate_turn_order(card_event.player)?;
        self.inner
            .hand_manager
            .validate_play_card_event(card_event.card, card_event.player)?;

        if let Some(suit) = &self.inner.trick_manager.suit_to_follow() {
            if card_event.card.suit != *suit
                && self
                    .inner
                    .hand_manager
                    .player_is_known_to_have_cards_left_in_suit(card_event.player, *suit)
            {
                return Err(BBError::InvalidCard(card_event.card));
            }
        }
        Ok(())
    }

    pub fn card_play_has_ended(&self) -> bool {
        self.inner.trick_manager.card_play_has_ended()
    }

    pub fn move_from_card_play_to_ended(self) -> GameState<Ended> {
        let tricks = self.inner.trick_manager.played_tricks().into();

        let result = self.calculate_game_result();

        let inner = Ended {
            bids: self.inner.bids,
            tricks,
            hands: self.inner.hand_manager,
            result,
        };

        GameState { inner }
    }

    fn calculate_game_result(&self) -> GameResult {
        GameResult::calculate_game_result(
            self.inner.contract,
            self.inner
                .trick_manager
                .tricks_won_by_axis(self.inner.contract.declarer),
        )
    }
}

impl GameState<Ended> {
    pub fn tricks_won_by_axis(&self, player: PlayerPosition) -> usize {
        self.inner
            .tricks
            .iter()
            .filter(|x| x.winner() == player || x.winner() == player.partner())
            .count()
    }

    pub fn calculate_score(&self, vulnerability: Vulnerability) -> ScorePoints {
        Score::score_result(self.inner.result, vulnerability)
    }
}
