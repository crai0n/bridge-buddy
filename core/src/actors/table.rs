use crate::actors::game_client::GameClient;
use crate::actors::game_manager::GameManager;
use crate::error::BBError;
use crate::game::scoring::ScorePoints;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::GameEvent;
use crate::primitives::Deal;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

pub struct Table<'a> {
    game_manager: Option<GameManager>,
    seats: BTreeMap<Seat, &'a mut (dyn GameClient)>,
}

impl<'a> Table<'a> {
    pub fn empty() -> Self {
        Table {
            game_manager: None,
            seats: BTreeMap::new(),
        }
    }

    pub fn seat_player(&mut self, player: &'a mut impl GameClient, seat: Seat) -> Result<(), BBError> {
        if let Entry::Vacant(e) = self.seats.entry(seat) {
            e.insert(player);
            Ok(())
        } else {
            Err(BBError::SeatTaken(seat))
        }
    }

    pub fn new_game(&mut self) -> Result<(), BBError> {
        let deal = Deal::new();
        self.new_game_from_deal(deal)
    }

    pub fn new_game_from_deal(&mut self, deal: Deal) -> Result<(), BBError> {
        self.game_manager = Some(GameManager::new_from_deal(deal));
        Ok(())
    }

    fn broadcast_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::DiscloseHand(dh_event) => {
                let seat = dh_event.seat;
                self.seats.get_mut(&seat).unwrap().process_game_event(event).unwrap();
            }
            _ => {
                for player in self.seats.values_mut() {
                    player.process_game_event(event).unwrap();
                }
            }
        }
    }

    pub fn run_game(&mut self) -> Result<ScorePoints, BBError> {
        if let Some(ref mut manager) = self.game_manager {
            manager.start_game()?;

            let mut published_events = Vec::new();

            let mut i = 0;

            let mut dummy = None;

            loop {
                let history = self.game_manager.as_ref().unwrap().history();

                for &event in &history[published_events.len()..] {
                    // println!("Found new event: {:?}", event);
                    self.broadcast_event(event);
                    published_events.push(event);

                    if let GameEvent::GameEnded(ge_event) = event {
                        return Ok(ge_event.score);
                    }

                    if let GameEvent::BiddingEnded(be_event) = event {
                        dummy = Some(be_event.final_contract.declarer.partner());
                    }
                }

                let next_player = self.game_manager.as_ref().unwrap().next_to_play().unwrap();
                // println!("Next Player: {:?}", next_player);

                let player_event = if Some(next_player) == dummy {
                    self.seats.get(&dummy.unwrap().partner()).unwrap().get_move().unwrap()
                } else {
                    self.seats.get(&next_player).unwrap().get_move().unwrap()
                };

                // println!("Player made move: {:?}", player_event);

                self.game_manager
                    .as_mut()
                    .unwrap()
                    .process_player_event(player_event)
                    .unwrap();

                i += 1;

                if i > 1000 {
                    return Err(BBError::GameStuck);
                }
            }
        } else {
            Err(BBError::NoGame)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::actors::game_client::auto_game_client::AutoGameClient;
    use crate::actors::table::Table;
    use crate::primitives::deal::Seat::*;

    #[test]
    fn run_game() {
        let mut table = Table::empty();

        let mut north_player = AutoGameClient::new(North);
        let mut south_player = AutoGameClient::new(South);
        let mut east_player = AutoGameClient::new(East);
        let mut west_player = AutoGameClient::new(West);

        table.seat_player(&mut north_player, North).unwrap();
        table.seat_player(&mut south_player, South).unwrap();
        table.seat_player(&mut east_player, East).unwrap();
        table.seat_player(&mut west_player, West).unwrap();

        table.new_game().unwrap();
        table.run_game().unwrap();
    }
}
