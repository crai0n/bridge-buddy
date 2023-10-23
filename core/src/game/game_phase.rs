use crate::error::BBError;
use crate::primitives::bid_line::BidLine;
use crate::primitives::contract::ContractDenomination;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::trick::{PlayedTrick, TrickManager};
use crate::primitives::{Card, Contract, Deal};

pub struct GamePhase<Phase> {
    phase: Phase,
    deal: Deal,
}

pub struct Setup {}

impl GamePhase<Setup> {
    pub fn new(deal: Deal) -> Self {
        GamePhase { deal, phase: Setup {} }
    }
    pub fn start(self) -> GamePhase<Bidding> {
        GamePhase {
            phase: Bidding {
                bid_line: BidLine::new(),
                current_turn: self.deal.dealer(),
            },
            deal: self.deal,
        }
    }
}

pub struct Bidding {
    bid_line: BidLine,
    current_turn: PlayerPosition,
}

impl GamePhase<Bidding> {
    pub fn bid(&mut self) -> Self {
        todo!()
    }

    fn can_bid() -> bool {
        todo!()
    }

    pub fn contract_is_final(&self) -> bool {
        self.phase.bid_line.contract_is_final()
    }

    pub fn implied_contract(&self) -> Option<Contract> {
        self.phase.bid_line.implied_contract()
    }

    pub fn implied_declarer(&self) -> Option<PlayerPosition> {
        todo!()
    }

    pub fn end_bidding(self) -> GamePhase<OpeningLead> {
        GamePhase {
            phase: OpeningLead {
                declarer: self.implied_declarer().unwrap(),
                contract: self.implied_contract().unwrap(),
                bid_line: self.phase.bid_line,
            },
            deal: self.deal,
        }
    }
}

pub struct OpeningLead {
    bid_line: BidLine,
    contract: Contract,
    declarer: PlayerPosition,
}

impl GamePhase<OpeningLead> {
    pub fn check_lead(self, card: Card, player: PlayerPosition) -> Result<(), BBError> {
        if player != self.phase.declarer + 1 {
            Err(BBError::OutOfTurn)
        } else if !self.deal.hand(player).contains(&card) {
            Err(BBError::WrongCard)
        } else {
            Ok(())
        }
    }
    pub fn lead(self, opening_lead: Card) -> GamePhase<CardPlay> {
        let trump_suit = match self.phase.contract.denomination {
            ContractDenomination::NoTrump => None,
            ContractDenomination::Trump(suit) => Some(suit),
        };
        let mut trick_manager = TrickManager::new(self.phase.declarer + 1, trump_suit);
        trick_manager.play(opening_lead);
        GamePhase {
            phase: CardPlay {
                bid_line: self.phase.bid_line,
                opening_lead,
                contract: self.phase.contract,
                declarer: self.phase.declarer,
                tricks: trick_manager,
            },
            deal: self.deal,
        }
    }
}

pub struct CardPlay {
    bid_line: BidLine,
    contract: Contract,
    opening_lead: Card,
    declarer: PlayerPosition,
    tricks: TrickManager,
}

impl GamePhase<CardPlay> {
    pub fn check_play(self, card: Card, player: PlayerPosition) -> Result<(), BBError> {
        if player != self.phase.tricks.turn() {
            Err(BBError::OutOfTurn)
        } else if !self.deal.hand(player).contains(&card) {
            Err(BBError::WrongCard)
        } else {
            Ok(())
        }
    }

    pub fn play_card(mut self, card: &Card) -> Self {
        self.phase.tricks.play(*card);
        self
    }

    pub fn card_play_has_ended(&self) -> bool {
        self.phase.tricks.count_played_tricks() == 13
    }

    pub fn end_play(self) -> GamePhase<Ended> {
        GamePhase {
            phase: Ended {
                bid_line: self.phase.bid_line,
                opening_lead: self.phase.opening_lead,
                contract: self.phase.contract,
                declarer: self.phase.declarer,
                played_tricks: self.phase.tricks.tricks(),
            },
            deal: self.deal,
        }
    }
}

pub struct Ended {
    bid_line: BidLine,
    contract: Contract,
    declarer: PlayerPosition,
    opening_lead: Card,
    played_tricks: Vec<PlayedTrick>,
}

impl GamePhase<Ended> {
    pub fn get_score(&self) -> usize {
        todo!()
    }
}
