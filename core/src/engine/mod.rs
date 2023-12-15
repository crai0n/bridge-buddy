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
    fn select_move_for(&self, game: &Game, seat: Seat) -> Result<PlayerEvent, BBError> {
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
                let bid = self.select_bid(state);
                Ok(make_bid(self.seat(), bid))
            }
            Game::OpeningLead(state) => {
                if seat != self.seat() {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let card = self.select_opening_lead(state);
                Ok(play_card_as(card, seat))
            }
            Game::CardPlay(state) => {
                if seat != state.inner.contract.declarer.partner() && seat != self.seat() {
                    return Err(BBError::CannotPlayFor(seat));
                }
                let card = self.select_card_for(state, seat);
                Ok(play_card_as(card, seat))
            }
            Game::WaitingForDummy(_) => Err(BBError::OutOfTurn(None)),
            Game::Ended(_) => Err(BBError::GameHasEnded),
        }
    }
}
