use crate::error::BBError;
use crate::game::bid_manager::BidManager;
use crate::game::game_data::ended::EndedState;
use crate::game::game_data::opening_lead::OpeningLeadState;
use crate::game::game_data::{GameData, NextToPlay};
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::game::GamePhaseState;
use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_event::{BidEvent, DiscloseHandEvent};
use crate::primitives::game_result::GameResult;
use crate::primitives::{Contract, Hand};

#[derive(Debug, Clone)]
pub struct BiddingState {
    pub bid_manager: BidManager,
    pub hand_manager: HandManager,
    pub board: Board,
}

impl GamePhaseState for BiddingState {
    fn implied_contract(&self) -> Option<Contract> {
        self.bid_manager.implied_contract()
    }
}

impl BiddingState {
    pub fn new(board: Board) -> Self {
        BiddingState {
            bid_manager: BidManager::new(board.dealer()),
            hand_manager: HandManager::new(),
            board,
        }
    }

    pub fn declarer(&self) -> Option<Seat> {
        None
    }

    pub fn hand_of(&self, player: Seat) -> Result<Hand<13>, BBError> {
        self.hand_manager.hand_of(player)
    }

    pub fn bidding_has_ended(&self) -> bool {
        self.bid_manager.bidding_has_ended()
    }

    pub fn board(&self) -> Board {
        self.board
    }

    pub fn validate_make_bid_event(&self, bid_event: BidEvent) -> Result<(), BBError> {
        self.validate_turn_order(bid_event.player)?;
        if !self.bid_manager.is_valid_bid(&bid_event.bid) {
            return Err(BBError::InvalidBid(bid_event.bid));
        }
        Ok(())
    }

    pub fn process_disclose_hand_event(&mut self, event: DiscloseHandEvent) -> Result<(), BBError> {
        self.hand_manager.register_known_hand(event.hand, event.seat)
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        self.validate_make_bid_event(bid_event)?;
        self.bid_manager.bid(bid_event.bid)?;

        Ok(())
    }

    pub fn move_to_opening_lead(self, contract: Contract) -> OpeningLeadState {
        OpeningLeadState {
            bids: self.bid_manager.bid_line().clone(),
            trick_manager: TrickManager::new(contract.declarer + 1, contract.trump_suit()),
            hand_manager: self.hand_manager,
            contract,
            board: self.board,
        }
    }

    pub fn move_to_ended_without_card_play(self) -> EndedState {
        EndedState {
            bids: self.bid_manager.bid_line(),
            tricks: Vec::new(),
            hands: self.hand_manager,
            result: GameResult::Unplayed,
            board: self.board,
        }
    }
}

impl NextToPlay for GameData<BiddingState> {
    fn next_to_play(&self) -> Seat {
        self.inner.bid_manager.next_to_play()
    }
}

impl NextToPlay for BiddingState {
    fn next_to_play(&self) -> Seat {
        self.bid_manager.next_to_play()
    }
}

impl GameData<BiddingState> {
    pub fn new(board: Board) -> Self {
        let inner = BiddingState::new(board);
        GameData { inner }
    }

    pub fn declarer(&self) -> Option<Seat> {
        None
    }

    pub fn hand_of(&self, player: Seat) -> Result<Hand<13>, BBError> {
        self.inner.hand_of(player)
    }

    pub fn bidding_has_ended(&self) -> bool {
        self.inner.bidding_has_ended()
    }

    pub fn board(&self) -> Board {
        self.inner.board()
    }

    pub fn validate_make_bid_event(&self, bid_event: BidEvent) -> Result<(), BBError> {
        self.inner.validate_make_bid_event(bid_event)
    }

    pub fn process_disclose_hand_event(&mut self, event: DiscloseHandEvent) -> Result<(), BBError> {
        self.inner.process_disclose_hand_event(event)
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        self.inner.process_make_bid_event(bid_event)
    }

    pub fn move_to_opening_lead(self, contract: Contract) -> OpeningLeadState {
        self.inner.move_to_opening_lead(contract)
    }

    pub fn move_to_ended_without_card_play(self) -> EndedState {
        self.inner.move_to_ended_without_card_play()
    }
}
