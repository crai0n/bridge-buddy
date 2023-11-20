use crate::error::BBError;
use crate::primitives::deal::PlayerPosition;
use crate::primitives::trick::Trick;
use crate::primitives::trick::{ActiveTrick, PlayedTrick};
use crate::primitives::{Card, Deal, Hand, Suit};
use std::collections::{BTreeMap, BTreeSet};
use std::mem;
use strum::IntoEnumIterator;

pub struct CardManager {
    trump_suit: Option<Suit>,
    current_trick: ActiveTrick,
    turn: PlayerPosition,
    played_tricks: Vec<PlayedTrick>,
    known_cards: BTreeMap<Card, PlayerPosition>,
    played_cards: BTreeSet<Card>,
}

impl CardManager {
    pub fn new(lead: PlayerPosition, trump_suit: Option<Suit>) -> CardManager {
        CardManager {
            current_trick: ActiveTrick::new(lead),
            turn: lead,
            played_tricks: Vec::with_capacity(13),
            known_cards: BTreeMap::new(),
            played_cards: BTreeSet::new(),
            trump_suit,
        }
    }

    pub fn new_with_deal_info(lead: PlayerPosition, trump_suit: Option<Suit>, deal: &Deal) -> CardManager {
        let mut card_manager = CardManager::new(lead, trump_suit);
        for player in PlayerPosition::iter() {
            for &card in deal.hand_of(player).cards() {
                card_manager.register_known_card(card, player).unwrap();
            }
        }
        card_manager
    }

    pub fn register_known_hand(&mut self, hand: Hand, player: PlayerPosition) -> Result<(), BBError> {
        for &card in hand.cards() {
            self.register_known_card(card, player)?;
        }
        Ok(())
    }

    pub fn register_known_card(&mut self, card: Card, player: PlayerPosition) -> Result<(), BBError> {
        match self.known_cards.insert(card, player) {
            None => Ok(()),
            Some(known_player) if known_player == player => Ok(()),
            _ => Err(BBError::Duplicate(card)),
        }
    }

    pub fn count_played_cards(&self) -> usize {
        self.played_cards.len()
    }

    pub fn play(&mut self, card: Card) -> Result<(), BBError> {
        if self.played_cards.contains(&card) {
            return Err(BBError::InvalidCard(card));
        } else if let Some(owner) = self.known_cards.get(&card) {
            if *owner != self.turn {
                return Err(BBError::InvalidCard(card));
            }
        }

        self.current_trick.play(card);
        if self.current_trick.cards().len() == 4 {
            self.move_to_next_trick();
        } else {
            self.turn = self.turn + 1;
        }

        self.played_cards.insert(card);
        self.known_cards.insert(card, self.turn);

        Ok(())
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

    pub fn card_play_has_ended(&self) -> bool {
        self.count_played_tricks() == 13
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
    use crate::game::card_manager::CardManager;
    use crate::primitives::card::Suit::*;
    use crate::primitives::deal::PlayerPosition::*;
    use crate::primitives::Card;
    use std::str::FromStr;

    #[test]
    fn trick_manager() {
        let mut manager = CardManager::new(North, Some(Spades));

        manager.play(Card::from_str("H8").unwrap()).unwrap();
        assert_eq!(manager.turn(), East);
        manager.play(Card::from_str("H9").unwrap()).unwrap();
        manager.play(Card::from_str("HA").unwrap()).unwrap();
        manager.play(Card::from_str("H2").unwrap()).unwrap();

        assert_eq!(manager.turn(), South);

        manager.play(Card::from_str("D2").unwrap()).unwrap();
        manager.play(Card::from_str("S2").unwrap()).unwrap();
        assert_eq!(manager.turn(), North);
        manager.play(Card::from_str("HK").unwrap()).unwrap();
        manager.play(Card::from_str("HQ").unwrap()).unwrap();

        assert_eq!(manager.turn(), West);

        assert_eq!(manager.count_played_tricks(), 2);

        manager.play(Card::from_str("C2").unwrap()).unwrap();
        manager.play(Card::from_str("S3").unwrap()).unwrap();
        manager.play(Card::from_str("C5").unwrap()).unwrap();
        manager.play(Card::from_str("D3").unwrap()).unwrap();
        assert_eq!(manager.turn(), North);

        manager.play(Card::from_str("D8").unwrap()).unwrap();
        manager.play(Card::from_str("DA").unwrap()).unwrap();
        manager.play(Card::from_str("S7").unwrap()).unwrap();
        manager.play(Card::from_str("D5").unwrap()).unwrap();
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
