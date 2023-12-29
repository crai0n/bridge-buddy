use crate::engine::subjective_game_view::subjective_vulnerability::SubjectiveVulnerability;
use crate::engine::subjective_game_view::subjectiviser::Subjectiviser;
use crate::error::BBError;
use crate::game::game_data::{Bidding, CardPlay, Ended, NextToPlay};
use crate::game::game_data::{GameData, OpeningLead, WaitingForDummy};
use crate::game::GameState;
use crate::primitives::bid::{Bid, ContractBid};
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Hand, Suit};

use crate::primitives::game_event::CardEvent;
use crate::primitives::trick::Trick;
pub use subjective_seat::SubjectiveSeat;

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
    pub fn next_to_play(&self) -> Option<SubjectiveSeat> {
        match &self {
            SubjectiveGameStateView::Bidding(state) => Some(state.next_to_play()),
            SubjectiveGameStateView::OpeningLead(state) => Some(state.next_to_play()),
            SubjectiveGameStateView::WaitingForDummy(state) => Some(state.next_to_play()),
            SubjectiveGameStateView::CardPlay(state) => Some(state.next_to_play()),
            SubjectiveGameStateView::Ended(_) => None,
        }
    }
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

    pub fn my_starting_hand(&self) -> Result<Hand, BBError> {
        match self {
            Self::Bidding(data) => data.my_starting_hand(),
            Self::OpeningLead(data) => data.my_starting_hand(),
            Self::WaitingForDummy(data) => data.my_starting_hand(),
            Self::CardPlay(data) => data.my_starting_hand(),
            Self::Ended(data) => data.my_starting_hand(),
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

impl<'a, T> SubjectiveGameDataView<'a, T>
where
    GameData<T>: NextToPlay,
{
    pub fn next_to_play(&self) -> SubjectiveSeat {
        let next = self.game_data.next_to_play();
        self.subjectiviser.subjective_seat(next)
    }
}

impl<'a> SubjectiveGameDataView<'a, Bidding> {
    pub fn new_bidding(game_data: &'a GameData<Bidding>, seat: Seat) -> Self {
        Self {
            seat,
            subjectiviser: Subjectiviser::new(seat),
            game_data,
        }
    }

    pub fn bidding_string(&self) -> String {
        format!("{}", self.game_data.inner.bid_manager)
    }

    pub fn validate_bid(&self, bid: Bid) -> Result<(), BBError> {
        self.game_data.inner.bid_manager.validate_bid(bid)
    }

    pub fn my_starting_hand(&self) -> Result<Hand, BBError> {
        self.game_data.hand_of(self.seat)
    }

    pub fn my_remaining_cards(&self) -> Vec<Card> {
        self.game_data.inner.hand_manager.known_remaining_cards_of(self.seat)
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

    pub fn last_contract_bid(&self) -> Option<ContractBid> {
        self.game_data.inner.bid_manager.last_contract_bid().copied()
    }

    pub fn lowest_available_contract_bid(&self) -> Option<ContractBid> {
        self.game_data.inner.bid_manager.lowest_available_contract_bid()
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

    pub fn my_starting_hand(&self) -> Result<Hand, BBError> {
        self.game_data.hand_of(self.seat)
    }

    pub fn my_remaining_cards(&self) -> Vec<Card> {
        self.game_data.inner.hand_manager.known_remaining_cards_of(self.seat)
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

    pub fn validate_lead(&self, card: Card) -> Result<(), BBError> {
        let card_play_event = CardEvent {
            player: self.seat,
            card,
        };
        self.game_data.validate_play_card_event(card_play_event)
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

    pub fn my_starting_hand(&self) -> Result<Hand, BBError> {
        self.game_data.hand_of(self.seat)
    }

    pub fn my_remaining_cards(&self) -> Vec<Card> {
        self.game_data.inner.hand_manager.known_remaining_cards_of(self.seat)
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

    pub fn suit_to_follow(&self) -> Option<Suit> {
        self.game_data.inner.trick_manager.suit_to_follow()
    }

    pub fn my_starting_hand(&self) -> Result<Hand, BBError> {
        self.game_data.hand_of(self.seat)
    }

    pub fn my_remaining_cards(&self) -> Vec<Card> {
        self.game_data.inner.hand_manager.known_remaining_cards_of(self.seat)
    }

    pub fn validate_card_play(&self, card: Card, seat: SubjectiveSeat) -> Result<(), BBError> {
        let seat = self.subjectiviser.objective_seat(seat);
        let card_play_event = CardEvent { player: seat, card };
        self.game_data.validate_play_card_event(card_play_event)
    }

    pub fn dummys_starting_hand(&self) -> Result<Hand, BBError> {
        self.game_data.hand_of(self.game_data.declarer().partner())
    }

    pub fn dummys_remaining_cards(&self) -> Vec<Card> {
        self.game_data
            .inner
            .hand_manager
            .known_remaining_cards_of(self.game_data.declarer().partner())
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

    pub fn trick_string(&self) -> String {
        match self.game_data.inner.trick_manager.current_trick() {
            None => "You are leading to the next trick!".to_string(),
            Some(active_trick) => {
                let lead = self.subjectiviser.subjective_seat(active_trick.lead());

                let first_line;
                let second_line;

                match lead {
                    SubjectiveSeat::LeftHandOpponent => {
                        let [c1, c2, c3] = active_trick.cards() else {
                            unreachable!()
                        };
                        first_line = format!("  {}  \n", c2);
                        second_line = format!("{}  {}\n", c1, c3);
                    }
                    SubjectiveSeat::Partner => {
                        let [c1, c2] = active_trick.cards() else { unreachable!() };
                        first_line = format!("  {}  \n", c1);
                        second_line = format!("[]  {}\n", c2);
                    }
                    SubjectiveSeat::RightHandOpponent => {
                        let [c1] = active_trick.cards() else { unreachable!() };
                        first_line = "  []  \n".to_string();
                        second_line = format!("[]  {}\n", c1);
                    }
                    SubjectiveSeat::Myself => unreachable!(),
                };
                first_line + &second_line
            }
        }
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

    pub fn my_starting_hand(&self) -> Result<Hand, BBError> {
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
