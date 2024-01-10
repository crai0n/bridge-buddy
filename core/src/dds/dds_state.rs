use crate::dds::dds_trick_manager::DdsTrickManager;
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Hand, Suit};
use itertools::Itertools;

pub struct DdsState<const N: usize> {
    hands: [Hand<N>; 4],
    trick_manager: DdsTrickManager<N>,
}

impl<const N: usize> DdsState<N> {
    pub fn new(hands: [Hand<N>; 4], opening_leader: Seat, trumps: Option<Suit>) -> Self {
        Self {
            trick_manager: DdsTrickManager::new(opening_leader, trumps),
            hands,
        }
    }

    pub fn play(&mut self, card: Card) {
        // println!("{} played {}", self.next_to_play(), card);
        self.trick_manager.play(card)
    }

    pub fn next_to_play(&self) -> Seat {
        self.trick_manager.next_to_play()
    }

    pub fn tricks_left(&self) -> usize {
        self.trick_manager.tricks_left()
    }

    pub fn last_trick_winner(&self) -> Option<Seat> {
        self.trick_manager.last_trick_winner()
    }

    pub fn tricks_won_by_axis(&self, player: Seat) -> usize {
        self.trick_manager.tricks_won_by_axis(player)
    }

    pub fn undo(&mut self) {
        self.trick_manager.undo()
    }

    pub fn starting_hand_of(&self, player: Seat) -> Hand<N> {
        self.hands[player as usize]
    }

    pub fn remaining_cards_of(&self, player: Seat) -> Vec<Card> {
        self.starting_hand_of(player)
            .cards()
            .filter(|card| !self.trick_manager.played_cards().contains(card))
            .copied()
            .collect_vec()
    }

    pub fn available_cards_of(&self, player: Seat) -> Vec<Card> {
        match self.trick_manager.suit_to_follow() {
            None => self.remaining_cards_of(player),
            Some(suit) => {
                let remaining_cards = self.remaining_cards_of(player);
                let cards_in_suit = remaining_cards.iter().filter(|&card| card.suit == suit).collect_vec();
                if cards_in_suit.is_empty() {
                    remaining_cards
                } else {
                    cards_in_suit.into_iter().copied().collect_vec()
                }
            }
        }
    }

    pub fn played_cards(&self) -> Vec<Card> {
        self.trick_manager.played_cards().to_vec()
    }
}
