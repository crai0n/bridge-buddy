use crate::engine::bidding_engine::SelectBid;
use crate::engine::card_play_engine::SelectCard;
use crate::engine::SelectMove;
use crate::error::BBError;

use crate::game::game_data::{Bidding, CardPlay, GameData, OpeningLead};

use crate::interactive::cli_bid_selector::CliBidSelector;
use crate::interactive::cli_card_selector::CliCardSelector;
use crate::interactive::cli_presenter::CliPresenter;

use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::GameEvent;
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

impl SelectMove for CliMoveSelector {
    fn process_game_event(&mut self, event: GameEvent) -> Result<(), BBError> {
        CliPresenter::print_game_event_to_console(event);
        Ok(())
    }
}

impl SelectBid for CliMoveSelector {
    fn select_bid(&self, game_state: &GameData<Bidding>) -> Bid {
        self.bid_selector.select_bid(game_state)
    }
}

impl SelectCard for CliMoveSelector {
    fn select_card(&self, game_state: &GameData<CardPlay>) -> Card {
        self.card_selector.select_card(game_state)
    }

    fn select_opening_lead(&self, game_state: &GameData<OpeningLead>) -> Card {
        self.card_selector.select_opening_lead(game_state)
    }
}
