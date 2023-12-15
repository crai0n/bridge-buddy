use crate::engine::bidding::BidFinder;
use crate::engine::card_play::CardFinder;
use crate::engine::MoveFinder;

use crate::game::game_state::{Bidding, CardPlay, GameState, OpeningLead};

use crate::interactive::cli_bid_finder::CliBidFinder;
use crate::interactive::cli_card_finder::CliCardFinder;

use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::Card;

#[allow(dead_code)]
pub struct CliMoveFinder {
    seat: Seat,
    bid_finder: CliBidFinder,
    card_finder: CliCardFinder,
}

impl CliMoveFinder {
    pub fn new(seat: Seat) -> Self {
        Self {
            seat,
            bid_finder: CliBidFinder::new(seat),
            card_finder: CliCardFinder::new(seat),
        }
    }
}

impl MoveFinder for CliMoveFinder {
    fn find_bid(&self, game_state: &GameState<Bidding>) -> Bid {
        self.bid_finder.find_bid(game_state)
    }

    fn pick_opening_lead(&self, game_state: &GameState<OpeningLead>) -> Card {
        self.card_finder.find_opening_lead(game_state)
    }

    fn pick_card_for(&self, game_state: &GameState<CardPlay>, seat: Seat) -> Card {
        self.card_finder.find_card_for(game_state, seat)
    }

    fn seat(&self) -> Seat {
        self.seat
    }
}
