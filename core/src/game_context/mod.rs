// Game context stored all the hints derived from openly available information about the game.

pub mod hand_info;
mod turn_rank;
mod vulnerability;

use crate::game_context::hand_info::HandInfo;
use crate::game_context::turn_rank::TurnRank;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::{Deal, Hand, Suit};
use vulnerability::Vulnerability;

pub struct GameContext<'a> {
    pub partner: Vec<HandInfo>,
    pub lho: Vec<HandInfo>,
    pub rho: Vec<HandInfo>,
    pub vulnerable: Vulnerability,
    pub my_turn_rank: TurnRank,
    pub my_hand: &'a Hand,
    pub trump_suit: Option<Suit>,
    pub long_suits_shown_by_opponents: Vec<Suit>,
}

impl<'a> GameContext<'a> {
    fn play_deal_as(deal: &Deal, position: PlayerPosition) -> GameContext {
        let partner = Vec::new();
        let lho = Vec::new();
        let rho = Vec::new();
        let long_suits_shown_by_opponents = Vec::new();
        let vulnerable = Vulnerability::interpret_board_vulnerability(&deal.board.vulnerable(), position);
        let my_turn_rank = TurnRank::turn_rank_for_board(&deal.board, position);
        let my_hand = deal.hand(position);
        let trump_suit = None;
        GameContext {
            partner,
            lho,
            rho,
            vulnerable,
            my_turn_rank,
            my_hand,
            trump_suit,
            long_suits_shown_by_opponents,
        }
    }

    pub fn basic_context_from_hand(hand: &Hand) -> GameContext {
        let partner = Vec::new();
        let lho = Vec::new();
        let rho = Vec::new();
        let long_suits_shown_by_opponents = Vec::new();
        let vulnerable = Vulnerability::None;
        let my_turn_rank = TurnRank::First;
        let my_hand = hand;
        let trump_suit = None;
        GameContext {
            partner,
            lho,
            rho,
            vulnerable,
            my_turn_rank,
            my_hand,
            trump_suit,
            long_suits_shown_by_opponents,
        }
    }
}

impl<'a> From<&'a Hand> for GameContext<'a> {
    fn from(hand: &'a Hand) -> Self {
        Self::basic_context_from_hand(hand)
    }
}
