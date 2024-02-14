use crate::card_manager::card_tracker::{CardTracker, SUIT_ARRAY};
use crate::state::virtual_card::VirtualCard;
use crate::state::virtualizer::Virtualizer;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::{Card, Suit};

use strum::IntoEnumIterator;

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

    pub fn count_cards_per_suit(&self) -> [usize; 4] {
        self.card_tracker.count_cards_per_suit()
    }

    fn absolute_to_virtual(&self, card: Card) -> Option<VirtualCard> {
        self.virtualizer.absolute_to_virtual(card)
    }

    fn virtual_to_absolute(&self, virtual_card: VirtualCard) -> Option<Card> {
        self.virtualizer.virtual_to_absolute(virtual_card)
    }

    pub fn all_cards(&self) -> impl DoubleEndedIterator<Item = VirtualCard> + '_ {
        self.card_tracker
            .all_cards()
            .map(|x| self.absolute_to_virtual(x).unwrap())
    }

    #[allow(dead_code)]
    pub fn cards_in(&self, suit: Suit) -> impl DoubleEndedIterator<Item = VirtualCard> + '_ {
        self.card_tracker
            .cards_in(suit)
            .map(|x| self.absolute_to_virtual(x).unwrap())
    }

    pub fn highest_card_in(&self, suit: Suit) -> Option<VirtualCard> {
        self.card_tracker
            .highest_card_in(suit)
            .map(|x| self.absolute_to_virtual(x).unwrap())
    }
    #[allow(dead_code)]
    pub fn lowest_card_in(&self, suit: Suit) -> Option<VirtualCard> {
        self.card_tracker
            .lowest_card_in(suit)
            .map(|x| self.absolute_to_virtual(x).unwrap())
    }

    pub fn count_high_cards_per_suit(&self) -> [usize; 4] {
        SUIT_ARRAY.map(|suit| {
            self.cards_in(suit)
                .rev()
                .zip(VirtualRank::iter().rev())
                .take_while(|(card, high_rank)| card.rank == *high_rank)
                .count()
        })
    }

    pub fn count_combined_high_cards_per_suit(&self, other: &Self) -> [[usize; 4]; 2] {
        let transposed_count = SUIT_ARRAY.map(|suit| {
            let mut my_iter = self.cards_in(suit).rev().peekable();
            let mut other_iter = other.cards_in(suit).rev().peekable();

            let mut count = [0usize, 0];
            for high_rank in VirtualRank::iter().rev() {
                if my_iter.next_if(|y| y.rank == high_rank).is_some() {
                    count[0] += 1;
                } else if other_iter.next_if(|y| y.rank == high_rank).is_some() {
                    count[1] += 1;
                } else {
                    break;
                }
            }
            count
        });
        let mut count = [[0; 4]; 2];

        for (index, counts) in transposed_count.iter().enumerate() {
            for (jndex, single_count) in counts.iter().enumerate() {
                count[jndex][index] = *single_count;
            }
        }

        count
    }

    #[allow(dead_code)]
    pub fn count_cards_higher_than(&self, card: VirtualCard) -> usize {
        let abs_card = self.virtualizer.virtual_to_absolute(card);
        match abs_card {
            Some(abs_card) => self.card_tracker.count_cards_higher_than(abs_card),
            None => 0,
        }
    }

    pub fn count_cards_lower_than(&self, card: VirtualCard) -> usize {
        let abs_card = self.virtualizer.virtual_to_absolute(card);
        match abs_card {
            Some(abs_card) => self.card_tracker.count_cards_lower_than(abs_card),
            None => 0,
        }
    }
}
