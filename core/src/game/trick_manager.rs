use crate::primitives::deal::PlayerPosition;
use crate::primitives::trick::Trick;
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

    fn trick_winner(&self) -> PlayerPosition {
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

    fn winning_card(&self) -> Card {
        let mut cards = self.current_trick.cards().iter();
        let mut winning_card = cards.next().unwrap();
        for card in cards {
            if let Some(trump) = self.trump_suit {
                if card.suit == trump && winning_card.suit != trump {
                    winning_card = card;
                }
            }
            if card.suit == winning_card.suit && card.denomination > winning_card.denomination {
                winning_card = card;
            }
        }
        *winning_card
    }

    fn move_to_next_trick(&mut self) {
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

#[cfg(test)]
mod test {
    use crate::game::trick_manager::TrickManager;
    use crate::primitives::card::Suit::*;
    use crate::primitives::deal::PlayerPosition::*;
    use crate::primitives::Card;
    use std::str::FromStr;

    #[test]
    fn trick_manager() {
        let mut manager = TrickManager::new(North, Some(Spades));

        manager.play(Card::from_str("H8").unwrap());
        assert_eq!(manager.turn(), East);
        manager.play(Card::from_str("H9").unwrap());
        manager.play(Card::from_str("HA").unwrap());
        manager.play(Card::from_str("H2").unwrap());

        assert_eq!(manager.turn(), South);

        manager.play(Card::from_str("H2").unwrap());
        manager.play(Card::from_str("S2").unwrap());
        assert_eq!(manager.turn(), North);
        manager.play(Card::from_str("HK").unwrap());
        manager.play(Card::from_str("HQ").unwrap());

        assert_eq!(manager.turn(), West);

        assert_eq!(manager.count_played_tricks(), 2);

        manager.play(Card::from_str("H2").unwrap());
        manager.play(Card::from_str("S3").unwrap());
        manager.play(Card::from_str("C2").unwrap());
        manager.play(Card::from_str("D2").unwrap());
        assert_eq!(manager.turn(), North);

        manager.play(Card::from_str("D8").unwrap());
        manager.play(Card::from_str("DA").unwrap());
        manager.play(Card::from_str("S7").unwrap());
        manager.play(Card::from_str("D5").unwrap());
        assert_eq!(manager.turn(), South);

        assert_eq!(manager.count_played_tricks(), 4);

        assert_eq!(manager.tricks_won_by_player(North), 1);
        assert_eq!(manager.tricks_won_by_player(South), 2);
        assert_eq!(manager.tricks_won_by_player(East), 0);
        assert_eq!(manager.tricks_won_by_player(West), 1);

        assert_eq!(manager.tricks_won_by_axis(North), 3);
        assert_eq!(manager.tricks_won_by_axis(South), 3);
        assert_eq!(manager.tricks_won_by_axis(East), 1);
        assert_eq!(manager.tricks_won_by_axis(West), 1);
    }
}
