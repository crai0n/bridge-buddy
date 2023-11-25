use crate::error::BBError;
use crate::game::game_state::waiting_for_dummy::WaitingForDummy;
use crate::game::game_state::GameState;
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::game_event::CardEvent;
use crate::primitives::Contract;

#[derive(Debug, Clone)]
pub struct OpeningLead {
    pub bids: BidLine,
    pub trick_manager: TrickManager,
    pub hand_manager: HandManager,
    pub contract: Contract,
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
