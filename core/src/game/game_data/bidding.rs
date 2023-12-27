use crate::error::BBError;
use crate::game::bid_manager::BidManager;
use crate::game::game_data::ended::Ended;
use crate::game::game_data::opening_lead::OpeningLead;
use crate::game::game_data::{GameData, NextToPlay};
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_event::{BidEvent, DiscloseHandEvent};
use crate::primitives::game_result::GameResult;
use crate::primitives::{Contract, Hand};

#[derive(Debug, Clone)]
pub struct Bidding {
    pub bid_manager: BidManager,
    pub hand_manager: HandManager,
    pub board: Board,
}

impl Bidding {
    pub fn new(board: Board) -> Self {
        Bidding {
            bid_manager: BidManager::new(board.dealer()),
            hand_manager: HandManager::new(),
            board,
        }
    }
}

impl NextToPlay for GameData<Bidding> {
    fn next_to_play(&self) -> Seat {
        self.inner.bid_manager.next_to_play()
    }
}

impl GameData<Bidding> {
    pub fn new(board: Board) -> Self {
        let inner = Bidding::new(board);
        GameData { inner }
    }

    pub fn declarer(&self) -> Option<Seat> {
        None
    }

    pub fn hand_of(&self, player: Seat) -> Result<Hand, BBError> {
        self.inner.hand_manager.hand_of(player)
    }

    pub fn bidding_has_ended(&self) -> bool {
        self.inner.bid_manager.bidding_has_ended()
    }

    pub fn board(&self) -> Board {
        self.inner.board
    }

    pub fn validate_make_bid_event(&self, bid_event: BidEvent) -> Result<(), BBError> {
        self.validate_turn_order(bid_event.player)?;
        if !self.inner.bid_manager.is_valid_bid(&bid_event.bid) {
            return Err(BBError::InvalidBid(bid_event.bid));
        }
        Ok(())
    }

    pub fn process_disclose_hand_event(&mut self, event: DiscloseHandEvent) -> Result<(), BBError> {
        self.inner.hand_manager.register_known_hand(event.hand, event.seat)
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        self.validate_make_bid_event(bid_event)?;
        self.inner.bid_manager.bid(bid_event.bid)?;

        Ok(())
    }

    pub fn move_to_opening_lead(self, contract: Contract) -> GameData<OpeningLead> {
        let inner = OpeningLead {
            bids: self.inner.bid_manager.bid_line().clone(),
            trick_manager: TrickManager::new(contract.declarer + 1, contract.trump_suit()),
            hand_manager: self.inner.hand_manager,
            contract,
            board: self.inner.board,
        };

        GameData { inner }
    }

    pub fn move_to_ended_without_card_play(self) -> GameData<Ended> {
        let inner = Ended {
            bids: self.inner.bid_manager.bid_line(),
            tricks: Vec::new(),
            hands: self.inner.hand_manager,
            result: GameResult::Unplayed,
            board: self.inner.board,
        };

        GameData { inner }
    }
}
