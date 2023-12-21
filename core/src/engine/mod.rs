use crate::engine::bidding_engine::SelectBid;
use crate::engine::card_play_engine::SelectCard;
use crate::error::BBError;
use crate::game::Game;
use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::{BidEvent, CardEvent};
use crate::primitives::player_event::PlayerEvent;
use crate::primitives::Card;

pub mod bidding_engine;
pub mod card_play_engine;
pub mod hand_evaluation;
pub mod mock_bridge_engine;

pub trait SelectMove: SelectCard + SelectBid {
    fn select_move(&self, game: &Game) -> Result<PlayerEvent, BBError> {
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
                let next_to_play = state.next_to_play();
                if next_to_play != self.seat() {
                    return Err(BBError::CannotPlayFor(next_to_play));
                }
                let bid = self.select_bid(state);
                Ok(make_bid(self.seat(), bid))
            }
            Game::OpeningLead(state) => {
                let next_to_play = state.next_to_play();
                if next_to_play != self.seat() {
                    return Err(BBError::CannotPlayFor(next_to_play));
                }
                let card = self.select_opening_lead(state);
                Ok(play_card_as(card, self.seat()))
            }
            Game::CardPlay(state) => {
                let next_to_play = state.next_to_play();
                if next_to_play != state.inner.contract.declarer.partner() && next_to_play != self.seat() {
                    return Err(BBError::CannotPlayFor(next_to_play));
                }
                let card = self.select_card(state);
                Ok(play_card_as(card, next_to_play))
            }
            Game::WaitingForDummy(_) => Err(BBError::OutOfTurn(None)),
            Game::Ended(_) => Err(BBError::GameHasEnded),
        }
    }
}
