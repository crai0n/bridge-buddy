use crate::primitives::bid::Bid;
use crate::primitives::bid_line::BidLine;
use crate::primitives::contract::ContractDenomination;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::trick::{PlayedTrick, TrickManager};
use crate::primitives::{Card, Contract, Deal};

// Together, the different versions of game form a state machine that encapsulates the logic of the game and contains
// all information that immediately follow from the rules of the game.

pub struct Game<Phase> {
    phase: Phase,
    deal: Deal,
}

enum GameWrapper {
    Setup(Game<Setup>),
    Bidding(Game<Bidding>),
    OpeningLead(Game<OpeningLead>),
    CardPlay(Game<CardPlay>),
    Ended(Game<Ended>),
}
struct Setup {}

impl Game<Setup> {
    fn new(deal: Deal) -> Self {
        Game { deal, phase: Setup {} }
    }
    fn start(self) -> Game<Bidding> {
        Game {
            phase: Bidding {
                bid_line: BidLine::new(),
                current_turn: self.deal.dealer(),
            },
            deal: self.deal,
        }
    }
}

struct Bidding {
    bid_line: BidLine,
    current_turn: PlayerPosition,
}

impl Game<Bidding> {
    fn bid() -> Self {
        todo!()
    }

    fn can_bid() -> bool {
        todo!()
    }

    fn contract_is_final(&self) -> bool {
        self.phase.bid_line.contract_is_final()
    }

    pub fn implied_contract(&self) -> Option<Contract> {
        self.phase.bid_line.implied_contract()
    }

    fn implied_declarer(&self) -> Option<PlayerPosition> {
        todo!()
    }

    fn end_bidding(self) -> Game<OpeningLead> {
        Game {
            phase: OpeningLead {
                declarer: self.implied_declarer().unwrap(),
                contract: self.implied_contract().unwrap(),
                bid_line: self.phase.bid_line,
            },
            deal: self.deal,
        }
    }
}

struct OpeningLead {
    bid_line: BidLine,
    contract: Contract,
    declarer: PlayerPosition,
}

impl Game<OpeningLead> {
    fn lead(self, opening_lead: Card) -> Game<CardPlay> {
        let trump_suit = match self.phase.contract.denomination {
            ContractDenomination::NoTrump => None,
            ContractDenomination::Trump(suit) => Some(suit),
        };
        let mut trick_manager = TrickManager::new(self.phase.declarer + 1, trump_suit);
        trick_manager.play(opening_lead);
        Game {
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

impl Game<CardPlay> {
    fn play_card(mut self, card: &Card) -> Self {
        self.phase.tricks.play(*card);
        self
    }

    fn end_play(self, _card: &Card) -> Game<Ended> {
        todo!()
    }
}

struct Ended {
    bid_line: BidLine,
    contract: Contract,
    declarer: PlayerPosition,
    opening_lead: Card,
    played_tricks: Vec<PlayedTrick>,
}

pub enum Move {
    Bid(Bid),
    Card(Card),
}

impl Game<CardPlay> {
    fn end(self) -> Game<Ended> {
        Game {
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
