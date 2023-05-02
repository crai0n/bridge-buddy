use crate::card::{Card, Denomination, Suit};
use crate::hand::Hand;
use log::{debug, info};
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct HandEvaluation {
    high_card_points: PointCount,
    length_points: PointCount,
    // expected_tricks: u8,
    // expected_losers: u8,
    // side_suit_distribution_points: PointCount,
    // trump_distribution_points: PointCount,
    // controls: BTreeMap<Suit, bool>,
    // stops: BTreeMap<Suit, bool>
}

#[derive(Debug, PartialEq, Eq)]
pub struct PointCount {
    by_suit: [(Suit, u8); 4],
}

impl PointCount {
    fn in_suit(&self, suit: &Suit) -> u8 {
        self.by_suit.iter().find(|x| x.0 == *suit).unwrap().1
    }

    fn total(&self) -> u8 {
        self.by_suit.iter().fold(0, |acc, x| acc + x.1)
    }
}

#[derive(Debug)]
pub struct ForumDPlus2015Evaluator {}

impl ForumDPlus2015Evaluator {
    fn evaluate(hand: &Hand) -> HandEvaluation {
        let (high_card_points, length_points) = Self::count_hcp_and_length_points(hand);
        HandEvaluation {
            high_card_points,
            length_points,
        }
    }

    fn count_hcp_and_length_points(hand: &Hand) -> (PointCount, PointCount) {
        let mut hcp = Vec::with_capacity(4);
        let mut lp = Vec::with_capacity(4);

        for suit in Suit::iter() {
            let acc = hand
                .cards_in(suit)
                .fold(0, |a, c| a + ForumDPlus2015Evaluator::card_value(c));
            hcp.push((suit, acc));
            // do length-points here, because we need hcp information
            let len = hand.cards_in(suit).count() as u8;
            if acc >= 3 && len >= 5 {
                lp.push((suit, len - 4))
            } else {
                lp.push((suit, 0u8))
            }
        }

        (
            PointCount {
                by_suit: hcp.try_into().unwrap(),
            },
            PointCount {
                by_suit: lp.try_into().unwrap(),
            },
        )
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
}

#[cfg(test)]
mod test {
    use crate::card::Suit;
    use crate::evaluator::*;
    use crate::hand::Hand;
    use test_case::test_case;

    #[test_case("♠:AKQJT9876,♥:5432", (PointCount {
        by_suit: [
            (Suit::Clubs, 0),
            (Suit::Diamonds, 0),
            (Suit::Hearts, 0),
            (Suit::Spades, 10)
        ]
    },PointCount {
        by_suit: [
            (Suit::Clubs, 0),
            (Suit::Diamonds, 0),
            (Suit::Hearts, 0),
            (Suit::Spades, 5)
        ],
    }); "Two-Suited")]
    #[test_case("♠:K74,♥:AQ32,♦:T986,♣:K2", (PointCount {
        by_suit: [
            (Suit::Clubs, 3),
            (Suit::Diamonds, 0),
            (Suit::Hearts, 6),
            (Suit::Spades, 3)
        ],
    }, PointCount {
        by_suit: [
            (Suit::Clubs, 0),
            (Suit::Diamonds, 0),
            (Suit::Hearts, 0),
            (Suit::Spades, 0)
        ],
    }); "Balanced")]
    fn test_evaluation(hand_str: &str, expected_point_count: (PointCount, PointCount)) {
        let hand = Hand::from_str(hand_str).unwrap();
        let HandEvaluation = ForumDPlus2015Evaluator::evaluate(&hand);
        assert_eq!(
            (HandEvaluation.high_card_points, HandEvaluation.length_points),
            expected_point_count
        );
    }
}
