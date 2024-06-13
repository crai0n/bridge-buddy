use crate::impossible_book;
use crate::impossible_book::{helper, N_PAGES};
use crate::primitives::deal::seat::SEAT_ARRAY;
use crate::primitives::{Deal, Hand};

pub fn find_human_page_number_for_deal_in_pavliceks_book(deal: Deal<13>) -> u128 {
    impossible_book::find_internal_page_number_for_deal_in_pavliceks_book(deal) + 1
}

pub fn deal_from_human_pavlicek_page(page: u128) -> Deal<13> {
    assert!(page > 0, "This page does not exist.");
    deal_from_internal_pavlicek_page(page - 1)
}

pub fn deal_from_internal_pavlicek_page(page: u128) -> Deal<13> {
    // This calculates the deal according to Richard Pavlicek's Algorithm
    // http://www.rpbridge.net/7z68.htm

    assert!(page < N_PAGES, "This page does not exist.");

    let mut remaining_possible_deals = N_PAGES;
    let mut relative_deal_index = page;

    let mut vacant_places = [13usize; 4];
    let mut card_values = [[0; 13]; 4];

    for cards_left in (1..=52usize).rev() {
        let card_index = 52 - cards_left;

        for (player, vp) in vacant_places.iter_mut().enumerate() {
            let deals_where_player_owns_this_card = remaining_possible_deals * *vp as u128 / cards_left as u128;
            if relative_deal_index < deals_where_player_owns_this_card {
                // assign card to this player
                card_values[player][*vp - 1] = card_index as u8;
                *vp -= 1;
                // the final deal must be one where this card belongs to this player
                remaining_possible_deals = deals_where_player_owns_this_card;
                break;
            } else {
                // this player doesn't own this card
                // we discard all deals where the card would belong to this player
                relative_deal_index -= deals_where_player_owns_this_card
            }
        }
    }

    let cards = card_values.map(|x| x.map(helper::u8_to_card));

    let hands = cards.map(|c| Hand::<13>::from_cards(&c).unwrap());

    Deal::<13>::from_hands(hands)
}

pub fn find_internal_page_number_for_deal_in_pavliceks_book(deal: Deal<13>) -> u128 {
    let marked_deck = helper::create_marked_deck_from_deal(deal);

    let mut vacant_places = [13u8; 4];
    let mut remaining_possible_deals = N_PAGES;
    let mut page_number = 0;

    for (card_index, owner) in marked_deck.iter().enumerate() {
        let cards_left = 52 - card_index;
        for seat in 0..4 {
            let deals_where_player_owns_this_card =
                remaining_possible_deals * vacant_places[seat] as u128 / cards_left as u128;
            if *owner == SEAT_ARRAY[seat] {
                vacant_places[seat] -= 1;
                remaining_possible_deals = deals_where_player_owns_this_card;
                break;
            } else {
                page_number += deals_where_player_owns_this_card;
            }
        }
    }
    page_number
}
