pub struct Card {
    denomination: Denomination,
    suit: Suit
}

#[derive(PartialEq)]
enum Denomination {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

#[derive(PartialEq)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.denomination == other.denomination && self.suit == other.suit
    }
}