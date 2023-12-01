use crate::error::BBError;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::{Card, Hand, Suit};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct HandManager {
    pub known_cards: BTreeMap<Card, PlayerPosition>,
    pub played_cards: BTreeSet<Card>,
}

impl HandManager {
    pub fn new() -> Self {
        HandManager {
            known_cards: BTreeMap::new(),
            played_cards: BTreeSet::new(),
        }
    }

    pub fn register_known_hand(&mut self, hand: Hand, player: PlayerPosition) -> Result<(), BBError> {
        for &card in hand.cards() {
            self.register_known_card(card, player)?;
        }
        Ok(())
    }

    pub fn register_known_card(&mut self, card: Card, player: PlayerPosition) -> Result<(), BBError> {
        if self.known_cards_of(player).len() >= 13 && !self.known_cards_of(player).contains(&card) {
            return Err(BBError::InvalidHandInfo);
        }

        match self.known_cards.insert(card, player) {
            None => Ok(()),
            Some(known_player) if known_player == player => Ok(()),
            _ => Err(BBError::Duplicate(card)),
        }
    }

    pub fn count_played_cards(&self) -> usize {
        self.played_cards.len()
    }

    fn card_could_belong_to_player(&self, card: &Card, player: PlayerPosition) -> bool {
        if let Some(owner) = self.known_cards.get(card) {
            *owner == player
        } else if self.count_known_cards_of(player) == 13 {
            self.full_hand_known_for(player) // we haven't seen card, but player's hand is full
        } else {
            true
        }
    }

    pub fn card_has_already_been_played(&self, card: &Card) -> bool {
        self.played_cards.contains(card)
    }

    pub fn process_play_card_event(&mut self, card: Card, player: PlayerPosition) -> Result<(), BBError> {
        self.validate_play_card_event(card, player)?;
        self.apply_play_card_event(card, player)
    }

    pub fn validate_play_card_event(&self, card: Card, player: PlayerPosition) -> Result<(), BBError> {
        if self.card_could_belong_to_player(&card, player) && !self.card_has_already_been_played(&card) {
            Ok(())
        } else {
            Err(BBError::InvalidCard(card))
        }
    }

    fn apply_play_card_event(&mut self, card: Card, player: PlayerPosition) -> Result<(), BBError> {
        self.register_known_card(card, player)?;
        self.played_cards.insert(card);
        Ok(())
    }

    pub fn player_is_known_to_have_cards_left_in_suit(&self, player: PlayerPosition, suit: Suit) -> bool {
        let known_cards_in_suit: BTreeSet<_> = self
            .known_cards
            .iter()
            .filter(|(c, p)| **p == player && c.suit == suit)
            .map(|(c, _)| *c)
            .collect();
        let remaining_cards_in_suit: Vec<_> = known_cards_in_suit.difference(&self.played_cards).collect();
        !remaining_cards_in_suit.is_empty()
    }

    pub fn known_cards_of(&self, player: PlayerPosition) -> Vec<Card> {
        self.known_cards
            .iter()
            .filter_map(|(card, owner)| if *owner == player { Some(*card) } else { None })
            .collect()
    }

    pub fn count_known_cards_of(&self, player: PlayerPosition) -> usize {
        self.known_cards_of(player).len()
    }

    pub fn full_hand_known_for(&self, player: PlayerPosition) -> bool {
        self.count_known_cards_of(player) == 13
    }

    pub fn hand_of(&self, player: PlayerPosition) -> Result<Hand, BBError> {
        let cards = self.known_cards_of(player);
        if cards.len() == 13 {
            Hand::from_cards(&cards)
        } else {
            Err(BBError::InsufficientInfo)
        }
    }
}

impl Default for HandManager {
    fn default() -> Self {
        Self::new()
    }
}
