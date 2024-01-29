use super::double_dummy_state::DoubleDummyState;
use crate::dds::card_manager::card_tracker::CardTracker;
use crate::dds::card_manager::suit_field::SuitField;
use crate::dds::dds_move::DdsMove;
use crate::dds::transposition_table::TTKey;
use crate::dds::virtual_card::VirtualCard;
use bridge_buddy_core::error::BBError;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::{Card, Hand, Suit};
use itertools::Itertools;
use strum::IntoEnumIterator;

#[allow(dead_code)]
pub struct VirtualState<const N: usize> {
    game: DoubleDummyState<N>,
    played: [SuitField; 4],
}

impl<const N: usize> VirtualState<N> {
    pub fn new(hands: [Hand<N>; 4], opening_leader: Seat, trumps: Option<Suit>) -> Self {
        let game = DoubleDummyState::new(hands, opening_leader, trumps);

        Self {
            game,
            played: [SuitField::empty(); 4],
        }
    }

    #[allow(dead_code)]
    fn generate_card_distribution(&self) -> [u32; 4] {
        let mut output = vec![];
        for player in Seat::iter() {
            let players_cards = self.remaining_cards_for_player(player);
            for card in players_cards.iter() {
                output.push((*card, player));
            }
        }
        Self::generate_distribution_field(&output)
    }

    fn generate_distribution_field(input: &[(VirtualCard, Seat)]) -> [u32; 4] {
        let mut fields = [0u32; 4];
        for &(card, seat) in input {
            let offset = 2 * card.rank as usize;
            fields[card.suit as usize] |= (seat as u32) << offset;
        }
        fields
    }

    pub fn count_played_cards(&self) -> usize {
        self.game.count_played_cards()
    }

    #[allow(dead_code)]
    pub fn generate_tt_key(&self) -> TTKey {
        TTKey {
            tricks_left: self.tricks_left(),
            trumps: self.trumps(),
            lead: self.next_to_play(),
            remaining_cards: self.generate_card_distribution(),
        }
    }

    pub fn play(&mut self, virtual_card: VirtualCard) -> Result<(), BBError> {
        let card = self.virtual_to_absolute(virtual_card);
        match card {
            Some(card) => {
                self.game.play(card);
                if self.game.player_is_leading() {
                    self.update_played();
                }
                Ok(())
            }
            _ => Err(BBError::UnknownCard("None".to_string())),
        }
    }

    fn update_played(&mut self) {
        let played_cards = self.game.out_of_play_cards();
        let tracker = CardTracker::from_cards(played_cards);
        for suit in Suit::iter() {
            self.played[suit as usize] = *tracker.suit_state(&suit);
        }
    }

    pub fn undo(&mut self) {
        self.game.undo();
        if self.game.count_cards_in_current_trick() == 3 {
            // we moved back a trick
            self.update_played()
        }
    }

    pub fn is_last_trick(&self) -> bool {
        self.game.is_last_trick()
    }

    pub fn next_to_play(&self) -> Seat {
        self.game.next_to_play()
    }

    fn virtual_to_absolute(&self, virtual_card: VirtualCard) -> Option<Card> {
        let suit = virtual_card.suit;
        let suit_field = self.played[suit as usize];
        let absolute_rank = suit_field.try_find_absolute(virtual_card.rank);
        absolute_rank.map(|rank| Card { rank, suit })
    }

    fn absolute_to_virtual(&self, card: Card) -> VirtualCard {
        let suit = card.suit;
        let suit_field = self.played[suit as usize];
        let virtual_rank = suit_field.find_relative(card.rank);
        VirtualCard {
            rank: virtual_rank,
            suit,
        }
    }

    fn remaining_cards_for_player(&self, player: Seat) -> Vec<VirtualCard> {
        self.game
            .remaining_cards_of(player)
            .iter()
            .map(|x| self.absolute_to_virtual(*x))
            .collect_vec()
    }

    fn valid_moves_for(&self, player: Seat) -> Vec<DdsMove> {
        let absolute_moves = self.game.valid_moves_for(player);
        absolute_moves
            .into_iter()
            .map(|x| DdsMove::new(self.absolute_to_virtual(x)))
            .collect_vec()
    }

    pub fn valid_moves(&self) -> Vec<DdsMove> {
        self.valid_moves_for(self.next_to_play())
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

    pub fn quick_tricks_for_player(&self, player: Seat) -> u8 {
        self.game.quick_tricks_for_player(player)
    }

    pub fn count_cards_in_current_trick(&self) -> usize {
        self.game.count_cards_in_current_trick()
    }

    pub fn trumps(&self) -> Option<Suit> {
        self.game.trumps()
    }

    pub fn count_trump_cards_for_player(&self, player: Seat) -> usize {
        self.game.count_trump_cards_for_player(player)
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
        winning_card.map(|x| self.absolute_to_virtual(x))
    }

    pub fn last_trick_winner(&self) -> Option<Seat> {
        self.game.last_trick_winner()
    }

    pub fn list_played_cards(&self) -> &[Card] {
        self.game.list_played_cards()
    }
}
