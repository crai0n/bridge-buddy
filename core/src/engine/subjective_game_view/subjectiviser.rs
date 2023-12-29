use crate::engine::subjective_game_view::subjective_axis::SubjectiveAxis;
use crate::engine::subjective_game_view::subjective_seat::SubjectiveSeat;
use crate::engine::subjective_game_view::subjective_vulnerability::SubjectiveVulnerability;
use crate::primitives::deal::axis::Axis;
use crate::primitives::deal::{Seat, Vulnerability};

pub struct Subjectiviser {
    seat: Seat,
}

impl Subjectiviser {
    pub fn new(seat: Seat) -> Self {
        Self { seat }
    }

    pub const fn subjective_seat(&self, seat: Seat) -> SubjectiveSeat {
        match (4 + seat as usize - self.seat as usize) % 4 {
            0 => SubjectiveSeat::Myself,
            1 => SubjectiveSeat::LeftHandOpponent,
            2 => SubjectiveSeat::Partner,
            3 => SubjectiveSeat::RightHandOpponent,
            _ => unreachable!(),
        }
    }

    pub const fn objective_seat(&self, seat: SubjectiveSeat) -> Seat {
        match (seat as usize + self.seat as usize) % 4 {
            0 => Seat::North,
            1 => Seat::East,
            2 => Seat::South,
            3 => Seat::West,
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub const fn subjective_axis(&self, axis: Axis) -> SubjectiveAxis {
        match (self.seat, axis) {
            (Seat::North | Seat::South, Axis::NorthSouth) => SubjectiveAxis::Us,
            (Seat::East | Seat::West, Axis::EastWest) => SubjectiveAxis::Us,
            _ => SubjectiveAxis::Them,
        }
    }

    pub const fn subjective_vulnerability(&self, vulnerability: Vulnerability) -> SubjectiveVulnerability {
        match (vulnerability, self.seat) {
            (Vulnerability::All, _) => SubjectiveVulnerability::All,
            (Vulnerability::None, _) => SubjectiveVulnerability::None,
            (Vulnerability::NorthSouth, Seat::North | Seat::South) => SubjectiveVulnerability::Us,
            (Vulnerability::NorthSouth, _) => SubjectiveVulnerability::Them,
            (Vulnerability::EastWest, Seat::East | Seat::West) => SubjectiveVulnerability::Us,
            (Vulnerability::EastWest, _) => SubjectiveVulnerability::Them,
        }
    }
}
