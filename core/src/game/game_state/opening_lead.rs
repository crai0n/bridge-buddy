use crate::error::BBError;
use crate::game::game_state::waiting_for_dummy::WaitingForDummy;
use crate::game::game_state::GameState;
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_event::CardEvent;
use crate::primitives::{Card, Contract, Hand};

#[derive(Debug, Clone)]
pub struct OpeningLead {
    pub bids: BidLine,
    pub trick_manager: TrickManager,
    pub hand_manager: HandManager,
    pub contract: Contract,
    pub board: Board,
}

impl GameState<OpeningLead> {
    pub fn next_to_play(&self) -> Seat {
        self.inner.trick_manager.next_to_play()
    }

    pub fn validate_turn_order(&self, player: Seat) -> Result<(), BBError> {
        let turn = self.next_to_play();
        if player != turn {
            return Err(BBError::OutOfTurn(Some(turn)));
        }
        Ok(())
    }
    pub fn hand_of(&self, player: Seat) -> Result<Hand, BBError> {
        self.inner.hand_manager.hand_of(player)
    }

    pub fn declarer(&self) -> Seat {
        self.inner.contract.declarer
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

        if self.player_violates_suit_rule(card_event.player, card_event.card) {
            return Err(BBError::InvalidCard(card_event.card));
        }
        Ok(())
    }

    pub fn player_violates_suit_rule(&self, player: Seat, card: Card) -> bool {
        if let Some(suit) = &self.inner.trick_manager.suit_to_follow() {
            card.suit != *suit
                && self
                    .inner
                    .hand_manager
                    .player_is_known_to_have_cards_left_in_suit(player, *suit)
        } else {
            false
        }
    }

    pub fn move_to_waiting_for_dummy(self) -> GameState<WaitingForDummy> {
        let inner = WaitingForDummy {
            bids: self.inner.bids,
            trick_manager: self.inner.trick_manager,
            hand_manager: self.inner.hand_manager,
            contract: self.inner.contract,
            board: self.inner.board,
        };

        GameState { inner }
    }

    pub fn board(&self) -> Board {
        self.inner.board
    }
}
