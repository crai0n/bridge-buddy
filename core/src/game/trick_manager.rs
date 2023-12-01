use crate::error::BBError;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::trick::Trick;
use crate::primitives::trick::{ActiveTrick, PlayedTrick};
use crate::primitives::{Card, Suit};

#[derive(Debug, Clone)]
pub struct TrickManager {
    trump_suit: Option<Suit>,
    current_trick: Option<ActiveTrick>,
    next_to_play: PlayerPosition,
    played_tricks: Vec<PlayedTrick>,
}

impl TrickManager {
    pub fn new(lead: PlayerPosition, trump_suit: Option<Suit>) -> Self {
        TrickManager {
            current_trick: None,
            next_to_play: lead,
            played_tricks: Vec::with_capacity(13),
            trump_suit,
        }
    }

    pub fn suit_to_follow(&self) -> Option<Suit> {
        match &self.current_trick {
            Some(trick) => trick.suit_to_follow(),
            None => None,
        }
    }

    pub fn set_lead(&mut self, lead: PlayerPosition) {
        self.next_to_play = lead;
    }

    pub fn next_to_play(&self) -> PlayerPosition {
        self.next_to_play
    }

    pub fn count_played_tricks(&self) -> usize {
        self.played_tricks.len()
    }

    pub fn card_play_has_ended(&self) -> bool {
        self.count_played_tricks() == 13
    }

    fn trick_winner(&self) -> PlayerPosition {
        let winning_card = self.winning_card();
        let winner = self.current_trick.as_ref().unwrap().lead()
            + self
                .current_trick
                .as_ref()
                .unwrap()
                .cards()
                .iter()
                .position(|x| *x == winning_card)
                .unwrap();
        winner
    }

    fn winning_card(&self) -> Card {
        let mut cards = self.current_trick.as_ref().unwrap().cards().iter();
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

        let played_trick = self.current_trick.take();
        let played_trick = PlayedTrick::from_active_trick(played_trick.unwrap(), winner);
        self.played_tricks.push(played_trick);
        self.next_to_play = winner;
    }

    pub fn played_tricks(&self) -> &[PlayedTrick] {
        &self.played_tricks
    }

    pub fn tricks_won_by_player(&self, player: PlayerPosition) -> usize {
        self.played_tricks.iter().filter(|x| x.winner() == player).count()
    }

    pub fn tricks_won_by_axis(&self, player: PlayerPosition) -> usize {
        self.tricks_won_by_player(player) + self.tricks_won_by_player(player.partner())
    }

    pub fn play(&mut self, card: Card) -> Result<(), BBError> {
        if self.current_trick.is_none() {
            self.current_trick = Some(ActiveTrick::new(self.next_to_play()));
        }

        self.current_trick.as_mut().unwrap().play(card);
        if self.current_trick.as_mut().unwrap().cards().len() == 4 {
            self.move_to_next_trick();
        } else {
            self.next_to_play = self.next_to_play + 1;
        }

        Ok(())
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

        manager.play(Card::from_str("H8").unwrap()).unwrap();
        assert_eq!(manager.next_to_play(), East);
        manager.play(Card::from_str("H9").unwrap()).unwrap();
        manager.play(Card::from_str("HA").unwrap()).unwrap();
        manager.play(Card::from_str("H2").unwrap()).unwrap();

        assert_eq!(manager.next_to_play(), South);

        manager.play(Card::from_str("D2").unwrap()).unwrap();
        manager.play(Card::from_str("S2").unwrap()).unwrap();
        assert_eq!(manager.next_to_play(), North);
        manager.play(Card::from_str("HK").unwrap()).unwrap();
        manager.play(Card::from_str("HQ").unwrap()).unwrap();

        assert_eq!(manager.next_to_play(), West);

        assert_eq!(manager.count_played_tricks(), 2);

        manager.play(Card::from_str("C2").unwrap()).unwrap();
        manager.play(Card::from_str("S3").unwrap()).unwrap();
        manager.play(Card::from_str("C5").unwrap()).unwrap();
        manager.play(Card::from_str("D3").unwrap()).unwrap();
        assert_eq!(manager.next_to_play(), North);

        manager.play(Card::from_str("D8").unwrap()).unwrap();
        manager.play(Card::from_str("DA").unwrap()).unwrap();
        manager.play(Card::from_str("S7").unwrap()).unwrap();
        manager.play(Card::from_str("D5").unwrap()).unwrap();
        assert_eq!(manager.next_to_play(), South);

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
