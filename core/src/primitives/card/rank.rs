use crate::error::BBError;
use crate::primitives::Card;
use std::cmp::Ordering;
use strum::{Display, EnumIter};

pub const N_RANKS: usize = 13;

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Rank {
    #[strum(serialize = "2")]
    Two = 2,
    #[strum(serialize = "3")]
    Three = 3,
    #[strum(serialize = "4")]
    Four = 4,
    #[strum(serialize = "5")]
    Five = 5,
    #[strum(serialize = "6")]
    Six = 6,
    #[strum(serialize = "7")]
    Seven = 7,
    #[strum(serialize = "8")]
    Eight = 8,
    #[strum(serialize = "9")]
    Nine = 9,
    #[strum(serialize = "T")]
    Ten = 10,
    #[strum(serialize = "J")]
    Jack = 11,
    #[strum(serialize = "Q")]
    Queen = 12,
    #[strum(serialize = "K")]
    King = 13,
    #[strum(serialize = "A")]
    Ace = 14,
}

impl From<Card> for Rank {
    fn from(card: Card) -> Rank {
        card.rank
    }
}

impl Rank {
    pub fn from_char(char: char) -> Result<Rank, BBError> {
        match char {
            'A' => Ok(Rank::Ace),
            'a' => Ok(Rank::Ace),
            'K' => Ok(Rank::King),
            'k' => Ok(Rank::King),
            'Q' => Ok(Rank::Queen),
            'q' => Ok(Rank::Queen),
            'J' => Ok(Rank::Jack),
            'j' => Ok(Rank::Jack),
            'T' => Ok(Rank::Ten),
            't' => Ok(Rank::Ten),
            '9' => Ok(Rank::Nine),
            '8' => Ok(Rank::Eight),
            '7' => Ok(Rank::Seven),
            '6' => Ok(Rank::Six),
            '5' => Ok(Rank::Five),
            '4' => Ok(Rank::Four),
            '3' => Ok(Rank::Three),
            '2' => Ok(Rank::Two),
            c => Err(BBError::UnknownRank(c.into())),
        }
    }

    pub fn touches(&self, other: &Denomination) -> bool {
        // println!("testing {} and {}", self, other);
        match self.cmp(other) {
            Ordering::Less => *other as usize - *self as usize == 1,
            Ordering::Greater => *self as usize - *other as usize == 1,
            Ordering::Equal => false,
        }
    }
}

impl std::str::FromStr for Rank {
    type Err = BBError;

    fn from_str(string: &str) -> Result<Rank, BBError> {
        let mut chars = string.trim().chars();
        let char = chars.next().ok_or(BBError::UnknownRank(string.into()))?;
        if chars.next().is_some() {
            return Err(BBError::UnknownRank(string.into()));
        }
        Rank::from_char(char)
    }
}

#[cfg(test)]
mod tests {
    use super::Rank::*;
    use crate::error::BBError;
    use crate::primitives::card::{Rank, Suit};
    use crate::primitives::Card;
    use std::str::FromStr;
    use strum::IntoEnumIterator;
    use test_case::test_case;

    #[test_case(King, Ace; "King and Ace")]
    #[test_case(Ten, Queen; "Ten and Queen")]
    #[test_case(Eight, Jack; "Eight and Jack")]
    #[test_case(Two, Ten; "Two and Ten")]
    fn relative_ranking(lower: Rank, higher: Rank) {
        assert!(lower < higher);
    }

    #[test_case('a', Ace; "A is Ace")]
    #[test_case('k', King; "k is King")]
    #[test_case('q', Queen; "q is Queen")]
    #[test_case('J', Jack; "J is Jack")]
    #[test_case('t', Ten; "t is Ten")]
    #[test_case('9', Nine; "9 is Nine")]
    #[test_case('7', Seven; "7 is Seven")]
    #[test_case('3', Three; "3 is Three")]
    fn parsing_char(input: char, expected: Rank) {
        assert_eq!(Rank::from_char(input).unwrap(), expected);
    }

    #[test_case("A", Ace; "A is Ace")]
    #[test_case("k", King; "k is King")]
    #[test_case("q", Queen; "q is Queen")]
    #[test_case("J", Jack; "J is Jack")]
    #[test_case("t", Ten; "t is Ten")]
    #[test_case("9", Nine; "9 is Nine")]
    #[test_case("7", Seven; "7 is Seven")]
    #[test_case("3", Three; "3 is Three")]
    fn parsing_str(input: &str, expected: Rank) {
        assert_eq!(Rank::from_str(input).unwrap(), expected);
    }

    #[test_case(""; "Empty string")]
    #[test_case(".k"; "additional char")]
    #[test_case("jk"; "two chars")]
    fn parsing_multi_char_str_fails(input: &str) {
        assert!(Rank::from_str(input).is_err());
    }

    #[test_case("h"; "suit hearts")]
    #[test_case("b"; "german jack")]
    #[test_case("l"; "unknown letter")]
    fn parsing_unknown_str_fails(input: &str) {
        assert_eq!(Rank::from_str(input), Err(BBError::UnknownRank(input.into())));
    }

    #[test_case(Ace, "A")]
    #[test_case(King, "K")]
    #[test_case(Queen, "Q")]
    #[test_case(Jack, "J")]
    #[test_case(Ten, "T")]
    #[test_case(Nine, "9")]
    #[test_case(Eight, "8")]
    #[test_case(Seven, "7")]
    #[test_case(Six, "6")]
    #[test_case(Five, "5")]
    #[test_case(Four, "4")]
    #[test_case(Three, "3")]
    #[test_case(Two, "2")]
    fn display(rank: Rank, expected: &str) {
        assert_eq!(format!("{}", rank), expected);
    }

    #[test_case(Ace)]
    #[test_case(King)]
    #[test_case(Queen)]
    #[test_case(Jack)]
    #[test_case(Ten)]
    #[test_case(Nine)]
    #[test_case(Eight)]
    #[test_case(Seven)]
    #[test_case(Six)]
    #[test_case(Five)]
    #[test_case(Four)]
    #[test_case(Three)]
    #[test_case(Two)]
    fn round_trip(rank: Rank) {
        let string = format!("{}", rank);
        let rank_char = string.chars().next().unwrap();
        let new_rank = Rank::from_char(rank_char).unwrap();
        assert_eq!(rank, new_rank);
    }

    #[test_case('.')]
    #[test_case('C')]
    #[test_case('H')]
    #[test_case('s')]
    #[test_case('d')]
    fn fail_misc_characters(input: char) {
        assert_eq!(Rank::from_char(input).unwrap_err(), BBError::UnknownRank(input.into()))
    }

    #[test]
    fn copy() {
        let mut x = King;
        let y = x;
        x = Queen;
        assert_eq!(x, Queen);
        assert_eq!(y, King);
    }

    #[test]
    fn iteration() {
        assert_eq!(
            Rank::iter().collect::<Vec<Rank>>(),
            vec![Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace]
        )
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Jack), "Jack")
    }

    #[test_case(Card { suit: Suit::Spades, rank: King}, Rank::King; "King of Spades is a King")]
    fn from_card(card: Card, expected: Rank) {
        assert_eq!(expected, card.into())
    }

    #[test_case("A", "K", true)]
    #[test_case("K", "A", true)]
    #[test_case("Q", "J", true)]
    #[test_case("Q", "T", false)]
    fn touches(one: &str, other: &str, expected: bool) {
        let one = Denomination::from_str(one).unwrap();
        let other = Denomination::from_str(other).unwrap();
        assert_eq!(one.touches(&other), expected);
    }
}
