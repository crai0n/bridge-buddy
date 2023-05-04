use crate::card::Denomination::*;
use crate::card::{Card, Denomination, Suit};
use crate::hand::{self, Hand};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
#[derive(Debug)]
pub struct ForumDPlus2015Evaluator {}

pub enum SuitQuality {
    Weak,           // less than acceptable
    Acceptable,     // at least 3 HCP
    Good,           // A or K with mid-values, or two of (A,K,D), or QJT
    VeryGood,       // Two of (A,K,D) with mid-values, for 7-card-suits and longer, two of (A,K,D) are sufficient
    AlmostStanding, // 4 honors of 5 (not AKQJ), for a 6-card-suit AKD is sufficient, for a 7-card-suit or longer KDB is sufficient
    Standing,       // AKQJ, for 7-card-suits and longer AKQ are sufficient
}

impl ForumDPlus2015Evaluator {
    fn suit_quality(hand: &Hand, suit: Suit) -> SuitQuality {
        let cards = hand.cards_in(suit).map(|c| c.denomination).rev().collect_vec();

        //check for Standing Suit
        if (cards.len() >= 7 && &cards[..3] == &[Ace, King, Queen]) || &cards[..4] == &[Ace, King, Queen, Jack] {
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
            || &cards[..3] == &[Queen, Jack, Ten]
        {
            return SuitQuality::Good;
        }

        // check for Acceptable Suit
        if Self::hcp_in(hand, suit) >= 3.0 {
            return SuitQuality::Acceptable;
        }

        SuitQuality::Weak
    }

    fn hcp(hand: &Hand) -> f64 {
        //basic hcp-count
        hand.cards()
            .fold(0.0, |hcp, c| hcp + ForumDPlus2015Evaluator::card_value(c) as f64)
    }

    fn hcp_in(hand: &Hand, suit: Suit) -> f64 {
        hand.cards_in(suit)
            .fold(0.0, |hcp, c| hcp + ForumDPlus2015Evaluator::card_value(c) as f64)
    }

    fn length_points(hand: &Hand, trump_suit: Option<Suit>, long_suits_shown_by_opponents: &[Suit]) -> f64 {
        let mut acc = 0.0;
        //in each suit that contains at least 3 HCP, is not the trump suit, and has not been named by the opponents, count 1 point for each card past the fourth.
        for suit in Suit::iter() {
            if trump_suit == Some(suit) {
                continue;
            }
            if long_suits_shown_by_opponents.contains(&suit) {
                continue;
            }
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

    fn distribution_points(
        hand: &Hand,
        trump_suit: Option<Suit>,
        count_trump_length: bool,
        count_double_fit: bool,
    ) -> f64 {
        todo!()
        // Self::side_suit_distribution_points(hand, trump_suit) + Self::trump_distribution_points(hand, trump_suit, partner_promised, promised_to_partner);
    }

    fn side_suit_distribution_points(hand: &Hand, trump_suit: Suit) -> f64 {
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

    fn trump_distribution_points(hand: &Hand, trump_suit: Suit, partner_promised: u8, promised_to_partner: u8) -> f64 {
        // implement trump point count
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

    fn adjustment_aces_and_tens(hand: &Hand) -> f64 {
        let tens = hand.cards().filter(|&&x| x.denomination == Ten).count();
        let aces = hand.cards().filter(|&&x| x.denomination == Ace).count();
        match ( tens, aces ) {
            (0, 0) => -1.0,
            (0, 1) | (1, 0) => -0.5,
            (3, _) => 1.0,
            (i, j) if i+j >= 4 => 1.0,
            _ => 0.0 
        }
    }

    fn adjustment_unguarded_honors(hand: &Hand) -> f64 {
        let mut acc = 0.0;
        for suit in Suit::iter() {
            let cards_vec = hand.cards_in(suit).map(|x| x.denomination).collect_vec();
            acc += match cards_vec.len() {
                1 if (cards_vec[0] >= Jack) => -1.0,
                2 if (cards_vec[1] >= Jack) => -1.0,
                _ => 0.0,
            }
        }
        acc
    }

    fn adjustment_partners_suit(hand: &Hand, suit: Suit) -> f64 {
        // honors and honor combinations in partner's suit gain 0.5 HCP 
        todo!()
    }

    fn adjustment_right_opponents_suit(hand: &Hand, suit: Suit) -> f64 {
        // we gain 1 HCP if we have one or more of the top three honors
        todo!()
    }

    fn adjustment_left_opponents_suit(hand: &Hand, suit: Suit) -> f64 {
        // decreases value as honors are probably badly positioned
        todo!()
    }

    fn adjustment_opponents_long_suit(hand: &Hand, suit: Suit) -> f64 {
        // no length points for a suit in which one opponent holds 5 cards
        todo!()
    }

    fn adjustment_shortness_in_opponents_suit(hand: &Hand, suit: Suit) -> f64 {
        // for a suit-contract, this increases ruffing opportunity, +1 V
        todo!()
    }

    fn adjustment_shortness_in_partners_suit(hand: &Hand, suit: Suit) -> f64 {
        // for a suit-contract, this decreases the value by at least -1 HCP        
        todo!()
    }

    fn double_fit_point(hand: &Hand) {
        // for a suit-contract +1 V
        todo!()
    }

    fn adjustment_unguarded_queen_and_jack_in_dummy() -> f64 {
        // if we are going to be dummy, low honors in unbid suits are mostly worthless
        todo!()
    }

    fn adjustment_misfit() -> f64 {
        // discount length points if we are in misfit with partner
        todo!()
    }


    fn playing_trick_count(hand: &Hand) -> f64 {
        let mut acc = 0.0;
        for suit in Suit::iter() {
            let mut card_vec = hand.cards_in(suit).rev().map(|c| c.denomination).collect_vec();
            acc += match card_vec.len() {
                0 => 0.0,
                l @ 1 | l @ 11..=12 => {
                    // In a singleton we make a trick if we have the ace. for a 11+-card-suit, we make all tricks if we have the ace
                    match &card_vec[..1] {
                        [Ace] => l as f64,
                        _ => l as f64 - 1.0,
                    }
                }
                2 => Self::two_card_trick_table(&card_vec.try_into().unwrap()),
                3 => Self::three_card_trick_table(&card_vec.try_into().unwrap()),
                4 => Self::four_card_trick_table(&card_vec.try_into().unwrap()), // value of fourth card depends on denominations
                5 => Self::five_card_trick_table(&card_vec.try_into().unwrap()), // value of fourth card depends on denominations
                6 => Self::six_card_trick_table(&card_vec.try_into().unwrap()), // value of fourth card depends on denominations
                7..=8 => card_vec.len() as f64 - 3.0 + Self::three_card_trick_table(&card_vec[..3].try_into().unwrap()), // fourth card is always a trick
                l @ 9..=10 => {
                    l as f64 - 2.0
                        + match &card_vec[..2] {
                            [Ace, King] => 2.0,
                            [Ace, Queen] => 1.5,
                            [Ace, Jack] => 1.5, // this is probably debatable
                            [Ace, _] => 1.0,
                            [King, Queen] => 1.0,
                            [King, Jack] => 1.0, // this is probably debatable
                            [King, _] => 0.5,
                            _ => 0.0,
                        }
                }
                _ => 13.0,
            }
        }
        acc
    }

    fn two_card_trick_table(den: &[Denomination; 2]) -> f64 {
        match den { // table generated using test-method below
            [Ace, King] => 2.0,
            [Ace, Queen] => 1.5,
            [Ace, Jack] => 1.5, // this is probably debatable
            [Ace, _] => 1.0,
            [King, Queen] => 1.0,
            [King, Jack] => 1.0, // this is probably debatable
            [King, _] => 0.5,
            [Queen, Jack] => 0.0,
            [Queen, _] => 0.0,
            [Jack, _] => 0.0,
            [_, _,] => 0.0,
        }
    }

    fn three_card_trick_table(den: &[Denomination; 3]) -> f64 {
        match den {// table generated using test-method below
            // 3 cards headed by the ace, 3 tricks max. 
            // Count 1 for each of the 5 honours and subtract 0.5 for each "hole" in between, Jack and Ten together are only 1.5 points
            [Ace, King, Queen] => 3.0,
            [Ace, King, Jack] => 2.5, // missing the queen
            [Ace, King, Ten] => 2.0, // missing the queen and the jack
            [Ace, King, _] => 2.0, // 
            [Ace, Queen, Jack] => 2.5, // missing the king
            [Ace, Queen, Ten] => 2.0, // missing the king and the jack 
            [Ace, Queen, _] => 1.5, // missing the king
            [Ace, Jack, Ten] => 1.5, // missing the King and Queen, this is probably debatable
            [Ace, Jack, _] => 1.0, // this is probably debatable
            [Ace, Ten, _] => 1.0,
            [Ace, _, _] => 1.0,
            // 3 cards headed by the king, 2 tricks max, lose 0.5 when missing the jack, lose 1 when missing the queen
            [King, Queen, Jack] => 2.0,
            [King, Queen, Ten] => 1.5,
            [King, Queen, _] => 1.5,
            [King, Jack, Ten] => 1.5, // this is probably debatable
            [King, Jack, _] => 1.0, // this is probably debatable
            [King, Ten, _] => 0.5,
            [King, _, _] => 0.5,
            // 3 cards headed by the queen: 1 trick max and lose 0.5 points for missing ten, lose 1 point for missing jack
            [Queen, Jack, Ten] => 1.0,
            [Queen, Jack, _] => 0.5,
            [Queen, Ten, _] => 0.0,
            [Queen, _, _] => 0.0,
            [Jack, Ten, _] => 0.0,
            [Jack, _, _] => 0.0,
            [Ten, _, _] => 0.0,
            [_, _, _] => 0.0,
        }
    }

    fn four_card_trick_table(den: &[Denomination; 4]) -> f64 {
        match den {// table generated using test-method below
            [Ace, King, Queen, Jack] => 0.0,
            [Ace, King, Queen, Ten] => 0.0,
            [Ace, King, Queen, Nine] => 0.0,
            [Ace, King, Queen, _] => 0.0,
            [Ace, King, Jack, Ten] => 0.0,
            [Ace, King, Jack, Nine] => 0.0,
            [Ace, King, Jack, _] => 0.0,
            [Ace, King, Ten, Nine] => 0.0,
            [Ace, King, Ten, _] => 0.0,
            [Ace, King, Nine, _] => 0.0,
            [Ace, King, _, _] => 0.0,
            [Ace, Queen, Jack, Ten] => 0.0,
            [Ace, Queen, Jack, Nine] => 0.0,
            [Ace, Queen, Jack, _] => 0.0,
            [Ace, Queen, Ten, Nine] => 0.0,
            [Ace, Queen, Ten, _] => 0.0,
            [Ace, Queen, Nine, _] => 0.0,
            [Ace, Queen, _, _] => 0.0,
            [Ace, Jack, Ten, Nine] => 0.0,
            [Ace, Jack, Ten, _] => 0.0,
            [Ace, Jack, Nine, _] => 0.0,
            [Ace, Jack, _, _] => 0.0,
            [Ace, Ten, Nine, _] => 0.0,
            [Ace, Ten, _, _] => 0.0,
            [Ace, Nine, _, _] => 0.0,
            [Ace, _, _, _] => 0.0,
            [King, Queen, Jack, Ten] => 0.0,
            [King, Queen, Jack, Nine] => 0.0,
            [King, Queen, Jack, _] => 0.0,
            [King, Queen, Ten, Nine] => 0.0,
            [King, Queen, Ten, _] => 0.0,
            [King, Queen, Nine, _] => 0.0,
            [King, Queen, _, _] => 0.0,
            [King, Jack, Ten, Nine] => 0.0,
            [King, Jack, Ten, _] => 0.0,
            [King, Jack, Nine, _] => 0.0,
            [King, Jack, _, _] => 0.0,
            [King, Ten, Nine, _] => 0.0,
            [King, Ten, _, _] => 0.0,
            [King, Nine, _, _] => 0.0,
            [King, _, _, _] => 0.0,
            [Queen, Jack, Ten, Nine] => 0.0,
            [Queen, Jack, Ten, _] => 0.0,
            [Queen, Jack, Nine, _] => 0.0,
            [Queen, Jack, _, _] => 0.0,
            [Queen, Ten, Nine, _] => 0.0,
            [Queen, Ten, _, _] => 0.0,
            [Queen, Nine, _, _] => 0.0,
            [Queen, _, _, _] => 0.0,
            [Jack, Ten, Nine, _] => 0.0,
            [Jack, Ten, _, _] => 0.0,
            [Jack, Nine, _, _] => 0.0,
            [Jack, _, _, _] => 0.0,
            [Ten, Nine, _, _] => 0.0,
            [Ten, _, _, _] => 0.0,
            [Nine, _, _, _] => 0.0,
            [_, _, _, _] => 0.0,
        }
    }

    fn five_card_trick_table(den: &[Denomination; 5]) -> f64 {
        match den {
            [Ace, King, Queen, Jack, Ten] => 0.0,
            [Ace, King, Queen, Jack, Nine] => 0.0,
            [Ace, King, Queen, Jack, _] => 0.0,
            [Ace, King, Queen, Ten, Nine] => 0.0,
            [Ace, King, Queen, Ten, _] => 0.0,
            [Ace, King, Queen, Nine, _] => 0.0,
            [Ace, King, Queen, _, _] => 0.0,
            [Ace, King, Jack, Ten, Nine] => 0.0,
            [Ace, King, Jack, Ten, _] => 0.0,
            [Ace, King, Jack, Nine, _] => 0.0,
            [Ace, King, Jack, _, _] => 0.0,
            [Ace, King, Ten, Nine, _] => 0.0,
            [Ace, King, Ten, _, _] => 0.0,
            [Ace, King, Nine, _, _] => 0.0,
            [Ace, King, _, _, _] => 0.0,
            [Ace, Queen, Jack, Ten, Nine] => 0.0,
            [Ace, Queen, Jack, Ten, _] => 0.0,
            [Ace, Queen, Jack, Nine, _] => 0.0,
            [Ace, Queen, Jack, _, _] => 0.0,
            [Ace, Queen, Ten, Nine, _] => 0.0,
            [Ace, Queen, Ten, _, _] => 0.0,
            [Ace, Queen, Nine, _, _] => 0.0,
            [Ace, Queen, _, _, _] => 0.0,
            [Ace, Jack, Ten, Nine, _] => 0.0,
            [Ace, Jack, Ten, _, _] => 0.0,
            [Ace, Jack, Nine, _, _] => 0.0,
            [Ace, Jack, _, _, _] => 0.0,
            [Ace, Ten, Nine, _, _] => 0.0,
            [Ace, Ten, _, _, _] => 0.0,
            [Ace, Nine, _, _, _] => 0.0,
            [Ace, _, _, _, _] => 0.0,
            [King, Queen, Jack, Ten, Nine] => 0.0,
            [King, Queen, Jack, Ten, _] => 0.0,
            [King, Queen, Jack, Nine, _] => 0.0,
            [King, Queen, Jack, _, _] => 0.0,
            [King, Queen, Ten, Nine, _] => 0.0,
            [King, Queen, Ten, _, _] => 0.0,
            [King, Queen, Nine, _, _] => 0.0,
            [King, Queen, _, _, _] => 0.0,
            [King, Jack, Ten, Nine, _] => 0.0,
            [King, Jack, Ten, _, _] => 0.0,
            [King, Jack, Nine, _, _] => 0.0,
            [King, Jack, _, _, _] => 0.0,
            [King, Ten, Nine, _, _] => 0.0,
            [King, Ten, _, _, _] => 0.0,
            [King, Nine, _, _, _] => 0.0,
            [King, _, _, _, _] => 0.0,
            [Queen, Jack, Ten, Nine, _] => 0.0,
            [Queen, Jack, Ten, _, _] => 0.0,
            [Queen, Jack, Nine, _, _] => 0.0,
            [Queen, Jack, _, _, _] => 0.0,
            [Queen, Ten, Nine, _, _] => 0.0,
            [Queen, Ten, _, _, _] => 0.0,
            [Queen, Nine, _, _, _] => 0.0,
            [Queen, _, _, _, _] => 0.0,
            [Jack, Ten, Nine, _, _] => 0.0,
            [Jack, Ten, _, _, _] => 0.0,
            [Jack, Nine, _, _, _] => 0.0,
            [Jack, _, _, _, _] => 0.0,
            [Ten, Nine, _, _, _] => 0.0,
            [Ten, _, _, _, _] => 0.0,
            [Nine, _, _, _, _] => 0.0,
            [_, _, _, _, _] => 0.0,
        }
    }

    fn six_card_trick_table(den: &[Denomination; 6]) -> f64 {
        match den {
            [Ace, King, Queen, Jack, Ten, Nine] => 0.0,
            [Ace, King, Queen, Jack, Ten, _] => 0.0,
            [Ace, King, Queen, Jack, Nine, _] => 0.0,
            [Ace, King, Queen, Jack, _, _] => 0.0,
            [Ace, King, Queen, Ten, Nine, _] => 0.0,
            [Ace, King, Queen, Ten, _, _] => 0.0,
            [Ace, King, Queen, Nine, _, _] => 0.0,
            [Ace, King, Queen, _, _, _] => 0.0,
            [Ace, King, Jack, Ten, Nine, _] => 0.0,
            [Ace, King, Jack, Ten, _, _] => 0.0,
            [Ace, King, Jack, Nine, _, _] => 0.0,
            [Ace, King, Jack, _, _, _] => 0.0,
            [Ace, King, Ten, Nine, _, _] => 0.0,
            [Ace, King, Ten, _, _, _] => 0.0,
            [Ace, King, Nine, _, _, _] => 0.0,
            [Ace, King, _, _, _, _] => 0.0,
            [Ace, Queen, Jack, Ten, Nine, _] => 0.0,
            [Ace, Queen, Jack, Ten, _, _] => 0.0,
            [Ace, Queen, Jack, Nine, _, _] => 0.0,
            [Ace, Queen, Jack, _, _, _] => 0.0,
            [Ace, Queen, Ten, Nine, _, _] => 0.0,
            [Ace, Queen, Ten, _, _, _] => 0.0,
            [Ace, Queen, Nine, _, _, _] => 0.0,
            [Ace, Queen, _, _, _, _] => 0.0,
            [Ace, Jack, Ten, Nine, _, _] => 0.0,
            [Ace, Jack, Ten, _, _, _] => 0.0,
            [Ace, Jack, Nine, _, _, _] => 0.0,
            [Ace, Jack, _, _, _, _] => 0.0,
            [Ace, Ten, Nine, _, _, _] => 0.0,
            [Ace, Ten, _, _, _, _] => 0.0,
            [Ace, Nine, _, _, _, _] => 0.0,
            [Ace, _, _, _, _, _] => 0.0,
            [King, Queen, Jack, Ten, Nine, _] => 0.0,
            [King, Queen, Jack, Ten, _, _] => 0.0,
            [King, Queen, Jack, Nine, _, _] => 0.0,
            [King, Queen, Jack, _, _, _] => 0.0,
            [King, Queen, Ten, Nine, _, _] => 0.0,
            [King, Queen, Ten, _, _, _] => 0.0,
            [King, Queen, Nine, _, _, _] => 0.0,
            [King, Queen, _, _, _, _] => 0.0,
            [King, Jack, Ten, Nine, _, _] => 0.0,
            [King, Jack, Ten, _, _, _] => 0.0,
            [King, Jack, Nine, _, _, _] => 0.0,
            [King, Jack, _, _, _, _] => 0.0,
            [King, Ten, Nine, _, _, _] => 0.0,
            [King, Ten, _, _, _, _] => 0.0,
            [King, Nine, _, _, _, _] => 0.0,
            [King, _, _, _, _, _] => 0.0,
            [Queen, Jack, Ten, Nine, _, _] => 0.0,
            [Queen, Jack, Ten, _, _, _] => 0.0,
            [Queen, Jack, Nine, _, _, _] => 0.0,
            [Queen, Jack, _, _, _, _] => 0.0,
            [Queen, Ten, Nine, _, _, _] => 0.0,
            [Queen, Ten, _, _, _, _] => 0.0,
            [Queen, Nine, _, _, _, _] => 0.0,
            [Queen, _, _, _, _, _] => 0.0,
            [Jack, Ten, Nine, _, _, _] => 0.0,
            [Jack, Ten, _, _, _, _] => 0.0,
            [Jack, Nine, _, _, _, _] => 0.0,
            [Jack, _, _, _, _, _] => 0.0,
            [Ten, Nine, _, _, _, _] => 0.0,
            [Ten, _, _, _, _, _] => 0.0,
            [Nine, _, _, _, _, _] => 0.0,
            [_, _, _, _, _, _] => 0.0,
        } // todo! use table generated from test below
    }

    fn losing_trick_count(hand: &Hand) -> f64 {
        let mut acc = 0.0;
        for suit in Suit::iter() {
            let mut card_vec = hand.cards_in(suit).rev().map(|c| c.denomination).collect_vec();
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
                3 | 7 => {
                    // Three-card suits can only have three losers, for 7-card-suits, don't add additional losers
                    3.0 - Denomination::iter()
                        .rev()
                        .take(3)
                        .filter(|d| card_vec.contains(&d))
                        .count() as f64
                } // subtract 1 for each of the top three honours
                4..=6 => {
                    3.0 - Denomination::iter()
                    .rev()
                    .take(3)
                    .filter(|d| card_vec.contains(&d))
                    .count() as f64 // subtract 1 for each of the top three honours
                    + Self::losers_for_midvalues(&card_vec[..])
                } // add 0.5 if we lack mid-values
                8 => match &card_vec[..3] {
                    // for 8-card-suits, missing the queen is only half a loser!
                    [Ace, King, Queen] => 0.0,
                    [Ace, King, _] => 0.5,
                    [Ace, Queen, _] => 1.0,
                    [King, Queen, _] => 1.0,
                    [Ace, _, _] => 1.5,
                    [King, _, _] => 1.5,
                    [Queen, _, _] => 2.0,
                    _ => 3.0,
                },
                _ => 0.0, // 13 card suits have no losers, Chicanes have no losers
            }
        }
        acc
    }

    fn losers_for_midvalues(den: &[Denomination]) -> f64 {
        // we already took care of Ace, King and Queen, disregard now, only look at midvalues
        if den.contains(&Jack) {
            // Jack is enough in any case
            0.0
        } else if den.contains(&Ten) && den.contains(&Nine) {
            // Ten and 9 together are also enough
            0.0
        } else {
            0.5
            // maybe refine this in the future using match statements??
            // [_, _, _, Ten] => 0.0, // AKQT
            // [_, _, Ten, _] => 0.0, // AKTx, AQTx, KQTx
            // [_, Ten, _, _] => 0.5, // ATxx, KTxx, QTxx
            // [Ten, _, _, _] => 0.5, // Txxx,
        }
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering::*;

    use crate::card::{Denomination, Suit};
    use crate::evaluator::*;
    use crate::hand::Hand;
    use test_case::test_case;

    #[test]
    fn generate_two_card_hands() {
        let cards = Denomination::iter().rev().collect::<Vec<_>>();
        let cut = Denomination::Ten;
        for i in 0..13 {
            match cards[i].cmp(&cut) {
                Less => continue,
                Equal => {
                    println!("[_, _,] => 0.0,");
                }
                Greater => {
                    for j in i + 1..13 {
                        match cards[j].cmp(&cut) {
                            Less => continue,
                            Equal => {
                                println!("[{:?}, _] => 0.0,", cards[i]);
                            }
                            Greater => {
                                println!("[{:?}, {:?}] => 0.0,", cards[i], cards[j]);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn generate_three_card_hands() {
        let cards = Denomination::iter().rev().collect::<Vec<_>>();
        let cut = Denomination::Nine;
        for i in 0..13 {
            match cards[i].cmp(&cut) {
                Less => continue,
                Equal => {
                    println!("[_, _, _] => 0.0,");
                }
                Greater => {
                    for j in i + 1..13 {
                        match cards[j].cmp(&cut) {
                            Less => continue,
                            Equal => {
                                println!("[{:?}, _, _] => 0.0,", cards[i]);
                            }
                            Greater => {
                                for k in j + 1..13 {
                                    match cards[k].cmp(&cut) {
                                        Less => continue,
                                        Equal => {
                                            println!("[{:?}, {:?}, _] => 0.0,", cards[i], cards[j]);
                                        }
                                        Greater => {
                                            println!("[{:?}, {:?}, {:?}] => 0.0,", cards[i], cards[j], cards[k]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn generate_four_card_hands() {
        let cards = Denomination::iter().rev().collect::<Vec<_>>();
        let cut = Denomination::Eight;
        for i in 0..13 {
            match cards[i].cmp(&cut) {
                Less => continue,
                Equal => {
                    println!("[_, _, _, _] => 0.0,");
                }
                Greater => {
                    for j in i + 1..13 {
                        match cards[j].cmp(&cut) {
                            Less => continue,
                            Equal => {
                                println!("[{:?}, _, _, _] => 0.0,", cards[i]);
                            }
                            Greater => {
                                for k in j + 1..13 {
                                    match cards[k].cmp(&cut) {
                                        Less => continue,
                                        Equal => {
                                            println!("[{:?}, {:?}, _, _] => 0.0,", cards[i], cards[j]);
                                        }
                                        Greater => {
                                            for l in k + 1..13 {
                                                match cards[l].cmp(&cut) {
                                                    Less => continue,
                                                    Equal => {
                                                        println!(
                                                            "[{:?}, {:?}, {:?}, _] => 0.0,",
                                                            cards[i], cards[j], cards[k]
                                                        );
                                                    }
                                                    Greater => {
                                                        println!(
                                                            "[{:?}, {:?}, {:?}, {:?}] => 0.0,",
                                                            cards[i], cards[j], cards[k], cards[l]
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn generate_five_card_hands() {
        let cards = Denomination::iter().rev().collect::<Vec<_>>();
        let cut = Denomination::Eight;
        for i in 0..13 {
            match cards[i].cmp(&cut) {
                Less => continue,
                Equal => {
                    println!("[_, _, _, _, _] => 0.0,");
                }
                Greater => {
                    for j in i + 1..13 {
                        match cards[j].cmp(&cut) {
                            Less => continue,
                            Equal => {
                                println!("[{:?}, _, _, _, _] => 0.0,", cards[i]);
                            }
                            Greater => {
                                for k in j + 1..13 {
                                    match cards[k].cmp(&cut) {
                                        Less => continue,
                                        Equal => {
                                            println!("[{:?}, {:?}, _, _, _] => 0.0,", cards[i], cards[j]);
                                        }
                                        Greater => {
                                            for l in k + 1..13 {
                                                match cards[l].cmp(&cut) {
                                                    Less => continue,
                                                    Equal => {
                                                        println!(
                                                            "[{:?}, {:?}, {:?}, _, _] => 0.0,",
                                                            cards[i], cards[j], cards[k]
                                                        );
                                                    }
                                                    Greater => {
                                                        for m in l + 1..13 {
                                                            match cards[m].cmp(&cut) {
                                                                Less => continue,
                                                                Equal => {
                                                                    println!(
                                                                        "[{:?}, {:?}, {:?}, {:?}, _] => 0.0,",
                                                                        cards[i], cards[j], cards[k], cards[l]
                                                                    );
                                                                }
                                                                Greater => {
                                                                    println!(
                                                                        "[{:?}, {:?}, {:?}, {:?}, {:?}] => 0.0,",
                                                                        cards[i],
                                                                        cards[j],
                                                                        cards[k],
                                                                        cards[l],
                                                                        cards[m]
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn generate_six_card_hands() {
        let cards = Denomination::iter().rev().collect::<Vec<_>>();
        let cut = Denomination::Eight;
        for i in 0..13 {
            match cards[i].cmp(&cut) {
                Less => continue,
                Equal => {
                    println!("[_, _, _, _, _, _] => 0.0,");
                }
                Greater => {
                    for j in i + 1..13 {
                        match cards[j].cmp(&cut) {
                            Less => continue,
                            Equal => {
                                println!("[{:?}, _, _, _, _, _] => 0.0,", cards[i]);
                            }
                            Greater => {
                                for k in j + 1..13 {
                                    match cards[k].cmp(&cut) {
                                        Less => continue,
                                        Equal => {
                                            println!("[{:?}, {:?}, _, _, _, _] => 0.0,", cards[i], cards[j]);
                                        }
                                        Greater => {
                                            for l in k + 1..13 {
                                                match cards[l].cmp(&cut) {
                                                    Less => continue,
                                                    Equal => {
                                                        println!(
                                                            "[{:?}, {:?}, {:?}, _, _, _] => 0.0,",
                                                            cards[i], cards[j], cards[k]
                                                        );
                                                    }
                                                    Greater => {
                                                        for m in l + 1..13 {
                                                            match cards[m].cmp(&cut) {
                                                                Less => continue,
                                                                Equal => {
                                                                    println!(
                                                                        "[{:?}, {:?}, {:?}, {:?}, _, _] => 0.0,",
                                                                        cards[i], cards[j], cards[k], cards[l]
                                                                    );
                                                                }
                                                                Greater => {
                                                                    for n in m + 1..13 {
                                                                        match cards[n].cmp(&cut) {
                                                                            Less => continue,
                                                                            Equal => {
                                                                                println!(
                                                                                "[{:?}, {:?}, {:?}, {:?}, {:?}, _] => 0.0,",
                                                                                cards[i], cards[j], cards[k], cards[l], cards[m]);
                                                                            }
                                                                            Greater => {
                                                                                println!("[{:?}, {:?}, {:?}, {:?}, {:?}, {:?}] => 0.0,",
                                                                                    cards[i], cards[j], cards[k], cards[l], cards[m], cards[n]);
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
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
        assert_eq!(
            ForumDPlus2015Evaluator::distribution_points(&hand, None, false, false),
            dp
        );
    }
}
