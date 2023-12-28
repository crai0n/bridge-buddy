use crate::engine::subjective_game_view::subjective_vulnerability::SubjectiveVulnerability;
use crate::engine::subjective_game_view::subjectiviser::Subjectiviser;
use crate::error::BBError;
use crate::game::game_data::{Bidding, CardPlay, Ended, NextToPlay};
use crate::game::game_data::{GameData, OpeningLead, WaitingForDummy};
use crate::game::GameState;
use crate::primitives::deal::Seat;
use crate::primitives::Hand;
use subjective_seat::SubjectiveSeat;

mod subjective_axis;
mod subjective_seat;
mod subjective_vulnerability;
mod subjectiviser;

pub enum SubjectiveGameStateView<'a> {
    Bidding(SubjectiveGameDataView<'a, Bidding>),
    OpeningLead(SubjectiveGameDataView<'a, OpeningLead>),
    WaitingForDummy(SubjectiveGameDataView<'a, WaitingForDummy>),
    CardPlay(SubjectiveGameDataView<'a, CardPlay>),
    Ended(SubjectiveGameDataView<'a, Ended>),
}

impl<'a> SubjectiveGameStateView<'a> {
    pub fn new(game_state: &'a GameState, seat: Seat) -> Self {
        match game_state {
            GameState::Bidding(data) => Self::Bidding(SubjectiveGameDataView::new_bidding(data, seat)),
            GameState::OpeningLead(data) => Self::OpeningLead(SubjectiveGameDataView::new_opening_lead(data, seat)),
            GameState::WaitingForDummy(data) => {
                Self::WaitingForDummy(SubjectiveGameDataView::new_waiting_for_dummy(data, seat))
            }
            GameState::CardPlay(data) => Self::CardPlay(SubjectiveGameDataView::new_card_play(data, seat)),
            GameState::Ended(data) => Self::Ended(SubjectiveGameDataView::new_ended(data, seat)),
        }
    }

    pub fn my_hand(&self) -> Result<Hand, BBError> {
        match self {
            Self::Bidding(_) => Err(BBError::InsufficientInfo),
            Self::OpeningLead(data) => data.my_hand(),
            Self::WaitingForDummy(data) => data.my_hand(),
            Self::CardPlay(data) => data.my_hand(),
            Self::Ended(data) => data.my_hand(),
        }
    }

    pub fn is_my_turn(&self) -> bool {
        match self {
            Self::Bidding(data) => data.is_my_turn(),
            Self::OpeningLead(data) => data.is_my_turn(),
            Self::WaitingForDummy(data) => data.is_my_turn(),
            Self::CardPlay(data) => data.is_my_turn(),
            Self::Ended(data) => data.is_my_turn(),
        }
    }

    pub fn dealer(&self) -> SubjectiveSeat {
        match &self {
            Self::Bidding(data) => data.dealer(),
            Self::OpeningLead(data) => data.dealer(),
            Self::WaitingForDummy(data) => data.dealer(),
            Self::CardPlay(data) => data.dealer(),
            Self::Ended(data) => data.dealer(),
        }
    }

    pub fn vulnerability(&self) -> SubjectiveVulnerability {
        match &self {
            Self::Bidding(data) => data.vulnerability(),
            Self::OpeningLead(data) => data.vulnerability(),
            Self::WaitingForDummy(data) => data.vulnerability(),
            Self::CardPlay(data) => data.vulnerability(),
            Self::Ended(data) => data.vulnerability(),
        }
    }

    pub fn declarer(&self) -> Option<SubjectiveSeat> {
        match &self {
            Self::Bidding(_) => None,
            Self::OpeningLead(data) => data.declarer(),
            Self::WaitingForDummy(data) => data.declarer(),
            Self::CardPlay(data) => data.declarer(),
            Self::Ended(data) => data.declarer(),
        }
    }
}

pub struct SubjectiveGameDataView<'a, T> {
    seat: Seat,
    subjectiviser: Subjectiviser,
    game_data: &'a GameData<T>,
}

impl<'a> SubjectiveGameDataView<'a, Bidding> {
    pub fn new_bidding(game_data: &'a GameData<Bidding>, seat: Seat) -> Self {
        Self {
            seat,
            subjectiviser: Subjectiviser::new(seat),
            game_data,
        }
    }

    pub fn my_hand(&self) -> Result<Hand, BBError> {
        self.game_data.hand_of(self.seat)
    }

    pub fn declarer(&self) -> Option<SubjectiveSeat> {
        self.game_data.declarer().map(|x| self.subjectiviser.subjective_seat(x))
    }

    pub fn dealer(&self) -> SubjectiveSeat {
        let dealer = self.game_data.board().dealer();
        self.subjectiviser.subjective_seat(dealer)
    }

    pub fn is_my_turn(&self) -> bool {
        self.game_data.next_to_play() == self.seat
    }

    pub fn vulnerability(&self) -> SubjectiveVulnerability {
        let vul = self.game_data.board().vulnerability();
        self.subjectiviser.subjective_vulnerability(vul)
    }
}

impl<'a> SubjectiveGameDataView<'a, OpeningLead> {
    pub fn new_opening_lead(game_data: &'a GameData<OpeningLead>, seat: Seat) -> Self {
        Self {
            seat,
            subjectiviser: Subjectiviser::new(seat),
            game_data,
        }
    }

    pub fn my_hand(&self) -> Result<Hand, BBError> {
        self.game_data.hand_of(self.seat)
    }

    pub fn declarer(&self) -> Option<SubjectiveSeat> {
        let declarer = self.game_data.declarer();
        Some(self.subjectiviser.subjective_seat(declarer))
    }

    pub fn dealer(&self) -> SubjectiveSeat {
        let dealer = self.game_data.board().dealer();
        self.subjectiviser.subjective_seat(dealer)
    }

    pub fn is_my_turn(&self) -> bool {
        self.game_data.next_to_play() == self.seat
    }

    pub fn vulnerability(&self) -> SubjectiveVulnerability {
        let vul = self.game_data.board().vulnerability();
        self.subjectiviser.subjective_vulnerability(vul)
    }
}

impl<'a> SubjectiveGameDataView<'a, WaitingForDummy> {
    pub fn new_waiting_for_dummy(game_data: &'a GameData<WaitingForDummy>, seat: Seat) -> Self {
        Self {
            seat,
            subjectiviser: Subjectiviser::new(seat),
            game_data,
        }
    }

    pub fn my_hand(&self) -> Result<Hand, BBError> {
        self.game_data.hand_of(self.seat)
    }

    pub fn declarer(&self) -> Option<SubjectiveSeat> {
        let declarer = self.game_data.declarer();
        Some(self.subjectiviser.subjective_seat(declarer))
    }

    pub fn dealer(&self) -> SubjectiveSeat {
        let dealer = self.game_data.board().dealer();
        self.subjectiviser.subjective_seat(dealer)
    }

    pub fn is_my_turn(&self) -> bool {
        self.game_data.next_to_play() == self.seat
    }

    pub fn vulnerability(&self) -> SubjectiveVulnerability {
        let vul = self.game_data.board().vulnerability();
        self.subjectiviser.subjective_vulnerability(vul)
    }
}

impl<'a> SubjectiveGameDataView<'a, CardPlay> {
    pub fn new_card_play(game_data: &'a GameData<CardPlay>, seat: Seat) -> Self {
        Self {
            seat,
            subjectiviser: Subjectiviser::new(seat),
            game_data,
        }
    }

    pub fn my_hand(&self) -> Result<Hand, BBError> {
        self.game_data.hand_of(self.seat)
    }

    pub fn declarer(&self) -> Option<SubjectiveSeat> {
        let declarer = self.game_data.declarer();
        Some(self.subjectiviser.subjective_seat(declarer))
    }

    pub fn dealer(&self) -> SubjectiveSeat {
        let dealer = self.game_data.board().dealer();
        self.subjectiviser.subjective_seat(dealer)
    }

    pub fn is_my_turn(&self) -> bool {
        self.game_data.next_to_play() == self.seat
    }

    pub fn vulnerability(&self) -> SubjectiveVulnerability {
        let vul = self.game_data.board().vulnerability();
        self.subjectiviser.subjective_vulnerability(vul)
    }
}

impl<'a> SubjectiveGameDataView<'a, Ended> {
    pub fn new_ended(game_data: &'a GameData<Ended>, seat: Seat) -> Self {
        Self {
            seat,
            subjectiviser: Subjectiviser::new(seat),
            game_data,
        }
    }

    pub fn my_hand(&self) -> Result<Hand, BBError> {
        self.game_data.hand_of(self.seat)
    }

    pub fn declarer(&self) -> Option<SubjectiveSeat> {
        self.game_data.declarer().map(|x| self.subjectiviser.subjective_seat(x))
    }

    pub fn dealer(&self) -> SubjectiveSeat {
        let dealer = self.game_data.board().dealer();
        self.subjectiviser.subjective_seat(dealer)
    }

    pub fn is_my_turn(&self) -> bool {
        false
    }

    pub fn vulnerability(&self) -> SubjectiveVulnerability {
        let vul = self.game_data.board().vulnerability();
        self.subjectiviser.subjective_vulnerability(vul)
    }
}
