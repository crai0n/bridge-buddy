use crate::engine::bidding_engine::SelectBid;
use crate::engine::card_play_engine::SelectCard;
use crate::engine::SelectMove;

use crate::game::game_state::{Bidding, CardPlay, GameState, OpeningLead};

use crate::interactive::cli_bid_selector::CliBidSelector;
use crate::interactive::cli_card_selector::CliCardSelector;

use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::Card;

#[allow(dead_code)]
pub struct CliMoveSelector {
    seat: Seat,
    bid_selector: CliBidSelector,
    card_selector: CliCardSelector,
}

impl CliMoveSelector {
    pub fn new(seat: Seat) -> Self {
        Self {
            seat,
            bid_selector: CliBidSelector::new(seat),
            card_selector: CliCardSelector::new(seat),
        }
    }
}

impl SelectMove for CliMoveSelector {}

impl SelectBid for CliMoveSelector {
    fn select_bid(&self, game_state: &GameState<Bidding>) -> Bid {
        self.bid_selector.select_bid(game_state)
    }
}

impl SelectCard for CliMoveSelector {
    fn select_card(&self, game_state: &GameState<CardPlay>) -> Card {
        self.card_selector.select_card(game_state)
    }

    fn select_opening_lead(&self, game_state: &GameState<OpeningLead>) -> Card {
        self.card_selector.select_opening_lead(game_state)
    }

    fn seat(&self) -> Seat {
        self.seat
    }
}
