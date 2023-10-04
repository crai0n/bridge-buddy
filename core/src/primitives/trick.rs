use crate::primitives::deal::PlayerPosition;
use crate::primitives::Card;

struct TrickData<S> {
    lead: PlayerPosition,
    state: S,
}

enum Trick {
    Empty(TrickData<EmptyTrick>),
    OneCard(TrickData<OneCardTrick>),
    TwoCard(TrickData<TwoCardTrick>),
    ThreeCard(TrickData<ThreeCardTrick>),
    Complete(TrickData<CompleteTrick>),
}

pub struct TrickManager {
    trick: Trick,
}

impl TrickManager {
    pub fn new(lead: PlayerPosition) -> TrickManager {
        TrickManager {
            trick: Trick::Empty(TrickData {
                lead,
                state: EmptyTrick::new(),
            }),
        }
    }
}

struct EmptyTrick {}

impl EmptyTrick {
    fn new() -> EmptyTrick {
        EmptyTrick {}
    }
}
struct OneCardTrick {
    first: Card,
}
struct TwoCardTrick {
    first: Card,
    second: Card,
}

struct ThreeCardTrick {
    first: Card,
    second: Card,
    third: Card,
}

struct CompleteTrick {
    first: Card,
    second: Card,
    third: Card,
    fourth: Card,
}

impl TrickData<EmptyTrick> {
    pub fn new(lead: PlayerPosition) -> TrickData<EmptyTrick> {
        TrickData {
            lead,
            state: EmptyTrick {},
        }
    }
    pub fn play(self, card: &Card) -> TrickData<OneCardTrick> {
        TrickData {
            lead: self.lead,
            state: OneCardTrick { first: *card },
        }
    }
}

impl TrickData<OneCardTrick> {
    pub fn play(self, card: &Card) -> TrickData<TwoCardTrick> {
        TrickData {
            lead: self.lead,
            state: TwoCardTrick {
                first: self.state.first,
                second: *card,
            },
        }
    }
}

impl TrickData<TwoCardTrick> {
    pub fn play(self, card: &Card) -> TrickData<ThreeCardTrick> {
        TrickData {
            lead: self.lead,
            state: ThreeCardTrick {
                first: self.state.first,
                second: self.state.second,
                third: *card,
            },
        }
    }
}

impl TrickData<ThreeCardTrick> {
    pub fn play(self, card: &Card) -> TrickData<CompleteTrick> {
        TrickData {
            lead: self.lead,
            state: CompleteTrick {
                first: self.state.first,
                second: self.state.second,
                third: self.state.third,
                fourth: *card,
            },
        }
    }
}

impl TrickData<CompleteTrick> {
    pub fn won_by(self, winner: PlayerPosition) -> PlayedTrick {
        PlayedTrick {
            lead: self.lead,
            cards: [self.state.first, self.state.second, self.state.third, self.state.fourth],
            winner,
        }
    }
}

pub struct PlayedTrick {
    lead: PlayerPosition,
    cards: [Card; 4],
    winner: PlayerPosition,
}
