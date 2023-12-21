use crate::error::BBError;

use crate::primitives::game_event::GameEvent;
use crate::primitives::player_event::PlayerEvent;

pub mod auto_game_client;

pub trait GameClient {
    fn process_game_event(&mut self, event: GameEvent) -> Result<(), BBError>;

    fn get_move(&self) -> Result<PlayerEvent, BBError>;
}

#[cfg(test)]
mod test {
    use crate::actors::game_client::auto_game_client::AutoGameClient;
    use crate::actors::game_client::GameClient;
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

        let mut player = AutoGameClient::new(seat);

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

        let mut player = AutoGameClient::new(seat);

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

        let player_event = player.get_move().unwrap();

        let expected_event = PlayerEvent::Card(CardEvent {
            player: seat,
            card: Card::from_str("CJ").unwrap(),
        });

        assert_eq!(player_event, expected_event);
    }
}
