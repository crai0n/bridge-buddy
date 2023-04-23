

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

impl Suit {
    fn from_char(char: &char) -> Result<Suit, ()> {
        match char{
            'S' => Ok(Suit::Spades),
            'H' => Ok(Suit::Hearts),
            'D' => Ok(Suit::Diamonds),
            'C' => Ok(Suit::Clubs),
            _ => Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Card {
    pub denomination: Denomination,
    pub suit: Option<Suit>
}

#[derive(PartialEq, Clone, Debug)]
pub enum Denomination {
    Ace = 14,
    King = 13,
    Queen = 12, 
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2
}


impl Denomination {
    fn from_char(char: &char) -> Result<Denomination, ()> {
        match char {
            'A' => Ok(Denomination::Ace),
            'K' => Ok(Denomination::King),
            'Q' => Ok(Denomination::Queen),
            'J' => Ok(Denomination::Jack),
            'T' => Ok(Denomination::Ten),
            '9' => Ok(Denomination::Nine),
            '8' => Ok(Denomination::Eight),
            '7' => Ok(Denomination::Seven),
            '6' => Ok(Denomination::Six),
            '5' => Ok(Denomination::Five),
            '4' => Ok(Denomination::Four),
            '3' => Ok(Denomination::Three),
            '2' => Ok(Denomination::Two),
            _ => Err(())
        }
    }
}


impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.denomination == other.denomination && match (&self.suit, &other.suit) {
            (Some(s), Some(o)) => s == o,
            _ => true
        }
    }
}


impl Card {
    fn from_str(string: &str) -> Result<Card, ()>  {
        match Denomination::from_char(&string.chars().nth(0).unwrap()) {
            Ok(d) => Ok(Card { denomination: d, suit: match Suit::from_char(&string.chars().nth(1).unwrap()) {
                Ok(s) => Some(s),
                Err(_e) => None
            }}),
            Err(e) => Err(e)
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_does_not_work() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}