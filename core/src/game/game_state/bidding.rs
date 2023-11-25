use crate::error::BBError;
use crate::game::bid_manager::BidManager;
use crate::game::game_state::ended::Ended;
use crate::game::game_state::opening_lead::OpeningLead;
use crate::game::game_state::GameState;
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::game_event::{BidEvent, DiscloseHandEvent};
use crate::primitives::game_result::GameResult;
use crate::primitives::Contract;

#[derive(Debug, Clone)]
pub struct Bidding {
    pub bid_manager: BidManager,
    pub hand_manager: HandManager,
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
