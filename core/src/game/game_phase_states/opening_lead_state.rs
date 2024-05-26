use crate::error::BBError;
use crate::game::game_phase_states::waiting_for_dummy_state::WaitingForDummyState;
use crate::game::game_phase_states::NextToPlay;
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::game::GamePhaseState;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_event::CardEvent;
use crate::primitives::{Card, Contract, Hand};

#[derive(Debug, Clone)]
pub struct OpeningLeadState {
    pub bids: BidLine,
    pub trick_manager: TrickManager<13>,
    pub hand_manager: HandManager,
    pub contract: Contract,
    pub board: Board,
}

impl GamePhaseState for OpeningLeadState {
    fn implied_contract(&self) -> Option<Contract> {
        Some(self.contract)
    }
}

impl NextToPlay for OpeningLeadState {
    fn next_to_play(&self) -> Seat {
        self.trick_manager.next_to_play()
    }
}

impl OpeningLeadState {
    pub fn hand_of(&self, player: Seat) -> Result<Hand<13>, BBError> {
        self.hand_manager.hand_of(player)
    }

    pub fn declarer(&self) -> Seat {
        self.contract.declarer
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        self.validate_play_card_event(card_event)?;
        self.trick_manager.play(card_event.card);
        self.hand_manager
            .process_play_card_event(card_event.card, card_event.player)?;
        Ok(())
    }

    pub fn validate_play_card_event(&self, card_event: CardEvent) -> Result<(), BBError> {
        self.validate_turn_order(card_event.player)?;
        self.hand_manager
            .validate_play_card_event(card_event.card, card_event.player)?;

        if self.player_violates_suit_rule(card_event.player, card_event.card) {
            return Err(BBError::InvalidCard(card_event.card));
        }
        Ok(())
    }

    pub fn player_violates_suit_rule(&self, player: Seat, card: Card) -> bool {
        if let Some(suit) = &self.trick_manager.suit_to_follow() {
            card.suit != *suit
                && self
                    .hand_manager
                    .player_is_known_to_have_cards_left_in_suit(player, *suit)
        } else {
            false
        }
    }

    pub fn move_to_waiting_for_dummy(self) -> WaitingForDummyState {
        WaitingForDummyState {
            bids: self.bids,
            trick_manager: self.trick_manager,
            hand_manager: self.hand_manager,
            contract: self.contract,
            board: self.board,
        }
    }

    pub fn board(&self) -> Board {
        self.board
    }
}
