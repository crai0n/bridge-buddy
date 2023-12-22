use bridge_buddy_core::actors::game_client::GameClient;
use bridge_buddy_core::actors::table::Table;
use bridge_buddy_core::engine::hand_evaluation::ForumDPlus2015Evaluator;
use bridge_buddy_core::engine::mock_bridge_engine::MockBridgeEngine;
use bridge_buddy_core::primitives::card::Suit;
use bridge_buddy_core::primitives::deal::Hand;
use bridge_buddy_core::primitives::deal::Seat::{East, North, South, West};
use bridge_buddy_core::IntoEnumIterator;
use clap::{Parser, Subcommand};
use std::io::stdin;
use std::process::exit;
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// Name of the person to greet
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Evaluate a bridge hand according to Forum D Plus 2015
    Evaluate {
        /// Hand to evaluate (if not given, it will be queried interactively)
        hand: Option<String>,
    },
    Play,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Play => {
            let mut table = Table::empty();

            let north_player = GameClient::new_with_engine(North);
            let south_player = GameClient::new_interactive(South);
            let east_player = GameClient::new_with_engine(East);
            let west_player = GameClient::new_with_move_selector(West, MockBridgeEngine::new(West));

            table.seat_player(north_player, North).unwrap();
            table.seat_player(south_player, South).unwrap();
            table.seat_player(east_player, East).unwrap();
            table.seat_player(west_player, West).unwrap();

            table.new_game().unwrap();
            table.run_game().unwrap();
        }
        Command::Evaluate { hand } => {
            let hand_result = match hand {
                None => {
                    let mut hand = String::new();
                    println!("Which cards do you hold in clubs?");
                    hand += "♣:";
                    stdin().read_line(&mut hand).unwrap();
                    println!("Which cards do you hold in diamonds?");
                    hand += "♦:";
                    stdin().read_line(&mut hand).unwrap();
                    println!("Which cards do you hold in hearts?");
                    hand += "♥:";
                    stdin().read_line(&mut hand).unwrap();
                    println!("Which cards do you hold in spades?");
                    hand += "♠:";
                    stdin().read_line(&mut hand).unwrap();
                    Hand::from_str(&hand)
                }
                Some(hand) => Hand::from_str(&hand),
            };
            match hand_result {
                Ok(hand) => {
                    println!("{}", hand);
                    println!("hand_type: {}", hand.hand_type());
                    println!("high-card points: {}", ForumDPlus2015Evaluator::hcp(&hand));
                    println!(
                        "length points: {}",
                        ForumDPlus2015Evaluator::length_points(&hand, None, &[])
                    );
                    println!("adjustments:");
                    println!(
                        "  aces and tens: {}",
                        ForumDPlus2015Evaluator::adjustment_aces_and_tens(&hand)
                    );
                    println!(
                        "  unguarded honors: {}",
                        ForumDPlus2015Evaluator::adjustment_unguarded_honors(&hand)
                    );
                    println!("suit qualities:");
                    for suit in Suit::iter().rev() {
                        println!("{}: {}", suit, ForumDPlus2015Evaluator::suit_quality(&hand, suit));
                    }
                }
                Err(err) => {
                    println!("invalid hand: {}", err);
                    exit(1);
                }
            }
        }
    }
}
