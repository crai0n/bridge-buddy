use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{hand::Hand, card::{Suit, Card, Denomination}};
use crate::card::Denomination::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SuitQuality {
    Weak,           // less than acceptable
    Acceptable,     // at least 3 HCP
    Good,           // A or K with mid-values, or two of (A,K,D), or QJT
    VeryGood,       // Two of (A,K,D) with mid-values, for 7-card-suits and longer, two of (A,K,D) are sufficient
    AlmostStanding, // 4 honors of 5 (not AKQJ), for a 6-card-suit AKD is sufficient, for a 7-card-suit or longer KDB is sufficient
    Standing,       // AKQJ, for 7-card-suits and longer AKQ are sufficient
}

#[derive(Debug)]
pub struct ForumDPlus2015Evaluator {}

impl ForumDPlus2015Evaluator {
    fn hcp(hand: &Hand) -> f64 {
        //basic hcp-count
        hand.cards()
            .fold(0.0, |hcp, c| hcp + ForumDPlus2015Evaluator::card_value(c) as f64)
    }

    fn hcp_in(hand: &Hand, suit: Suit) -> f64 {
        hand.cards_in(suit)
            .fold(0.0, |hcp, c| hcp + ForumDPlus2015Evaluator::card_value(c) as f64)
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

    fn suit_quality(hand: &Hand, suit: Suit) -> SuitQuality {
        let cards = hand.cards_in(suit).map(|c| c.denomination).rev().collect_vec();

        //check for Standing Suit
        if (cards.len() >= 7 && &cards[..3] == &[Ace, King, Queen]) || cards.len() >= 4 && &cards[..4] == &[Ace, King, Queen, Jack] {
            return SuitQuality::Standing;
        }

        //check for AlmostStanding Suit
        if (Denomination::iter()
            .rev()
            .take(5)
            .filter(|d| cards.contains(&d))
            .count()
            >= 4) // four of top five honors
            || (cards.len() >= 6 && &cards[..3] == &[Ace, King, Queen])
            || (cards.len() >= 7 && &cards[..3] == &[King, Queen, Jack])
        {
            return SuitQuality::AlmostStanding;
        }

        //check for VeryGood Suit
        if Denomination::iter()
            .rev()
            .take(3)
            .filter(|d| cards.contains(&d))
            .count()
            >= 2 // two of the top three honors
            && (cards.contains(&Jack) || (cards.contains(&Ten) && cards.contains(&Nine)) || cards.len() >= 7)
            || cards.len() >= 3 && &cards[..3] == &[Ace, King, Queen] // Three top honors
        // mid-values or length
        {
            return SuitQuality::VeryGood;
        }

        // check for Good Suit
        if ((cards.contains(&Ace) || cards.contains(&King))
            && (cards.contains(&Jack) || (cards.contains(&Ten) && cards.contains(&Nine))))
            || Denomination::iter()
                .rev()
                .take(3)
                .filter(|d| cards.contains(&d))
                .count()
                >= 2
            || cards.len() >= 3 && &cards[..3] == &[Queen, Jack, Ten]
        {
            return SuitQuality::Good;
        }

        // check for Acceptable Suit
        if Self::hcp_in(hand, suit) >= 3.0 {
            return SuitQuality::Acceptable;
        }

        SuitQuality::Weak
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering::*;

    use crate::card::{Denomination, Suit};
    use crate::evaluator::*;
    use crate::hand::Hand;
    use test_case::test_case;

    #[test_case("S:T93,H:AKQ5,D:QJ,C:T542", 12.0 ; "Board 1.N")]
    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", 15.0 ; "Board 1.E")]
    #[test_case("S:8,H:JT762,D:K64,C:J963", 5.0 ; "Board 1.S")]
    #[test_case("S:AKJ52,H:943,D:982,C:87", 8.0 ; "Board 1.W")]
    #[test_case("S:963,H:T97,D:KT42,C:AJT", 8.0 ; "Board 2.N")]
    #[test_case("S:AQT74,H:Q43,D:A85,C:86", 12.0 ; "Board 2.E")]
    #[test_case("S:8,H:A86,D:QJ963,C:Q952", 9.0 ; "Board 2.S")]
    #[test_case("S:KJ52,H:KJ52,D:7,C:K743", 11.0 ; "Board 2.W")]
    #[test_case("S:K653,H:KJ7,D:AKQ5,C:53", 16.0 ; "Board 3.N")]
    #[test_case("S:AQ7,H:9532,D:74,C:KJ74", 10.0 ; "Board 3.E")]
    #[test_case("S:JT2,H:A4,D:T92,C:AQT62", 11.0 ; "Board 3.S")]
    #[test_case("S:984,H:QT86,D:J863,C:98", 3.0 ; "Board 3.W")]
    fn test_hcp(hand_str: &str, hcp: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::hcp(&hand), hcp);
    }

    #[test_case("S:T93,H:AKQ5,D:QJ,C:T542", Suit::Hearts, 9.0 ; "Board 1.N")]
    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", Suit::Hearts, 0.0 ; "Board 1.E")]
    #[test_case("S:8,H:JT762,D:K64,C:J963", Suit::Clubs, 1.0 ; "Board 1.S")]
    #[test_case("S:AKJ52,H:943,D:982,C:87", Suit::Diamonds, 0.0 ; "Board 1.W")]
    #[test_case("S:963,H:T97,D:KT42,C:AJT", Suit::Clubs, 5.0 ; "Board 2.N")]
    #[test_case("S:AQT74,H:Q43,D:A85,C:86", Suit::Spades, 6.0 ; "Board 2.E")]
    #[test_case("S:8,H:A86,D:QJ963,C:Q952", Suit::Diamonds, 3.0 ; "Board 2.S")]
    #[test_case("S:KJ52,H:KJ52,D:7,C:K743", Suit::Clubs, 3.0 ; "Board 2.W")]
    #[test_case("S:K653,H:KJ7,D:AKQ5,C:53", Suit::Spades, 3.0 ; "Board 3.N")]
    #[test_case("S:AQ7,H:9532,D:74,C:KJ74", Suit::Spades, 6.0 ; "Board 3.E")]
    #[test_case("S:JT2,H:A4,D:T92,C:AQT62", Suit::Hearts, 4.0; "Board 3.S")]
    #[test_case("S:984,H:QT86,D:J863,C:98", Suit::Diamonds, 1.0 ; "Board 3.W")]
    fn test_hcp_in(hand_str: &str, suit: Suit, hcp: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::hcp_in(&hand, suit), hcp);
    }

    #[test_case("S:T93,H:AKQ5,D:QJ,C:T542", Suit::Hearts, SuitQuality::VeryGood ; "Board 1.N")]
    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", Suit::Diamonds, SuitQuality::Acceptable ; "Board 1.E")]
    #[test_case("S:8,H:JT762,D:K64,C:J963", Suit::Hearts, SuitQuality::Weak ; "Board 1.S")]
    #[test_case("S:AKQJ96,H:T,D:A,C:Q9763", Suit::Spades, SuitQuality::Standing ; "Board 4.N")]
    #[test_case("S:43,H:543,D:T63,C:AKJT5", Suit::Clubs, SuitQuality::AlmostStanding ; "Board 4.S")]
    #[test_case("S:AQT74,H:Q43,D:A85,C:86", Suit::Spades, SuitQuality::Good ; "Board 2.E")]
    #[test_case("S:8,H:A86,D:QJ963,C:Q952", Suit::Diamonds, SuitQuality::Acceptable ; "Board 2.S")]
    #[test_case("S:KJ52,H:KJ52,D:7,C:K743", Suit::Spades, SuitQuality::Good ; "Board 2.W")]
    #[test_case("S:K653,H:KJ7,D:AKQ5,C:53", Suit::Diamonds, SuitQuality::VeryGood ; "Board 3.N")]
    #[test_case("S:AQ7,H:9532,D:74,C:KJ74", Suit::Diamonds, SuitQuality::Weak; "Board 3.E")]
    #[test_case("S:JT2,H:A4,D:T92,C:AQT62", Suit::Hearts, SuitQuality::Acceptable; "Board 3.S")]
    #[test_case("S:984,H:QT86,D:J863,C:98", Suit::Clubs, SuitQuality::Weak ; "Board 3.W")]
    fn test_suit_quality(hand_str: &str, suit: Suit, quality: SuitQuality) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::suit_quality(&hand, suit), quality);
    }
}