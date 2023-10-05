// GameContext contains a reference to the publicly available game-state, which it uses to present information in a
// format usable to the engine (abstracting away the absolute seating position for example). In addition, it contains
// information that is not explicitly available from the Game's state, that is which does
// not immediately follow from the rules of the game, but is learned through interpretation of the actions of the
// players according to the inner rules of the engine. It is therefore the main data structure for communication between
// the different parts of the engine, that is the separate modules responsible for hand evaluation, bid interpretation
// and card play.

pub mod hand_description;
mod turn_rank;
mod vulnerability;

use crate::game_context::hand_description::HandDescription;
use crate::game_context::turn_rank::TurnRank;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::{Deal, Hand, Suit};
use vulnerability::Vulnerability;

pub struct GameContext<'a> {
    pub partner: Vec<HandDescription>,
    pub lho: Vec<HandDescription>,
    pub rho: Vec<HandDescription>,
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
