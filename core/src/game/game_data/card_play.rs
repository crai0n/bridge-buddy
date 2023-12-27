use crate::error::BBError;
use crate::game::game_data::ended::Ended;
use crate::game::game_data::{GameData, NextToPlay};
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_event::CardEvent;
use crate::primitives::game_result::GameResult;
use crate::primitives::{Card, Contract, Hand};

#[derive(Debug, Clone)]
pub struct CardPlay {
    pub bids: BidLine,
    pub trick_manager: TrickManager,
    pub hand_manager: HandManager,
    pub contract: Contract,
    pub board: Board,
}

impl NextToPlay for GameData<CardPlay> {
    fn next_to_play(&self) -> Seat {
        self.inner.trick_manager.next_to_play()
    }
}

impl GameData<CardPlay> {
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

        self.validate_suit_rule(card_event.player, card_event.card)?;
        Ok(())
    }

    pub fn validate_suit_rule(&self, player: Seat, card: Card) -> Result<(), BBError> {
        if let Some(suit) = &self.inner.trick_manager.suit_to_follow() {
            if card.suit != *suit
                && self
                    .inner
                    .hand_manager
                    .player_is_known_to_have_cards_left_in_suit(player, *suit)
            {
                Err(BBError::FollowSuit(*suit))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    pub fn card_play_has_ended(&self) -> bool {
        self.inner.trick_manager.card_play_has_ended()
    }

    pub fn move_from_card_play_to_ended(self) -> GameData<Ended> {
        let tricks = self.inner.trick_manager.played_tricks().into();

        let result = self.calculate_game_result();

        let inner = Ended {
            bids: self.inner.bids,
            tricks,
            hands: self.inner.hand_manager,
            result,
            board: self.inner.board,
        };

        GameData { inner }
    }

    pub fn calculate_game_result(&self) -> GameResult {
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
