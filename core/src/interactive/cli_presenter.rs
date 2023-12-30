use crate::engine::subjective_game_view::{SubjectiveGameDataView, SubjectiveSeat};
use crate::game::game_data::{Bidding, CardPlay};
use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::primitives::game_event::{
    BidEvent, BiddingEndedEvent, CardEvent, DiscloseHandEvent, DummyUncoveredEvent, GameEndedEvent, GameEvent,
    NewGameEvent,
};
use crate::primitives::{Card, Hand, Suit};

pub struct CliPresenter {}

impl CliPresenter {
    pub fn print_game_event_to_console(event: GameEvent) {
        match event {
            GameEvent::NewGame(ng_event) => Self::print_new_game_event_to_console(ng_event),
            GameEvent::DiscloseHand(dh_event) => Self::print_disclose_hand_event_to_console(dh_event),
            GameEvent::Bid(b_event) => Self::print_bid_event_to_console(b_event),
            GameEvent::BiddingEnded(mtcp_event) => Self::print_move_to_card_play_event_to_console(mtcp_event),
            GameEvent::Card(c_event) => Self::print_card_event_to_console(c_event),
            GameEvent::DummyUncovered(du_event) => Self::print_dummy_uncovered_event_to_console(du_event),
            GameEvent::GameEnded(ge_event) => Self::print_game_ended_event_to_console(ge_event),
        }
    }

    fn print_new_game_event_to_console(event: NewGameEvent) {
        println!("A new game has started!");
        println!(
            "We are playing board no. {}, {} is dealer, Vulnerable: {:?}",
            event.board.number(),
            event.board.dealer(),
            event.board.vulnerability()
        );
    }

    fn print_disclose_hand_event_to_console(event: DiscloseHandEvent) {
        println!("You've been dealt");
        for card in event.hand.cards() {
            print!("{}", card)
        }
        println!();
    }

    fn print_bid_event_to_console(event: BidEvent) {
        println!("{} bid {}", event.player, event.bid)
    }

    fn print_move_to_card_play_event_to_console(event: BiddingEndedEvent) {
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

    fn print_card_event_to_console(event: CardEvent) {
        println!("{} played {}", event.player, event.card)
    }

    fn print_game_ended_event_to_console(event: GameEndedEvent) {
        println!("The game ended");
        println!("Result is {:?}", event.result);
        println!("Final Score is: {:?}", event.score)
    }

    fn print_dummy_uncovered_event_to_console(event: DummyUncoveredEvent) {
        println!("Dummy has shown their hand:");
        println!("{}", event.dummy)
    }

    pub fn display_bidding_state_for_user(data: &SubjectiveGameDataView<Bidding>) {
        println!("You  LHO  Ptn  RHO");
        println!("------------------");

        let mut x = match data.dealer() {
            SubjectiveSeat::Myself => 0,
            SubjectiveSeat::LeftHandOpponent => 1,
            SubjectiveSeat::Partner => 2,
            SubjectiveSeat::RightHandOpponent => 3,
        };

        let pad = "     ";

        for _i in 0..x {
            print!("{}", pad);
        }

        for bid in data.bids() {
            let bid_str = format!("{}", bid);
            print!("{:<5}", bid_str);
            x += 1;
            if x % 4 == 0 {
                println!();
            }
        }
        if x % 4 != 0 {
            println!();
        }
    }

    pub fn display_hand_for_user(cards: &[Card]) {
        for suit in Suit::iter().rev() {
            let suited_cards = cards.iter().filter(|x| x.suit == suit).rev().collect_vec();
            if !suited_cards.is_empty() {
                print!("{}", suit);
            }
            for card in suited_cards {
                print!("{}", card.denomination);
            }
        }
        println!();
    }

    pub fn display_starting_hand_for_user(hand: Hand) {
        println!("Your hand:");
        Self::display_hand_for_user(hand.cards().copied().collect_vec().as_slice())
    }

    pub fn display_dummys_hand_for_user(cards: &[Card], declarer: SubjectiveSeat) {
        match declarer {
            SubjectiveSeat::Myself => {
                for suit in Suit::iter().rev() {
                    let suited_cards = cards.iter().filter(|x| x.suit == suit).rev().collect_vec();
                    if !suited_cards.is_empty() {
                        print!("{}", suit);
                    }
                    for card in suited_cards {
                        print!("{}", card.denomination);
                    }
                }
                println!();
            }
            SubjectiveSeat::RightHandOpponent => {
                for suit in Suit::iter().rev() {
                    let suited_cards = cards.iter().filter(|x| x.suit == suit);
                    print!("{}", suit);
                    for card in suited_cards.rev() {
                        print!("{}", card.denomination);
                    }
                    println!();
                }
            }
            SubjectiveSeat::LeftHandOpponent => {
                for suit in Suit::iter().rev() {
                    let suited_cards = cards.iter().filter(|x| x.suit == suit);

                    let mut card_string = String::new();
                    for card in suited_cards {
                        card_string += &format!("{}", card.denomination);
                    }
                    card_string += &format!("{}", suit);
                    println!("{:>15}", card_string);
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn display_trick_for_user(state: &SubjectiveGameDataView<CardPlay>) {
        let trick = state.active_trick();

        match (trick.cards(), trick.lead(), state.next_to_play()) {
            ([], SubjectiveSeat::Myself, SubjectiveSeat::Myself) => {
                println!();
                println!();
                println!("  ↓↓");
            }
            ([], SubjectiveSeat::Partner, SubjectiveSeat::Partner) => {
                println!("  ↑↑");
                println!();
                println!();
            }
            ([c1], SubjectiveSeat::RightHandOpponent, SubjectiveSeat::Myself) => {
                println!();
                println!("    {}", c1);
                println!("  ↓↓");
            }
            ([c1, c2], SubjectiveSeat::Partner, SubjectiveSeat::Myself) => {
                println!("  {}", c1);
                println!("    {}", c2);
                println!("  ↓↓");
            }
            ([c1, c2, c3], SubjectiveSeat::LeftHandOpponent, SubjectiveSeat::Myself) => {
                println!("  {}", c2);
                println!("{}  {}", c1, c3);
                println!("  ↓↓");
            }
            ([c1], SubjectiveSeat::LeftHandOpponent, SubjectiveSeat::Partner) => {
                println!("  ↑↑");
                println!("{}    ", c1);
                println!();
            }
            ([c1, c2], SubjectiveSeat::Myself, SubjectiveSeat::Partner) => {
                println!("  ↑↑");
                println!("{}", c2);
                println!("  {}", c1);
            }
            ([c1, c2, c3], SubjectiveSeat::RightHandOpponent, SubjectiveSeat::Partner) => {
                println!("  ↑↑");
                println!("{}  {}", c3, c1);
                println!("  {}", c2);
            }
            _ => unreachable!(),
        }
    }
}
