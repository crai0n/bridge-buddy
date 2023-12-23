use crate::engine::card_play_engine::SelectCard;
use crate::game::game_state::{CardPlay, GameState, NextToPlay, OpeningLead};
use crate::interactive::cli_presenter::CliPresenter;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::CardEvent;
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

    fn get_card_from_user(&self, state: &GameState<CardPlay>) -> Card {
        CliPresenter::display_dummys_hand_for_user(
            &state
                .inner
                .hand_manager
                .known_remaining_cards_of(state.inner.contract.declarer.partner()),
        );
        CliPresenter::display_trick_for_user(state);
        CliPresenter::display_hand_for_user(&state.inner.hand_manager.known_remaining_cards_of(self.seat));

        let seat = state.next_to_play();
        if seat == self.seat {
            println!("You have to play from your own hand!");
        } else {
            println!("You have to play from dummy's hand!");
        }

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
            let event = CardEvent {
                player: seat,
                card: user_card,
            };

            if state.validate_play_card_event(event).is_ok() {
                break;
            } else {
                println!("You can't play that card!");
            }
        }

        user_card
    }

    fn get_opening_lead_from_user(&self, state: &GameState<OpeningLead>) -> Card {
        CliPresenter::display_final_contract_for_user(state);
        CliPresenter::display_hand_for_user(&state.inner.hand_manager.known_remaining_cards_of(self.seat));

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
            let event = CardEvent {
                player: self.seat,
                card: user_card,
            };

            if state.validate_play_card_event(event).is_ok() {
                break;
            } else {
                println!("You can't play that card!");
            }
        }

        user_card
    }
}

impl SelectCard for CliCardSelector {
    fn select_card(&self, state: &GameState<CardPlay>) -> Card {
        self.get_card_from_user(state)
    }

    fn select_opening_lead(&self, state: &GameState<OpeningLead>) -> Card {
        self.get_opening_lead_from_user(state)
    }
}
