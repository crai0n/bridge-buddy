use crate::impossible_book;
use crate::impossible_book::helper::u8_to_card;
use crate::impossible_book::{helper, HumanPageNumber, InternalPageNumber};
use crate::primitives::deal::seat::SEAT_ARRAY;
use crate::primitives::deal::Seat;
use crate::primitives::{Deal, Hand};

const NORTH_MAX: u128 = 635013559600; // 52 choose 13
const EAST_MAX: u128 = 8122425444; // 39 choose 13
const SOUTH_MAX: u128 = 10400600; // 26 choose 13

pub fn deal_from_human_andrews_page(page: HumanPageNumber) -> Deal<13> {
    impossible_book::deal_from_internal_andrews_page(page.into())
}

pub fn find_internal_page_number_for_deal_in_andrews_book(deal: Deal<13>) -> InternalPageNumber {
    // println!("Going backwards");

    let marked_deck = helper::create_marked_deck_from_deal(deal);

    let sequences = create_sequences_from_marked_deck(marked_deck);

    // println!("Sequences in Andrews Book:");
    // println!("North: {:?}", sequences[0]);
    // println!("East: {:?}", sequences[1]);
    // println!("South: {:?}", sequences[2]);

    let indexes = sequences.map(impossible_book::sequence_to_index);

    // println!(
    //     "Indexes in Andrews Book: N:{}, E:{}, S:{}",
    //     indexes[0], indexes[1], indexes[2]
    // );

    indexes_to_page(indexes)
}

pub fn find_human_page_number_for_deal_in_andrews_book(deal: Deal<13>) -> HumanPageNumber {
    find_internal_page_number_for_deal_in_andrews_book(deal).into()
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

pub fn deal_from_internal_andrews_page(page: InternalPageNumber) -> Deal<13> {
    // This calculates the deal according to Thomas Andrews' Algorithm
    // https://bridge.thomasoandrews.com/bridge/impossible/algorithm.html
    // using page in  0..<N_PAGES

    // println!("Going forwards:");
    //
    // println!("Page in Andrews Book: {}", page);

    let indexes = page_to_indexes(page);

    // println!(
    //     "Indexes in Andrews Book: N:{}, E:{}, S:{}",
    //     indexes[0], indexes[1], indexes[2]
    // );

    let sequences = indexes.map(impossible_book::index_to_sequence);

    // println!("Sequences in Andrews Book:");
    // println!("North: {:?}", sequences[0]);
    // println!("East: {:?}", sequences[1]);
    // println!("South: {:?}", sequences[2]);

    let marked_deck = mark_cards(sequences);

    let hands = distribute_cards(marked_deck);

    Deal::<13>::from_hands(hands)
}

fn mark_cards(sequences: [[u8; 13]; 3]) -> [Seat; 52] {
    // Creates a list of the Form NSENWNWNENSNE...NSW
    // Describing which player gets each card from a sorted deck
    let mut result = [Seat::West; 52];

    let mut counts = [0; 3];
    let mut cards_seen_by_player = [0; 3];

    for owner in &mut result {
        for player_index in 0..3 {
            cards_seen_by_player[player_index] += 1; // skip this card next

            if counts[player_index] < 13 {
                let player_should_get_this_card =
                    sequences[player_index][counts[player_index]] == cards_seen_by_player[player_index] as u8 - 1;
                if player_should_get_this_card {
                    *owner = SEAT_ARRAY[player_index];
                    counts[player_index] += 1;
                    break;
                }
            }
        }
    }
    result
}

fn distribute_cards(result: [Seat; 52]) -> [Hand<13>; 4] {
    let mut values = [[0; 13]; 4];
    let mut counts = [0; 4];

    for (i, x) in result.iter().enumerate() {
        values[*x as usize][counts[*x as usize]] = i as u8;
        counts[*x as usize] += 1;
    }

    // println!("Card Numbers in Andrews Book:");
    // println!("North: {:?}", values[0]);
    // println!("East: {:?}", values[1]);
    // println!("South: {:?}", values[2]);
    // println!("West: {:?}", values[3]);

    let cards = values.map(|x| x.map(u8_to_card));

    cards.map(|c| Hand::<13>::from_cards(&c).unwrap())
}

fn page_to_indexes(page: InternalPageNumber) -> [u128; 3] {
    let s_index = page.0 % SOUTH_MAX;
    let temp = page.0 / SOUTH_MAX;
    let e_index = temp % EAST_MAX;
    let n_index = temp / EAST_MAX % NORTH_MAX; // temp / EAST_MAX < NORTH_MAX iff page < N_PAGES !

    [n_index, e_index, s_index]
}

fn indexes_to_page(indexes: [u128; 3]) -> InternalPageNumber {
    InternalPageNumber::new(indexes[0] * EAST_MAX * SOUTH_MAX + indexes[1] * SOUTH_MAX + indexes[2])
}

pub fn sequence_to_index(seq: [usize; 13]) -> u128 {
    seq.iter()
        .enumerate()
        .fold(0, |sum, (i, x)| sum + helper::choose(*x as u8, i as u8 + 1))
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
            next_value = helper::choose(k, l);
        }
        current_index -= prev_value;
        result[(l - 1) as usize] = k - 1;
    }
    result
}
