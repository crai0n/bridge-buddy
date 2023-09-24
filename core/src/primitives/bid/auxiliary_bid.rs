use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuxiliaryBid {
    #[strum(serialize = "p")]
    #[strum(serialize = "P")]
    #[strum(to_string = "Pass")]
    Pass,
    #[strum(serialize = "x")]
    #[strum(to_string = "X")]
    Double,
    #[strum(serialize = "xx")]
    #[strum(serialize = "Xx")]
    #[strum(serialize = "xX")]
    #[strum(to_string = "XX")]
    Redouble,
}

// impl std::fmt::Display for AuxiliaryBid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             AuxiliaryBid::Pass => write!(f, "Pass")?
//             AuxiliaryBid::Double => write!(f, "X")?
//             AuxiliaryBid::Redouble => write!(f, "XX")?
//         }
//         Ok(())
//     }
// }

#[cfg(test)]
mod test {
    use super::AuxiliaryBid;
    use super::AuxiliaryBid::*;
    use std::cmp::Ordering;
    use std::cmp::Ordering::*;

    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("p", Pass; "p is Pass")]
    #[test_case("P", Pass; "P_Pass")]
    #[test_case("X", Double; "x is Double")]
    #[test_case("x", Double; "x_Double")]
    #[test_case("XX", Redouble; "XX_Redouble")]
    fn from_str(str: &str, bid: AuxiliaryBid) {
        assert_eq!(AuxiliaryBid::from_str(str).unwrap(), bid)
    }

    #[test_case(Pass, "Pass"; "Pass")]
    #[test_case(Double, "X"; "Double")]
    #[test_case(Redouble, "XX"; "Redouble")]
    fn serialize(bid: AuxiliaryBid, expected: &str) {
        assert_eq!(format!("{}", bid), expected);
    }

    #[test_case(Pass, Pass, Equal; "Pass is equal to Pass")]
    #[test_case(Double, Redouble, Less; "Double is less than Redouble")]
    #[test_case(Redouble, Pass, Greater; "Redouble is greater than Pass")]
    fn ordering(one: AuxiliaryBid, other: AuxiliaryBid, expected: Ordering) {
        assert_eq!(one.cmp(&other), expected)
    }
}
