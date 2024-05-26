use crate::engine::card_play_engine::SelectCard;
use crate::engine::subjective_game_view::SubjectiveGameDataView;
use crate::game::game_phase_states::{CardPlayState, OpeningLeadState};
use crate::interactive::cli_presenter::CliPresenter;
use crate::primitives::deal::Seat;
use crate::primitives::Card;
use std::io::stdin;
use std::str::FromStr;

#[allow(dead_code)]
pub struct CliCardSelector {
    seat: Seat,
}

impl CliCardSelector {
    pub fn new(seat: Seat) -> Self {
        CliCardSelector { seat }
    }

    fn get_card_from_user(&self, state: SubjectiveGameDataView<CardPlayState>) -> Card {
        CliPresenter::display_dummys_hand_for_user(&state.dummys_remaining_cards(), state.declarer());
        CliPresenter::display_trick_for_user(&state);
        CliPresenter::display_hand_for_user(&state.my_remaining_cards());

        let seat = state.next_to_play();
        // if seat == SubjectiveSeat::Myself {
        //     println!("You have to play from your own hand!");
        // } else {
        //     println!("You have to play from dummy's hand!");
        // }

        println!("What card do you want to play?");

        let mut user_input;
        let mut user_card: Card;

        loop {
            user_input = String::new();
            stdin().read_line(&mut user_input).unwrap();
            user_card = match Card::from_str(user_input.trim()) {
                Ok(card) => card,
                _ => {
                    println!("That's not a valid card!");
                    continue;
                }
            };

            if state.validate_card_play(user_card, seat).is_ok() {
                break;
            } else {
                println!("You can't play that card!");
            }
        }

        user_card
    }

    fn get_opening_lead_from_user(&self, state: SubjectiveGameDataView<OpeningLeadState>) -> Card {
        CliPresenter::display_hand_for_user(&state.my_remaining_cards());

        println!("What card do you want to play?");

        let mut user_input;
        let mut user_card: Card;

        loop {
            user_input = String::new();
            stdin().read_line(&mut user_input).unwrap();
            user_card = match Card::from_str(user_input.trim()) {
                Ok(card) => card,
                _ => {
                    println!("That's not a valid card!");
                    continue;
                }
            };

            if state.validate_lead(user_card).is_ok() {
                break;
            } else {
                println!("You can't play that card!");
            }
        }

        user_card
    }
}

impl SelectCard for CliCardSelector {
    fn select_card(&self, state: SubjectiveGameDataView<CardPlayState>) -> Card {
        self.get_card_from_user(state)
    }

    fn select_opening_lead(&self, state: SubjectiveGameDataView<OpeningLeadState>) -> Card {
        self.get_opening_lead_from_user(state)
    }
}
