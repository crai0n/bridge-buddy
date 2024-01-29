use crate::primitives::deal::Seat;
use crate::primitives::trick::{ActiveTrick, PlayedTrick};
use crate::primitives::{Card, Suit};
use itertools::Itertools;
use std::iter;

#[derive(Debug, Clone)]
pub struct TrickManager<const N: usize> {
    played_cards: Vec<Card>,
    opening_leader: Seat,
    next_to_play: Seat,
    trumps: Option<Suit>,
    winners: Vec<Seat>,
}

impl<const N: usize> TrickManager<N> {
    pub fn new(opening_leader: Seat, trumps: Option<Suit>) -> Self {
        Self {
            played_cards: Vec::with_capacity(4 * N),
            opening_leader,
            next_to_play: opening_leader,
            winners: Vec::with_capacity(N),
            trumps,
        }
    }

    pub fn suit_to_follow(&self) -> Option<Suit> {
        let cards = self.cards_in_current_trick();
        cards.first().map(|card| card.suit)
    }

    fn cards_in_current_trick(&self) -> &[Card] {
        let last_lead_index = (self.played_cards.len() / 4) * 4;
        &self.played_cards[last_lead_index..]
    }

    pub fn current_trick(&self) -> ActiveTrick {
        let cards = self.cards_in_current_trick();
        ActiveTrick::new_with_cards(self.last_leader(), cards).unwrap()
    }

    pub fn last_leader(&self) -> Seat {
        match self.winners.last() {
            Some(leader) => *leader,
            None => self.opening_leader,
        }
    }

    pub fn trumps(&self) -> Option<Suit> {
        self.trumps
    }

    pub fn next_to_play(&self) -> Seat {
        self.next_to_play
    }

    pub fn count_played_cards(&self) -> usize {
        self.played_cards.len()
    }

    pub fn count_played_tricks(&self) -> usize {
        self.count_played_cards() / 4
    }

    pub fn card_play_has_ended(&self) -> bool {
        self.count_played_tricks() == N
    }

    pub fn tricks_left(&self) -> usize {
        N - self.count_played_tricks()
    }

    fn trick_complete(&self) -> bool {
        self.played_cards.len() % 4 == 0
    }

    fn move_to_next_trick(&mut self) {
        let winner = self.trick_winner();
        // println!("The real winner is {}", winner);
        self.next_to_play = winner;
        self.winners.push(winner);
    }

    pub fn last_trick_winner(&self) -> Option<Seat> {
        self.winners.last().copied()
    }

    fn trick_winner(&self) -> Seat {
        let n_cards = self.played_cards.len();
        let cards = &self.played_cards[n_cards - 4..];
        let winner_card = self.winner_card(cards);
        let winner_index = cards.iter().position(|card| *card == winner_card).unwrap();
        let leader = self.last_leader();
        // println!("leader was {}, winner_index is {}", leader, winner_index);
        leader + winner_index
    }

    fn winner_card(&self, cards: &[Card]) -> Card {
        let mut cards = cards.iter();
        let mut winning_card = cards.next().unwrap();
        for card in cards {
            if let Some(trump) = self.trumps {
                if card.suit == trump && winning_card.suit != trump {
                    winning_card = card;
                }
            }
            if card.suit == winning_card.suit && card.rank > winning_card.rank {
                winning_card = card;
            }
        }
        // println!("The winning card is {}", winner_card);
        *winning_card
    }

    pub fn tricks_won_by_player(&self, player: Seat) -> usize {
        self.winners.iter().filter(|&&x| x == player).count()
    }

    pub fn tricks_won_by_axis(&self, player: Seat) -> usize {
        self.tricks_won_by_player(player) + self.tricks_won_by_player(player.partner())
    }

    fn leaders(&self) -> impl Iterator<Item = &Seat> {
        iter::once(&self.opening_leader).chain(self.winners.iter())
    }

    pub fn played_tricks(&self) -> Vec<PlayedTrick> {
        let winners = self.winners.iter();
        let leaders = self.leaders();
        let cards = self.played_cards.chunks_exact(4);

        leaders
            .zip(winners)
            .zip(cards)
            .map(|((lead, winner), cards)| PlayedTrick::new(*lead, cards.try_into().unwrap(), *winner))
            .collect_vec()
    }

    /// A card is played once it leaves a player's hand
    pub fn played_cards(&self) -> &[Card] {
        &self.played_cards
    }

    /// A card is out of play once it is turned over after a trick
    pub fn out_of_play_cards(&self) -> &[Card] {
        let length = self.played_cards.len();
        let cards_in_current_trick = length % 4;
        &self.played_cards[..length - cards_in_current_trick]
    }

    pub fn count_cards_in_current_trick(&self) -> usize {
        self.played_cards.len() % 4
    }

    pub fn play(&mut self, card: Card) {
        self.played_cards.push(card);
        if self.trick_complete() {
            self.move_to_next_trick();
        } else {
            self.next_to_play = self.next_to_play + 1;
        }
    }

    pub fn undo(&mut self) -> Option<Card> {
        if !self.played_cards.is_empty() {
            if self.trick_complete() {
                self.winners.pop();
                self.next_to_play = self.last_leader() + 3;
            } else {
                self.next_to_play = self.next_to_play + 3;
            }
        }
        self.played_cards.pop()
    }
}

#[cfg(test)]
mod test {
    use crate::primitives::card::Suit::*;
    use crate::primitives::deal::Seat::*;
    use crate::primitives::Card;
    use itertools::Itertools;
    // use rand::Rng;
    use crate::game::trick_manager::TrickManager;
    use rand::Rng;
    use std::str::FromStr;

    const CARDS: [&str; 52] = [
        "C2", "C7", "CK", "C3", "CJ", "S6", "C4", "C8", "D4", "D6", "D7", "DJ", "C6", "S9", "C5", "C9", "D5", "D8",
        "D9", "D2", "CA", "S2", "ST", "CT", "DT", "D3", "DK", "H2", "H5", "H3", "H7", "H4", "DQ", "DA", "H6", "S3",
        "S4", "SQ", "H8", "S8", "SK", "CQ", "SJ", "SA", "S7", "HJ", "HK", "HA", "S5", "HT", "HQ", "H9",
    ];

    #[test]
    fn run_through_with_undo() {
        let mut manager = TrickManager::<13>::new(North, Some(Spades));
        let cards = CARDS.iter().map(|x| Card::from_str(x).unwrap()).collect_vec();
        let mut rng = rand::thread_rng();

        for i in 0..cards.len() {
            manager.play(cards[i]);

            let undo_count = rng.gen_range(0..=manager.count_played_cards());

            let next_to_play = manager.next_to_play();

            for _ in 0..undo_count {
                manager.undo();
            }
            for j in (0..undo_count).rev() {
                manager.play(cards[i - j])
            }
            assert_eq!(next_to_play, manager.next_to_play);
        }
    }

    #[test]
    fn dds_trick_manager() {
        let mut manager = TrickManager::<13>::new(North, Some(Spades));
        let mut cards = CARDS.iter().map(|x| Card::from_str(x).unwrap());

        manager.play(cards.next().unwrap());
        assert_eq!(manager.next_to_play(), East);
        for _ in 0..3 {
            manager.play(cards.next().unwrap());
        }
        assert_eq!(manager.next_to_play(), South);

        for _ in 0..2 {
            manager.play(cards.next().unwrap());
        }

        assert_eq!(manager.next_to_play(), North);

        for _ in 0..2 {
            manager.play(cards.next().unwrap());
        }

        assert_eq!(manager.next_to_play(), West);

        assert_eq!(manager.count_played_tricks(), 2);

        for _ in 0..4 {
            manager.play(cards.next().unwrap());
        }
        assert_eq!(manager.next_to_play(), South);

        for _ in 0..4 {
            manager.play(cards.next().unwrap());
        }
        assert_eq!(manager.next_to_play(), West);

        assert_eq!(manager.count_played_tricks(), 4);

        assert_eq!(manager.tricks_won_by_player(North), 0);
        assert_eq!(manager.tricks_won_by_player(South), 2);
        assert_eq!(manager.tricks_won_by_player(East), 0);
        assert_eq!(manager.tricks_won_by_player(West), 2);

        assert_eq!(manager.tricks_won_by_axis(North), 2);
        assert_eq!(manager.tricks_won_by_axis(South), 2);
        assert_eq!(manager.tricks_won_by_axis(East), 2);
        assert_eq!(manager.tricks_won_by_axis(West), 2);
    }

    #[test]
    fn trick_manager() {
        let mut manager = TrickManager::<13>::new(North, Some(Spades));

        manager.play(Card::from_str("H8").unwrap());
        assert_eq!(manager.next_to_play(), East);
        manager.play(Card::from_str("H9").unwrap());
        manager.play(Card::from_str("HA").unwrap());
        manager.play(Card::from_str("H2").unwrap());

        assert_eq!(manager.next_to_play(), South);

        manager.play(Card::from_str("D2").unwrap());
        manager.play(Card::from_str("S2").unwrap());
        assert_eq!(manager.next_to_play(), North);
        manager.play(Card::from_str("HK").unwrap());
        manager.play(Card::from_str("HQ").unwrap());

        assert_eq!(manager.next_to_play(), West);

        assert_eq!(manager.count_played_tricks(), 2);

        manager.play(Card::from_str("C2").unwrap());
        manager.play(Card::from_str("S3").unwrap());
        manager.play(Card::from_str("C5").unwrap());
        manager.play(Card::from_str("D3").unwrap());
        assert_eq!(manager.next_to_play(), North);

        manager.play(Card::from_str("D8").unwrap());
        manager.play(Card::from_str("DA").unwrap());
        manager.play(Card::from_str("S7").unwrap());
        manager.play(Card::from_str("D5").unwrap());
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
