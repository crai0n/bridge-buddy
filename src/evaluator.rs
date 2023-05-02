use crate::card::{Card, Denomination, Suit};
use crate::hand::Hand;
use std::collections::BTreeMap;
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
            Denomination::Ace => 4,
            Denomination::King => 3,
            Denomination::Queen => 2,
            Denomination::Jack => 1,
            _ => 0,
        }
    }

    // fn expected_tricks(hand: &Hand) -> f32 {
    //     for suit in Suit::iter() {
    //         // count high card tricks
    //         hand.cards_in(suit).map(|x| x.denomination)



    //         // count length tricks
    //         if hand.cards_in(suit).count() > 7 {

    //         } else {

    //         }
    //     }
    // }

    // fn tricks_in_short_suit(denominations: &mut [Denomination]) -> f32 {
    //     denominations.sort().rev();
    //     match denominations.len() {
    //         0 => 0,
    //         1 => match denominations.nth(0) {
    //             Denomination::Ace => 1,
    //             _ => 0,
    //         },
    //         2 => match denominations.nth(0)
    //         }
    //     }
    //     cards.sort().rev() 
    // }
}

#[cfg(test)]
mod test {
    use crate::card::Suit;
    use crate::evaluator::*;
    use crate::hand::Hand;
    use test_case::test_case;

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
}
