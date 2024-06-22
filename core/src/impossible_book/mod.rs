// Re-Implementation of https://bridge.thomasoandrews.com/bridge/impossible/

mod andrews;
mod helper;
mod pavlicek;

pub use andrews::*;
pub use pavlicek::*;

pub const N_PAGES: u128 = 53644737765488792839237440000;

#[cfg(test)]
mod test {

    use crate::impossible_book::andrews;
    use crate::primitives::{Deal, Hand};
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case(1, "S:AKQJT98765432", "H:AKQJT98765432", "D:AKQJT98765432", "C:AKQJT98765432")]
    #[test_case(
        10,
        "S:AKQJT98765432",
        "H:AKQJT98765432",
        "D:AKQJ98765432, C:A",
        "C:KQJT98765432, D:T"
    )]
    #[test_case(
        1000000000000000000000000000,
        "♠:KT862,♥:J62,♦:9632,♣:A",
        "♠:A93,♥:K94,♦:J4,♣:K9765",
        "♠:4,♥:A5,♦:AKT75,♣:QJT43",
        "♠:QJ75,♥:QT873,♦:Q8,♣:82"
    )]
    fn andrews_book(page: u128, north: &str, east: &str, south: &str, west: &str) {
        let north = Hand::<13>::from_str(north).unwrap();
        let east = Hand::<13>::from_str(east).unwrap();
        let south = Hand::<13>::from_str(south).unwrap();
        let west = Hand::<13>::from_str(west).unwrap();

        let expected = Deal::from_hands([north, east, south, west]);

        let deal = andrews::deal_from_human_andrews_page(page);

        let constructed_page = andrews::find_human_page_number_for_deal_in_andrews_book(deal);

        assert_eq!(deal, expected);
        assert_eq!(constructed_page, page);
    }

    #[test_case(1, "S:AKQJT98765432", "H:AKQJT98765432", "D:AKQJT98765432", "C:AKQJT98765432")]
    #[test_case(
        10,
        "S:AKQJT98765432",
        "H:AKQJT98765432",
        "D:AKQJT9876543, C:6",
        "C:AKQJT9875432, D:2"
    )]
    #[test_case(
        1000000000000000000000000000,
        "♠:AK72,♥:Q543,♦:T863,♣:8",
        "♠:QJ84,♥:KT762,♦:AK4,♣:7",
        "♠:T953,♥:9,♦:972,♣:QJ643",
        "♠:6,♥:AJ8,♦:QJ5,♣:AKT952"
    )]
    fn pavliceks_book(page: u128, north: &str, east: &str, south: &str, west: &str) {
        let north = Hand::<13>::from_str(north).unwrap();
        let east = Hand::<13>::from_str(east).unwrap();
        let south = Hand::<13>::from_str(south).unwrap();
        let west = Hand::<13>::from_str(west).unwrap();

        let expected = Deal::from_hands([north, east, south, west]);

        let deal = super::deal_from_human_pavlicek_page(page);

        let constructed_page = super::find_human_page_number_for_deal_in_pavliceks_book(deal);

        assert_eq!(deal, expected);
        assert_eq!(constructed_page, page);
    }
}
