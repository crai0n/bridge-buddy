use rand::random;

pub struct Deal {
    deal_number: u8,
    vulnerable: Vulnerable,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Vulnerable {
    None,
    NorthSouth,
    EastWest,
    All
}


impl Deal {
    pub fn new() -> Deal {
        let deal_number = (random::<u8>() % 32) + 1;
        Self::new_from_number(deal_number)
    }

    pub fn new_from_number(deal_number: u8) -> Deal {
        let v = deal_number - 1;
        let vul = v + v / 4 ;
        let vulnerable = match vul % 4 {
            0 => Vulnerable::None,
            1 => Vulnerable::NorthSouth,
            2 => Vulnerable::EastWest,
            _ => Vulnerable::All,
        };
        Deal {deal_number, vulnerable}
    }
}

#[cfg(test)]
mod tests {
    use super::Deal;
    use super::Vulnerable;

    #[test]
    fn test_vulnerability() {
        // Pattern follows the "BONE"-chart
        assert_eq!(Deal::new_from_number(1).vulnerable, Vulnerable::None);
        assert_eq!(Deal::new_from_number(2).vulnerable, Vulnerable::NorthSouth);
        assert_eq!(Deal::new_from_number(3).vulnerable, Vulnerable::EastWest);
        assert_eq!(Deal::new_from_number(4).vulnerable, Vulnerable::All);

        assert_eq!(Deal::new_from_number(5).vulnerable, Vulnerable::NorthSouth);
        assert_eq!(Deal::new_from_number(6).vulnerable, Vulnerable::EastWest);
        assert_eq!(Deal::new_from_number(7).vulnerable, Vulnerable::All);
        assert_eq!(Deal::new_from_number(8).vulnerable, Vulnerable::None);

        assert_eq!(Deal::new_from_number(9).vulnerable, Vulnerable::EastWest);
        assert_eq!(Deal::new_from_number(10).vulnerable, Vulnerable::All);
        assert_eq!(Deal::new_from_number(11).vulnerable, Vulnerable::None);
        assert_eq!(Deal::new_from_number(12).vulnerable, Vulnerable::NorthSouth);

        assert_eq!(Deal::new_from_number(13).vulnerable, Vulnerable::All);
        assert_eq!(Deal::new_from_number(14).vulnerable, Vulnerable::None);
        assert_eq!(Deal::new_from_number(15).vulnerable, Vulnerable::NorthSouth);
        assert_eq!(Deal::new_from_number(16).vulnerable, Vulnerable::EastWest);
        // Pattern repeats after 16 hands
        assert_eq!(Deal::new_from_number(17).vulnerable, Vulnerable::None);
        assert_eq!(Deal::new_from_number(18).vulnerable, Vulnerable::NorthSouth);
    }

}