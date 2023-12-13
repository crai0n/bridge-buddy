use bridge_buddy_core::evaluator::ForumDPlus2015Evaluator;
use bridge_buddy_core::player::auto_player::AutoPlayer;
use bridge_buddy_core::player::cli_player::CliPlayer;
use bridge_buddy_core::primitives::card::Suit;
use bridge_buddy_core::primitives::deal::Hand;
use bridge_buddy_core::primitives::deal::Seat::{East, North, South, West};
use bridge_buddy_core::table::Table;
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

            let mut north_player = AutoPlayer::new(North);
            let mut south_player = CliPlayer::new(South);
            let mut east_player = AutoPlayer::new(East);
            let mut west_player = AutoPlayer::new(West);

            table.seat_player(&mut north_player, North).unwrap();
            table.seat_player(&mut south_player, South).unwrap();
            table.seat_player(&mut east_player, East).unwrap();
            table.seat_player(&mut west_player, West).unwrap();

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
