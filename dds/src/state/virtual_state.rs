use super::double_dummy_state::DoubleDummyState;

use super::virtual_card::VirtualCard;
use crate::state::virtual_card_tracker::VirtualCardTracker;
use crate::state::virtualizer::Virtualizer;
use crate::transposition_table::TTKey;
use bridge_buddy_core::error::BBError;
use bridge_buddy_core::primitives::card::suit::SUIT_ARRAY;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::{Card, Hand, Suit};

use bridge_buddy_core::primitives::deal::seat::SEAT_ARRAY;

pub struct VirtualState<const N: usize> {
    game: DoubleDummyState<N>,
    virtualizer: Virtualizer,
}

#[allow(dead_code)]
impl<const N: usize> VirtualState<N> {
    pub fn new(hands: [Hand<N>; 4], opening_leader: Seat, trumps: Option<Suit>) -> Self {
        let game = DoubleDummyState::new(hands, opening_leader, trumps);

        Self {
            game,
            virtualizer: Virtualizer::default(),
        }
    }

    pub fn suit_to_follow(&self) -> Option<Suit> {
        self.game.suit_to_follow()
    }

    fn generate_distribution_field(&self) -> [u32; 4] {
        SUIT_ARRAY.map(|suit| {
            let mut field = 0u32;
            for player in SEAT_ARRAY {
                if player != Seat::North {
                    // North's id is 00 anyway
                    for rank in self.cards_of(player).ranks_in(suit) {
                        let offset = 2 * rank as usize;
                        field |= (player as u32) << offset;
                    }
                }
                let count = self.cards_of(player).count_cards_in(suit) as u32;
                field += count << 28; // count the cards still in play on the highest 4 bits
            }
            field
        })
    }

    pub fn count_played_cards(&self) -> usize {
        self.game.count_played_cards()
    }

    pub fn generate_tt_key(&self) -> TTKey {
        TTKey {
            tricks_left: self.tricks_left(),
            trumps: self.trump_suit(),
            lead: self.next_to_play(),
            remaining_cards: self.generate_distribution_field(),
        }
    }

    pub fn play(&mut self, virtual_card: &VirtualCard) -> Result<(), BBError> {
        let card = self.virtual_to_absolute(virtual_card);
        match card {
            Some(card) => {
                self.game.play(card);
                if self.game.player_is_leading() {
                    self.update_virtualizer();
                }
                Ok(())
            }
            _ => Err(BBError::UnknownCard("None".to_string())),
        }
    }

    fn update_virtualizer(&mut self) {
        // let out_of_play_cards = self.game.out_of_play_cards();
        // let tracker = CardTracker::from_cards(out_of_play_cards);
        // self.virtualizer = Virtualizer::new(tracker);
        self.virtualizer = Virtualizer::new(self.game.out_of_play_cards().clone());
    }

    pub fn undo(&mut self) {
        self.game.undo();
        if self.game.count_cards_in_current_trick() == 3 {
            // we moved back a trick
            self.update_virtualizer()
        }
    }

    pub fn is_last_trick(&self) -> bool {
        self.game.is_last_trick()
    }

    pub fn next_to_play(&self) -> Seat {
        self.game.next_to_play()
    }

    pub fn owner_of(&self, card: VirtualCard) -> Option<Seat> {
        SEAT_ARRAY
            .into_iter()
            .find(|&player| self.cards_of(player).contains(&card))
    }

    pub fn owner_of_winning_rank_in(&self, suit: Suit) -> Option<Seat> {
        SEAT_ARRAY
            .into_iter()
            .find(|&seat| self.cards_of(seat).contains_winning_rank_in(suit))
    }

    pub fn owner_of_runner_up_in(&self, suit: Suit) -> Option<Seat> {
        SEAT_ARRAY
            .into_iter()
            .find(|&seat| self.cards_of(seat).contains_runner_up_in(suit))
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
}
