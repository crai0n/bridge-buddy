pub use player_position::PlayerPosition;
use rand::random;
pub use vulnerability::Vulnerability;

pub mod player_position;
pub mod vulnerability;

pub struct Board {
    number: usize,
}

impl Board {
    const MAX_NUMBER: usize = 64;

    pub fn new() -> Self {
        Board::from_number(random())
    }

    pub fn from_number(num: usize) -> Self {
        match num {
            0 => Board { number: 64 },
            _ => Board {
                number: (num - 1) % Board::MAX_NUMBER + 1,
            },
        }
    }

    pub const fn vulnerable(&self) -> Vulnerability {
        let v = self.number - 1;
        let vul = v + v / 4;
        match vul % 4 {
            0 => Vulnerability::None,
            1 => Vulnerability::NorthSouth,
            2 => Vulnerability::EastWest,
            _ => Vulnerability::All,
        }
    }

    pub const fn dealer(&self) -> PlayerPosition {
        match (self.number - 1) % 4 {
            0 => PlayerPosition::North,
            1 => PlayerPosition::East,
            2 => PlayerPosition::South,
            _ => PlayerPosition::West,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

#[cfg(test)]
mod test {
    use super::Board;
    use crate::primitives::board::vulnerability::Vulnerability;
    use crate::primitives::board::vulnerability::Vulnerability::*;
    use crate::primitives::board::PlayerPosition;
    use crate::primitives::board::PlayerPosition::*;
    use test_case::test_case;

    #[test_case(0, EastWest, West)]
    #[test_case(1, None, North)]
    #[test_case(2, NorthSouth, East)]
    #[test_case(3, EastWest, South)]
    #[test_case(4, All, West)]
    #[test_case(5, NorthSouth, North)]
    #[test_case(6, EastWest, East)]
    #[test_case(7, All, South)]
    #[test_case(8, None, West)]
    #[test_case(9, EastWest, North)]
    #[test_case(10, All, East)]
    #[test_case(11, None, South)]
    #[test_case(12, NorthSouth, West)]
    #[test_case(13, All, North)]
    #[test_case(14, None, East)]
    #[test_case(15, NorthSouth, South)]
    #[test_case(16, EastWest, West)]
    #[test_case(17, None, North)]
    #[test_case(18, NorthSouth, East)]
    fn construction(number: usize, vulnerable: Vulnerability, dealer: PlayerPosition) {
        let deal = Board::from_number(number);
        assert_eq!(deal.dealer(), dealer);
        assert_eq!(deal.vulnerable(), vulnerable);
    }
}
