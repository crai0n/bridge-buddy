use crate::error::BBError;
use crate::game::Game;
use crate::player::engine::{MockBiddingEngine, MockCardPlayEngine};
use crate::primitives::bid::Bid;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::GameEvent;
use crate::primitives::player_event::{BidEvent, CardEvent, PlayerEvent};
use crate::primitives::Card;

pub mod engine;

pub trait Player {
    fn process_game_event(&mut self, event: GameEvent) -> Result<(), BBError>;
    fn make_move(&self) -> Result<PlayerEvent, BBError>;
}

pub struct AutoPlayer {
    seat: Seat,
    game: Option<Game>,
    bidding_engine: MockBiddingEngine,
    card_play_engine: MockCardPlayEngine,
}

impl Player for AutoPlayer {
    fn process_game_event(&mut self, event: GameEvent) -> Result<(), BBError> {
        match event {
            GameEvent::NewGame(new_game_event) => {
                self.game = Some(Game::from_new_game_event(new_game_event));
                Ok(())
            }
            _ => match &mut self.game {
                None => Err(BBError::GameHasNotStarted)?,
                Some(game) => game.process_game_event(event),
            },
        }
    }

    fn make_move(&self) -> Result<PlayerEvent, BBError> {
        match &self.game.as_ref().unwrap() {
            Game::Bidding(state) => {
                let bid = self.bidding_engine.find_bid(state);
                Ok(self.make_bid(bid))
            }
            Game::CardPlay(state) => {
                let card = self.card_play_engine.pick_card(state);
                Ok(self.play_card(card))
            }
            Game::OpeningLead(state) => {
                let card = self.card_play_engine.pick_opening_lead(state);
                Ok(self.play_card(card))
            }
            Game::WaitingForDummy(_) => Err(BBError::OutOfTurn(None)),
            Game::Ended(_) => Err(BBError::GameHasEnded),
        }
    }
}

impl AutoPlayer {
    fn make_bid(&self, bid: Bid) -> PlayerEvent {
        let bid_event = BidEvent { player: self.seat, bid };
        PlayerEvent::Bid(bid_event)
    }

    fn play_card(&self, card: Card) -> PlayerEvent {
        let card_event = CardEvent {
            player: self.seat,
            card,
        };
        PlayerEvent::Card(card_event)
    }

    pub fn new(seat: Seat) -> Self {
        AutoPlayer {
            seat,
            game: None,
            bidding_engine: MockBiddingEngine::new(),
            card_play_engine: MockCardPlayEngine::new(seat),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::player::{AutoPlayer, Player};
    use crate::primitives::bid::{Bid, ContractBid};
    use crate::primitives::deal::Board;
    use crate::primitives::game_event::GameEvent::DiscloseHand;
    use crate::primitives::game_event::{BidEvent, CardEvent, DiscloseHandEvent, GameEvent, NewGameEvent};
    use crate::primitives::player_event::PlayerEvent;
    use crate::primitives::{Card, Hand};
    use std::str::FromStr;

    // use test_case::test_case;

    #[test]
    fn player() {
        let hand = Hand::from_str("S:AKQ,H:AKQ,D:AKQ,C:AKQJ").unwrap();
        let board = Board::from_number(5);
        // let game = Game::new_from_board(board);

        let seat = board.dealer();

        let mut player = AutoPlayer::new(seat);

        let ng_event = NewGameEvent { board };
        let event = GameEvent::NewGame(ng_event);

        player.process_game_event(event).unwrap();

        let hand_event = DiscloseHand(DiscloseHandEvent { seat, hand });

        player.process_game_event(hand_event).unwrap();

        let player_event = player.make_move().unwrap();

        let expected_event = PlayerEvent::Bid(BidEvent {
            player: seat,
            bid: Bid::Contract(ContractBid::from_str("1C").unwrap()),
        });

        assert_eq!(player_event, expected_event);

        let game_event = GameEvent::from(player_event);

        player.process_game_event(game_event).unwrap();

        let player_event = player.make_move().unwrap();

        let expected_event = PlayerEvent::Bid(BidEvent {
            player: seat,
            bid: Bid::Contract(ContractBid::from_str("1D").unwrap()),
        });

        assert_eq!(player_event, expected_event);
    }

    #[test]
    fn card_player() {
        let hand = Hand::from_str("S:AKQ,H:AKQ,D:AKQ,C:AKQJ").unwrap();
        let board = Board::from_number(5);
        // let game = Game::new_from_board(board);

        let seat = board.dealer();

        let mut player = AutoPlayer::new(seat);

        let ng_event = NewGameEvent { board };
        let event = GameEvent::NewGame(ng_event);

        player.process_game_event(event).unwrap();

        let hand_event = DiscloseHand(DiscloseHandEvent { seat, hand });

        player.process_game_event(hand_event).unwrap();

        let bids = ["P", "P", "P", "1NT", "P", "P", "P"];

        let mut player_pos = board.dealer();

        for bid in bids {
            let bid = Bid::from_str(bid).unwrap();
            let event = GameEvent::Bid(BidEvent {
                player: player_pos,
                bid,
            });
            player.process_game_event(event).unwrap();
            player_pos = player_pos + 1;
        }

        let player_event = player.make_move().unwrap();

        let expected_event = PlayerEvent::Card(CardEvent {
            player: seat,
            card: Card::from_str("CJ").unwrap(),
        });

        assert_eq!(player_event, expected_event);
    }
}
