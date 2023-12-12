use crate::error::BBError;
use crate::game::game_state::ended::Ended;
use crate::game::game_state::GameState;
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::{Board, PlayerPosition};
use crate::primitives::game_event::CardEvent;
use crate::primitives::game_result::GameResult;
use crate::primitives::{Contract, Hand};

#[derive(Debug, Clone)]
pub struct CardPlay {
    pub bids: BidLine,
    pub trick_manager: TrickManager,
    pub hand_manager: HandManager,
    pub contract: Contract,
    pub board: Board,
}

impl GameState<CardPlay> {
    pub fn next_to_play(&self) -> PlayerPosition {
        self.inner.trick_manager.next_to_play()
    }

    pub fn hand_of(&self, player: PlayerPosition) -> Result<Hand, BBError> {
        self.inner.hand_manager.hand_of(player)
    }

    pub fn validate_turn_order(&self, player: PlayerPosition) -> Result<(), BBError> {
        let turn = self.next_to_play();
        if player != turn {
            return Err(BBError::OutOfTurn(Some(turn)));
        }
        Ok(())
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
            board: self.inner.board,
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

    pub fn board(&self) -> Board {
        self.inner.board
    }
}
