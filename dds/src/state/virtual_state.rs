use super::double_dummy_state::DoubleDummyState;

use super::virtual_card::VirtualCard;
use crate::state::virtual_card_tracker::VirtualCardTracker;
use crate::state::virtualizer::Virtualizer;
use crate::transposition_table::TTKey;
use bridge_buddy_core::error::BBError;
use bridge_buddy_core::primitives::card::suit::SUIT_ARRAY;
use bridge_buddy_core::primitives::deal::seat::SEAT_ARRAY;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::{Card, Hand, Suit};

use crate::state::distribution_field::DistFieldManager;
use crate::trick_estimations::EstimationState;

pub struct VirtualState<const N: usize> {
    game: DoubleDummyState<N>,
    virtualizer: Virtualizer,
    distribution_field: DistFieldManager,
}

#[allow(dead_code)]
impl<const N: usize> VirtualState<N> {
    pub fn new(hands: [Hand<N>; 4], opening_leader: Seat, trumps: Option<Suit>) -> Self {
        let game = DoubleDummyState::new(hands, opening_leader, trumps);

        // let starting_field = Self::generate_distribution_field_from_game(&game);
        // let starting_field =

        Self {
            virtualizer: Virtualizer::default(),
            distribution_field: DistFieldManager::new_for_game(&game),
            game,
        }
    }

    pub fn suit_to_follow(&self) -> Option<Suit> {
        self.game.suit_to_follow()
    }

    pub fn count_played_cards(&self) -> usize {
        self.game.count_played_cards()
    }

    pub fn generate_tt_key(&self) -> TTKey {
        TTKey::new(
            self.tricks_left(),
            self.trump_suit(),
            self.next_to_play(),
            self.distribution_field.get_field(),
        )
    }

    pub fn play(&mut self, virtual_card: &VirtualCard) -> Result<(), BBError> {
        let card = self.virtual_to_absolute(virtual_card);
        match card {
            Some(card) => {
                self.game.play(card);
                if self.game.player_is_leading() {
                    // print!("Removing virt cards from dist field: ");
                    let cards = self
                        .game
                        .cards_in_last_trick()
                        .iter()
                        .map(|card| self.virtualizer.absolute_to_virtual_card(card).unwrap());
                    self.distribution_field.remove_cards(cards);
                    self.update_virtualizer();
                }
                Ok(())
            }
            _ => Err(BBError::UnknownCard("None".to_string())),
        }
    }

    fn update_virtualizer(&mut self) {
        self.virtualizer = Virtualizer::new(self.game.out_of_play_cards().clone());
    }

    pub fn undo(&mut self) {
        if self.game.trick_complete() {
            // we are moving back a trick, update virtualizer and distribution field
            self.game.undo();
            self.update_virtualizer();
            self.distribution_field.step_back();
        } else {
            self.game.undo();
        }
    }

    pub fn is_last_trick(&self) -> bool {
        self.game.is_last_trick()
    }

    pub fn next_to_play(&self) -> Seat {
        self.game.next_to_play()
    }

    pub fn owner_of(&self, card: VirtualCard) -> Option<Seat> {
        self.distribution_field.owner_of(&card)
    }

    pub fn owner_of_winning_rank_in(&self, suit: Suit) -> Option<Seat> {
        self.distribution_field.owner_of_winning_rank_in(suit)
    }

    pub fn owner_of_runner_up_in(&self, suit: Suit) -> Option<Seat> {
        self.distribution_field.owner_of_runner_up_in(suit)
    }

    pub fn player_can_ruff_suit(&self, suit: Suit, player: Seat) -> bool {
        match self.trump_suit() {
            None => false,
            Some(trump_suit) => self.cards_of(player).is_void_in(suit) && !self.cards_of(player).is_void_in(trump_suit),
        }
    }

    pub fn player_is_leading(&self) -> bool {
        self.game.player_is_leading()
    }

    pub fn tricks_left(&self) -> usize {
        self.game.tricks_left()
    }

    pub fn tricks_won_by_axis(&self, player: Seat) -> usize {
        self.game.tricks_won_by_axis(player)
    }

    pub fn count_cards_in_current_trick(&self) -> usize {
        self.game.count_cards_in_current_trick()
    }

    pub fn trump_suit(&self) -> Option<Suit> {
        self.game.trump_suit()
    }

    pub fn count_trump_cards_for_player(&self, player: Seat) -> usize {
        match self.trump_suit() {
            None => 0,
            Some(trump_suit) => self.cards_of(player).count_cards_in(trump_suit),
        }
    }

    pub fn count_trump_cards_for_axis(&self, player: Seat) -> usize {
        self.game.count_trump_cards_for_axis(player)
    }

    pub fn count_this_sides_trump_cards(&self) -> usize {
        self.game.count_this_sides_trump_cards()
    }

    pub fn count_opponents_trump_cards(&self) -> usize {
        self.game.count_opponents_trump_cards()
    }

    pub fn current_trick_winner(&self) -> Seat {
        self.game.current_trick_winner()
    }

    pub fn currently_winning_card(&self) -> Option<VirtualCard> {
        let winning_card = self.game.currently_winning_card();
        match winning_card {
            None => None,
            Some(winning_card) => self.absolute_to_virtual(&winning_card),
        }
    }

    fn absolute_to_virtual(&self, card: &Card) -> Option<VirtualCard> {
        self.virtualizer.absolute_to_virtual_card(card)
    }

    fn virtual_to_absolute(&self, virtual_card: &VirtualCard) -> Option<Card> {
        self.virtualizer.virtual_to_absolute_card(virtual_card)
    }

    pub fn partner_has_higher_cards_than_opponents(&self, suit: Suit, leader: Seat) -> bool {
        self.game.partner_has_higher_cards_than_opponents(suit, leader)
    }

    pub fn would_win_over_current_winner(&self, card: VirtualCard) -> bool {
        let real_card = self.virtual_to_absolute(&card).unwrap();
        self.game.would_win_over_current_winner(&real_card)
    }

    pub fn last_trick_winner(&self) -> Option<Seat> {
        self.game.last_trick_winner()
    }

    pub fn cards_of(&self, player: Seat) -> VirtualCardTracker {
        let card_tracker = self.game.cards_of(player);
        VirtualCardTracker::from_card_tracker(card_tracker, &self.virtualizer)
    }

    pub fn create_estimation_state(&self) -> EstimationState {
        let my_seat = self.next_to_play();
        let card_counts = SEAT_ARRAY.map(|player| self.cards_of(player).count_cards_per_suit());
        let ace_owners = SUIT_ARRAY.map(|suit| self.owner_of_winning_rank_in(suit));
        let king_owners = SUIT_ARRAY.map(|suit| self.owner_of_runner_up_in(suit));

        let mut high_card_counts = [[0usize; 4]; 4];
        let high_card_counts_transposed = SUIT_ARRAY.map(|suit| match ace_owners[suit as usize] {
            None => [0; 4],
            Some(ace_owner) => {
                let mut arr = [0; 4];
                arr[ace_owner as usize] = match king_owners[suit as usize] {
                    Some(king_owner) if king_owner == ace_owner => self.cards_of(ace_owner).count_high_cards_in(suit),
                    _ => 1,
                };
                arr
            }
        });
        for (suit_index, suit_high_cards) in high_card_counts_transposed.into_iter().enumerate() {
            for (player_index, high_card_count) in suit_high_cards.into_iter().enumerate() {
                high_card_counts[player_index][suit_index] = high_card_count;
            }
        }

        let mut our_combined_high_card_count = [[0; 4]; 2];

        let transposed_combined_high_card_count = SUIT_ARRAY.map(|suit| match ace_owners[suit as usize] {
            Some(ace_owner) if ace_owner.same_axis(&my_seat) => self
                .cards_of(my_seat)
                .count_combined_high_cards_in(suit, &self.cards_of(my_seat.partner())),
            _ => [0; 2],
        });

        for (suit_index, player_counts) in transposed_combined_high_card_count.iter().enumerate() {
            for (player_index, single_count) in player_counts.iter().enumerate() {
                our_combined_high_card_count[player_index][suit_index] = *single_count;
            }
        }

        EstimationState {
            lead_suit: self.suit_to_follow(),
            trump_suit: self.trump_suit(),
            my_seat,
            card_counts,
            ace_owners,
            king_owners,
            high_card_counts,
            our_combined_high_card_count,
        }
    }
}
