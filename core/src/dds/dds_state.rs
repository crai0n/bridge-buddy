use crate::dds::card_tracker::CardTracker;
use crate::dds::dds_trick_manager::DdsTrickManager;
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Hand, Suit};
use itertools::Itertools;

pub struct DdsState<const N: usize> {
    hands: [Hand<N>; 4],
    trick_manager: DdsTrickManager<N>,
    played_cards_tracker: CardTracker,
    remaining_cards_tracker: [CardTracker; 4],
}

impl<const N: usize> DdsState<N> {
    pub fn new(hands: [Hand<N>; 4], opening_leader: Seat, trumps: Option<Suit>) -> Self {
        let remaining_cards_tracker: [CardTracker; 4] = hands
            .iter()
            .map(|hand| CardTracker::from_hand(*hand))
            .collect_vec()
            .try_into()
            .unwrap();

        Self {
            trick_manager: DdsTrickManager::new(opening_leader, trumps),
            hands,
            played_cards_tracker: CardTracker::empty(),
            remaining_cards_tracker,
        }
    }

    pub fn play(&mut self, card: Card) {
        // println!("{} played {}", self.next_to_play(), card);
        self.remaining_cards_tracker[self.next_to_play() as usize].remove_card(card);
        self.played_cards_tracker.add_card(card);
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
        if let Some(card) = self.trick_manager.undo() {
            self.played_cards_tracker.remove_card(card);
            self.remaining_cards_tracker[self.next_to_play() as usize].add_card(card);
        }
    }

    pub fn starting_hand_of(&self, player: Seat) -> Hand<N> {
        self.hands[player as usize]
    }

    pub fn remaining_cards_of(&self, player: Seat) -> Vec<Card> {
        self.remaining_cards_tracker[player as usize].all_contained_cards()
    }

    pub fn available_cards_of(&self, player: Seat) -> Vec<Card> {
        match self.trick_manager.suit_to_follow() {
            None => self.remaining_cards_of(player),
            Some(suit) => {
                let suit_cards = self.remaining_cards_tracker[player as usize].contained_cards_in_suit(suit);
                if suit_cards.is_empty() {
                    self.remaining_cards_of(player)
                } else {
                    suit_cards
                }
            }
        }
    }

    pub fn indistinguishable_moves_for(&self, player: Seat) -> Vec<Card> {
        let full_tracker = self
            .played_cards_tracker
            .union(&self.remaining_cards_tracker[player as usize]);

        let all_tops_field = full_tracker.tops_of_sequences_field(); // this gives artifacts. it marks cards that I do not own (in case I dont have cards touching a sequence of played cards
        let my_tops_field = self.remaining_cards_tracker[player as usize].tops_of_sequences_field();
        let pl_tops_field = self.played_cards_tracker.tops_of_sequences_field();

        let unique_field = (all_tops_field | my_tops_field) & (!pl_tops_field);

        let unique_tracker = CardTracker::from_field(unique_field);

        let moves = unique_tracker.all_contained_cards();

        println!("new method found the following moves {:?}", moves);
        moves
    }

    pub fn available_indistinguishable_moves_for(&self, player: Seat) -> Vec<Card> {
        let full_tracker = self
            .played_cards_tracker
            .union(&self.remaining_cards_tracker[player as usize]);

        let all_tops_field = full_tracker.tops_of_sequences_field(); // this gives artifacts. it marks cards that I do not own (in case I dont have cards touching a sequence of played cards
        let my_tops_field = self.remaining_cards_tracker[player as usize].tops_of_sequences_field();
        let pl_tops_field = self.played_cards_tracker.tops_of_sequences_field();

        let unique_field = (all_tops_field | my_tops_field) & (!pl_tops_field);

        let unique_tracker = CardTracker::from_field(unique_field);

        match self.trick_manager.suit_to_follow() {
            None => unique_tracker.all_contained_cards(),
            Some(suit) => {
                // println!("Looking only for cards in {}", suit);
                let suit_cards = unique_tracker.contained_cards_in_suit(suit);
                if suit_cards.is_empty() {
                    // println!("Found no cards in {}", suit);
                    unique_tracker.all_contained_cards()
                } else {
                    // println!("Found these cards: {:?}", suit_cards);
                    suit_cards
                }
            }
        }
    }

    pub fn all_played_cards(&self) -> Vec<Card> {
        self.played_cards_tracker.all_contained_cards()
    }
}
