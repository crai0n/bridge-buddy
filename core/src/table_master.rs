use crate::primitives::bid_line::BidLine;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::trick::{PlayedTrick, TrickManager};
use crate::primitives::{Card, Contract, Deal};

pub struct Game<Phase> {
    phase: Phase,
    deal: Deal,
}

struct Setup {}
impl Game<Setup> {
    fn new(deal: Deal) -> Self {
        Game {
            deal,
            phase: Setup::new(),
        }
    }
}

impl Setup {
    fn new() -> Self {
        Setup {}
    }
}

struct BiddingPhase {
    bid_line: BidLine,
    current_turn: PlayerPosition,
    declarer: Option<PlayerPosition>,
    contract: Option<Contract>,
}

impl Game<Setup> {
    fn start(self) -> Game<BiddingPhase> {
        Game {
            phase: BiddingPhase {
                bid_line: BidLine::new(),
                current_turn: self.deal.dealer(),
                declarer: None,
                contract: None,
            },
            deal: self.deal,
        }
    }
}

struct OpeningLeadPhase {
    bid_line: BidLine,
    contract: Contract,
    declarer: PlayerPosition,
}
impl Game<BiddingPhase> {
    fn end_bidding(self, declarer: PlayerPosition, contract: Contract) -> Game<OpeningLeadPhase> {
        Game {
            phase: OpeningLeadPhase {
                bid_line: self.phase.bid_line,
                declarer,
                contract,
            },
            deal: self.deal,
        }
    }
}

struct CardPlayPhase {
    bid_line: BidLine,
    contract: Contract,
    opening_lead: Card,
    declarer: PlayerPosition,
    current_turn: PlayerPosition,
    current_trick: TrickManager,
    played_tricks: Vec<PlayedTrick>,
}

impl Game<OpeningLeadPhase> {
    fn lead(self, opening_lead: Card) -> Game<CardPlayPhase> {
        let first_trick = TrickManager::new(self.phase.declarer + 1);
        Game {
            phase: CardPlayPhase {
                bid_line: self.phase.bid_line,
                current_turn: self.phase.declarer + 2,
                opening_lead,
                contract: self.phase.contract,
                declarer: self.phase.declarer,
                current_trick: first_trick,
                played_tricks: vec![],
            },
            deal: self.deal,
        }
    }
}

struct EndedPhase {
    bid_line: BidLine,
    contract: Contract,
    declarer: PlayerPosition,
    opening_lead: Card,
    played_tricks: Vec<PlayedTrick>,
}

impl Game<CardPlayPhase> {
    fn end(self) -> Game<EndedPhase> {
        Game {
            phase: EndedPhase {
                bid_line: self.phase.bid_line,
                opening_lead: self.phase.opening_lead,
                contract: self.phase.contract,
                declarer: self.phase.declarer,
                played_tricks: self.phase.played_tricks,
            },
            deal: self.deal,
        }
    }
}

enum GameWrapper {
    Setup(Game<Setup>),
    Bidding(Game<BiddingPhase>),
    OpeningLead(Game<OpeningLeadPhase>),
    CardPlay(Game<CardPlayPhase>),
    Ended(Game<EndedPhase>),
}
