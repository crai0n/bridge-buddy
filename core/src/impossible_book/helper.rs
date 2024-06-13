use crate::primitives::card::Rank;
use crate::primitives::deal::seat::SEAT_ARRAY;
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Deal, Suit};

pub fn create_marked_deck_from_deal(deal: Deal<13>) -> [Seat; 52] {
    let mut marked_deck = [Seat::West; 52];
    for (seat_index, hand) in deal.hands.iter().enumerate() {
        for card in hand.cards() {
            let suit_index = 3 - card.suit as usize;
            let rank_index = 12 - card.rank as usize;
            let index = suit_index * 13 + rank_index;
            marked_deck[index] = SEAT_ARRAY[seat_index];
        }
    }
    marked_deck
}

pub fn u8_to_card(n: u8) -> Card {
    // use u8 as index into a deck that is ordered from Spades to Clubs and from Ace to Two
    // This is unusual, because we normally rank from low to high
    let suit_index = 3 - (n as u16 / 13);
    let rank_index = 12 - (n as u32 % 13);
    let suit = Suit::from(suit_index);
    let rank = Rank::try_from(rank_index).unwrap();
    Card { suit, rank }
}

pub fn choose(n: u8, k: u8) -> u128 {
    // taken from https://blog.plover.com/math/choose.html
    if k > n {
        0
    } else {
        let mut r = 1;
        let mut n = n as u128;

        for d in 1..=k {
            r *= n;
            r /= d as u128; // this will always be integer
            n -= 1;
        }
        r
    }
}
