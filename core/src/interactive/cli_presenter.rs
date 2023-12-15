use crate::presentation::PresentEvent;
use crate::primitives::deal::Seat;
use crate::primitives::game_event::{
    BidEvent, BiddingEndedEvent, CardEvent, DiscloseHandEvent, DummyUncoveredEvent, GameEndedEvent, GameEvent,
    NewGameEvent,
};

#[allow(dead_code)]
pub struct CliPresenter {
    pub seat: Seat,
}

impl PresentEvent for CliPresenter {
    fn present_event(&self, event: GameEvent) {
        self.print_game_event_to_console(event)
    }
}

impl CliPresenter {
    fn print_game_event_to_console(&self, event: GameEvent) {
        match event {
            GameEvent::NewGame(ng_event) => self.print_new_game_event_to_console(ng_event),
            GameEvent::DiscloseHand(dh_event) => self.print_disclose_hand_event_to_console(dh_event),
            GameEvent::Bid(b_event) => self.print_bid_event_to_console(b_event),
            GameEvent::BiddingEnded(mtcp_event) => self.print_move_to_card_play_event_to_console(mtcp_event),
            GameEvent::Card(c_event) => self.print_card_event_to_console(c_event),
            GameEvent::DummyUncovered(du_event) => self.print_dummy_uncovered_event_to_console(du_event),
            GameEvent::GameEnded(ge_event) => self.print_game_ended_event_to_console(ge_event),
        }
    }

    fn print_new_game_event_to_console(&self, event: NewGameEvent) {
        println!("A new game has started!");
        println!(
            "We are playing board no. {}, {} is dealer, Vulnerable: {:?}",
            event.board.number(),
            event.board.dealer(),
            event.board.vulnerability()
        );
    }

    fn print_disclose_hand_event_to_console(&self, event: DiscloseHandEvent) {
        println!("You've been dealt");
        for card in event.hand.cards() {
            print!("{}", card)
        }
        println!();
    }

    fn print_bid_event_to_console(&self, event: BidEvent) {
        println!("{} bid {}", event.player, event.bid)
    }

    fn print_move_to_card_play_event_to_console(&self, event: BiddingEndedEvent) {
        println!("Bidding has ended!");
        println!(
            "The final contract is {}{}{} played by {}",
            event.final_contract.level,
            event.final_contract.denomination,
            event.final_contract.state,
            event.final_contract.declarer
        );
        println!("{} plays the opening lead", event.final_contract.declarer + 1);
    }

    fn print_card_event_to_console(&self, event: CardEvent) {
        println!("{} played {}", event.player, event.card)
    }

    fn print_game_ended_event_to_console(&self, event: GameEndedEvent) {
        println!("The game ended");
        println!("Result is {:?}", event.result);
        println!("Final Score is: {:?}", event.score)
    }

    fn print_dummy_uncovered_event_to_console(&self, event: DummyUncoveredEvent) {
        println!("Dummy has shown their hand:");
        println!("{}", event.dummy)
    }
}
