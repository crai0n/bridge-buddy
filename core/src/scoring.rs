use crate::primitives::contract::{ContractDenomination, ContractLevel, ContractState};
use crate::primitives::deal::{Seat, Vulnerability};
use crate::primitives::game_result::GameResult;
use crate::primitives::Contract;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Copy, Debug)]
pub struct ScorePoints(isize);

impl Add<ScorePoints> for ScorePoints {
    type Output = ScorePoints;

    fn add(self, rhs: ScorePoints) -> ScorePoints {
        ScorePoints(self.0 + rhs.0)
    }
}

impl AddAssign<ScorePoints> for ScorePoints {
    fn add_assign(&mut self, rhs: ScorePoints) {
        *self = ScorePoints(self.0 + rhs.0);
    }
}

impl Mul<isize> for ScorePoints {
    type Output = ScorePoints;

    fn mul(self, rhs: isize) -> ScorePoints {
        ScorePoints(self.0 * rhs)
    }
}

impl Mul<usize> for ScorePoints {
    type Output = ScorePoints;

    fn mul(self, rhs: usize) -> ScorePoints {
        ScorePoints(self.0 * rhs as isize)
    }
}

impl MulAssign<isize> for ScorePoints {
    fn mul_assign(&mut self, rhs: isize) {
        *self = ScorePoints(self.0 * rhs)
    }
}

pub struct ScoreCalculator;
impl ScoreCalculator {
    pub const NO_SCORE: ScorePoints = ScorePoints(0);
    const MINOR_TRICK_POINTS: ScorePoints = ScorePoints(20);
    const MAJOR_TRICK_POINTS: ScorePoints = ScorePoints(30);
    const NO_TRUMP_EXTRA_TRICK_POINTS: ScorePoints = ScorePoints(10);
    const PART_SCORE_BONUS: ScorePoints = ScorePoints(50);
    const GAME_THRESHOLD: ScorePoints = ScorePoints(100);
    const GAME_BONUS_VULNERABLE: ScorePoints = ScorePoints(500);
    const GAME_BONUS_NOT_VULNERABLE: ScorePoints = ScorePoints(300);
    const SLAM_BONUS_VULNERABLE: ScorePoints = ScorePoints(750);
    const SLAM_BONUS_NOT_VULNERABLE: ScorePoints = ScorePoints(500);
    const GRAND_SLAM_BONUS_VULNERABLE: ScorePoints = ScorePoints(1500);
    const GRAND_SLAM_BONUS_NOT_VULNERABLE: ScorePoints = ScorePoints(1000);
    const FOR_INSULT: ScorePoints = ScorePoints(50);

    pub fn score_result(result: GameResult, vulnerability: Vulnerability) -> ScorePoints {
        match result {
            GameResult::Failed { contract, undertricks } => Self::score_lose(contract, undertricks, vulnerability),
            GameResult::Made { contract, overtricks } => Self::score_win(contract, overtricks, vulnerability),
            GameResult::Unplayed => Self::NO_SCORE,
        }
    }

    fn score_lose(contract: Contract, undertricks: usize, vulnerability: Vulnerability) -> ScorePoints {
        let declarer_is_vulnerable = contract.declarer.is_vulnerable(vulnerability);

        let mut score = match contract.state {
            ContractState::Passed => Self::score_lose_passed(undertricks, declarer_is_vulnerable),
            ContractState::Doubled => Self::score_lose_doubled(undertricks, declarer_is_vulnerable),
            ContractState::Redoubled => Self::score_lose_doubled(undertricks, declarer_is_vulnerable) * 2_usize,
        };
        if contract.declarer == Seat::East || contract.declarer == Seat::West {
            score *= -1_isize;
        }
        score
    }

    fn score_lose_passed(undertricks: usize, declarer_is_vulnerable: bool) -> ScorePoints {
        if declarer_is_vulnerable {
            ScorePoints(-100) * undertricks
        } else {
            ScorePoints(-50) * undertricks
        }
    }

    fn score_lose_doubled(undertricks: usize, declarer_is_vulnerable: bool) -> ScorePoints {
        if declarer_is_vulnerable {
            ScorePoints(-300) * (undertricks - 1) + ScorePoints(-200)
        } else {
            match undertricks {
                1 => ScorePoints(-100),
                2 => ScorePoints(-300),
                3 => ScorePoints(-500),
                n => ScorePoints(-300) * (n - 3) + ScorePoints(-500),
            }
        }
    }

    fn score_win(contract: Contract, overtricks: usize, vulnerability: Vulnerability) -> ScorePoints {
        let declarer_is_vulnerable = contract.declarer.is_vulnerable(vulnerability);

        let mut score = Self::score_bid_tricks(contract);

        score += Self::score_game_bonus(score, declarer_is_vulnerable);

        score += Self::score_slam_bonus(contract, declarer_is_vulnerable);

        score += Self::score_overtricks(contract, overtricks);

        score += Self::score_insult(contract);

        if contract.declarer == Seat::East || contract.declarer == Seat::West {
            score *= -1_isize;
        }
        score
    }

    fn score_game_bonus(trick_score: ScorePoints, declarer_is_vulnerable: bool) -> ScorePoints {
        if trick_score >= Self::GAME_THRESHOLD {
            if declarer_is_vulnerable {
                Self::GAME_BONUS_VULNERABLE
            } else {
                Self::GAME_BONUS_NOT_VULNERABLE
            }
        } else {
            Self::PART_SCORE_BONUS
        }
    }

    fn score_insult(contract: Contract) -> ScorePoints {
        match contract.state {
            ContractState::Passed => ScorePoints(0),
            ContractState::Doubled => Self::FOR_INSULT,
            ContractState::Redoubled => Self::FOR_INSULT * 2_usize,
        }
    }

    fn score_slam_bonus(contract: Contract, declarer_is_vulnerable: bool) -> ScorePoints {
        if contract.level == ContractLevel::Six {
            if declarer_is_vulnerable {
                Self::SLAM_BONUS_VULNERABLE
            } else {
                Self::SLAM_BONUS_NOT_VULNERABLE
            }
        } else if contract.level == ContractLevel::Seven {
            if declarer_is_vulnerable {
                Self::GRAND_SLAM_BONUS_VULNERABLE
            } else {
                Self::GRAND_SLAM_BONUS_NOT_VULNERABLE
            }
        } else {
            Self::NO_SCORE
        }
    }

    fn score_bid_tricks(contract: Contract) -> ScorePoints {
        let trick_score = match contract.denomination {
            ContractDenomination::NoTrump => {
                Self::MAJOR_TRICK_POINTS * contract.level as isize + Self::NO_TRUMP_EXTRA_TRICK_POINTS
            }
            ContractDenomination::Trump(suit) if suit.is_major() => Self::MAJOR_TRICK_POINTS * contract.level as isize,
            ContractDenomination::Trump(_) => Self::MINOR_TRICK_POINTS * contract.level as isize,
        };
        match contract.state {
            ContractState::Passed => trick_score,
            ContractState::Doubled => trick_score * 2_isize,
            ContractState::Redoubled => trick_score * 4_isize,
        }
    }

    fn score_overtricks(contract: Contract, overtricks: usize) -> ScorePoints {
        match contract.denomination {
            ContractDenomination::Trump(suit) if suit.is_minor() => Self::MINOR_TRICK_POINTS * overtricks,
            _ => Self::MAJOR_TRICK_POINTS * overtricks,
        }
    }
}

#[cfg(test)]
mod test {
    use super::ScoreCalculator;
    use super::ScorePoints;
    use crate::primitives::deal::Vulnerability;
    use crate::primitives::deal::Vulnerability::*;
    use crate::primitives::game_result::GameResult;
    use crate::primitives::Contract;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("E1H", 9,  None, -140; "Board 1")]
    #[test_case("N4Sx", 8,  NorthSouth, -500; "Board 2")]
    #[test_case("N2D", 10,  EastWest, 130; "Board 3")]
    #[test_case("W1NT", 9,  All, -150; "Board 4")]
    #[test_case("S3NT", 8,  NorthSouth, -100; "Board 5")]
    #[test_case("S3H", 10,  EastWest, 170; "Board 6")]
    #[test_case("W3C", 9,  All, -110; "Board 7")]
    #[test_case("E7H", 13,  None, -1510; "Board 8")]
    #[test_case("N4Sx", 10,  EastWest, 590; "Board 9")]
    #[test_case("E2NT", 8,  All, -120; "Board 10")]
    #[test_case("N6C", 12,  None, 920; "Board 11")]
    #[test_case("E2D", 9,  NorthSouth, -110; "Board 12")]
    #[test_case("W4HXX", 10,  All, -1080; "Board 13")]
    #[test_case("S5S", 10,  None, -50; "Board 14")]
    #[test_case("E4H", 11,  NorthSouth, -450; "Board 15")]
    #[test_case("N3NT", 9, EastWest, 400; "Board 16")]
    fn score(contract_string: &str, actual_tricks: usize, vulnerability: Vulnerability, expected: isize) {
        let contract = Contract::from_str(contract_string).unwrap();

        let result = GameResult::calculate_game_result(contract, actual_tricks);

        let score = ScoreCalculator::score_result(result, vulnerability);
        assert_eq!(score, ScorePoints(expected));
    }
}
