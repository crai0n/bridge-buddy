use crate::error::BBError;
use crate::game::game_state::{Bidding, CardPlay, GameState, OpeningLead};
use crate::game::Game;
use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::{BidEvent, CardEvent};
use crate::primitives::player_event::PlayerEvent;
use crate::primitives::Card;

pub mod auto_move_finder;
pub mod bidding;
pub mod card_play;
pub mod evaluator;

pub trait MoveFinder {
    fn find_bid(&self, game_state: &GameState<Bidding>) -> Bid;

    fn pick_opening_lead(&self, game_state: &GameState<OpeningLead>) -> Card;

    fn pick_card_for(&self, game_state: &GameState<CardPlay>, seat: Seat) -> Card;

    fn seat(&self) -> Seat;

    fn find_move_for(&self, game: &Game, seat: Seat) -> Result<PlayerEvent, BBError> {
        fn make_bid(seat: Seat, bid: Bid) -> PlayerEvent {
            let bid_event = BidEvent { player: seat, bid };
            PlayerEvent::Bid(bid_event)
        }

        fn play_card_as(card: Card, seat: Seat) -> PlayerEvent {
            let card_event = CardEvent { player: seat, card };
            PlayerEvent::Card(card_event)
        }

        match game {
            Game::Bidding(state) => {
                if seat != self.seat() {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let bid = self.find_bid(state);
                Ok(make_bid(self.seat(), bid))
            }
            Game::OpeningLead(state) => {
                if seat != self.seat() {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let card = self.pick_opening_lead(state);
                Ok(play_card_as(card, seat))
            }
            Game::CardPlay(state) => {
                if seat != state.inner.contract.declarer.partner() && seat != self.seat() {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let card = self.pick_card_for(state, seat);
                Ok(play_card_as(card, seat))
            }
            Game::WaitingForDummy(_) => Err(BBError::OutOfTurn(None)),
            Game::Ended(_) => Err(BBError::GameHasEnded),
        }
    }
}
