use crate::primitives::deal::PlayerPosition;
use crate::primitives::{Card, Suit};

pub struct ActiveTrick {
    lead: PlayerPosition,
    cards: Vec<Card>,
}

impl ActiveTrick {
    pub fn new(lead: PlayerPosition) -> ActiveTrick {
        ActiveTrick {
            lead,
            cards: Vec::with_capacity(4),
        }
    }
    pub fn play(&mut self, card: Card) {
        self.cards.push(card);
    }
}

pub struct TrickManager {
    trump_suit: Option<Suit>,
    current_trick: ActiveTrick,
    turn: PlayerPosition,
    played_tricks: Vec<PlayedTrick>,
}

impl TrickManager {
    pub fn new(lead: PlayerPosition, trump_suit: Option<Suit>) -> TrickManager {
        TrickManager {
            current_trick: ActiveTrick::new(lead),
            turn: lead,
            played_tricks: Vec::with_capacity(13),
            trump_suit,
        }
    }

    pub fn play(&mut self, card: Card) {
        self.current_trick.play(card);
        if self.current_trick.cards.len() == 4 {
            self.move_to_next_trick();
        } else {
            self.turn = self.turn + 1;
        }
    }

    pub fn trick_winner(&self) -> PlayerPosition {
        let winning_card = self.winning_card();
        let winner = self.current_trick.lead
            + self
                .current_trick
                .cards
                .iter()
                .position(|x| *x == winning_card)
                .unwrap();
        winner
    }

    pub fn winning_card(&self) -> Card {
        let mut cards = self.current_trick.cards.iter();
        let mut winner = cards.next().unwrap();
        for card in cards {
            if let Some(trump) = self.trump_suit {
                if card.suit == trump && winner.suit != trump {
                    winner = card;
                }
            } else if card.suit == winner.suit && card.denomination > winner.denomination {
                winner = card;
            }
        }
        *winner
    }

    pub fn move_to_next_trick(&mut self) {
        let winner = self.trick_winner();
        let played_trick = PlayedTrick {
            lead: self.current_trick.lead,
            cards: self.current_trick.cards.clone(),
            winner,
        };
        self.played_tricks.push(played_trick);
        self.current_trick = ActiveTrick::new(winner);
        self.turn = winner;
    }

    pub fn tricks(&self) -> Vec<PlayedTrick> {
        self.played_tricks.clone()
    }
}

#[derive(Clone)]
pub struct PlayedTrick {
    lead: PlayerPosition,
    cards: Vec<Card>,
    winner: PlayerPosition,
}
