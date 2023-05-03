use crate::card::{Card, Denomination, Suit};
use crate::card::Denomination::*;
use crate::hand::{Hand, self};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use itertools::Itertools;
use strum::IntoEnumIterator;
#[derive(Debug)]
pub struct ForumDPlus2015Evaluator {}

impl ForumDPlus2015Evaluator {

    fn hcp(hand: &Hand) -> f64 {
        //basic hcp-count
        hand.cards().fold(0.0, |hcp, c| hcp + ForumDPlus2015Evaluator::card_value(c) as f64)
    }
    
    fn hcp_in(hand: &Hand, suit: Suit) -> f64 {
        hand.cards_in(suit).fold(0.0,|hcp, c| hcp + ForumDPlus2015Evaluator::card_value(c) as f64)
    }
    
    fn length_points(hand: &Hand, trump_suit: Option<Suit>, long_suits_shown_by_opponents:  &[Suit]) -> f64 {
        let mut acc = 0.0;
        //in each suit that contains at least 3 HCP, is not the trump suit, and has not been named by the opponents, count 1 point for each card past the fourth. 
        for suit in Suit::iter() {
            if trump_suit == Some(suit) { continue }
            if long_suits_shown_by_opponents.contains(&suit) {continue}
            if Self::hcp_in(hand, suit) >= 3.0 { 
                    acc += std::cmp::max(0, hand.length_in(suit) - 4) as f64;
            }
        }
        acc
    }

    fn card_value(card: &Card) -> u8 {
        match card.denomination {
            Ace => 4,
            King => 3,
            Queen => 2,
            Jack => 1,
            _ => 0,
        }
    }

    fn distribution_points(hand: &Hand, trump_suit: Option<Suit>, count_trump_length: bool, count_double_fit: bool) -> f64 {
        todo!()
        // Self::side_suit_distribution_points(hand, trump_suit) + Self::trump_distribution_points(hand, trump_suit, partner_promised, promised_to_partner); 
    }

    fn side_suit_distribution_points(hand: &Hand, trump_suit: Suit) -> f64 {
        let mut acc = 0.0;
        for suit in Suit::iter() {
            if trump_suit == suit { continue }
            acc += match hand.length_in(suit) {
                0 => 3.0,
                1 => 2.0,
                2 => 1.0,
                _ => 0.0,
            }
        }
        acc
    }

    fn trump_distribution_points(hand: &Hand, trump_suit: Suit, partner_promised: u8, promised_to_partner: u8) -> f64 {
        // implement trump point count
        // count 2 points for the ninth trump card, and 1 more point for each additional card
        // however: each card can only be counted once, so we need to keep track of who counted which cards first. 

        let total_length = hand.length_in(trump_suit) + partner_promised;
        let partners_count = promised_to_partner + partner_promised; 
        assert_ne!(total_length.cmp(&partners_count), Ordering::Less);
        match (partners_count.cmp(&9), total_length.cmp(&9)) { // only start counting points from the 9th card
            (Ordering::Less, Ordering::Equal) => 2.0, // if partner did not count the 9th card, count an additional point 
            (Ordering::Less, Ordering::Greater) => total_length as f64 - 7.0, // if partner did not count the 9th card, count an additional point 
            (Ordering::Equal, Ordering::Greater) => total_length as f64 - partners_count as f64, // partner counted the 9th card, only count a single point for each additional card
            (Ordering::Greater, Ordering::Greater) => total_length as f64 - partners_count as f64, // partner counted the 9th card, only count a single point for each additional card
            _ => 0.0
        } 
    }

    // fn double_fit_point(hand: &Hand) {} todo!()

    fn expected_tricks(hand: &Hand) -> f64 {
        let mut acc = 0.0;
        for suit in Suit::iter() {
            let mut card_vec = hand.cards_in(suit).rev().map(|c| c.denomination).collect_vec(); 
            acc += match card_vec.len() {
                0 => 0.0,
                1 => match &card_vec[..] {
                    // 4 HCP
                    [Ace] => 1.0,
                    // all other cases
                    _ => 0.0
                },
                2 => Self::two_card_trick_table(&card_vec.try_into().unwrap()),
                3 => Self::three_card_trick_table(&card_vec.try_into().unwrap()),
                4 => Self::four_card_trick_table(&card_vec.try_into().unwrap()), // value of fourth card depends on denominations
                5 => Self::five_card_trick_table(&card_vec.try_into().unwrap()), // value of fourth card depends on denominations
                6 => Self::six_card_trick_table(&card_vec.try_into().unwrap()), // value of fourth card depends on denominations
                _ => card_vec.len() as f64 - 3.0 + Self::three_card_trick_table(&card_vec[..3].try_into().unwrap()), // fourth card is always a trick
            }
        }
        acc
    }

    fn two_card_trick_table(den: &[Denomination;2]) -> f64 {
        match den { // in a 2 card suit, Ten and Jack are completely worthless
            // 7 HCP
            [Ace, King] => 2.0,
            // 6 HCP
            [Ace, Queen] => 1.5,
            // 5 HCP
            [King, Queen] => 1.0,
            // 4 HCP
            [Ace, _] => 1.0,
            // 3 HCP
            [King, _] => 0.5,
            // all other cases
            _ => 0.0
        }
    }

    fn three_card_trick_table(den: &[Denomination;3]) -> f64 {
        match den { 
            _ => 0.0 
        }
    }

    fn four_card_trick_table(den: &[Denomination;4]) -> f64 {
        match den {
            _ => 0.0 
        } // todo! use table generated from test below
    }

    fn five_card_trick_table(den: &[Denomination;5]) -> f64 {
        match den {
            _ => 0.0 
        } // todo! use table generated from test below
    }

    fn six_card_trick_table(den: &[Denomination;6]) -> f64 {
        match den {
            _ => 0.0 
        } // todo! use table generated from test below
    }

    fn expected_losers(hand: &Hand) -> f64 {
        let mut acc = 0.0;
        for suit in Suit::iter() {
            let mut card_vec = hand.cards_in(suit).rev().map(|c| c.denomination).collect_vec(); 
            acc += match card_vec.len() {
                0 => 0.0,
                1 => match &card_vec[..] {
                    [Ace] => 0.0,
                    // all other cases
                    _ => 1.0
                },
                2 => Self::two_card_loser_table(&card_vec.try_into().unwrap()),
                3 => Self::three_card_loser_table(&card_vec.try_into().unwrap()),
                4 => Self::four_card_loser_table(&card_vec.try_into().unwrap()), // fourth card might be half a loser
                5 => Self::five_card_loser_table(&card_vec.try_into().unwrap()), // fourth card might be half a loser
                6 => Self::six_card_loser_table(&card_vec.try_into().unwrap()), // fourth card might be half a loser
                _ => Self::three_card_loser_table(&card_vec[..3].try_into().unwrap()), // fourth card is not a loser
            }
        }
        acc
    }

    fn two_card_loser_table(den: &[Denomination;2]) -> f64 {
        match den {
            _ => 0.0 
        } // todo! use table generated from test below
    }

    fn three_card_loser_table(den: &[Denomination;3]) -> f64 {
        match den {
            _ => 0.0 
        } // todo! use table generated from test below
    }

    fn four_card_loser_table(den: &[Denomination;4]) -> f64 {
        match den {
            _ => 0.0 
        } // todo! use table generated from test below
    }

    fn five_card_loser_table(den: &[Denomination;5]) -> f64 {
        match den {
            _ => 0.0 
        } // todo! use table generated from test below
    }

    fn six_card_loser_table(den: &[Denomination;6]) -> f64 {
        match den {
            _ => 0.0 
        } // todo! use table generated from test below
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering::*;

    use crate::card::{Suit, Denomination};
    use crate::evaluator::*;
    use crate::hand::Hand;
    use test_case::test_case;

    #[test]
    fn generate_two_card_hands() {
        let cards = Denomination::iter().rev().collect::<Vec<_>>();
        let i_cut = Denomination::Queen;
        let j_cut = Denomination::Jack;
        for i in 0..13 {
            let i_str = match cards[i].cmp(&i_cut) {
                Less => continue,
                Equal => "_".to_string(),
                Greater => format!("{:?}", cards[i]),
            };
            for j in i+1..13 {
                let j_str = match cards[j].cmp(&j_cut) {
                    Less => continue,
                    Equal => "_".to_string(),
                    Greater => format!("{:?}", cards[j]),
                };
                println!("[{}, {}] => 0.0,", i_str, j_str);
            }
        }
    }

    #[test]
    fn generate_three_card_hands() {
        let cards = Denomination::iter().rev().collect::<Vec<_>>();
        let i_cut = Denomination::Ten;
        let j_cut = Denomination::Nine;
        let k_cut = Denomination::Eight;
        for i in 0..13 {
            let i_str = match cards[i].cmp(&i_cut) {
                Less => continue,
                Equal => "_".to_string(),
                Greater => format!("{:?}", cards[i]),
            };
            for j in i+1..13 {
                let j_str = match cards[j].cmp(&j_cut) {
                    Less => continue,
                    Equal => "_".to_string(),
                    Greater => format!("{:?}", cards[j]),
                };
                for k in j+1..13 {
                    let k_str = match cards[k].cmp(&k_cut) {
                        Less => continue,
                        Equal => "_".to_string(),
                        Greater => format!("{:?}", cards[k]),
                    };
                    println!("[{}, {}, {}] => 0.0,", i_str, j_str, k_str);
                }
            }
        }
    }

    #[test]
    fn generate_four_card_hands() {
        let cards = Denomination::iter().rev().collect::<Vec<_>>();
        let i_cut = Denomination::Nine;
        let j_cut = Denomination::Eight;
        let k_cut = Denomination::Seven;
        let l_cut = Denomination::Six;
        for i in 0..13 {
            let i_str = match cards[i].cmp(&i_cut) {
                Less => continue,
                Equal => "_".to_string(),
                Greater => format!("{:?}", cards[i]),
            };
            for j in i+1..13 {
                let j_str = match cards[j].cmp(&j_cut) {
                    Less => continue,
                    Equal => "_".to_string(),
                    Greater => format!("{:?}", cards[j]),
                };
                for k in j+1..13 {
                    let k_str = match cards[k].cmp(&k_cut) {
                        Less => continue,
                        Equal => "_".to_string(),
                        Greater => format!("{:?}", cards[k]),
                    };
                    for l in k+1..13 {
                        let l_str = match cards[l].cmp(&l_cut) {
                            Less => continue,
                            Equal => "_".to_string(),
                            Greater => format!("{:?}", cards[l]),
                        };
                        println!("[{}, {}, {}, {}] => 0.0,", i_str, j_str, k_str, l_str);
                    }
                }
            }
        }
    }

    #[test]
    fn generate_five_card_hands() {
        let cards = Denomination::iter().rev().collect::<Vec<_>>();
        let i_cut = Denomination::Eight;
        let j_cut = Denomination::Seven;
        let k_cut = Denomination::Six;
        let l_cut = Denomination::Five;
        let m_cut = Denomination::Four;
        for i in 0..13 {
            let i_str = match cards[i].cmp(&i_cut) {
                Less => continue,
                Equal => "_".to_string(),
                Greater => format!("{:?}", cards[i]),
            };
            for j in i+1..13 {
                let j_str = match cards[j].cmp(&j_cut) {
                    Less => continue,
                    Equal => "_".to_string(),
                    Greater => format!("{:?}", cards[j]),
                };
                for k in j+1..13 {
                    let k_str = match cards[k].cmp(&k_cut) {
                        Less => continue,
                        Equal => "_".to_string(),
                        Greater => format!("{:?}", cards[k]),
                    };
                    for l in k+1..13 {
                        let l_str = match cards[l].cmp(&l_cut) {
                            Less => continue,
                            Equal => "_".to_string(),
                            Greater => format!("{:?}", cards[l]),
                        };
                        for m in l+1..13 {
                            let m_str = match cards[m].cmp(&m_cut) {
                                Less => continue,
                                Equal => "_".to_string(),
                                Greater => format!("{:?}", cards[m]),
                            };
                            println!("[{}, {}, {}, {}, {}] => 0.0,", i_str, j_str, k_str, l_str, m_str);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn generate_six_card_hands() {
        let cards = Denomination::iter().rev().collect::<Vec<_>>();
        let i_cut = Denomination::Jack;
        let j_cut = Denomination::Ten;
        let k_cut = Denomination::Nine;
        let l_cut = Denomination::Eight;
        let m_cut = Denomination::Seven;
        let n_cut = Denomination::Six;
        for i in 0..13 {
            let i_str = match cards[i].cmp(&i_cut) {
                Less => continue,
                Equal => "_".to_string(),
                Greater => format!("{:?}", cards[i]),
            };
            for j in i+1..13 {
                let j_str = match cards[j].cmp(&j_cut) {
                    Less => continue,
                    Equal => "_".to_string(),
                    Greater => format!("{:?}", cards[j]),
                };
                for k in j+1..13 {
                    let k_str = match cards[k].cmp(&k_cut) {
                        Less => continue,
                        Equal => "_".to_string(),
                        Greater => format!("{:?}", cards[k]),
                    };
                    for l in k+1..13 {
                        let l_str = match cards[l].cmp(&l_cut) {
                            Less => continue,
                            Equal => "_".to_string(),
                            Greater => format!("{:?}", cards[l]),
                        };
                        for m in l+1..13 {
                            let m_str = match cards[m].cmp(&m_cut) {
                                Less => continue,
                                Equal => "_".to_string(),
                                Greater => format!("{:?}", cards[m]),
                            };
                            for n in m+1..13 {
                                let n_str = match cards[n].cmp(&n_cut) {
                                    Less => continue,
                                    Equal => "_".to_string(),
                                    Greater => format!("{:?}", cards[n]),
                                };
                                println!("[{}, {}, {}, {}, {}, {}] => 0.0,", i_str, j_str, k_str, l_str, m_str, n_str);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test_case("♠:AKQJT9876,♥:5432", 10.0 ; "10 HCP")]
    fn test_hcp(hand_str: &str, hcp: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::hcp(&hand), hcp);
    }

    #[test_case("♠:AKQJT9876,♥:5432", Suit::Spades, 10.0 ; "10 HCP")]
    fn test_hcp_in(hand_str: &str, suit: Suit, hcp: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::hcp(&hand), hcp);
    }

    #[test_case("♠:AKQJT9876,♥:5432", 5.0 ; "5 LP")]
    fn test_length_points(hand_str: &str, lp: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::length_points(&hand, None, &[]), lp);
    }

    #[test_case("♠:AKQJT9876,♥:5432", 6.0 ; "6 DP")]
    fn test_distribution_points(hand_str: &str, dp: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::distribution_points(&hand, None, false, false), dp);
    }
}
