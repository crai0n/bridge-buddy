use crate::error::BBError;
use crate::game::game_data::ended::EndedState;
use crate::game::game_data::{GameData, NextToPlay};
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::game::GamePhaseState;
use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::{Board, Seat};
use crate::primitives::game_event::CardEvent;
use crate::primitives::game_result::GameResult;
use crate::primitives::{Card, Contract, Hand};

#[derive(Debug, Clone)]
pub struct CardPlayState {
    pub bids: BidLine,
    pub trick_manager: TrickManager<13>,
    pub hand_manager: HandManager,
    pub contract: Contract,
    pub board: Board,
}

impl GamePhaseState for CardPlayState {
    fn implied_contract(&self) -> Option<Contract> {
        Some(self.contract)
    }
}

impl NextToPlay for GameData<CardPlayState> {
    fn next_to_play(&self) -> Seat {
        self.inner.trick_manager.next_to_play()
    }
}
impl NextToPlay for CardPlayState {
    fn next_to_play(&self) -> Seat {
        self.trick_manager.next_to_play()
    }
}

impl GameData<CardPlayState> {
    pub fn hand_of(&self, player: Seat) -> Result<Hand<13>, BBError> {
        self.inner.hand_of(player)
    }

    pub fn declarer(&self) -> Seat {
        self.inner.declarer()
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        self.inner.process_play_card_event(card_event)
    }

    pub fn validate_play_card_event(&self, card_event: CardEvent) -> Result<(), BBError> {
        self.inner.validate_play_card_event(card_event)
    }

    pub fn validate_suit_rule(&self, player: Seat, card: Card) -> Result<(), BBError> {
        self.inner.validate_suit_rule(player, card)
    }

    pub fn card_play_has_ended(&self) -> bool {
        self.inner.card_play_has_ended()
    }

    pub fn move_from_card_play_to_ended(self) -> EndedState {
        self.inner.move_from_card_play_to_ended()
    }

    pub fn calculate_game_result(&self) -> GameResult {
        self.inner.calculate_game_result()
    }

    pub fn board(&self) -> Board {
        self.inner.board()
    }
}

impl CardPlayState {
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

        self.validate_suit_rule(card_event.player, card_event.card)?;
        Ok(())
    }

    pub fn validate_suit_rule(&self, player: Seat, card: Card) -> Result<(), BBError> {
        if let Some(suit) = &self.trick_manager.suit_to_follow() {
            if card.suit != *suit
                && self
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
        self.trick_manager.card_play_has_ended()
    }

    pub fn move_from_card_play_to_ended(self) -> EndedState {
        let tricks = self.trick_manager.played_tricks();

        let result = self.calculate_game_result();

        EndedState {
            bids: self.bids,
            tricks,
            hands: self.hand_manager,
            result,
            board: self.board,
        }
    }

    pub fn calculate_game_result(&self) -> GameResult {
        GameResult::calculate_game_result(
            self.contract,
            self.trick_manager.tricks_won_by_axis(self.contract.declarer),
        )
    }

    pub fn board(&self) -> Board {
        self.board
    }
}
