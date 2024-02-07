use crate::card_manager::card_tracker::CardTracker;
use crate::primitives::VirtualCard;
use crate::state::virtualizer::Virtualizer;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::{Card, Suit};

pub struct VirtualCardTracker<'a> {
    card_tracker: &'a CardTracker,
    virtualizer: Virtualizer,
}

impl<'a> VirtualCardTracker<'a> {
    pub fn from_card_tracker(card_tracker: &'a CardTracker, virtualizer: Virtualizer) -> Self {
        Self {
            card_tracker,
            virtualizer,
        }
    }

    pub fn is_void_in(&self, suit: Suit) -> bool {
        self.card_tracker.is_void_in(suit)
    }

    pub fn has_cards_in(&self, suit: Suit) -> bool {
        self.card_tracker.has_cards_in(suit)
    }

    pub fn has_singleton_in(&self, suit: Suit) -> bool {
        self.card_tracker.has_singleton_in(suit)
    }

    pub fn has_doubleton_in(&self, suit: Suit) -> bool {
        self.card_tracker.has_doubleton_in(suit)
    }

    pub fn contains_winning_rank_in(&self, suit: Suit) -> bool {
        self.contains(&VirtualCard {
            suit,
            rank: VirtualRank::Ace,
        })
    }

    pub fn contains_runner_up_in(&self, suit: Suit) -> bool {
        self.contains(&VirtualCard {
            suit,
            rank: VirtualRank::King,
        })
    }

    pub fn has_singleton_winner_in(&self, suit: Suit) -> bool {
        self.has_singleton_in(suit) && self.contains_winning_rank_in(suit)
    }

    pub fn has_doubleton_runner_up_in(&self, suit: Suit) -> bool {
        self.has_doubleton_in(suit) && self.contains_runner_up_in(suit)
    }

    pub fn contains(&self, card: &VirtualCard) -> bool {
        let real_card = self.virtual_to_absolute(*card);
        match real_card {
            None => false,
            Some(card) => self.card_tracker.contains(&card),
        }
    }

    #[allow(dead_code)]
    pub fn count_cards(&self) -> usize {
        self.card_tracker.count_cards()
    }

    pub fn count_cards_in(&self, suit: Suit) -> usize {
        self.card_tracker.count_cards_in(suit)
    }

    fn absolute_to_virtual(&self, card: Card) -> Option<VirtualCard> {
        self.virtualizer.absolute_to_virtual(card)
    }

    fn virtual_to_absolute(&self, virtual_card: VirtualCard) -> Option<Card> {
        self.virtualizer.virtual_to_absolute(virtual_card)
    }

    pub fn all_cards(&self) -> impl Iterator<Item = VirtualCard> + '_ {
        self.card_tracker
            .all_cards()
            .map(|x| self.absolute_to_virtual(x).unwrap())
    }

    #[allow(dead_code)]
    pub fn cards_in(&self, suit: Suit) -> impl Iterator<Item = VirtualCard> + '_ {
        self.card_tracker
            .cards_in(suit)
            .map(|x| self.absolute_to_virtual(x).unwrap())
    }
}
