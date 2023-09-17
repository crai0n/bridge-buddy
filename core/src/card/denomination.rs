use crate::error::ParseError;
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Denomination {
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    #[strum(serialize = "T")]
    Ten,
    #[strum(serialize = "J")]
    Jack,
    #[strum(serialize = "Q")]
    Queen,
    #[strum(serialize = "K")]
    King,
    #[strum(serialize = "A")]
    Ace,
}

impl Denomination {
    pub fn from_char(char: char) -> Result<Denomination, ParseError> {
        match char {
            'A' => Ok(Denomination::Ace),
            'a' => Ok(Denomination::Ace),
            'K' => Ok(Denomination::King),
            'k' => Ok(Denomination::King),
            'Q' => Ok(Denomination::Queen),
            'q' => Ok(Denomination::Queen),
            'J' => Ok(Denomination::Jack),
            'j' => Ok(Denomination::Jack),
            'T' => Ok(Denomination::Ten),
            't' => Ok(Denomination::Ten),
            '9' => Ok(Denomination::Nine),
            '8' => Ok(Denomination::Eight),
            '7' => Ok(Denomination::Seven),
            '6' => Ok(Denomination::Six),
            '5' => Ok(Denomination::Five),
            '4' => Ok(Denomination::Four),
            '3' => Ok(Denomination::Three),
            '2' => Ok(Denomination::Two),
            c => Err(ParseError {
                cause: c.into(),
                description: "unknown denomination",
            }),
        }
    }
}
