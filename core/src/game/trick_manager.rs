use crate::primitives::deal::PlayerPosition;
use crate::primitives::trick::{ActiveTrick, PlayedTrick};
use crate::primitives::{Card, Suit};
use std::mem;

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
        if self.current_trick.cards().len() == 4 {
            self.move_to_next_trick();
        } else {
            self.turn = self.turn + 1;
        }
    }

    pub fn trick_winner(&self) -> PlayerPosition {
        let winning_card = self.winning_card();
        let winner = self.current_trick.lead()
            + self
                .current_trick
                .cards()
                .iter()
                .position(|x| *x == winning_card)
                .unwrap();
        winner
    }

    pub fn winning_card(&self) -> Card {
        let mut cards = self.current_trick.cards().iter();
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
        let played_trick = mem::replace(&mut self.current_trick, ActiveTrick::new(winner));
        let played_trick = PlayedTrick::from_active_trick(played_trick, winner);
        self.played_tricks.push(played_trick);
        self.turn = winner;
    }

    pub fn played_tricks(&self) -> &[PlayedTrick] {
        &self.played_tricks
    }

    pub fn turn(&self) -> PlayerPosition {
        self.turn
    }

    pub fn count_played_tricks(&self) -> usize {
        self.played_tricks.len()
    }
    pub fn tricks_won_by_player(&self, player: PlayerPosition) -> usize {
        self.played_tricks.iter().filter(|x| x.winner() == player).count()
    }

    pub fn tricks_won_by_axis(&self, player: PlayerPosition) -> usize {
        self.tricks_won_by_player(player) + self.tricks_won_by_player(player.partner())
    }
}
