use crate::engine_context::EngineContext;
use crate::evaluator::hcp::HcpValue;
use crate::primitives::card::Denomination;
use crate::primitives::card::Denomination::*;
use crate::primitives::{Card, Suit};
use itertools::Itertools;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct HcpEvaluator {}

impl HcpEvaluator {
    pub fn hcp(game: &EngineContext) -> HcpValue {
        Self::hcp_for_cards(&mut game.my_hand.cards())
    }

    pub fn hcp_in(suit: Suit, game: &EngineContext) -> HcpValue {
        Self::hcp_for_cards(&mut game.my_hand.cards_in(suit))
    }

    fn hcp_for_cards<'a>(cards: &mut impl DoubleEndedIterator<Item = &'a Card>) -> HcpValue {
        cards.fold(HcpValue(0.0), |hcp, &c| hcp + Self::hcp_value(&(c.into())))
    }

    fn hcp_value(denomination: &Denomination) -> HcpValue {
        HcpValue::from(*denomination)
    }

    pub fn adjustment_aces_and_tens(game: &EngineContext) -> f64 {
        let tens = game.my_hand.cards().filter(|&&x| x.denomination == Ten).count();
        let aces = game.my_hand.cards().filter(|&&x| x.denomination == Ace).count();
        match (tens, aces) {
            (0, 0) => -1.0,
            (0, 1) | (1, 0) => -0.5,
            (3, _) => 1.0,
            (i, j) if i + j >= 4 => 1.0,
            _ => 0.0,
        }
    }

    pub fn adjustment_unguarded_honors(game: &EngineContext) -> f64 {
        let mut acc = 0.0;
        for suit in Suit::iter() {
            let cards_vec = game.my_hand.cards_in(suit).rev().map(|x| x.denomination).collect_vec();
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
}
