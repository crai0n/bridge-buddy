use crate::card_manager::card_tracker::CardTracker;
use crate::card_manager::suit_field::SuitField;
use crate::primitives::VirtualCard;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::{Card, Suit};
use itertools::Itertools;

pub struct VirtualCardTracker<'a> {
    card_tracker: &'a CardTracker,
    played: [SuitField; 4],
}

impl<'a> VirtualCardTracker<'a> {
    pub fn from_card_tracker(card_tracker: &'a CardTracker, played: [SuitField; 4]) -> Self {
        Self { card_tracker, played }
    }

    pub fn is_void_in(&self, suit: &Suit) -> bool {
        self.card_tracker.is_void_in(suit)
    }

    pub fn has_cards_in(&self, suit: &Suit) -> bool {
        self.card_tracker.has_cards_in(suit)
    }

    pub fn has_singleton_in(&self, suit: &Suit) -> bool {
        self.card_tracker.has_singleton_in(suit)
    }

    pub fn has_doubleton_in(&self, suit: &Suit) -> bool {
        self.card_tracker.has_doubleton_in(suit)
    }

    pub fn contains_winning_rank_in(&self, suit: &Suit) -> bool {
        self.contains(&VirtualCard {
            suit: *suit,
            rank: VirtualRank::Ace,
        })
    }

    pub fn contains_runner_up_in(&self, suit: &Suit) -> bool {
        self.contains(&VirtualCard {
            suit: *suit,
            rank: VirtualRank::King,
        })
    }

    pub fn has_singleton_winner_in(&self, suit: &Suit) -> bool {
        self.has_singleton_in(suit) && self.contains_winning_rank_in(suit)
    }

    pub fn has_doubleton_runner_up_in(&self, suit: &Suit) -> bool {
        self.has_doubleton_in(suit) && self.contains_runner_up_in(suit)
    }

    pub fn contains(&self, card: &VirtualCard) -> bool {
        let real_card = self.virtual_to_absolute(card);
        match real_card {
            None => false,
            Some(card) => self.card_tracker.contains(&card),
        }
    }

    #[allow(dead_code)]
    pub fn count_cards(&self) -> usize {
        self.card_tracker.count_cards()
    }

    pub fn count_cards_in(&self, suit: &Suit) -> usize {
        self.card_tracker.count_cards_in(suit)
    }

    fn absolute_to_virtual(&self, card: &Card) -> Option<VirtualCard> {
        let suit = card.suit;
        let suit_field = self.played[suit as usize];
        let virtual_rank = suit_field.try_find_relative(card.rank);
        virtual_rank.map(|rank| VirtualCard { rank, suit })
    }

    fn virtual_to_absolute(&self, virtual_card: &VirtualCard) -> Option<Card> {
        let suit = virtual_card.suit;
        let suit_field = self.played[suit as usize];
        let absolute_rank = suit_field.try_find_absolute(virtual_card.rank);
        absolute_rank.map(|rank| Card { rank, suit })
    }

    pub fn all_cards(&self) -> Vec<VirtualCard> {
        self.card_tracker
            .all_cards()
            .iter()
            .map(|x| self.absolute_to_virtual(x).unwrap())
            .collect_vec()
    }

    pub fn cards_in(&self, suit: &Suit) -> Vec<VirtualCard> {
        self.card_tracker
            .cards_in(suit)
            .iter()
            .map(|x| self.absolute_to_virtual(x).unwrap())
            .collect_vec()
    }

    pub fn valid_moves(&self, lead_suit: &Option<Suit>) -> Vec<VirtualCard> {
        match lead_suit {
            None => self.all_cards(),
            Some(lead_suit) => {
                let cards_in_suit = self.cards_in(lead_suit);
                if cards_in_suit.is_empty() {
                    self.all_cards()
                } else {
                    cards_in_suit
                }
            }
        }
    }
}
