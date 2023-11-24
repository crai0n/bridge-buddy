use crate::error::BBError;
use crate::game::bid_manager::BidManager;
use crate::game::game_event::{BidEvent, CardEvent, DiscloseHandEvent, DummyUncoveredEvent};
use crate::game::hand_manager::HandManager;
use crate::game::trick_manager::TrickManager;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::Contract;

#[derive(Debug, Clone)]
pub struct GameState {
    pub bid_manager: BidManager,
    pub tricks: Option<TrickManager>,
    pub hands: Option<HandManager>,
    pub contract: Option<Contract>,
    pub declarer: Option<PlayerPosition>,
}

impl GameState {
    pub fn next_to_play(&self) -> Option<PlayerPosition> {
        if let Some(manager) = &self.tricks {
            Some(manager.next_to_play())
        } else {
            Some(self.bid_manager.next_to_play())
        }
    }

    pub fn bidding_has_ended(&self) -> bool {
        self.bid_manager.bidding_has_ended()
    }

    pub fn validate_make_bid_event(&self, bid_event: BidEvent) -> Result<(), BBError> {
        self.validate_turn_order(bid_event.player)?;
        if !self.bid_manager.is_valid_bid(&bid_event.bid) {
            return Err(BBError::InvalidBid(bid_event.bid));
        }
        Ok(())
    }

    pub fn validate_play_card_event(&self, card_event: CardEvent) -> Result<(), BBError> {
        self.validate_turn_order(card_event.player)?;
        self.hands
            .as_ref()
            .unwrap()
            .validate_play_card_event(card_event.card, card_event.player)?;

        if let Some(suit) = &self.tricks.as_ref().unwrap().suit_to_follow() {
            if card_event.card.suit != *suit
                && self
                    .hands
                    .as_ref()
                    .unwrap()
                    .player_is_known_to_have_cards_left_in_suit(card_event.player, *suit)
            {
                return Err(BBError::InvalidCard(card_event.card));
            }
        }
        Ok(())
    }

    pub fn process_dummy_uncovered_event(&mut self, event: DummyUncoveredEvent) -> Result<(), BBError> {
        self.hands
            .as_mut()
            .unwrap()
            .register_known_hand(event.dummy, self.declarer.unwrap().partner())?;

        Ok(())
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        self.validate_make_bid_event(bid_event)?;
        self.bid_manager.bid(bid_event.bid)?;

        Ok(())
    }

    pub fn process_play_card_event(&mut self, card_event: CardEvent) -> Result<(), BBError> {
        self.validate_play_card_event(card_event)?;
        self.tricks.as_mut().unwrap().play(card_event.card)?;
        self.hands
            .as_mut()
            .unwrap()
            .process_play_card_event(card_event.card, card_event.player)?;
        Ok(())
    }

    pub fn card_play_has_ended(&self) -> bool {
        self.tricks.as_ref().unwrap().card_play_has_ended()
    }

    pub fn process_disclose_hand_event(&mut self, event: DiscloseHandEvent) -> Result<(), BBError> {
        self.hands.as_mut().unwrap().register_known_hand(event.hand, event.seat)
    }

    pub fn set_up_card_play(&mut self, contract: Contract, declarer: PlayerPosition) {
        self.contract = Some(contract);
        self.declarer = Some(declarer);
        self.tricks = Some(TrickManager::new(declarer + 1, contract.trump_suit()));
        self.hands = Some(HandManager::new());
    }

    pub fn validate_turn_order(&self, player: PlayerPosition) -> Result<(), BBError> {
        if let Some(next_to_play) = self.next_to_play() {
            if player == next_to_play {
                return Ok(());
            }
        }
        Err(BBError::OutOfTurn(self.next_to_play()))
    }

    pub fn tricks_won_by_axis(&self, player: PlayerPosition) -> usize {
        self.tricks.as_ref().unwrap().tricks_won_by_axis(player)
    }
}
#[derive(Debug, Clone)]
pub struct NewGameState<Phase> {
    pub inner: Phase,
}
#[derive(Debug, Clone)]
pub struct Bidding {
    pub bid_manager: BidManager,
    pub tricks: Option<TrickManager>,
    pub hands: HandManager,
    pub contract: Option<Contract>,
}

#[derive(Debug, Clone)]
pub struct OpeningLead {
    pub bid_manager: BidManager,
    pub tricks: TrickManager,
    pub hands: HandManager,
    pub contract: Contract,
}

#[derive(Debug, Clone)]
pub struct WaitingForDummy {
    pub bid_manager: BidManager,
    pub tricks: TrickManager,
    pub hands: HandManager,
    pub contract: Contract,
}

#[derive(Debug, Clone)]
pub struct CardPlay {
    pub bid_manager: BidManager,
    pub tricks: TrickManager,
    pub hands: HandManager,
    pub contract: Contract,
}

#[derive(Debug, Clone)]
pub struct Ended {
    pub bid_manager: BidManager,
    pub tricks: Option<TrickManager>,
    pub hands: HandManager,
    pub contract: Option<Contract>,
}

impl NewGameState<Bidding> {
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
        self.inner.hands.register_known_hand(event.hand, event.seat)
    }

    pub fn process_make_bid_event(&mut self, bid_event: BidEvent) -> Result<(), BBError> {
        self.validate_make_bid_event(bid_event)?;
        self.inner.bid_manager.bid(bid_event.bid)?;

        Ok(())
    }

    pub fn move_to_opening_lead(self, contract: Contract) -> NewGameState<OpeningLead> {
        let inner = OpeningLead {
            bid_manager: self.inner.bid_manager,
            tricks: TrickManager::new(contract.declarer + 1, contract.trump_suit()),
            hands: self.inner.hands,
            contract,
        };

        NewGameState { inner }
    }

    pub fn move_to_ended_without_card_play(self) -> NewGameState<Ended> {
        let inner = Ended {
            bid_manager: self.inner.bid_manager,
            tricks: None,
            hands: self.inner.hands,
            contract: None,
        };

        NewGameState { inner }
    }
}

impl NewGameState<OpeningLead> {
    pub fn next_to_play(&self) -> Option<PlayerPosition> {
        Some(self.inner.tricks.next_to_play())
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
        self.inner.tricks.play(card_event.card)?;
        self.inner
            .hands
            .process_play_card_event(card_event.card, card_event.player)?;
        Ok(())
    }

    pub fn validate_play_card_event(&self, card_event: CardEvent) -> Result<(), BBError> {
        self.validate_turn_order(card_event.player)?;
        self.inner
            .hands
            .validate_play_card_event(card_event.card, card_event.player)?;

        if let Some(suit) = &self.inner.tricks.suit_to_follow() {
            if card_event.card.suit != *suit
                && self
                    .inner
                    .hands
                    .player_is_known_to_have_cards_left_in_suit(card_event.player, *suit)
            {
                return Err(BBError::InvalidCard(card_event.card));
            }
        }
        Ok(())
    }

    pub fn move_to_waiting_for_dummy(self) -> NewGameState<WaitingForDummy> {
        let inner = WaitingForDummy {
            bid_manager: self.inner.bid_manager,
            tricks: self.inner.tricks,
            hands: self.inner.hands,
            contract: self.inner.contract,
        };

        NewGameState { inner }
    }
}

impl NewGameState<WaitingForDummy> {
    pub fn next_to_play(&self) -> Option<PlayerPosition> {
        Some(self.inner.tricks.next_to_play())
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
            .hands
            .register_known_hand(event.dummy, self.inner.contract.declarer.partner())?;

        Ok(())
    }

    pub fn move_to_card_play(self) -> NewGameState<CardPlay> {
        let inner = CardPlay {
            bid_manager: self.inner.bid_manager,
            tricks: self.inner.tricks,
            hands: self.inner.hands,
            contract: self.inner.contract,
        };

        NewGameState { inner }
    }
}

impl NewGameState<CardPlay> {
    pub fn next_to_play(&self) -> Option<PlayerPosition> {
        Some(self.inner.tricks.next_to_play())
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
        self.inner.tricks.play(card_event.card)?;
        self.inner
            .hands
            .process_play_card_event(card_event.card, card_event.player)?;
        Ok(())
    }

    pub fn validate_play_card_event(&self, card_event: CardEvent) -> Result<(), BBError> {
        self.validate_turn_order(card_event.player)?;
        self.inner
            .hands
            .validate_play_card_event(card_event.card, card_event.player)?;

        if let Some(suit) = &self.inner.tricks.suit_to_follow() {
            if card_event.card.suit != *suit
                && self
                    .inner
                    .hands
                    .player_is_known_to_have_cards_left_in_suit(card_event.player, *suit)
            {
                return Err(BBError::InvalidCard(card_event.card));
            }
        }
        Ok(())
    }

    pub fn card_play_has_ended(&self) -> bool {
        self.inner.tricks.card_play_has_ended()
    }

    pub fn move_from_card_play_to_ended(self) -> NewGameState<Ended> {
        let inner = Ended {
            bid_manager: self.inner.bid_manager,
            tricks: Some(self.inner.tricks),
            hands: self.inner.hands,
            contract: Some(self.inner.contract),
        };

        NewGameState { inner }
    }
}

impl NewGameState<Ended> {
    pub fn tricks_won_by_axis(&self, player: PlayerPosition) -> usize {
        self.inner.tricks.as_ref().unwrap().tricks_won_by_axis(player)
    }
}
