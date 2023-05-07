use bridge_buddy_core::evaluator::ForumDPlus2015Evaluator;
use bridge_buddy_core::hand::{Hand, Suit};
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
