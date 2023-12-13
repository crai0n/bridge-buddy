use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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
    #[test_case("Pass", Pass; "Pass_Pass")]
    #[test_case("X", Double; "x is Double")]
    #[test_case("x", Double; "x_Double")]
    #[test_case("XX", Redouble; "XX_Redouble")]
    #[test_case("Xx", Redouble; "XX_Redouble1")]
    #[test_case("xX", Redouble; "XX_Redouble2")]
    #[test_case("xx", Redouble; "XX_Redouble3")]
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
    #[test_case(Double, Double, Equal; "Double is equal to Double")]
    #[test_case(Redouble, Redouble, Equal; "Redouble is equal to Redouble")]
    #[test_case(Double, Redouble, Less; "Double is less than Redouble")]
    #[test_case(Redouble, Pass, Greater; "Redouble is greater than Pass")]
    fn ordering(one: AuxiliaryBid, other: AuxiliaryBid, expected: Ordering) {
        assert_eq!(one.cmp(&other), expected)
    }

    #[test]
    fn is_clone() {
        let one = AuxiliaryBid::Pass;
        let two = one;

        assert_eq!(one, two);
    }
}
