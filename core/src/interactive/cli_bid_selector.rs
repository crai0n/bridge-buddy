use crate::engine::bidding_engine::SelectBid;
use crate::engine::subjective_game_view::SubjectiveGameDataView;
use crate::game::game_phase_states::BiddingState;
use crate::interactive::cli_presenter::CliPresenter;
use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use std::io::stdin;
use std::str::FromStr;

pub struct CliBidSelector {
    _seat: Seat,
}

impl CliBidSelector {
    pub fn new(seat: Seat) -> Self {
        CliBidSelector { _seat: seat }
    }

    pub fn get_bid_from_user(&self, state: SubjectiveGameDataView<BiddingState>) -> Bid {
        CliPresenter::display_bidding_state_for_user(&state);
        CliPresenter::display_starting_hand_for_user(state.my_starting_hand().unwrap());

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

            if state.validate_bid(user_bid).is_ok() {
                break;
            } else {
                println!("That bid is not available anymore!");
            }
        }

        user_bid
    }
}

impl SelectBid for CliBidSelector {
    fn select_bid(&self, state: SubjectiveGameDataView<BiddingState>) -> Bid {
        self.get_bid_from_user(state)
    }
}
