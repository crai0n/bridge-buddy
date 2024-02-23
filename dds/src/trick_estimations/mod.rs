mod losing_tricks;
mod quick_tricks;

use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::Suit;
pub use losing_tricks::losing_tricks_for_leader;
pub use quick_tricks::quick_tricks_for_leader;
pub use quick_tricks::quick_tricks_for_second_hand;

#[derive(Clone)]
pub struct EstimationState {
    pub lead_suit: Option<Suit>,
    pub trump_suit: Option<Suit>,
    pub my_seat: Seat,
    pub card_counts: [[usize; 4]; 4],      // first index player, second index suit
    pub high_card_counts: [[usize; 4]; 4], // first index player, second index suit, only one player can ever have high cards in a suit
    pub our_combined_high_card_count: [[usize; 4]; 2],
    pub ace_owners: [Option<Seat>; 4],
    pub king_owners: [Option<Seat>; 4],
}
