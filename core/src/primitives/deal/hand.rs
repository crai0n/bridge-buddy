use crate::error::BBError;
use crate::primitives::{card::Rank, Card, Suit};
use std::cmp::Ordering;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hand<const N: usize> {
    cards: [Card; N],
}

impl<const N: usize> std::fmt::Display for Hand<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for suit in Suit::iter().rev() {
            // Spades, then Hearts, ...
            write!(f, "{}: ", suit)?;
            for card in self.cards_in(suit).rev() {
                write!(f, "{}", card.rank)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const N: usize> FromStr for Hand<N> {
    type Err = BBError;

    fn from_str(string: &str) -> Result<Hand<N>, Self::Err> {
        let mut cards: Vec<Card> = vec![];

        let separate_suits = string.trim().split(['\n', ',']);
        for cards_in_suit in separate_suits {
            let (suit_symbol, ranks) = Hand::<N>::split_at_colon(cards_in_suit)?;
            let suit_cards = Hand::<N>::read_cards_for_suit(suit_symbol, ranks)?;
            cards.extend_from_slice(&suit_cards);
        }
        Hand::from_cards(&cards)
    }
}

impl<const N: usize> Hand<N> {
    fn split_at_colon(string: &str) -> Result<(&str, &str), BBError> {
        string.split_once(':').ok_or(BBError::ParseError(
            string.into(),
            "missing colon between suit and cards",
        ))
    }

    fn read_cards_for_suit(suit_symbol: &str, ranks: &str) -> Result<Vec<Card>, BBError> {
        let mut suit_cards = vec![];
        let suit = Suit::from_str(suit_symbol)?;
        for rank_char in ranks.trim().chars() {
            let rank = Rank::from_char(rank_char)?;
            suit_cards.push(Card { rank, suit });
        }
        Ok(suit_cards)
    }

    pub fn from_cards(cards: &[Card]) -> Result<Self, BBError> {
        let cards = Hand::sanitize_cards(cards)?;
        Ok(Hand { cards })
    }

    fn sanitize_cards(cards: &[Card]) -> Result<[Card; N], BBError> {
        let mut cards: [Card; N] = cards.try_into().or(Err(BBError::CardCount))?;
        cards.sort_unstable();
        Hand::check_for_duplicates(&cards)?;
        Ok(cards)
    }

    fn check_for_duplicates(&cards: &[Card; N]) -> Result<(), BBError> {
        for i in 0..cards.len() - 1 {
            if cards[i] == cards[i + 1] {
                return Err(BBError::Duplicate(cards[i]));
            }
        }
        Ok(())
    }

    pub fn cards(&self) -> impl DoubleEndedIterator<Item = &Card> {
        self.cards.iter()
    }

    pub fn cards_in(&self, suit: Suit) -> impl DoubleEndedIterator<Item = &Card> {
        self.cards.iter().filter(move |&card| card.suit == suit)
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    pub fn length_in(&self, suit: Suit) -> u8 {
        self.cards_in(suit).count() as u8
    }
}

impl Hand<13> {
    pub fn hand_type(&self) -> crate::primitives::deal::hand::HandType {
        let mut suit_lengths = Suit::iter().map(|s| (s, self.length_in(s))).collect::<Vec<_>>();

        suit_lengths.sort_unstable_by(Hand::descending_length_and_suit_value);

        match suit_lengths[..] {
            // three suits with at least 4 cards (third cannot have more than four)
            [(s1, _), (s2, _), (s3, 4), _] => HandType::ThreeSuited(s1, s2, s3),
            [(s1, 5..), (s2, 4..), _, _] => HandType::TwoSuited(s1, s2),
            [(s1, 6..), _, _, _] => HandType::SingleSuited(s1),
            [(s1, 5..), _, _, _] => HandType::Balanced(Some(s1)),
            _ => HandType::Balanced(None),
        }
    }

    fn descending_length_and_suit_value(one: &(Suit, u8), other: &(Suit, u8)) -> Ordering {
        match other.1.cmp(&one.1) {
            Ordering::Equal => other.0.cmp(&one.0), //for equal length, order by suit value
            ord => ord,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum HandType {
    ThreeSuited(Suit, Suit, Suit),
    TwoSuited(Suit, Suit),
    SingleSuited(Suit),
    Balanced(Option<Suit>), // might contain a 5-card suit
}

impl std::fmt::Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ThreeSuited(s1, s2, s3) => write!(f, "three-suited: {}, {} and {}", s1, s2, s3),
            Self::TwoSuited(s1, s2) => write!(f, "two-suited: {} and {}", s1, s2),
            Self::SingleSuited(s) => write!(f, "single-suited: {}", s),
            Self::Balanced(Some(s)) => write!(f, "balanced with 5 cards in {}", s),
            Self::Balanced(None) => write!(f, "balanced"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Hand, HandType};
    use crate::error::BBError;
    use crate::primitives::card::Rank::*;
    use crate::primitives::Suit::*;
    use crate::primitives::{Card, Suit};
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("H:, ♠:9J7A2T6K8Q, ♦: AK, C: A", ["cA", "dA", "dK", "sA", "sK", "sQ", "sJ", "sT", "s9", "s8", "s7", "s6", "s2"]; "single-suited")]
    #[test_case("H:8JK, ♠:9J2T6, ♦: AJ36, C: 4", ["c4", "dA", "dJ", "d3", "d6", "s9", "sJ", "s2", "sT", "s6", "h8", "hJ", "hK"]; "balanced")]
    fn hand_from_str(string: &str, cards: [&str; 13]) {
        let hand1 =
            Hand::<13>::from_cards(&cards.iter().map(|c| Card::from_str(c).unwrap()).collect::<Vec<Card>>()).unwrap();
        let hand2 = Hand::<13>::from_str(string).unwrap();
        assert_eq!(hand1, hand2);
    }

    #[test_case("H:8JK, ♠:9J2T6, ♦: AJ36, C: 4", ["c4", "dA", "dJ", "d3", "d6", "s9", "sJ", "s2", "sT", "s6", "h8", "hJ", "hK"]; "balanced")]
    fn cards(string: &str, cards: [&str; 13]) {
        let hand = Hand::<13>::from_str(string).unwrap();
        let mut cards = cards.iter().map(|c| Card::from_str(c).unwrap()).collect::<Vec<Card>>();
        cards.sort_unstable();
        assert!(hand.cards().eq(cards.iter()));
    }

    #[test_case("H:8JK, ♠:9J2T6, ♦: AJ36, C: 4", Spades, &["s9", "sJ", "s2", "sT", "s6"]; "5 spades")]
    #[test_case("H:8JK, ♠:9J2T6, ♦: AJ36, C: 4", Clubs, &["c4"]; "1 club")]
    #[test_case("H:, ♠:9J7A2T6K8Q, ♦: AK, C: A", Hearts, &[]; "no hearts")]
    fn cards_in(string: &str, suit: Suit, cards: &[&str]) {
        let hand = Hand::<13>::from_str(string).unwrap();
        let mut cards = cards.iter().map(|c| Card::from_str(c).unwrap()).collect::<Vec<Card>>();
        cards.sort_unstable();
        assert!(hand.cards_in(suit).eq(cards.iter()));
    }

    #[test_case(&["cA", "dA", "dK", "sA", "sK", "sQ", "sJ", "sT", "s9", "s8", "s7", "s6", "s2"]; "one club")]
    fn from_cards_is_sorted(cards: &[&str]) {
        let hand =
            Hand::<13>::from_cards(&cards.iter().map(|c| Card::from_str(c).unwrap()).collect::<Vec<Card>>()).unwrap();
        assert_eq!(
            hand.cards().nth(1).unwrap(),
            &Card {
                suit: Diamonds,
                rank: King,
            }
        );
    }

    #[test_case(["cA", "dA", "dK", "sA", "sK", "sQ", "sJ", "sT", "s9", "s8", "s7", "s6", "s2"], 10, 0, 2, 1; "10-0-0-2")]
    fn suit_count(cards: [&str; 13], spades: usize, hearts: usize, diamonds: usize, clubs: usize) {
        let hand =
            Hand::<13>::from_cards(&cards.iter().map(|c| Card::from_str(c).unwrap()).collect::<Vec<Card>>()).unwrap();
        assert_eq!(hand.cards_in(Spades).count(), spades);
        assert_eq!(hand.cards_in(Hearts).count(), hearts);
        assert_eq!(hand.cards_in(Diamonds).count(), diamonds);
        assert_eq!(hand.cards_in(Clubs).count(), clubs);
    }

    #[test_case("♠:AKQJT98765432", HandType::SingleSuited(Spades) ; "13-0-0-0")]
    #[test_case("♥:AKQJT98765,♠:432", HandType::SingleSuited(Hearts) ; "10-3-0-0")]
    #[test_case("♦:AKQJT9,♥:876,♠:54,♣:32", HandType::SingleSuited(Diamonds) ; "6-3-2-2")]
    #[test_case("♠:AKQJT9876,♥:5432", HandType::TwoSuited(Spades, Hearts) ; "9-4-0-0")]
    #[test_case("♠:AKQJT,♦:9876,♥:543,♣:2", HandType::TwoSuited(Spades, Diamonds); "5-4-3-1")]
    #[test_case("♦:AKQJT,♣:9876,♠:54,♥:32", HandType::TwoSuited(Diamonds, Clubs); "5-4-2-2")]
    #[test_case("♠:AKQJT,♥:9876,♦:5432", HandType::ThreeSuited(Spades, Hearts, Diamonds); "5-4-4-0")]
    #[test_case("♣:AKQJ,♥:T987,♦:6543,♠:2", HandType::ThreeSuited(Hearts, Diamonds, Clubs); "4-4-4-1")]
    #[test_case("♠:AKQJT,♥:987,♦:654,♣:32", HandType::Balanced(Some(Spades)) ; "5-3-3-2")]
    #[test_case("♠:AKQJ,♥:T98,♦:765,♣:432", HandType::Balanced(None); "4-3-3-3")]
    #[test_case("♠:AKQJ,♥:T987,♦:654,♣:32", HandType::Balanced(None); "4-4-3-2")]
    fn hand_type(hand: &str, expected_hand_type: HandType) {
        let hand = Hand::from_str(hand).unwrap();
        assert_eq!(hand.hand_type(), expected_hand_type)
    }

    #[test_case(["cA", "dA", "dK", "sA", "sK", "sQ", "sJ", "sT", "s9", "s8", "s7", "s6", "s2"], "dA", "dQ"; "diamond Ace but not diamond Queen")]
    fn contains(cards: [&str; 13], this: &str, not: &str) {
        let hand =
            Hand::<13>::from_cards(&cards.iter().map(|c| Card::from_str(c).unwrap()).collect::<Vec<Card>>()).unwrap();
        let this = Card::from_str(this).unwrap();
        let not = Card::from_str(not).unwrap();
        assert!(!hand.contains(&not));
        assert!(hand.contains(&this));
    }

    #[test_case("♠: AKQJT98762\n♥: \n♦: AK\n♣: A\n")]
    fn round_trip(input: &str) {
        let hand = Hand::<13>::from_str(input).unwrap();
        assert_eq!(format!("{}", hand), input);
    }

    #[test_case("d:23456789TJQK2", "D2"; "diamonds two")]
    fn find_duplicates(input: &str, card: &str) {
        let hand = Hand::<13>::from_str(input);
        assert_eq!(hand, Err(BBError::Duplicate(Card::from_str(card).unwrap())));
    }
}
