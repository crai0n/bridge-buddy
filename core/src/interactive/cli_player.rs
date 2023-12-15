use crate::error::BBError;
use crate::game::game_state::{Bidding, CardPlay, GameState, OpeningLead};
use crate::game::Game;
use crate::interactive::cli_presenter::CliPresenter;
use crate::player::{Move, Player};
use crate::presentation::PresentEvent;
use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::{BidEvent, CardEvent, GameEvent};
use crate::primitives::player_event::PlayerEvent;
use crate::primitives::Card;
use std::io::stdin;
use std::str::FromStr;

pub struct CliPlayer {
    seat: Seat,
    game: Option<Game>,
    presenter: CliPresenter,
}

impl Player for CliPlayer {
    fn process_game_event(&mut self, event: GameEvent) -> Result<(), BBError> {
        match event {
            GameEvent::NewGame(new_game_event) => {
                self.game = Some(Game::from_new_game_event(new_game_event));
                self.presenter.present_event(GameEvent::NewGame(new_game_event));
                Ok(())
            }
            _ => match &mut self.game {
                None => Err(BBError::GameHasNotStarted)?,
                Some(game) => {
                    game.process_game_event(event)?;
                    self.presenter.present_event(event);
                    Ok(())
                }
            },
        }
    }
}
impl Move for CliPlayer {
    fn get_move(&self) -> Result<PlayerEvent, BBError> {
        self.get_move_for(self.seat)
    }

    fn get_dummy_move(&self) -> Result<PlayerEvent, BBError> {
        self.get_move_for(self.seat.partner())
    }
}

impl CliPlayer {
    fn get_move_for(&self, seat: Seat) -> Result<PlayerEvent, BBError> {
        match &self.game.as_ref().unwrap() {
            Game::Bidding(state) => {
                if seat != self.seat {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let bid = self.get_bid_from_user(state);
                Ok(self.make_bid_as(bid, seat))
            }
            Game::OpeningLead(state) => {
                if seat != self.seat {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let card = self.get_opening_lead_from_user(state);
                Ok(self.play_card_as(card, seat))
            }
            Game::CardPlay(state) => {
                if seat != state.inner.contract.declarer.partner() && seat != self.seat {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let card = self.get_card_from_user_for(state, seat);
                Ok(self.play_card_as(card, seat))
            }
            Game::WaitingForDummy(_) => Err(BBError::OutOfTurn(None)),
            Game::Ended(_) => Err(BBError::GameHasEnded),
        }
    }

    fn make_bid_as(&self, bid: Bid, seat: Seat) -> PlayerEvent {
        let bid_event = BidEvent { player: seat, bid };
        PlayerEvent::Bid(bid_event)
    }

    fn get_bid_from_user(&self, state: &GameState<Bidding>) -> Bid {
        self.display_bidding_state_for_user(state);
        self.display_hand_for_user(&state.inner.hand_manager.known_remaining_cards_of(self.seat));

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

    fn display_bidding_state_for_user(&self, state: &GameState<Bidding>) {
        println!("The bidding so far is: ");
        print!("{}", state.inner.bid_manager)
    }

    pub fn display_final_contract_for_user(&self, state: &GameState<OpeningLead>) {
        println!("The final contract is: {}", state.inner.contract);
    }

    fn display_hand_for_user(&self, cards: &[Card]) {
        println!("Your hand:");
        for card in cards {
            print!("{}", card)
        }
        println!();
    }

    fn display_dummys_hand_for_user(&self, cards: &[Card]) {
        println!("Dummies Hand:");
        for card in cards {
            print!("{}", card)
        }
        println!();
    }

    fn display_trick_for_user(&self, state: &GameState<CardPlay>) {
        if let Some(trick) = state.inner.trick_manager.current_trick() {
            println!("Current Trick: {}", trick)
        }
    }

    fn get_card_from_user_for(&self, state: &GameState<CardPlay>, seat: Seat) -> Card {
        self.display_dummys_hand_for_user(
            &state
                .inner
                .hand_manager
                .known_remaining_cards_of(state.inner.contract.declarer.partner()),
        );
        self.display_trick_for_user(state);
        self.display_hand_for_user(&state.inner.hand_manager.known_remaining_cards_of(self.seat));

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
        self.display_final_contract_for_user(state);
        self.display_hand_for_user(&state.inner.hand_manager.known_remaining_cards_of(self.seat));

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

    fn play_card_as(&self, card: Card, seat: Seat) -> PlayerEvent {
        let card_event = CardEvent { player: seat, card };
        PlayerEvent::Card(card_event)
    }

    pub fn new(seat: Seat) -> Self {
        CliPlayer {
            seat,
            game: None,
            presenter: CliPresenter { seat },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::Game;
    use crate::interactive::cli_player::CliPlayer;
    use crate::player::Player;
    use crate::primitives::deal::Board;
    use crate::primitives::game_event::GameEvent::DiscloseHand;
    use crate::primitives::game_event::{DiscloseHandEvent, GameEvent, NewGameEvent};
    use crate::primitives::Hand;
    use std::str::FromStr;

    #[allow(dead_code)]
    fn display_hand() {
        let hand = Hand::from_str("S:AKQ,H:AKQ,D:AKQ,C:AKQJ").unwrap();
        let board = Board::from_number(5);

        let seat = board.dealer();

        let mut player = CliPlayer::new(seat);

        let ng_event = NewGameEvent { board };
        let event = GameEvent::NewGame(ng_event);

        player.process_game_event(event).unwrap();

        let hand_event = DiscloseHand(DiscloseHandEvent { seat, hand });

        player.process_game_event(hand_event).unwrap();

        let _bid = match player.game.as_ref().unwrap() {
            Game::Bidding(state) => player.get_bid_from_user(state),
            _ => panic!(),
        };
    }
}
