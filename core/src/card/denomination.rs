use crate::error::BBError;
use crate::util;
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Denomination {
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    #[strum(serialize = "T")]
    Ten,
    #[strum(serialize = "J")]
    Jack,
    #[strum(serialize = "Q")]
    Queen,
    #[strum(serialize = "K")]
    King,
    #[strum(serialize = "A")]
    Ace,
}

impl Denomination {
    pub fn from_char(char: char) -> Result<Denomination, BBError> {
        match char {
            'A' => Ok(Denomination::Ace),
            'a' => Ok(Denomination::Ace),
            'K' => Ok(Denomination::King),
            'k' => Ok(Denomination::King),
            'Q' => Ok(Denomination::Queen),
            'q' => Ok(Denomination::Queen),
            'J' => Ok(Denomination::Jack),
            'j' => Ok(Denomination::Jack),
            'T' => Ok(Denomination::Ten),
            't' => Ok(Denomination::Ten),
            '9' => Ok(Denomination::Nine),
            '8' => Ok(Denomination::Eight),
            '7' => Ok(Denomination::Seven),
            '6' => Ok(Denomination::Six),
            '5' => Ok(Denomination::Five),
            '4' => Ok(Denomination::Four),
            '3' => Ok(Denomination::Three),
            '2' => Ok(Denomination::Two),
            c => Err(BBError::UnknownDenomination(c)),
        }
    }
}

impl std::str::FromStr for Denomination {
    type Err = BBError;

    fn from_str(string: &str) -> Result<Denomination, BBError> {
        let char = util::single_char_from_str(string)?;
        Denomination::from_char(char)
    }
}

#[cfg(test)]
mod tests {
    use super::Denomination::*;
    use crate::card::Denomination;
    use crate::error::BBError;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case(King, Ace; "King and Ace")]
    #[test_case(Ten, Queen; "Ten and Queen")]
    #[test_case(Eight, Jack; "Eight and Jack")]
    #[test_case(Two, Ten; "Two and Ten")]
    fn relative_ranking(lower: Denomination, higher: Denomination) {
        assert!(lower < higher);
    }

    #[test_case('A', Ace; "A is Ace")]
    #[test_case('k', King; "k is King")]
    #[test_case('q', Queen; "q is Queen")]
    #[test_case('J', Jack; "J is Jack")]
    #[test_case('t', Ten; "t is Ten")]
    #[test_case('9', Nine; "9 is Nine")]
    #[test_case('7', Seven; "7 is Seven")]
    #[test_case('3', Three; "3 is Three")]
    fn parsing_char(input: char, expected: Denomination) {
        assert_eq!(Denomination::from_char(input).unwrap(), expected);
    }

    #[test_case("A", Ace; "A is Ace")]
    #[test_case("k", King; "k is King")]
    #[test_case("q", Queen; "q is Queen")]
    #[test_case("J", Jack; "J is Jack")]
    #[test_case("t", Ten; "t is Ten")]
    #[test_case("9", Nine; "9 is Nine")]
    #[test_case("7", Seven; "7 is Seven")]
    #[test_case("3", Three; "3 is Three")]
    fn parsing_str(input: &str, expected: Denomination) {
        assert_eq!(Denomination::from_str(input).unwrap(), expected);
    }

    #[test_case(""; "Empty string")]
    #[test_case(".k"; "additional char")]
    #[test_case("jk"; "two chars")]
    fn parsing_multi_char_str_fails(input: &str) {
        assert!(Denomination::from_str(input).is_err());
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
    fn display(denomination: Denomination, expected: &str) {
        assert_eq!(format!("{}", denomination), expected);
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
    fn round_trip(denomination: Denomination) {
        let string = format!("{}", denomination);
        let den_char = string.chars().next().unwrap();
        let new_denomination = Denomination::from_char(den_char).unwrap();
        assert_eq!(denomination, new_denomination);
    }

    #[test_case('.')]
    #[test_case('C')]
    #[test_case('H')]
    #[test_case('s')]
    #[test_case('d')]
    fn fail_misc_characters(input: char) {
        assert_eq!(
            Denomination::from_char(input).unwrap_err(),
            BBError::UnknownDenomination(input)
        )
    }
}
