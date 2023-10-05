use crate::primitives::bid::Bid;
use crate::primitives::bid_line::BidLine;
use crate::primitives::contract::ContractDenomination;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::trick::{PlayedTrick, TrickManager};
use crate::primitives::{Card, Contract, Deal};

// Game is a state machine that contains all information that immediately follow from the rules of the game.

pub struct Game<Phase> {
    phase: Phase,
    deal: Deal,
}

enum GameWrapper {
    Setup(Game<Setup>),
    Bidding(Game<BiddingPhase>),
    OpeningLead(Game<OpeningLeadPhase>),
    CardPlay(Game<CardPlayPhase>),
    Ended(Game<EndedPhase>),
}
struct Setup {}

impl Game<Setup> {
    fn new(deal: Deal) -> Self {
        Game { deal, phase: Setup {} }
    }
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

struct BiddingPhase {
    bid_line: BidLine,
    current_turn: PlayerPosition,
    declarer: Option<PlayerPosition>,
    contract: Option<Contract>,
}

impl Game<BiddingPhase> {
    fn bid() -> Self {
        todo!()
    }

    fn can_bid() -> bool {
        todo!()
    }

    fn contract_is_final() -> bool {
        todo!()
    }

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

struct OpeningLeadPhase {
    bid_line: BidLine,
    contract: Contract,
    declarer: PlayerPosition,
}

impl Game<OpeningLeadPhase> {
    fn lead(self, opening_lead: Card) -> Game<CardPlayPhase> {
        let trump_suit = match self.phase.contract.denomination {
            ContractDenomination::NoTrump => None,
            ContractDenomination::Trump(suit) => Some(suit),
        };
        let mut trick_manager = TrickManager::new(self.phase.declarer + 1, trump_suit);
        trick_manager.play(opening_lead);
        Game {
            phase: CardPlayPhase {
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

pub struct CardPlayPhase {
    bid_line: BidLine,
    contract: Contract,
    opening_lead: Card,
    declarer: PlayerPosition,
    tricks: TrickManager,
}

impl Game<CardPlayPhase> {
    fn play_card(mut self, card: &Card) -> Self {
        self.phase.tricks.play(*card);
        self
    }

    fn end_play(self, _card: &Card) -> Game<EndedPhase> {
        todo!()
    }
}

struct EndedPhase {
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

impl Game<CardPlayPhase> {
    fn end(self) -> Game<EndedPhase> {
        Game {
            phase: EndedPhase {
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
