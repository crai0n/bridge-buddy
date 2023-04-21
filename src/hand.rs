
use crate::card::Card;

struct Hand {
    cards: [Card; 13] 
}

impl Hand {
    fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

}