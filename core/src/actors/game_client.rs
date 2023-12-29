use crate::engine::mock_bridge_engine::MockBridgeEngine;
use crate::engine::subjective_game_view::SubjectiveGameStateView;
use crate::engine::{Move, SelectMove};
use crate::error::BBError;
use crate::game::GameState;
use crate::interactive::cli_move_selector::CliMoveSelector;
use crate::primitives::deal::Seat;

use crate::primitives::game_event::{BidEvent, CardEvent, GameEvent};
use crate::primitives::player_event::PlayerEvent;

pub struct GameClient<'a> {
    seat: Seat,
    game: Option<GameState>,
    move_selector: Box<dyn SelectMove + 'a>,
}

impl<'a> GameClient<'a> {
    pub fn process_game_event(&mut self, event: GameEvent) -> Result<(), BBError> {
        match event {
            GameEvent::NewGame(new_game_event) => {
                self.game = Some(GameState::from_new_game_event(new_game_event));
                Ok(())
            }
            _ => match &mut self.game {
                None => Err(BBError::GameHasNotStarted)?,
                Some(game) => game.process_game_event(event),
            },
        }?;
        self.move_selector.process_game_event(event)
    }

    pub fn get_move(&self) -> Result<PlayerEvent, BBError> {
        match &self.game {
            None => Err(BBError::GameHasNotStarted),
            Some(game) => self.get_move_for_game(game),
        }
    }

    fn get_move_for_game(&self, game: &GameState) -> Result<PlayerEvent, BBError> {
        match game.next_to_play() {
            Some(next_player)
                if next_player == self.seat || Some(next_player) == self.dummy() && self.can_play_for_dummy() =>
            {
                let chosen_move = self
                    .move_selector
                    .select_move(SubjectiveGameStateView::new(game, self.seat))?;
                Ok(Self::wrap_move_in_event(chosen_move, next_player))
            }
            Some(next_player) => Err(BBError::CannotPlayFor(next_player)),
            None => Err(BBError::OutOfTurn(None)),
        }
    }

    fn wrap_move_in_event(chosen_move: Move, player: Seat) -> PlayerEvent {
        match chosen_move {
            Move::Bid(bid) => PlayerEvent::Bid(BidEvent { player, bid }),
            Move::Card(card) => PlayerEvent::Card(CardEvent { player, card }),
        }
    }

    pub fn new_with_engine(seat: Seat) -> Self {
        GameClient {
            seat,
            game: None,
            move_selector: Box::new(MockBridgeEngine::new(seat)),
        }
    }

    pub fn new_interactive(seat: Seat) -> Self {
        GameClient {
            seat,
            game: None,
            move_selector: Box::new(CliMoveSelector::new(seat)),
        }
    }

    pub fn new_with_move_selector<T: SelectMove + 'a>(seat: Seat, selector: T) -> Self {
        GameClient {
            seat,
            game: None,
            move_selector: Box::new(selector),
        }
    }

    pub fn can_play_for_dummy(&self) -> bool {
        match &self.game {
            Some(GameState::CardPlay(state)) => state.declarer() == self.seat,
            _ => false,
        }
    }

    pub fn dummy(&self) -> Option<Seat> {
        match &self.game {
            Some(GameState::CardPlay(state)) => Some(state.declarer().partner()),
            _ => None,
        }
    }

    pub fn seat(&self) -> Seat {
        self.seat
    }
}

#[cfg(test)]
mod test {
    use crate::actors::game_client::GameClient;
    use crate::primitives::bid::{Bid, ContractBid};
    use crate::primitives::contract::Contract;
    use crate::primitives::deal::Board;
    use crate::primitives::game_event::GameEvent::{BiddingEnded, DiscloseHand};
    use crate::primitives::game_event::{
        BidEvent, BiddingEndedEvent, CardEvent, DiscloseHandEvent, GameEvent, NewGameEvent,
    };
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

        let mut player = GameClient::new_with_engine(seat);

        let ng_event = NewGameEvent { board };
        let event = GameEvent::NewGame(ng_event);

        player.process_game_event(event).unwrap();

        let hand_event = DiscloseHand(DiscloseHandEvent { seat, hand });

        player.process_game_event(hand_event).unwrap();

        let player_event = player.get_move().unwrap();

        let expected_event = PlayerEvent::Bid(BidEvent {
            player: seat,
            bid: Bid::Contract(ContractBid::from_str("1C").unwrap()),
        });

        assert_eq!(player_event, expected_event);
    }

    #[test]
    fn card_player() {
        let hand = Hand::from_str("S:AKQ,H:AKQ,D:AKQ,C:AKQJ").unwrap();
        let board = Board::from_number(5);
        // let game = Game::new_from_board(board);

        let seat = board.dealer();

        let mut player = GameClient::new_with_engine(seat);

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

        let final_contract = Contract::from_str("W1NT").unwrap();

        let bidding_ended_event = BiddingEnded(BiddingEndedEvent { final_contract });
        player.process_game_event(bidding_ended_event).unwrap();

        let player_event = player.get_move().unwrap();

        let expected_event = PlayerEvent::Card(CardEvent {
            player: seat,
            card: Card::from_str("CJ").unwrap(),
        });

        assert_eq!(player_event, expected_event);
    }
}
