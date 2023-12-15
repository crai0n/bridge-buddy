use crate::error::BBError;
use crate::game::game_state::{Bidding, CardPlay, GameState, OpeningLead};
use crate::game::Game;
use crate::interactive::cli_bid_finder::CliBidFinder;
use crate::interactive::cli_card_finder::CliCardFinder;
use crate::interactive::cli_presenter::CliPresenter;
use crate::player::{Move, Player};
use crate::presentation::PresentEvent;
use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::{BidEvent, CardEvent, GameEvent};
use crate::primitives::player_event::PlayerEvent;
use crate::primitives::Card;

#[allow(dead_code)]
pub struct CliMoveFinder {
    bid_finder: CliBidFinder,
    card_finder: CliCardFinder,
}

impl CliMoveFinder {
    pub fn new(seat: Seat) -> Self {
        Self {
            bid_finder: CliBidFinder::new(seat),
            card_finder: CliCardFinder::new(seat),
        }
    }

    pub fn get_bid_from_user(&self, state: &GameState<Bidding>, presenter: &CliPresenter) -> Bid {
        self.bid_finder.get_bid_from_user(state, presenter)
    }

    pub fn get_card_from_user_for(&self, state: &GameState<CardPlay>, seat: Seat, presenter: &CliPresenter) -> Card {
        self.card_finder.get_card_from_user_for(state, seat, presenter)
    }

    pub fn get_opening_lead_from_user(&self, state: &GameState<OpeningLead>, presenter: &CliPresenter) -> Card {
        self.card_finder.get_opening_lead_from_user(state, presenter)
    }
}

#[allow(dead_code)]
pub struct CliPlayer {
    seat: Seat,
    game: Option<Game>,
    presenter: CliPresenter,
    move_finder: CliMoveFinder,
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
                let bid = self.move_finder.get_bid_from_user(state, &self.presenter);
                Ok(self.make_bid_as(bid, seat))
            }
            Game::OpeningLead(state) => {
                if seat != self.seat {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let card = self.move_finder.get_opening_lead_from_user(state, &self.presenter);
                Ok(self.play_card_as(card, seat))
            }
            Game::CardPlay(state) => {
                if seat != state.inner.contract.declarer.partner() && seat != self.seat {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let card = self.move_finder.get_card_from_user_for(state, seat, &self.presenter);
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

    fn play_card_as(&self, card: Card, seat: Seat) -> PlayerEvent {
        let card_event = CardEvent { player: seat, card };
        PlayerEvent::Card(card_event)
    }

    pub fn new(seat: Seat) -> Self {
        CliPlayer {
            seat,
            game: None,
            presenter: CliPresenter { seat },
            move_finder: CliMoveFinder::new(seat),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::Game;
    use crate::interactive::cli_move_finder::CliPlayer;
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
            Game::Bidding(state) => player.move_finder.get_bid_from_user(state, &player.presenter),
            _ => panic!(),
        };
    }
}
