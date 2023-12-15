use crate::engine::bidding::BidFinder;
use crate::game::game_state::{Bidding, GameState};
use crate::interactive::cli_presenter::CliPresenter;
use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::BidEvent;
use std::io::stdin;
use std::str::FromStr;

pub struct CliBidFinder {
    seat: Seat,
}

impl CliBidFinder {
    pub fn new(seat: Seat) -> Self {
        CliBidFinder { seat }
    }

    pub fn get_bid_from_user(&self, state: &GameState<Bidding>) -> Bid {
        CliPresenter::display_bidding_state_for_user(state);
        CliPresenter::display_hand_for_user(&state.inner.hand_manager.known_remaining_cards_of(self.seat));

        println!("What do you want to bid?");

        let mut user_input;
        let mut user_bid: Bid;

        loop {
            user_input = String::new();
            stdin().read_line(&mut user_input).unwrap();
            user_bid = match Bid::from_str(user_input.trim()) {
                Ok(bid) => bid,
                _ => {
                    println!("That's not a valid bid!");
                    continue;
                }
            };

            let event = BidEvent {
                player: self.seat,
                bid: user_bid,
            };

            if state.validate_make_bid_event(event).is_ok() {
                break;
            } else {
                println!("That bid is not available anymore!");
            }
        }

        user_bid
    }
}

impl BidFinder for CliBidFinder {
    fn find_bid(&self, state: &GameState<Bidding>) -> Bid {
        self.get_bid_from_user(state)
    }
}
