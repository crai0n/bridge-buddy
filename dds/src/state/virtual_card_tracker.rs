use crate::card_manager::card_tracker::CardTracker;
use crate::state::virtual_card::VirtualCard;
use crate::state::virtualizer::Virtualizer;
use bridge_buddy_core::primitives::card::suit::SUIT_ARRAY;
use bridge_buddy_core::primitives::card::virtual_rank::{VirtualRank, VIRTUAL_RANK_ARRAY};
use bridge_buddy_core::primitives::card::Rank;
use bridge_buddy_core::primitives::{Card, Suit};

pub struct VirtualCardTracker<'a> {
    card_tracker: &'a CardTracker,
    virtualizer: &'a Virtualizer,
}

impl<'a> VirtualCardTracker<'a> {
    pub fn from_card_tracker(card_tracker: &'a CardTracker, virtualizer: &'a Virtualizer) -> Self {
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
        self.contains_in(&VirtualRank::Ace, suit)
    }

    pub fn contains_runner_up_in(&self, suit: Suit) -> bool {
        self.contains_in(&VirtualRank::King, suit)
    }

    pub fn has_singleton_winner_in(&self, suit: Suit) -> bool {
        self.has_singleton_in(suit) && self.contains_winning_rank_in(suit)
    }

    pub fn has_doubleton_runner_up_in(&self, suit: Suit) -> bool {
        self.has_doubleton_in(suit) && self.contains_runner_up_in(suit)
    }

    #[allow(dead_code)]
    pub fn contains(&self, card: &VirtualCard) -> bool {
        let real_card = self.virtual_to_absolute_card(card);
        match real_card {
            None => false,
            Some(card) => self.card_tracker.contains(&card),
        }
    }

    pub fn contains_in(&self, rank: &VirtualRank, suit: Suit) -> bool {
        let real_rank = self.virtual_to_absolute_rank(rank, suit);
        match real_rank {
            None => false,
            Some(rank) => self.card_tracker.contains_in(&rank, suit),
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

    fn absolute_to_virtual_card(&self, card: &Card) -> Option<VirtualCard> {
        self.virtualizer.absolute_to_virtual_card(card)
    }

    #[allow(dead_code)]
    fn virtual_to_absolute_card(&self, virtual_card: &VirtualCard) -> Option<Card> {
        self.virtualizer.virtual_to_absolute_card(virtual_card)
    }

    fn absolute_to_virtual_rank(&self, rank: &Rank, suit: Suit) -> Option<VirtualRank> {
        self.virtualizer.absolute_to_virtual_rank(rank, suit)
    }

    #[allow(dead_code)]
    fn virtual_to_absolute_rank(&self, virtual_rank: &VirtualRank, suit: Suit) -> Option<Rank> {
        self.virtualizer.virtual_to_absolute_rank(virtual_rank, suit)
    }

    pub fn all_cards(&self) -> impl DoubleEndedIterator<Item = VirtualCard> + '_ {
        self.card_tracker
            .all_cards()
            .map(|x| self.absolute_to_virtual_card(&x).unwrap())
    }

    #[allow(dead_code)]
    pub fn all_cards_in(&self, suit: Suit) -> impl DoubleEndedIterator<Item = VirtualCard> + '_ {
        self.all_ranks_in(suit)
            .map(move |vrank| VirtualCard { rank: vrank, suit })
    }

    pub fn all_ranks_in(&self, suit: Suit) -> impl DoubleEndedIterator<Item = VirtualRank> + '_ {
        self.card_tracker
            .ranks_in(suit)
            .filter_map(move |rank| self.absolute_to_virtual_rank(&rank, suit))
    }

    pub fn highest_card_in(&self, suit: Suit) -> Option<VirtualCard> {
        self.card_tracker
            .highest_card_in(suit)
            .map(|x| self.absolute_to_virtual_card(&x).unwrap())
    }
    #[allow(dead_code)]
    pub fn lowest_card_in(&self, suit: Suit) -> Option<VirtualCard> {
        self.card_tracker
            .lowest_card_in(suit)
            .map(|x| self.absolute_to_virtual_card(&x).unwrap())
    }

    pub fn count_high_cards_per_suit(&self) -> [usize; 4] {
        SUIT_ARRAY.map(|suit| self.count_high_cards_in(suit))
    }

    pub fn count_high_cards_in(&self, suit: Suit) -> usize {
        match self.contains_winning_rank_in(suit) {
            true => match self.contains_runner_up_in(suit) {
                true => self
                    .all_ranks_in(suit)
                    .rev()
                    .zip(VIRTUAL_RANK_ARRAY.iter().rev())
                    .take_while(|(rank, &high_rank)| *rank == high_rank)
                    .count(),
                false => 1,
            },
            false => 0,
        }
    }

    pub fn count_combined_high_cards_in(&self, suit: Suit, other: &Self) -> [usize; 2] {
        let mut my_iter = self.all_cards_in(suit).rev().peekable();
        let mut other_iter = other.all_cards_in(suit).rev().peekable();

        let mut count = [0usize, 0];
        for high_rank in VIRTUAL_RANK_ARRAY.into_iter().rev() {
            if my_iter.next_if(|y| y.rank == high_rank).is_some() {
                count[0] += 1;
            } else if other_iter.next_if(|y| y.rank == high_rank).is_some() {
                count[1] += 1;
            } else {
                break;
            }
        }
        count
    }

    #[allow(dead_code)]
    pub fn count_combined_high_cards_per_suit(&self, other: &Self) -> [[usize; 4]; 2] {
        let transposed_count = SUIT_ARRAY.map(|suit| self.count_combined_high_cards_in(suit, other));

        let mut count = [[0; 4]; 2];
        for (suit_index, player_counts) in transposed_count.iter().enumerate() {
            for (player_index, single_count) in player_counts.iter().enumerate() {
                count[player_index][suit_index] = *single_count;
            }
        }

        count
    }

    #[allow(dead_code)]
    pub fn count_cards_higher_than(&self, card: &VirtualCard) -> usize {
        let abs_card = self.virtualizer.virtual_to_absolute_card(card);
        match abs_card {
            Some(abs_card) => self.card_tracker.count_cards_higher_than(abs_card),
            None => 0,
        }
    }

    pub fn count_cards_lower_than(&self, card: &VirtualCard) -> usize {
        let abs_card = self.virtualizer.virtual_to_absolute_card(card);
        match abs_card {
            Some(abs_card) => self.card_tracker.count_cards_lower_than(abs_card),
            None => 0,
        }
    }
}
