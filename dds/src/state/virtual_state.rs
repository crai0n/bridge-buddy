use super::double_dummy_state::DoubleDummyState;
use itertools::Itertools;

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
    distribution_field: [u32; 4],
}

#[allow(dead_code)]
impl<const N: usize> VirtualState<N> {
    pub fn new(hands: [Hand<N>; 4], opening_leader: Seat, trumps: Option<Suit>) -> Self {
        let game = DoubleDummyState::new(hands, opening_leader, trumps);

        let starting_field = Self::generate_distribution_field_from_game(&game);

        Self {
            game,
            virtualizer: Virtualizer::default(),
            distribution_field: starting_field,
        }
    }

    pub fn suit_to_follow(&self) -> Option<Suit> {
        self.game.suit_to_follow()
    }

    fn generate_distribution_field_from_game(game: &DoubleDummyState<N>) -> [u32; 4] {
        // println!("starting fields are: ");
        SUIT_ARRAY.map(|suit| {
            let mut field = 0u32;
            for player in SEAT_ARRAY {
                for rank in game.cards_of(player).ranks_in(suit) {
                    let offset = 2 * rank as usize;
                    field |= (player as u32) << offset;
                }
                let count = game.cards_of(player).count_cards_in(suit) as u32;
                // println!("found {} cards for player {} in suit", count, player);
                field += count << 28; // count the cards still in play on the highest 4 bits
            }

            // println!("{:032b}", field);

            field
        })
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

    fn remove_card_from_distribution_field(&mut self, card: VirtualCard) {
        let suit = card.suit;
        let virt_rank = card.rank;

        let suit_distribution = self.distribution_field[suit as usize];

        let index = 2 * virt_rank as usize;
        let lower_mask = (1 << index) - 1;
        // println!("lower mask:");
        // println!("{:32b}", lower_mask);
        let index_mask = 3 << index;
        // println!("index mask:");
        // println!("{:32b}", index_mask);
        let upper_mask = !(lower_mask | index_mask);
        // println!("{:32b}", upper_mask);

        let upper_field = suit_distribution & upper_mask;
        let lower_field = (suit_distribution & lower_mask) << 2;

        let suit_distribution = upper_field | lower_field;

        self.distribution_field[suit as usize] = suit_distribution - (1 << 28); // lower count
    }

    fn add_card_to_distribution_field(&mut self, card: VirtualCard, owner: Seat) {
        let suit = card.suit;
        let virt_rank = card.rank;

        let suit_distribution = self.distribution_field[suit as usize];

        let index = 2 * (virt_rank as usize);
        let lower_mask = (1 << (index + 2)) - 1;
        let upper_mask = !lower_mask;

        let upper_field = suit_distribution & upper_mask;
        let lower_field = (suit_distribution & lower_mask) >> 2;

        let suit_distribution = upper_field | lower_field | ((owner as u32) << index);

        self.distribution_field[suit as usize] = suit_distribution + (1 << 28); // increase count
    }

    pub fn count_played_cards(&self) -> usize {
        self.game.count_played_cards()
    }

    pub fn generate_tt_key(&self) -> TTKey {
        TTKey {
            tricks_left: self.tricks_left(),
            trumps: self.trump_suit(),
            lead: self.next_to_play(),
            remaining_cards: self.distribution_field,
        }
    }

    pub fn play(&mut self, virtual_card: &VirtualCard) -> Result<(), BBError> {
        let card = self.virtual_to_absolute(virtual_card);
        // println!(
        //     "Playing virtual card {}, which refers to real card {}",
        //     virtual_card,
        //     card.unwrap()
        // );
        match card {
            Some(card) => {
                self.game.play(card);
                if self.game.player_is_leading() {
                    // print!("Removing virt cards from dist field: ");
                    for card in self
                        .game
                        .cards_in_last_trick()
                        .iter()
                        .map(|card| self.virtualizer.absolute_to_virtual_card(card).unwrap())
                        .sorted_unstable_by_key(|card| card.rank)
                    {
                        // println!("removing {}", card);
                        self.remove_card_from_distribution_field(card);

                        // println!("{:0<32b}", self.distribution_field[card.suit as usize]);
                    }
                    // println!();
                    self.update_virtualizer();
                    // let a = self.distribution_field;
                    // let b = self.generate_distribution_field();
                    // println!("fields are: ");
                    // for index in 0..4 {
                    //     println!("{:032b} v {:032b}", a[index], b[index]);
                    // }
                    // assert_eq!(a, b);
                }
                Ok(())
            }
            _ => Err(BBError::UnknownCard("None".to_string())),
        }
    }

    fn update_virtualizer(&mut self) {
        // println!("virtualizer before {:?}", self.virtualizer.out_of_play);
        self.virtualizer = Virtualizer::new(self.game.out_of_play_cards().clone());
        // println!("virtualizer  after {:?}", self.virtualizer.out_of_play);
    }

    pub fn undo(&mut self) {
        if self.game.trick_complete() {
            // we are moving back a trick, update virtualizer and distribution field
            let card = self.game.undo().unwrap();
            self.update_virtualizer();
            let last_leader = self.game.trick_leader();
            // print!("Putting virt cards back into dist field: ");
            for (index, card) in self
                .game
                .cards_in_current_trick()
                .iter()
                .chain(std::iter::once(&card))
                // .inspect(|card| println!("real card is {}", card))
                .map(|card| self.virtualizer.absolute_to_virtual_card(card).unwrap())
                .enumerate()
                .sorted_unstable_by_key(|(_, card)| card.rank)
                .rev()
            {
                // println!("{:032b}", self.distribution_field[card.suit as usize]);
                // println!("adding {}", card);
                self.add_card_to_distribution_field(card, last_leader + index);
                // println!("{:032b}", self.distribution_field[card.suit as usize]);
            }
            // let a = self.distribution_field;
            // let b = self.generate_distribution_field();
            // println!("fields are: ");
            // for index in 0..4 {
            //     println!("{:032b} v {:032b}", a[index], b[index]);
            // }
            // assert_eq!(a, b);
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
