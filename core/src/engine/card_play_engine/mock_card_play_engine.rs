use crate::engine::card_play_engine::SelectCard;
use crate::engine::subjective_game_view::{SubjectiveGameDataView, SubjectiveSeat};
use crate::game::game_data::{CardPlayState, OpeningLeadState};
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Suit};
use itertools::Itertools;

pub struct MockCardPlayEngine {
    _seat: Seat,
}

impl MockCardPlayEngine {
    pub fn new(seat: Seat) -> Self {
        Self { _seat: seat }
    }

    fn pick_opening_lead(&self, data: SubjectiveGameDataView<OpeningLeadState>) -> Card {
        let hand = data.my_remaining_cards();
        let card = hand.first().unwrap();
        *card
    }

    fn pick_card(&self, state: SubjectiveGameDataView<CardPlayState>) -> Card {
        match state.suit_to_follow() {
            None => self.pick_lead(state),
            Some(suit) => self.pick_card_to_trick(suit, state),
        }
    }

    fn pick_lead(&self, state: SubjectiveGameDataView<CardPlayState>) -> Card {
        let remaining_cards = match state.next_to_play() {
            SubjectiveSeat::Myself => state.my_remaining_cards(),
            SubjectiveSeat::Partner => state.dummys_remaining_cards(),
            _ => unreachable!(),
        };
        let card = remaining_cards.first().unwrap();
        *card
    }

    fn pick_card_to_trick(&self, suit: Suit, state: SubjectiveGameDataView<CardPlayState>) -> Card {
        let remaining_cards = match state.next_to_play() {
            SubjectiveSeat::Myself => state.my_remaining_cards(),
            SubjectiveSeat::Partner => state.dummys_remaining_cards(),
            _ => unreachable!(),
        };

        let cards_in_suit = remaining_cards.iter().filter(|x| x.suit == suit).collect_vec();

        if cards_in_suit.is_empty() {
            self.pick_discard(&remaining_cards, state)
        } else {
            self.pick_card_from(&cards_in_suit, state)
        }
    }

    fn pick_discard(&self, cards: &[Card], _state: SubjectiveGameDataView<CardPlayState>) -> Card {
        *cards.first().unwrap()
    }

    fn pick_card_from(&self, choices: &[&Card], _state: SubjectiveGameDataView<CardPlayState>) -> Card {
        **choices.first().unwrap()
    }
}

impl SelectCard for MockCardPlayEngine {
    fn select_card(&self, state: SubjectiveGameDataView<CardPlayState>) -> Card {
        self.pick_card(state)
    }

    fn select_opening_lead(&self, state: SubjectiveGameDataView<OpeningLeadState>) -> Card {
        self.pick_opening_lead(state)
    }
}

impl Default for MockCardPlayEngine {
    fn default() -> Self {
        Self::new(Seat::South)
    }
}
