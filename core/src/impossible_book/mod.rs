// Re-Implementation of https://bridge.thomasoandrews.com/bridge/impossible/

use crate::primitives::card::Rank;
use crate::primitives::deal::seat::SEAT_ARRAY;
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Deal, Hand, Suit};

const N_PAGES: u128 = 53644737765488792839237440000;
const NORTH_MAX: u128 = 635013559600; // 52 choose 13
const EAST_MAX: u128 = 8122425444; // 39 choose 13
const SOUTH_MAX: u128 = 10400600; // 26 choose 13

pub fn deal_from_human_andrews_page(page: u128) -> Deal<13> {
    // this uses numbers in Range 1..=N_PAGES
    assert!(page > 0, "This page does not exist.");
    deal_from_internal_andrews_page(page - 1)
}

pub fn find_internal_page_number_for_deal_in_andrews_book(deal: Deal<13>) -> u128 {
    println!("Going backwards");

    let marked_deck = create_marked_deck_from_deal(deal);

    let sequences = create_sequences_from_marked_deck(marked_deck);

    println!("Sequences in Andrews Book:");
    println!("North: {:?}", sequences[0]);
    println!("East: {:?}", sequences[1]);
    println!("South: {:?}", sequences[2]);

    let indexes = sequences.map(sequence_to_index);

    println!(
        "Indexes in Andrews Book: N:{}, E:{}, S:{}",
        indexes[0], indexes[1], indexes[2]
    );

    indexes_to_page(indexes)
}

pub fn find_human_page_number_for_deal_in_andrews_book(deal: Deal<13>) -> u128 {
    find_internal_page_number_for_deal_in_andrews_book(deal) + 1
}

pub fn find_human_page_number_for_deal_in_pavliceks_book(deal: Deal<13>) -> u128 {
    find_internal_page_number_for_deal_in_pavliceks_book(deal) + 1
}

fn create_sequences_from_marked_deck(marked_deck: [Seat; 52]) -> [[usize; 13]; 3] {
    let mut sequences = [[0; 13]; 3];
    let mut counts = [0; 3];

    for (index, seat) in marked_deck.iter().enumerate() {
        match seat {
            Seat::North => {
                sequences[0][counts[0]] = index;
                counts[0] += 1;
            }
            Seat::East => {
                sequences[1][counts[1]] = index - counts[0];
                counts[1] += 1;
            }
            Seat::South => {
                sequences[2][counts[2]] = index - counts[0] - counts[1];
                counts[2] += 1;
            }
            _ => continue,
        }
    }
    sequences
}

fn create_marked_deck_from_deal(deal: Deal<13>) -> [Seat; 52] {
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

pub fn deal_from_internal_andrews_page(page: u128) -> Deal<13> {
    // This calculates the deal according to Thomas Andrews' Algorithm
    // https://bridge.thomasoandrews.com/bridge/impossible/algorithm.html
    // using page in  0..<N_PAGES

    assert!(page < N_PAGES, "This page does not exist.");

    println!("Going forwards:");

    println!("Page in Andrews Book: {}", page);

    let indexes = page_to_indexes(page);

    println!(
        "Indexes in Andrews Book: N:{}, E:{}, S:{}",
        indexes[0], indexes[1], indexes[2]
    );

    let sequences = indexes.map(index_to_sequence);

    println!("Sequences in Andrews Book:");
    println!("North: {:?}", sequences[0]);
    println!("East: {:?}", sequences[1]);
    println!("South: {:?}", sequences[2]);

    let marked_deck = mark_cards(sequences);

    let hands = distribute_cards(marked_deck);

    Deal::<13>::from_hands(hands)
}

fn distribute_cards(result: [Seat; 52]) -> [Hand<13>; 4] {
    let mut values = [[0; 13]; 4];
    let mut counts = [0; 4];

    for (i, x) in result.iter().enumerate() {
        values[*x as usize][counts[*x as usize]] = i as u8;
        counts[*x as usize] += 1;
    }

    println!("Card Numbers in Andrews Book:");
    println!("North: {:?}", values[0]);
    println!("East: {:?}", values[1]);
    println!("South: {:?}", values[2]);
    println!("West: {:?}", values[3]);

    let cards = values.map(|x| x.map(u8_to_card));

    cards.map(|c| Hand::<13>::from_cards(&c).unwrap())
}

fn mark_cards(sequences: [[u8; 13]; 3]) -> [Seat; 52] {
    // Creates a list of the Form NSENWNWNENSNE...NSW
    // Describing which player gets each card from a sorted deck
    let mut result = [Seat::West; 52];
    let [n_seq, e_seq, s_seq] = sequences;

    let [mut index_n, mut count_n, mut index_e, mut count_e, mut index_s, mut count_s] = [0; 6];

    for res in &mut result {
        index_n += 1;
        if count_n < 13 && n_seq[count_n] == index_n as u8 - 1 {
            *res = Seat::North;
            count_n += 1;
            continue;
        }

        index_e += 1;
        if count_e < 13 && e_seq[count_e] == index_e as u8 - 1 {
            *res = Seat::East;
            count_e += 1;
            continue;
        }

        index_s += 1;
        if count_s < 13 && s_seq[count_s] == index_s as u8 - 1 {
            *res = Seat::South;
            count_s += 1;
            continue;
        }
    }
    result
}

fn page_to_indexes(page: u128) -> [u128; 3] {
    let s_index = page % SOUTH_MAX;
    let temp = page / SOUTH_MAX;
    let e_index = temp % EAST_MAX;
    let n_index = temp / EAST_MAX % NORTH_MAX; // temp / EAST_MAX < NORTH_MAX iff page < N_PAGES !

    [n_index, e_index, s_index]
}

fn indexes_to_page(indexes: [u128; 3]) -> u128 {
    indexes[0] * EAST_MAX * SOUTH_MAX + indexes[1] * SOUTH_MAX + indexes[2]
}

fn u8_to_card(n: u8) -> Card {
    // use u8 as index into a deck that is ordered from Spades to Clubs and from Ace to Two
    // This is unusual, because we normally rank from low to high
    let suit_index = 3 - (n as u16 / 13);
    let rank_index = 12 - (n as u32 % 13);
    let suit = Suit::from(suit_index);
    let rank = Rank::try_from(rank_index).unwrap();
    Card { suit, rank }
}

pub fn deal_from_human_pavlicek_page(page: u128) -> Deal<13> {
    assert!(page > 0);
    deal_from_internal_pavlicek_page(page - 1)
}

pub fn deal_from_internal_pavlicek_page(page: u128) -> Deal<13> {
    // This calculates the deal according to Richard Pavlicek's Algorithm
    // http://www.rpbridge.net/7z68.htm

    assert!(page < N_PAGES);

    let mut remaining_possible_deals = N_PAGES;
    let mut relative_deal_index = page;

    let mut vacant_places = [13usize; 4];
    let mut card_values = [[0; 13]; 4];

    for cards_left in (1..=52usize).rev() {
        let card_index = 52 - cards_left;
        let deals_where_card_is_owned_by_player =
            vacant_places.map(|vp| remaining_possible_deals * vp as u128 / cards_left as u128);

        let mut deals_where_card_is_before_player = [0; 4];

        for i in 1..=3 {
            deals_where_card_is_before_player[i] =
                deals_where_card_is_before_player[i - 1] + deals_where_card_is_owned_by_player[i - 1]
        }

        if relative_deal_index < deals_where_card_is_before_player[1] {
            card_values[0][vacant_places[0] - 1] = card_index as u8;
            vacant_places[0] -= 1;
            relative_deal_index -= deals_where_card_is_before_player[0];
            remaining_possible_deals = deals_where_card_is_owned_by_player[0];
        } else if relative_deal_index < deals_where_card_is_before_player[2] {
            card_values[1][vacant_places[1] - 1] = card_index as u8;
            vacant_places[1] -= 1;
            relative_deal_index -= deals_where_card_is_before_player[1];
            remaining_possible_deals = deals_where_card_is_owned_by_player[1];
        } else if relative_deal_index < deals_where_card_is_before_player[3] {
            card_values[2][vacant_places[2] - 1] = card_index as u8;
            vacant_places[2] -= 1;
            relative_deal_index -= deals_where_card_is_before_player[2];
            remaining_possible_deals = deals_where_card_is_owned_by_player[2];
        } else {
            card_values[3][vacant_places[3] - 1] = card_index as u8;
            vacant_places[3] -= 1;
            relative_deal_index -= deals_where_card_is_before_player[3];
            remaining_possible_deals = deals_where_card_is_owned_by_player[3];
        }
    }

    let cards = card_values.map(|x| x.map(u8_to_card));

    let hands = cards.map(|c| Hand::<13>::from_cards(&c).unwrap());

    Deal::<13>::from_hands(hands)
}

pub fn find_internal_page_number_for_deal_in_pavliceks_book(deal: Deal<13>) -> u128 {
    let marked_deck = create_marked_deck_from_deal(deal);

    let mut vacant_places = [13u8; 4];
    let mut remaining_possible_deals = N_PAGES;
    let mut page_number = 0;

    for (card_index, seat) in marked_deck.iter().enumerate() {
        let cards_left = 52 - card_index;
        let deals_where_north_owns_this_card = remaining_possible_deals * vacant_places[0] as u128 / cards_left as u128;
        if *seat == Seat::North {
            vacant_places[0] -= 1;
            remaining_possible_deals = deals_where_north_owns_this_card;
        } else {
            page_number += deals_where_north_owns_this_card; // n doesn't own this card
            let deals_where_east_owns_this_card =
                remaining_possible_deals * vacant_places[1] as u128 / cards_left as u128;
            if *seat == Seat::East {
                vacant_places[1] -= 1;
                remaining_possible_deals = deals_where_east_owns_this_card;
            } else {
                page_number += deals_where_east_owns_this_card;
                let deals_where_south_owns_this_card =
                    remaining_possible_deals * vacant_places[2] as u128 / cards_left as u128;
                if *seat == Seat::South {
                    vacant_places[2] -= 1;
                    remaining_possible_deals = deals_where_south_owns_this_card;
                } else {
                    page_number += deals_where_south_owns_this_card;
                    let deals_where_west_owns_this_card =
                        remaining_possible_deals * vacant_places[3] as u128 / cards_left as u128;
                    vacant_places[3] -= 1;
                    remaining_possible_deals = deals_where_west_owns_this_card;
                }
            }
        }
    }
    page_number
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

pub fn sequence_to_index(seq: [usize; 13]) -> u128 {
    seq.iter()
        .enumerate()
        .fold(0, |sum, (i, x)| sum + choose(*x as u8, i as u8 + 1))
}

pub fn index_to_sequence(index: u128) -> [u8; 13] {
    let mut result = [0; 13];
    let mut current_index = index;
    for l in (1..=13u8).rev() {
        // the lth element of the sequence is k such that
        // k choose l <= current_index and (k+1) choose l > current_index
        // while current_index
        let mut k = l;
        let mut prev_value = 0;
        let mut next_value = 1;
        while next_value <= current_index {
            k += 1;
            prev_value = next_value;
            next_value = choose(k, l);
        }
        current_index -= prev_value;
        result[(l - 1) as usize] = k - 1;
    }
    result
}

#[cfg(test)]
mod test {

    use crate::primitives::{Deal, Hand};
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case(1, "S:AKQJT98765432", "H:AKQJT98765432", "D:AKQJT98765432", "C:AKQJT98765432")]
    #[test_case(
        10,
        "S:AKQJT98765432",
        "H:AKQJT98765432",
        "D:AKQJ98765432, C:A",
        "C:KQJT98765432, D:T"
    )]
    #[test_case(
        1000000000000000000000000000,
        "♠:KT862,♥:J62,♦:9632,♣:A",
        "♠:A93,♥:K94,♦:J4,♣:K9765",
        "♠:4,♥:A5,♦:AKT75,♣:QJT43",
        "♠:QJ75,♥:QT873,♦:Q8,♣:82"
    )]
    fn andrews_book(page: u128, north: &str, east: &str, south: &str, west: &str) {
        let north = Hand::<13>::from_str(north).unwrap();
        let east = Hand::<13>::from_str(east).unwrap();
        let south = Hand::<13>::from_str(south).unwrap();
        let west = Hand::<13>::from_str(west).unwrap();

        let expected = Deal::from_hands([north, east, south, west]);

        let deal = super::deal_from_human_andrews_page(page);

        let constructed_page = super::find_human_page_number_for_deal_in_andrews_book(deal);

        assert_eq!(deal, expected);
        assert_eq!(constructed_page, page);
    }

    #[test_case(1, "S:AKQJT98765432", "H:AKQJT98765432", "D:AKQJT98765432", "C:AKQJT98765432")]
    #[test_case(
        10,
        "S:AKQJT98765432",
        "H:AKQJT98765432",
        "D:AKQJT9876543, C:6",
        "C:AKQJT9875432, D:2"
    )]
    #[test_case(
        1000000000000000000000000000,
        "♠:AK72,♥:Q543,♦:T863,♣:8",
        "♠:QJ84,♥:KT762,♦:AK4,♣:7",
        "♠:T953,♥:9,♦:972,♣:QJ643",
        "♠:6,♥:AJ8,♦:QJ5,♣:AKT952"
    )]
    fn pavliceks_book(page: u128, north: &str, east: &str, south: &str, west: &str) {
        let north = Hand::<13>::from_str(north).unwrap();
        let east = Hand::<13>::from_str(east).unwrap();
        let south = Hand::<13>::from_str(south).unwrap();
        let west = Hand::<13>::from_str(west).unwrap();

        let expected = Deal::from_hands([north, east, south, west]);

        let deal = super::deal_from_human_pavlicek_page(page);

        let constructed_page = super::find_human_page_number_for_deal_in_pavliceks_book(deal);

        assert_eq!(deal, expected);
        assert_eq!(constructed_page, page);
    }
}
