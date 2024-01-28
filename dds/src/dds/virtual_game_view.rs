use super::double_dummy_state::DoubleDummyState;
use crate::dds::card_manager::suit_field::SuitField;
use bridge_buddy_core::error::BBError;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
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
        let played_cards = self.game.played_cards();
        for suit in Suit::iter() {
            self.played[suit as usize] = *played_cards.suit_state(&suit);
        }
    }

    pub fn undo(&mut self) {
        self.game.undo()
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

    pub fn valid_moves_for(&self, player: Seat) -> Vec<VirtualCard> {
        let absolute_moves = self.game.valid_moves_for(player);
        absolute_moves
            .into_iter()
            .map(|x| self.absolute_to_virtual(x))
            .collect_vec()
    }
}

#[allow(dead_code)]
pub struct VirtualCard {
    suit: Suit,
    rank: VirtualRank,
}
