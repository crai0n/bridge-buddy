use itertools::Itertools;
use strum::IntoEnumIterator;
use std::cmp::Ordering;
use crate::card::Denomination::*;
use crate::{
    card::{Card, Denomination, Suit},
    hand::Hand,
};

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
    pub fn hcp(hand: &Hand) -> f64 {
        //basic hcp-count
        hand.cards()
            .fold(0.0, |hcp, c| hcp + ForumDPlus2015Evaluator::card_value(c) as f64)
    }

    pub fn hcp_in(hand: &Hand, suit: Suit) -> f64 {
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

    pub fn adjustment_aces_and_tens(hand: &Hand) -> f64 {
        let tens = hand.cards().filter(|&&x| x.denomination == Ten).count();
        let aces = hand.cards().filter(|&&x| x.denomination == Ace).count();
        match (tens, aces) {
            (0, 0) => -1.0,
            (0, 1) | (1, 0) => -0.5,
            (3, _) => 1.0,
            (i, j) if i + j >= 4 => 1.0,
            _ => 0.0,
        }
    }

    pub fn adjustment_unguarded_honors(hand: &Hand) -> f64 {
        let mut acc = 0.0;
        for suit in Suit::iter() {
            let cards_vec = hand.cards_in(suit).rev().map(|x| x.denomination).collect_vec();
            acc += match cards_vec.len() {
                1 if (cards_vec[0] >= Jack) => -1.0,
                2 if (cards_vec[1] >= Jack) => -1.0,
                _ => 0.0,
            }
        }
        acc
    }

    pub fn suit_quality(hand: &Hand, suit: Suit) -> SuitQuality {
        let cards = hand.cards_in(suit).map(|c| c.denomination).rev().collect_vec();

        //check for Standing Suit
        if (cards.len() >= 7 && cards[..3] == [Ace, King, Queen])
            || cards.len() >= 4 && cards[..4] == [Ace, King, Queen, Jack]
        {
            return SuitQuality::Standing;
        }

        //check for AlmostStanding Suit
        if (Denomination::iter()
            .rev()
            .take(5)
            .filter(|d| cards.contains(d))
            .count()
            >= 4) // four of top five honors
            || (cards.len() >= 6 && cards[..3] == [Ace, King, Queen])
            || (cards.len() >= 7 && cards[..3] == [King, Queen, Jack])
        {
            return SuitQuality::AlmostStanding;
        }

        //check for VeryGood Suit
        if Denomination::iter()
            .rev()
            .take(3)
            .filter(|d| cards.contains(d))
            .count()
            >= 2 // two of the top three honors
            && (cards.contains(&Jack) || (cards.contains(&Ten) && cards.contains(&Nine)) || cards.len() >= 7)
            || cards.len() >= 3 && cards[..3] == [Ace, King, Queen]
        // Three top honors
        // mid-values or length
        {
            return SuitQuality::VeryGood;
        }

        // check for Good Suit
        if ((cards.contains(&Ace) || cards.contains(&King))
            && (cards.contains(&Jack) || (cards.contains(&Ten) && cards.contains(&Nine))))
            || Denomination::iter().rev().take(3).filter(|d| cards.contains(d)).count() >= 2
            || cards.len() >= 3 && cards[..3] == [Queen, Jack, Ten]
        {
            return SuitQuality::Good;
        }

        // check for Acceptable Suit
        if Self::hcp_in(hand, suit) >= 3.0 {
            return SuitQuality::Acceptable;
        }

        SuitQuality::Weak
    }

    pub fn length_points(hand: &Hand, trump_suit: Option<Suit>, long_suits_shown_by_opponents: &[Suit]) -> f64 {
        let mut acc = 0.0;
        //in each suit that contains at least 3 HCP, is not the trump suit, and for which no opponent has shown 5+ cards, count 1 point for each card past the fourth.
        for suit in Suit::iter() {
            if trump_suit == Some(suit) || long_suits_shown_by_opponents.contains(&suit) {
                continue;
            }
            if Self::hcp_in(hand, suit) >= 3.0 {
                acc += match hand.length_in(suit) {
                    0..=4 => 0.0,
                    _ => hand.length_in(suit) as f64 - 4.0,
                }
            }
        }
        acc
    }

    pub fn side_suit_distribution_points(hand: &Hand, trump_suit: Suit) -> f64 {
        let mut acc = 0.0;
        for suit in Suit::iter() {
            if trump_suit == suit {
                continue;
            }
            acc += match hand.length_in(suit) {
                0 => 3.0,
                1 => 2.0,
                2 => 1.0,
                _ => 0.0,
            }
        }
        acc
    }

    pub fn trump_distribution_points(hand: &Hand, trump_suit: Suit, partner_promised: u8, promised_to_partner: u8) -> f64 {
        // count 2 points for the ninth trump card, and 1 more point for each additional card
        // however: each card can only be counted once, so we need to keep track of who counted which cards first.
        let total_length = hand.length_in(trump_suit) + partner_promised;
        let partners_count = promised_to_partner + partner_promised;
        assert_ne!(total_length.cmp(&partners_count), Ordering::Less);
        match (partners_count.cmp(&9), total_length.cmp(&9)) {
            // only start counting points from the 9th card
            (Ordering::Less, Ordering::Equal) => 2.0, // if partner did not count the 9th card, count an additional point
            (Ordering::Less, Ordering::Greater) => total_length as f64 - 7.0, // if partner did not count the 9th card, count an additional point
            (Ordering::Equal, Ordering::Greater) => total_length as f64 - partners_count as f64, // partner counted the 9th card, only count a single point for each additional card
            (Ordering::Greater, Ordering::Greater) => total_length as f64 - partners_count as f64, // partner counted the 9th card, only count a single point for each additional card
            _ => 0.0, // no additional points
        }
    }

    //
    // adjustment on bids by other players
    //

    // pub fn adjustment_partners_suits(hand: &Hand, suits: &[Suit]) -> f64 {
    //     // honors and honor combinations in partner's suits gain 0.5 HCP 
    //     todo!()
    // }

    // pub fn adjustment_right_opponents_suits(hand: &Hand, suits: &[Suit]) -> f64 {
    //     // we gain 1 HCP if we have one or more of the top three honors in a suit named by our right-hand opponent
    //     todo!()
    // }

    // pub fn adjustment_left_opponents_suits(hand: &Hand, suits: &[Suit]) -> f64 {
    //     // we lose 1 HCP if we have honors in a suit named by our left-hand opponent.
    //     todo!()
    // }

    // pub fn adjustment_misfit() -> f64 {
    //     // disregard all length points if we are in misfit with partner
    //     todo!()
    // }

    //
    // adjustments for suit contracts
    //

    // pub fn adjustment_double_fit(hand: &Hand) {
    //     // for a suit-contract +1 V
    //     todo!()
    // }

    // pub fn adjustment_shortness_in_opponents_suit(hand: &Hand, trump: Suit, suits: &[Suit]) -> f64 {
    //     // for a suit-contract, this increases ruffing opportunity, +1 V
    //     todo!()
    // }

    // pub fn adjustment_partners_short_suit(hand: &Hand, trump: Suit, short_suits: &[Suit]) -> f64 {
    //     // for a suit-contract, this decreases the value of K,D or B by at least -1 HCP        
    //     todo!()
    // }

    // pub fn adjustment_unguarded_queen_and_jack_in_dummy(hand: &Hand, trump: Suit, unbid_suits: &[Suit]) -> f64 {
    //     // for a suit-contract, if we are going to be dummy, low honors in unbid suits are mostly worthless
    //     todo!()
    // }

    //
    // Playing Trick Count (PTC)
    //
    // There are different opinions on how exactly Playing Tricks are counted. The difference mostly stems from disagreements about the value of Jack and Ten, especially for suits with 4-6 cards.
    // 



    //
    // Losing Trick Count (LTC)
    //
    // There are different opinions on how exactly Losing Tricks are counted. The difference mostly stems from disagreements about the value of Jack and Ten, especially for suits with 4-6 cards.
    // For now, we implement a basic approach, never counting more than 3 losers per suit. From these, one loser is deducted for each of the Ace, King and Queen.
    // This means that Qxx is valued the same as Axx though, which should be refined todo!
    // A possible approach is to start considering the Queen as only worth half a trick/loser and valuing the Jack and Ten more instead.
    pub fn losing_trick_count(hand: &Hand) -> f64 {
        let mut acc = 0.0;
        for suit in Suit::iter() {
            let card_vec = hand.cards_in(suit).rev().map(|c| c.denomination).collect_vec();
            acc += match card_vec.len() {
                1 | 11..=12 => match &card_vec[..1] {
                    // Singletons can only have one loser. If we have 11+ cards, only the Ace matters
                    [Ace] => 0.0,
                    _ => 1.0,
                },
                2 | 9..=10 => match &card_vec[..2] {
                    // Doubletons can one have two losers. If we have 9+ cards, only Ace and King matter
                    [Ace, King] => 0.0,
                    [Ace, _] => 1.0,
                    [King, _] => 1.0,
                    _ => 2.0,
                },
                3..=8 => {
                    // Three-card suits can only have three losers, don't add additional losers in longer suits
                    // subtract one for each of the top three honors
                    3.0 - Denomination::iter()
                        .rev()
                        .take(3)
                        .filter(|d| card_vec.contains(&d))
                        .count() as f64
                }
                _ => 0.0, // 13 card suits have no losers, Chicanes have no losers
            }
        }
        acc
    }


}

#[cfg(test)]
mod test {
    use crate::card::Suit;
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

    #[test_case("S:T93,H:AKQ5,D:QJ,C:T542", 0.0 ; "Board 1.N")]
    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", 0.0 ; "Board 1.E")]
    #[test_case("S:8,H:JT762,D:K64,C:J963", -0.5 ; "Board 1.S")]
    #[test_case("S:AKJ52,H:943,D:982,C:87", -0.5 ; "Board 1.W")]
    #[test_case("S:963,H:T97,D:KT42,C:AJT", 1.0 ; "Board 2.N")]
    #[test_case("S:AQT74,H:Q43,D:A85,C:86", 0.0 ; "Board 2.E")]
    #[test_case("S:8,H:A86,D:QJ963,C:Q952", -0.5 ; "Board 2.S")]
    #[test_case("S:KJ52,H:KJ52,D:7,C:K743", -1.0 ; "Board 2.W")]
    #[test_case("S:K653,H:KJ7,D:AKQ5,C:53", -0.5 ; "Board 3.N")]
    #[test_case("S:AQ7,H:9532,D:74,C:KJ74", -0.5 ; "Board 3.E")]
    #[test_case("S:JT2,H:A4,D:T92,C:AQT62", 1.0; "Board 3.S")]
    #[test_case("S:984,H:QT86,D:J863,C:98", -0.5 ; "Board 3.W")]
    fn test_adjustment_aces_and_tens(hand_str: &str, adjustment: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::adjustment_aces_and_tens(&hand), adjustment);
    }

    #[test_case("S:T93,H:AKQ5,D:QJ,C:T542", -1.0 ; "Board 1.N")]
    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", 0.0 ; "Board 1.E")]
    #[test_case("S:A,H:JT762,D:K64,C:J963", -1.0 ; "Board 1.S")]
    #[test_case("S:AKJ52,H:943,D:982,C:QJ", -1.0 ; "Board 1.W")]
    #[test_case("S:963,H:T97,D:KT42,C:AJT", 0.0 ; "Board 2.N")]
    #[test_case("S:AQT74,H:Q43,D:A85,C:AT", 0.0 ; "Board 2.E")]
    #[test_case("S:J,H:A86,D:QJ963,C:Q952", -1.0 ; "Board 2.S")]
    #[test_case("S:KJ52,H:KJ52,D:K,C:K743", -1.0 ; "Board 2.W")]
    #[test_case("S:K653,H:KJ7,D:AKQ5,C:J3", 0.0 ; "Board 3.N")]
    #[test_case("S:AQ7,H:9532,D:K4,C:KJ74", 0.0 ; "Board 3.E")]
    #[test_case("S:JT2,H:AQ,D:T92,C:AQT62", -1.0; "Board 3.S")]
    #[test_case("S:984,H:QT86,D:J863,C:AK", -1.0 ; "Board 3.W")]
    fn test_adjustment_unguarded_honors(hand_str: &str, adjustment: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::adjustment_unguarded_honors(&hand), adjustment);
    }

    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", Some(Suit::Spades), &[Suit::Hearts, Suit::Clubs], 1.0 ; "Board 1.E")]
    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", Some(Suit::Spades), &[Suit::Diamonds], 0.0 ; "Board 1.Ea")]
    #[test_case("S:984,H:QT86,D:J863,C:AK", None, &[Suit::Hearts], 0.0 ; "Board 3.W")]
    #[test_case("S:AKJ9532,H:9,D:982,C:87", Some(Suit::Diamonds), &[], 3.0 ; "Board 1.W")]
    fn test_length_points(hand_str: &str, trump_suit: Option<Suit>, suits: &[Suit], lp: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(
            ForumDPlus2015Evaluator::length_points(&hand, trump_suit, &suits[..]),
            lp
        );
    }

    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", Suit::Spades, 2.0 ; "2 V")]
    #[test_case("S:Q764,H:,D:AT8753,C:AKQ", Suit::Spades, 3.0 ; "3 V")]
    #[test_case("S:984,H:QT86,D:J863,C:AK", Suit::Hearts, 1.0 ; "1 V")]
    #[test_case("S:AK52,H:943,D:982,C:872", Suit::Spades, 0.0 ; "0 V")]
    fn test_side_suit_dp(hand_str: &str, trump_suit: Suit, dp: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::side_suit_distribution_points(&hand, trump_suit), dp);
    }

    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", Suit::Spades, 4, 4, 0.0 ; "Eight-card fit")]
    #[test_case("S:Q764,H:,D:AT8753,C:AKQ", Suit::Spades, 5, 4, 0.0 ; "Partner counts the ninth card")]
    #[test_case("S:984,H:QT86,D:J863,C:AK", Suit::Hearts, 5, 3, 2.0 ; "We count the ninth card")]
    #[test_case("S:AK582,H:93,D:982,C:872", Suit::Spades, 5, 4, 1.0 ; "Partner counts the ninth, we count the tenth")]
    fn test_trump_suit_dp(hand_str: &str, trump_suit: Suit, partner_promised: u8, promised_to_partner: u8, dp: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::trump_distribution_points(&hand, trump_suit, partner_promised, promised_to_partner), dp);
    }

    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", 5.0)]
    #[test_case("S:AK582,H:93,D:982,C:872", 9.0)]
    #[test_case("S:984,H:QT86,D:J863,C:AK", 8.0)]
    #[test_case("S:AKJT984,H:QT8,D:J3,C:K", 6.0)]
    fn test_losing_trick_count(hand_str: &str, ltc: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::losing_trick_count(&hand), ltc)
    }

}
