use crate::primitives::card::Denomination::*;
use crate::primitives::{card::Denomination, Card, Hand, Suit};
use itertools::Itertools;
use std::cmp::Ordering;
use strum::{Display, IntoEnumIterator};

#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum SuitQuality {
    #[strum(serialize = "weak")]
    Weak,
    #[strum(serialize = "acceptable")]
    Acceptable,
    #[strum(serialize = "good")]
    Good,
    #[strum(serialize = "very good")]
    VeryGood,
    #[strum(serialize = "almost standing")]
    AlmostStanding,
    #[strum(serialize = "standing")]
    Standing,
}

#[derive(Debug)]
pub struct ForumDPlus2015Evaluator {}

impl ForumDPlus2015Evaluator {
    pub fn hcp(hand: &Hand) -> f64 {
        //basic hcp-count
        hand.cards()
            .fold(0.0, |hcp, c| hcp + ForumDPlus2015Evaluator::card_value(c) as f64)
    }

    pub fn hcp_in(suit: Suit, hand: &Hand) -> f64 {
        hand.cards_in(suit)
            .fold(0.0, |hcp, c| hcp + ForumDPlus2015Evaluator::card_value(c) as f64)
    }

    const fn card_value(card: &Card) -> u8 {
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
                1 if (cards_vec[0] >= Jack) => -1.0, // downgrade single honors, even single Ace
                2 => match cards_vec[..2] {
                    [King, Queen] | [Queen, _] | [Jack, _] => -1.0, // downgrade KQ, Qx and Jx
                    _ => 0.0,
                },
                _ => 0.0,
            }
        }
        acc
    }

    pub fn suit_quality(hand: &Hand, suit: Suit) -> SuitQuality {
        let cards = hand.cards_in(suit).map(|c| c.denomination).rev().collect_vec();

        //check for Standing Suit
        // AKQJ, for 7-card-suits and longer AKQ are sufficient
        if (cards.len() >= 7 && cards[..3] == [Ace, King, Queen])
            || cards.len() >= 4 && cards[..4] == [Ace, King, Queen, Jack]
        {
            return SuitQuality::Standing;
        }

        //check for AlmostStanding Suit
        // 4 honors of 5 (not AKQJ), for a 6-card-suit AKD is sufficient, for a 7-card-suit or longer KDB is sufficient
        if Self::count_honors_out_of_top(5, &cards) >= 4 // four of top five honors
            || (cards.len() >= 6 && cards[..3] == [Ace, King, Queen])
            || (cards.len() >= 7 && cards[..3] == [King, Queen, Jack])
        {
            return SuitQuality::AlmostStanding;
        }

        //check for VeryGood Suit
        // Two of (A,K,D) with mid-values, for 7-card-suits and longer, two of (A,K,D) are sufficient
        if Self::count_honors_out_of_top(3, &cards) >= 2 // two of the top three honors
            && (cards.contains(&Jack) || (cards.contains(&Ten) && cards.contains(&Nine)) || cards.len() >= 7)
            || cards.len() >= 3 && cards[..3] == [Ace, King, Queen]
        // Three top honors
        // mid-values or length
        {
            return SuitQuality::VeryGood;
        }

        // check for Good Suit
        // A or K with mid-values, or two of (A,K,D), or QJT
        if ((cards.contains(&Ace) || cards.contains(&King))
            && (cards.contains(&Jack) || (cards.contains(&Ten) && cards.contains(&Nine))))
            || Self::count_honors_out_of_top(3, &cards) >= 2
            || cards.len() >= 3 && cards[..3] == [Queen, Jack, Ten]
        {
            return SuitQuality::Good;
        }

        // check for Acceptable Suit
        // at least 3 HCP
        if Self::hcp_in(suit, hand) >= 3.0 {
            return SuitQuality::Acceptable;
        }

        SuitQuality::Weak // less than acceptable
    }

    fn count_honors_out_of_top(n: usize, cards: &[Denomination]) -> usize {
        let l = std::cmp::min(5, n); // there are only 5 honors
        Denomination::iter().rev().take(l).filter(|d| cards.contains(d)).count()
    }

    pub fn length_points(hand: &Hand, trump_suit: Option<Suit>, long_suits_shown_by_opponents: &[Suit]) -> f64 {
        let mut acc = 0.0;
        //in each suit that contains at least 3 HCP, is not the trump suit, and for which no opponent has shown 5+ cards, count 1 point for each card past the fourth.
        for suit in Suit::iter() {
            if trump_suit == Some(suit) || long_suits_shown_by_opponents.contains(&suit) {
                continue;
            }
            if Self::hcp_in(suit, hand) >= 3.0 {
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

    pub fn trump_distribution_points(
        hand: &Hand,
        trump_suit: Suit,
        partner_promised: u8,
        promised_to_partner: u8,
    ) -> f64 {
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
    // For now, we implement a basic approach, where we only evaluate at most the first three cards of the suit.
    pub fn playing_trick_count(hand: &Hand) -> f64 {
        let mut acc = 0.0;
        for suit in Suit::iter() {
            let card_vec = hand.cards_in(suit).rev().map(|c| c.denomination).collect_vec();
            acc += match card_vec.len() {
                0 => 0.0,
                l @ 1 | l @ 11..=12 => {
                    // In a singleton we make a trick if we have the ace. for a 11+-card-suit, we make all tricks if we have the ace
                    match &card_vec[..1] {
                        [Ace] => l as f64,
                        _ => l as f64 - 1.0,
                    }
                }
                l @ 2 | l @ 9..=10 => l as f64 - 2.0 + Self::two_card_trick_table(&card_vec[..2].try_into().unwrap()),
                l @ 3 | l @ 7..=8 => l as f64 - 3.0 + Self::three_card_trick_table(&card_vec[..3].try_into().unwrap()), // fourth card is always a trick
                l @ 4..=6 => {
                    if card_vec[4..].contains(&Jack) || card_vec[4..].contains(&Ten) && card_vec[4..].contains(&Nine) {
                        // fourth card is a full trick
                        l as f64 - 3.0 + Self::three_card_trick_table(&card_vec[..3].try_into().unwrap())
                    } else {
                        // fourth card is half a trick
                        l as f64 - 3.5 + Self::three_card_trick_table(&card_vec[..3].try_into().unwrap())
                    }
                }
                _ => 13.0,
            }
        }
        acc
    }

    const fn two_card_trick_table(den: &[Denomination; 2]) -> f64 {
        match den {
            // table generated using test-method below
            [Ace, King] => 2.0,
            [Ace, Queen] => 1.5,
            [Ace, _] => 1.0,
            [King, Queen] => 1.0,
            [King, _] => 0.5,
            [_, _] => 0.0,
        }
    }

    const fn three_card_trick_table(den: &[Denomination; 3]) -> f64 {
        match den {
            // table generated using test-method below
            // 3 cards headed by the ace, 3 tricks max.
            // Count 1 for each of the 5 honours and subtract 0.5 for each "hole" in between, Jack and Ten together are only 1.5 points
            [Ace, King, Queen] => 3.0,
            [Ace, King, Jack] => 2.5,  // missing the queen
            [Ace, King, _] => 2.0,     //
            [Ace, Queen, Jack] => 2.5, // missing the king
            [Ace, Queen, Ten] => 2.0,  // missing the king and the jack
            [Ace, Queen, _] => 1.5,    // missing the king
            [Ace, Jack, Ten] => 1.5,   // missing the King and Queen
            [Ace, _, _] => 1.0,
            // 3 cards headed by the king, 2 tricks max, lose 0.5 when missing the jack, lose 1 when missing the queen
            [King, Queen, Jack] => 2.0,
            [King, Queen, Ten] => 1.5,
            [King, Queen, _] => 1.5,
            [King, Jack, Ten] => 1.5,
            [King, Jack, _] => 1.0,
            [King, Ten, _] => 0.5,
            [King, _, _] => 0.5,
            // 3 cards headed by the queen: 1 trick max and lose 0.5 points for missing ten, lose 1 point for missing jack
            [Queen, Jack, Ten] => 1.0,
            [Queen, Jack, _] => 0.5,
            [_, _, _] => 0.0,
        }
    }

    //
    // Losing Trick Count (LTC)
    //
    // There are different opinions on how exactly Losing Tricks are counted. The difference mostly stems from disagreements about the value of Jack and Ten, especially for suits with 4-6 cards.
    // For now, we implement a basic approach, never counting more than 3 losers per suit. From these, one loser is deducted for each of the Ace, King and Queen.
    // This means that Qxx is valued the same as Axx though, which should be refined todo!
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
                3 | 7..=8 => {
                    // Three-card suits can only have three losers, in a 7-card suit, don't add additional losers
                    // subtract one for each of the top three honors
                    3.0 - Self::count_honors_out_of_top(3, &card_vec) as f64
                }
                4..=6 => {
                    // in a 4- to 6-card-suit, add half a loser if we lack mid-values
                    3.0 - Self::count_honors_out_of_top(3, &card_vec) as f64 + Self::losers_for_midvalues(&card_vec)
                }
                _ => 0.0, // 13 card suits have no losers, Chicanes have no losers
            }
        }
        acc
    }

    fn losers_for_midvalues(den: &[Denomination]) -> f64 {
        // we already took care of Ace, King and Queen, disregard now, only look at midvalues
        if den.contains(&Jack) || den.contains(&Ten) && den.contains(&Nine) {
            // Jack is enough in any case
            0.0
        } else {
            0.5
        }
    }

    pub fn first_round_control_in(suit: Suit, hand: &Hand, trump: Option<Suit>) -> bool {
        let card_vec = hand.cards_in(suit).rev().map(|c| c.denomination).collect_vec();
        if card_vec.contains(&Ace) {
            return true;
        }
        if let Some(t) = trump {
            // in a suit-contract voids also act as 1st-round-controls
            return card_vec.is_empty() && hand.cards_in(t).count() > 0; // safety-check for trump-cards
        }
        false
    }

    pub fn second_round_control_in(suit: Suit, hand: &Hand, trump: Option<Suit>) -> bool {
        let card_vec = hand.cards_in(suit).rev().map(|c| c.denomination).collect_vec();
        if card_vec.len() >= 2 && card_vec.contains(&King) {
            return true; // Kx
        }
        if let Some(t) = trump {
            // in a suit-contract singletons also act as 2nd-round-controls
            match card_vec.len() {
                0 => hand.cards_in(t).count() > 1, // safety-check for 2 trump-cards
                1 => hand.cards_in(t).count() > 0, // safety-check for trump-cards
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn honor_in(suit: Suit, hand: &Hand) -> bool {
        let card_vec = hand.cards_in(suit).rev().map(|c| c.denomination).collect_vec();
        Denomination::iter()
            .rev()
            .take(5)
            .filter(|x| card_vec.contains(x))
            .count()
            >= 1
    }

    pub fn stops(suit: Suit, hand: &Hand, is_declarer: bool) -> bool {
        let card_vec = hand.cards_in(suit).rev().map(|c| c.denomination).collect_vec();
        match card_vec.len() {
            0 => false,
            1 => card_vec.contains(&Ace),
            2 => Self::two_card_stopper_table(&card_vec[..2].try_into().unwrap(), is_declarer),
            3 => Self::three_card_stopper_table(&card_vec[..3].try_into().unwrap(), is_declarer),
            4 => Self::four_card_stopper_table(&card_vec[..4].try_into().unwrap(), is_declarer),
            _ => true,
        }
    }

    const fn two_card_stopper_table(den: &[Denomination; 2], is_declarer: bool) -> bool {
        match den {
            [Ace, _] => true,
            [King, Queen] => true,
            [King, _] => is_declarer,
            [Queen, _] => is_declarer, // risky, but usually worth it
            [_, _] => false,
        }
    }

    const fn three_card_stopper_table(den: &[Denomination; 3], is_declarer: bool) -> bool {
        match den {
            [Ace, _, _] => true,
            [King, Queen, _] => true,
            [King, Jack, Ten] => true,
            [King, _, _] => is_declarer,
            [Queen, Jack, Ten] => true,
            [Queen, _, _] => is_declarer,
            [_, _, _] => false,
        }
    }

    const fn four_card_stopper_table(den: &[Denomination; 4], is_declarer: bool) -> bool {
        match den {
            [Ace, _, _, _] => true,
            [King, Queen, _, _] => true,
            [King, Jack, Ten, _] => true,
            [King, _, _, _] => is_declarer,
            [Queen, Jack, Ten, _] => true,
            [Queen, _, _, _] => is_declarer,
            [Jack, Ten, _, _] => true, // risky, but usually worth it
            [Jack, _, _, _] => is_declarer,
            [_, _, _, _] => false,
        }
    }

    pub fn rule_of_twenty(hand: &Hand) -> bool {
        Suit::iter()
            .map(|x| hand.length_in(x))
            .sorted()
            .rev()
            .take(2)
            .sum::<u8>() as f64
            + Self::hcp(hand)
            >= 20.0
    }

    pub fn rule_of_fifteen(hand: &Hand) -> bool {
        hand.length_in(Suit::Spades) as f64 + Self::hcp(hand) >= 15.0
    }
}

#[cfg(test)]
mod test {
    use crate::engine::hand_evaluation::*;
    use crate::primitives::Hand;
    use crate::primitives::Suit;
    use std::str::FromStr;
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
        assert_eq!(ForumDPlus2015Evaluator::hcp_in(suit, &hand), hcp);
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

    #[test_case("S:Q764,H:T,D:AT753,C:AKQ", 0.0 ; "No unguarded honors")]
    #[test_case("S:A,H:JT762,D:K64,C:J963", -1.0 ; "Downgrade A")]
    #[test_case("S:K,H:JT762,D:K64,C:J963", -1.0 ; "Downgrade K")]
    #[test_case("S:Q,H:JT762,D:K64,C:J963", -1.0 ; "Downgrade Q")]
    #[test_case("S:J,H:JT762,D:K64,C:J963", -1.0 ; "Downgrade J")]
    #[test_case("S:J,H:Q,D:K,C:AKQJT98765", -3.0 ; "Downgrade in 3 suits")]
    #[test_case("S:KQ,H:AKQ5,D:T93,C:T542", -1.0 ; "Downgrade KQ")]
    #[test_case("S:QJ,H:AKQ5,D:T93,C:T542", -1.0 ; "Downgrade QJ")]
    #[test_case("S:Q2,H:AKQ5,D:T93,C:T542", -1.0 ; "Downgrade Qx")]
    #[test_case("S:J2,H:AKQ5,D:T93,C:T542", -1.0 ; "Downgrade Jx")]
    #[test_case("S:AK,H:AQ,D:KJ,C:T987654", 0.0 ; "Do not downgrade AK, AQ, KJ")]
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
        assert_eq!(ForumDPlus2015Evaluator::length_points(&hand, trump_suit, suits), lp);
    }

    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", Suit::Spades, 2.0 ; "2 V")]
    #[test_case("S:Q764,H:,D:AT8753,C:AKQ", Suit::Spades, 3.0 ; "3 V")]
    #[test_case("S:984,H:QT86,D:J863,C:AK", Suit::Hearts, 1.0 ; "1 V")]
    #[test_case("S:AK52,H:943,D:982,C:872", Suit::Spades, 0.0 ; "0 V")]
    fn test_side_suit_dp(hand_str: &str, trump_suit: Suit, dp: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(
            ForumDPlus2015Evaluator::side_suit_distribution_points(&hand, trump_suit),
            dp
        );
    }

    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", Suit::Spades, 4, 4, 0.0 ; "Eight-card fit")]
    #[test_case("S:Q764,H:,D:AT8753,C:AKQ", Suit::Spades, 5, 4, 0.0 ; "Partner counts the ninth card")]
    #[test_case("S:984,H:QT86,D:J863,C:AK", Suit::Hearts, 5, 3, 2.0 ; "We count the ninth card")]
    #[test_case("S:AK582,H:93,D:982,C:872", Suit::Spades, 5, 4, 1.0 ; "Partner counts the ninth, we count the tenth")]
    fn test_trump_suit_dp(hand_str: &str, trump_suit: Suit, partner_promised: u8, promised_to_partner: u8, dp: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(
            ForumDPlus2015Evaluator::trump_distribution_points(
                &hand,
                trump_suit,
                partner_promised,
                promised_to_partner
            ),
            dp
        );
    }

    #[test_case("S:Q764,H:8,D:AT753,C:AKQ", 6.0; "Six losers")]
    #[test_case("S:AK582,H:93,D:982,C:872", 9.5; "Nine and a half losers")]
    #[test_case("S:984,H:QT86,D:J863,C:AK", 8.5; "Eight and a half losers")]
    #[test_case("S:AJT9874,H:QT8,D:J3,C:K", 7.0; "Seven losers")]
    fn test_losing_trick_count(hand_str: &str, ltc: f64) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::losing_trick_count(&hand), ltc)
    }
    #[test_case("S:AKQJ976,H:,D:A,C:Q9763", Suit::Hearts, Some(Suit::Spades), true)]
    #[test_case("S:AKQJ976,H:,D:A,C:Q9763", Suit::Diamonds, None, true)]
    #[test_case("S:AKQJ96,H:T,D:A,C:Q9763", Suit::Hearts, Some(Suit::Spades), false)]
    fn first_round_control_in(hand_str: &str, suit: Suit, trump_suit: Option<Suit>, exp: bool) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(
            ForumDPlus2015Evaluator::first_round_control_in(suit, &hand, trump_suit),
            exp
        )
    }
    #[test_case("S:AKQJ96,H:T,D:A,C:Q9763", Suit::Diamonds, None, false)]
    #[test_case("S:AKQJ6,H:KT,D:A,C:Q9763", Suit::Hearts, None, true)]
    #[test_case("S:AKQJ96,H:T,D:A,C:Q9763", Suit::Hearts, Some(Suit::Spades), true)]
    fn second_round_control_in(hand_str: &str, suit: Suit, trump_suit: Option<Suit>, exp: bool) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(
            ForumDPlus2015Evaluator::second_round_control_in(suit, &hand, trump_suit),
            exp
        )
    }

    #[test_case("S:AKQJ96,H:T,D:A,C:Q9763", Suit::Diamonds, true)]
    #[test_case("S:AKQJ6,H:KT,D:A,C:Q9763", Suit::Hearts, true)]
    #[test_case("S:AKQJ96,H:9,D:A,C:Q9763", Suit::Hearts, false)]
    fn honor_in(hand_str: &str, suit: Suit, exp: bool) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::honor_in(suit, &hand), exp)
    }

    #[test_case("S:AKQJ96,H:T,D:A,C:Q9763", Suit::Diamonds, true, true)]
    #[test_case("S:AKQJ6,H:KT,D:A,C:Q9763", Suit::Spades, true, true)]
    #[test_case("S:AK96,H:96,D:AQ,C:QJT63", Suit::Spades, false, true)]
    #[test_case("S:AK96,H:96,D:AQ,C:QJT63", Suit::Hearts, true, false)]
    #[test_case("S:AK96,H:6,D:Q8,C:AQJ763", Suit::Diamonds, true, true)]
    #[test_case("S:AK96,H:6,D:Q7,C:AQJ763", Suit::Diamonds, false, false)]
    fn stops(hand_str: &str, suit: Suit, is_declarer: bool, exp: bool) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::stops(suit, &hand, is_declarer), exp)
    }

    #[test_case("S:AKT96,H:QT96,D:Q9,C:63", true)]
    #[test_case("S:AQT96,H:QT96,D:Q9,C:63", false)]
    #[test_case("S:AKT96,H:QT9,D:Q97,C:63", false)]
    fn rule_of_twenty(hand_str: &str, exp: bool) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::rule_of_twenty(&hand), exp)
    }

    #[test_case("S:AKT96,H:QT96,D:J9,C:63", true)]
    #[test_case("S:AKT9,H:QT96,D:JT9,C:63", false)]
    #[test_case("S:AKT96,H:JT96,D:J9,C:63", false)]
    fn rule_of_fifteen(hand_str: &str, exp: bool) {
        let hand = Hand::from_str(hand_str).unwrap();
        assert_eq!(ForumDPlus2015Evaluator::rule_of_fifteen(&hand), exp)
    }
}
