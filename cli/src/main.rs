use bridge_buddy_core::bid_reader::situation_mapper::SituationMapper;
use bridge_buddy_core::evaluator::ForumDPlus2015Evaluator;
use bridge_buddy_core::game_context::GameContext;
use bridge_buddy_core::primitives::bid_line::BidLine;
use bridge_buddy_core::primitives::card::Suit;
use bridge_buddy_core::primitives::deal::Hand;
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
    AnalyzeBids {
        bid_line: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
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
                    let game = GameContext::basic_context_from_hand(&hand);
                    println!("high-card points: {}", ForumDPlus2015Evaluator::hcp(&game));
                    println!("length points: {}", ForumDPlus2015Evaluator::length_points(&game));
                    println!("adjustments:");
                    println!(
                        "  aces and tens: {}",
                        ForumDPlus2015Evaluator::adjustment_aces_and_tens(&game)
                    );
                    println!(
                        "  unguarded honors: {}",
                        ForumDPlus2015Evaluator::adjustment_unguarded_honors(&game)
                    );
                    println!("suit qualities:");
                    for suit in Suit::iter().rev() {
                        println!("{}: {}", suit, ForumDPlus2015Evaluator::suit_quality(suit, &game));
                    }
                }
                Err(err) => {
                    println!("invalid hand: {}", err);
                    exit(1);
                }
            }
        }
        Command::AnalyzeBids { bid_line } => {
            let bid_line_result = match bid_line {
                None => {
                    let mut bid_string = String::new();
                    println!("What was the bidding until now?");
                    stdin().read_line(&mut bid_string).unwrap();
                    BidLine::from_str(&bid_string)
                }
                Some(bid_string) => BidLine::from_str(&bid_string),
            };
            match bid_line_result {
                Ok(bl) => {
                    let situation_finder = SituationMapper::from_file("config/situation_rules.bb").unwrap();
                    let situation = situation_finder.situation_from_bid_line(bl);
                    println!("Your situation is {}", situation)
                }
                Err(err) => {
                    println!("{}", err);
                    exit(1);
                }
            }
        }
    }
}
