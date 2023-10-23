use crate::primitives::deal::board_vulnerability::BoardVulnerability;
use crate::primitives::deal::player_position::PlayerPosition;
use rand::prelude::*;

pub struct Board {
    pub number: usize,
}

impl Board {
    pub const MAX_NUMBER: usize = 32;

    pub fn new() -> Self {
        let mut rng = thread_rng();
        Self::from_rng(&mut rng)
    }

    pub fn from_rng(rng: &mut impl Rng) -> Self {
        let num = rng.gen_range(1..=Self::MAX_NUMBER);
        Self::from_number(num)
    }

    pub fn from_number(num: usize) -> Self {
        let number = match num {
            0 => Self::MAX_NUMBER,
            1..=Self::MAX_NUMBER => num,
            _ => (num - 1) % Self::MAX_NUMBER + 1,
        };
        Board { number }
    }

    pub fn vulnerable(&self) -> BoardVulnerability {
        let v = self.number - 1;
        let vul = v + v / 4;
        match vul % 4 {
            0 => BoardVulnerability::None,
            1 => BoardVulnerability::NorthSouth,
            2 => BoardVulnerability::EastWest,
            _ => BoardVulnerability::All,
        }
    }

    pub fn is_vulnerable(&self, player: PlayerPosition) -> bool {
        match self.vulnerable() {
            BoardVulnerability::None => false,
            BoardVulnerability::All => true,
            BoardVulnerability::EastWest => matches!(player, PlayerPosition::East | PlayerPosition::West),
            BoardVulnerability::NorthSouth => matches!(player, PlayerPosition::North | PlayerPosition::South),
        }
    }

    pub fn dealer(&self) -> PlayerPosition {
        match (self.number - 1) % 4 {
            0 => PlayerPosition::North,
            1 => PlayerPosition::East,
            2 => PlayerPosition::South,
            _ => PlayerPosition::West,
        }
    }

    pub fn number(&self) -> usize {
        self.number
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
    use crate::primitives::deal::board::PlayerPosition;
    use crate::primitives::deal::board::PlayerPosition::*;
    use crate::primitives::deal::board_vulnerability::BoardVulnerability;
    use crate::primitives::deal::board_vulnerability::BoardVulnerability::*;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;
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
    fn construction(number: usize, vulnerable: BoardVulnerability, dealer: PlayerPosition) {
        let deal = Board::from_number(number);
        assert_eq!(deal.dealer(), dealer);
        assert_eq!(deal.vulnerable(), vulnerable);
    }

    #[test_case( 1u64,  20; "Test A")]
    #[test_case( 2u64,  29; "Test B")]
    #[test_case( 3u64,  11; "Test C")]
    #[test_case( 4u64,  24; "Test D")]
    #[test_case( 5u64,   7; "Test E")]
    #[test_case( 6u64,  14; "Test F")]
    #[test_case( 7u64,   6; "Test G")]
    #[test_case( 8u64,   3; "Test H")]
    #[test_case( 9u64,  26; "Test I")]
    #[test_case(10u64,   1; "Test Range Beginning")]
    #[test_case(35u64,  32; "Test Range End")]
    #[test_case( 1234567890123456789u64, 31; "Test 1")]
    #[test_case( 9274615494946216468u64,  6; "Test 2")]
    #[test_case(10284072810178401816u64, 22; "Test 3")]
    #[test_case( 3756139473478105616u64,  3; "Test 4")]
    #[test_case( 9375569024856384856u64, 24; "Test 5")]
    #[test_case( 1294661341901337513u64,  1; "Test 6")]
    #[test_case(18446744073709551615u64, 13; "Test Max")]
    fn determinism(seed: u64, expected: usize) {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let board = Board::from_rng(&mut rng);
        assert_eq!(board.number(), expected);
    }
}
