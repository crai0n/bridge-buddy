use crate::error::BBError;
use crate::primitives::Suit;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Strain {
    Trump(Suit),
    NoTrump,
}

impl std::fmt::Display for Strain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Strain::Trump(s) => {
                write!(f, "{}", s)
            }
            Strain::NoTrump => {
                write!(f, "NT")
            }
        }
    }
}

impl std::str::FromStr for Strain {
    type Err = BBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() == 1 {
            let char = s.chars().next().unwrap();
            match Suit::from_char(char) {
                Ok(s) => Ok(Strain::Trump(s)),
                Err(e) => Err(e),
            }
        } else {
            match s.to_uppercase().as_ref() {
                "SA" | "NT" => Ok(Strain::NoTrump),
                _ => Err(BBError::UnknownStrain(s.into())),
            }
        }
    }
}

impl Strain {
    pub const fn next(&self) -> Self {
        match self {
            Strain::Trump(suit) => match suit {
                Suit::Spades => Strain::NoTrump,
                s => Strain::Trump(s.next()),
            },
            Strain::NoTrump => Strain::Trump(Suit::Clubs),
        }
    }

    pub const fn previous(&self) -> Self {
        match self {
            Strain::Trump(suit) => match suit {
                Suit::Clubs => Strain::NoTrump,
                s => Strain::Trump(s.previous()),
            },
            Strain::NoTrump => Strain::Trump(Suit::Spades),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Strain;
    use super::Strain::*;
    use crate::primitives::Suit::*;
    use std::cmp::Ordering;
    use std::cmp::Ordering::*;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("NT", NoTrump; "No Trump")]
    #[test_case("S", Trump(Spades); "Spades")]
    #[test_case("d", Trump(Diamonds); "Diamonds")]
    #[test_case("♥", Trump(Hearts); "Hearts")]
    fn from_str(str: &str, strain: Strain) {
        assert_eq!(Strain::from_str(str).unwrap(), strain)
    }

    #[test_case("NP")]
    #[test_case("PT")]
    #[test_case("SS")]
    #[test_case("K")]
    #[test_case("i")]
    fn from_str_fails(str: &str) {
        assert!(Strain::from_str(str).is_err());
    }

    #[test_case(NoTrump, Trump(Clubs))]
    #[test_case(Trump(Clubs), Trump(Diamonds))]
    #[test_case(Trump(Spades), NoTrump)]
    fn next(input: Strain, expected: Strain) {
        assert_eq!(input.next(), expected);
    }

    #[test_case(NoTrump, Trump(Clubs))]
    #[test_case(Trump(Clubs), Trump(Diamonds))]
    #[test_case(Trump(Spades), NoTrump)]
    fn previous(expected: Strain, input: Strain) {
        assert_eq!(input.previous(), expected);
    }

    #[test_case(NoTrump, "NT"; "No Trump")]
    fn serialize(strain: Strain, expected: &str) {
        let contract_str = format!("{}", strain);
        assert_eq!(&contract_str, expected);
    }

    #[test_case(NoTrump, Trump(Spades), Greater; "No Trump is higher than Spades")]
    #[test_case(Trump(Spades), Trump(Hearts), Greater; "Spades is higher than Hearts")]
    #[test_case(Trump(Hearts), Trump(Diamonds), Greater; "Hearts is higher than Diamonds")]
    #[test_case(Trump(Diamonds), Trump(Clubs), Greater; "Diamonds is higher than Clubs")]
    #[test_case(Trump(Hearts), Trump(Hearts), Equal; "Hearts is equal to Hearts")]
    fn ordering(one: Strain, other: Strain, expected: Ordering) {
        let ord = one.cmp(&other);
        assert_eq!(ord, expected);
    }

    #[test]
    fn is_clone() {
        let mut one = NoTrump;
        let two = one;

        assert_eq!(one, two);

        one = Trump(Clubs);

        assert_eq!(one, Trump(Clubs));
        assert_eq!(two, NoTrump);
    }
}
