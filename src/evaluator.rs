use crate::card::{Suit};
use crate::hand::Hand;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct HandEvaluation {
    high_card_points: PointCount,
    // length_points: PointCount,
    // expected_tricks: u8,
    // expected_losers: u8,
    // side_suit_distribution_points: PointCount,
    // trump_distribution_points: PointCount,
    // controls: BTreeMap<Suit, bool>,
    // stops: BTreeMap<Suit, bool>
}

#[derive(Debug, PartialEq, Eq)]
pub struct PointCount {
    by_suit: BTreeMap<Suit, u8>,
    total: u8,
}

#[derive(Debug)]
pub struct ForumDPlus2015Evaluator {

}

impl ForumDPlus2015Evaluator {
    fn evaluate(hand: &Hand) -> HandEvaluation {
        //TODO
        let high_card_points = PointCount{ by_suit: BTreeMap::from([(Suit::Clubs, 0), (Suit::Diamonds, 0), (Suit::Hearts, 0), (Suit::Spades, 0)]), total: 0};
        HandEvaluation { high_card_points}
    }
}

#[cfg(test)]
mod test {
use crate::card::{Suit};
use crate::hand::Hand;
use crate::evaluator::*;

    #[test]
    fn test_evaluation() {
        let hand = Hand::from_str("♠: K865\n♥: ADT74\n♦: 6\n♣: KT2\n").unwrap();
        let HandEvaluation = ForumDPlus2015Evaluator::evaluate(&hand);
        assert_eq!(HandEvaluation.high_card_points, PointCount{ by_suit: BTreeMap::from([(Suit::Clubs, 3), (Suit::Diamonds, 0), (Suit::Hearts, 6), (Suit::Spades, 3)]), total: 12})
    }
}