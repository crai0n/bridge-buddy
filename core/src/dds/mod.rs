use crate::dds::dds_state::DdsState;
use crate::primitives::deal::Seat;
use crate::primitives::{Card, Deal, Suit};
use double_dummy_result::DoubleDummyResult;
use strum::IntoEnumIterator;

pub mod card_tracker;
pub mod dds_state;
pub mod dds_trick_manager;
mod double_dummy_result;
mod relative_rank;

pub struct DoubleDummySolver<const N: usize> {}

impl<const N: usize> DoubleDummySolver<N> {
    pub fn solve(deal: Deal<N>) -> DoubleDummyResult {
        let mut result_vec = Vec::new();

        for declarer in Seat::iter() {
            let result = Self::calculate_max_tricks_for_declarer(deal, declarer);
            result_vec.extend_from_slice(&result);
        }

        DoubleDummyResult {
            max_tricks: result_vec.try_into().unwrap(),
        }
    }

    fn calculate_max_tricks_for_declarer(deal: Deal<N>, declarer: Seat) -> [usize; 5] {
        let mut result = Vec::new();

        let max_tricks = Self::calculate_max_tricks_for_declarer_with_trumps(deal, declarer, None);
        result.push(max_tricks);
        for suit in Suit::iter().rev() {
            let max_tricks = Self::calculate_max_tricks_for_declarer_with_trumps(deal, declarer, Some(suit));
            result.push(max_tricks);
        }
        result.try_into().unwrap()
    }

    fn calculate_max_tricks_for_declarer_with_trumps(deal: Deal<N>, declarer: Seat, trumps: Option<Suit>) -> usize {
        Self::determine_maximum_achievable_tricks(deal, trumps, declarer)
    }

    fn determine_maximum_achievable_tricks(deal: Deal<N>, trumps: Option<Suit>, declarer: Seat) -> usize {
        let max_tricks = N;

        // adapted from Min-Sheng Chang (1996)
        let mut at_least = 0;
        let mut at_most = max_tricks;

        while at_least < at_most {
            let target = (at_least + at_most + 1) / 2;
            // println!("------------------------");
            // println!(
            //     "Testing {} tricks for {} as declarer and {:?} as trumps.",
            //     target, declarer, trumps
            // );

            if Self::declarer_can_achieve_target(deal, declarer, trumps, target) {
                // println!("Declarer can reach their target!");
                at_least = target;
            } else {
                // println!("Defenders can reach their target!");
                at_most = target - 1;
            }
        }
        at_least
    }

    fn declarer_can_achieve_target(deal: Deal<N>, declarer: Seat, trumps: Option<Suit>, target: usize) -> bool {
        !Self::defenders_can_achieve_target(deal, declarer, trumps, N + 1 - target)
    }

    fn defenders_can_achieve_target(deal: Deal<N>, declarer: Seat, trumps: Option<Suit>, target: usize) -> bool {
        let opening_leader = declarer + 1;
        let mut start_state = DdsState::new(deal.hands, opening_leader, trumps);
        Self::can_achieve_target(&mut start_state, target)
    }

    fn can_achieve_target(state: &mut DdsState<N>, target: usize) -> bool {
        // println!("Now checking target {} for player {}", target, state.next_to_play());
        // println!(
        //     "{:?} has won {} tricks",
        //     state.next_to_play().axis(),
        //     state.tricks_won_by_axis(state.next_to_play())
        // );
        // println!("There are {} tricks left to play", state.tricks_left());
        if Self::already_enough_tricks_for_target(state, target) {
            // println!("Already won enough tricks!");
            return true;
        };
        if Self::not_enough_tricks_left_to_achieve_target(state, target) {
            // println!("Not enough tricks left!");
            return false;
        };
        if Self::have_enough_quick_tricks_for_target(state, target) {
            // println!("Enough quick tricks for target!");
            return true;
        };
        if Self::only_one_trick_left_to_play(state) {
            // println!("Checking last trick!");
            return Self::we_win_last_trick(state);
        }
        // println!("generating possible moves!");
        let available_moves = Self::generate_moves(state);

        for candidate_move in available_moves {
            // println!("trying card {} for {}!", test_move, state.next_to_play());
            let move_achieves_target = Self::apply_move_and_recurse(state, target, candidate_move);
            if move_achieves_target {
                return true;
            }
        }
        false
    }

    fn have_enough_quick_tricks_for_target(state: &DdsState<N>, target: usize) -> bool {
        target <= state.tricks_won_by_axis(state.next_to_play()) + Self::quick_tricks_for_player(state)
    }

    fn quick_tricks_for_player(_state: &DdsState<N>) -> usize {
        0
    }

    fn only_one_trick_left_to_play(state: &mut DdsState<{ N }>) -> bool {
        state.tricks_left() == 1
    }

    fn not_enough_tricks_left_to_achieve_target(state: &mut DdsState<{ N }>, target: usize) -> bool {
        target > state.tricks_left() + state.tricks_won_by_axis(state.next_to_play())
    }

    fn already_enough_tricks_for_target(state: &mut DdsState<{ N }>, target: usize) -> bool {
        target <= state.tricks_won_by_axis(state.next_to_play())
    }

    fn apply_move_and_recurse(state: &mut DdsState<{ N }>, target: usize, test_move: Card) -> bool {
        let current_player = state.next_to_play();
        state.play(test_move);
        let new_player = state.next_to_play();
        let we_can_achieve_target = if current_player.same_axis(&new_player) {
            Self::can_achieve_target(state, target)
        } else {
            !Self::can_achieve_target(state, N + 1 - target)
        };
        state.undo();
        we_can_achieve_target
    }

    fn we_win_last_trick(state: &mut DdsState<N>) -> bool {
        let lead = state.next_to_play();

        Self::play_last_trick(state);

        // println!("{:?} has won the last trick!", state.last_trick_winner());

        let result = state.last_trick_winner().unwrap().same_axis(&lead);

        // if result {
        //     println!("{:?} has won the last trick!", lead.axis())
        // } else {
        //     println!("{:?} has lost the last trick!", lead.axis())
        // }

        Self::undo_last_trick(state);

        result
    }

    fn play_last_trick(state: &mut DdsState<N>) {
        for _ in 0..4 {
            let last_card_of_player = *state.available_cards_of(state.next_to_play()).first().unwrap();

            state.play(last_card_of_player);
        }
    }

    fn undo_last_trick(state: &mut DdsState<N>) {
        for _ in 0..4 {
            state.undo();
        }
    }

    fn generate_moves(state: &DdsState<N>) -> Vec<Card> {
        let available_cards = state.available_cards_of(state.next_to_play());

        let mut relevant_cards = state.played_cards();
        relevant_cards.extend_from_slice(&available_cards);

        let mut moves = Vec::with_capacity(available_cards.len());

        for card in available_cards.iter().rev() {
            let mut equivalent = card;

            // if we have touching cards in our hand or already played, this is not a new move
            while let Some(other) = relevant_cards.iter().find(|other| other.touches(equivalent)) {
                // this can be optimized when relevant_cards is sorted correctly
                equivalent = other;
            }
            if !moves.contains(equivalent) {
                moves.push(*card)
            }
        }

        moves
    }
}

#[cfg(test)]
mod test {
    use crate::dds::DoubleDummySolver;
    use std::str::FromStr;
    use strum::IntoEnumIterator;
    // use crate::primitives::deal::Seat;
    use crate::primitives::{Deal, Hand, Suit};
    // use strum::IntoEnumIterator;
    use crate::primitives::deal::{Board, Seat};
    use test_case::test_case;

    #[test_case( 1u64, [0,0,1,0,1,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0]; "Test A")]
    #[test_case( 2u64, [0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,0,0,1,0,1]; "Test B")]
    fn solve1(seed: u64, expected: [usize; 20]) {
        let deal: Deal<1> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let dds_result = DoubleDummySolver::solve(deal);
        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 1u64, [0,2,2,0,0,0,0,0,2,2,0,2,2,0,0,0,0,0,2,2]; "Test A")]
    #[test_case( 2u64, [1,1,0,1,2,0,1,2,1,0,0,1,0,0,2,1,1,2,1,0]; "Test B")]
    fn solve2(seed: u64, expected: [usize; 20]) {
        let deal: Deal<2> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let dds_result = DoubleDummySolver::solve(deal);
        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    // #[ignore]
    #[test_case( 1u64, [0, 0, 0, 1, 0, 2, 1, 2, 1, 2, 1, 2, 0, 1, 0, 2, 1, 2, 1, 2]; "Test A")]
    #[test_case( 2u64, [2, 0, 3, 3, 3, 0, 3, 0, 1, 0, 2, 0, 3, 2, 3, 0, 3, 0, 0, 0]; "Test B")]
    fn solve3(seed: u64, expected: [usize; 20]) {
        let deal: Deal<3> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let dds_result = DoubleDummySolver::solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 1u64, [3, 2, 4, 3, 4, 2, 2, 1, 2, 1, 3, 2, 4, 3, 4, 2, 2, 1, 2, 1]; "Test A")]
    #[test_case( 2u64, [0, 2, 1, 0, 3, 4, 3, 4, 5, 2, 1, 2, 1, 0, 3, 4, 3, 4, 5, 2]; "Test B")]
    fn solve5(seed: u64, expected: [usize; 20]) {
        let deal: Deal<5> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let dds_result = DoubleDummySolver::solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( "S:A", "H:A", "C:A", "D:A", [0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0]; "Test A")]
    fn solve_explicit1(north: &str, east: &str, south: &str, west: &str, expected: [usize; 20]) {
        let north_hand = Hand::<1>::from_str(north).unwrap();
        let east_hand = Hand::<1>::from_str(east).unwrap();
        let south_hand = Hand::<1>::from_str(south).unwrap();
        let west_hand = Hand::<1>::from_str(west).unwrap();

        let deal = Deal {
            board: Board::from_number(1),
            hands: [north_hand, east_hand, south_hand, west_hand],
        };

        let dds_result = DoubleDummySolver::solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[ignore]
    #[test_case( "S:8654,H:J964,D:75,C:K98", "S:J92,H:KT83,D:AK64,C:AQ", "S:AQ7,H:A7,D:QJ83,C:J764", "S:KT3, H:Q52,D:T92,C:T532", [0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0]; "Test A")]
    fn solve_explicit13(north: &str, east: &str, south: &str, west: &str, expected: [usize; 20]) {
        let north_hand = Hand::<13>::from_str(north).unwrap();
        let east_hand = Hand::<13>::from_str(east).unwrap();
        let south_hand = Hand::<13>::from_str(south).unwrap();
        let west_hand = Hand::<13>::from_str(west).unwrap();

        let deal = Deal {
            board: Board::from_number(1),
            hands: [north_hand, east_hand, south_hand, west_hand],
        };

        for (seat, hand) in Seat::iter().zip(deal.hands) {
            println!("{}:\n{}", seat, hand)
        }

        let dds_result = DoubleDummySolver::solve(deal);

        println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[ignore]
    #[test_case( "S:8654,H:J964,D:75,C:K98", "S:J92,H:KT83,D:AK64,C:AQ", "S:AQ7,H:A7,D:QJ83,C:J764", "S:KT3, H:Q52,D:T92,C:T532", None, Seat::West, 9; "Test A")]
    fn solve_single13(
        north: &str,
        east: &str,
        south: &str,
        west: &str,
        trumps: Option<Suit>,
        declarer: Seat,
        expected: usize,
    ) {
        let north_hand = Hand::<13>::from_str(north).unwrap();
        let east_hand = Hand::<13>::from_str(east).unwrap();
        let south_hand = Hand::<13>::from_str(south).unwrap();
        let west_hand = Hand::<13>::from_str(west).unwrap();

        let deal = Deal {
            board: Board::from_number(1),
            hands: [north_hand, east_hand, south_hand, west_hand],
        };

        for (seat, hand) in Seat::iter().zip(deal.hands) {
            println!("{}:\n{}", seat, hand)
        }

        let dds_result = DoubleDummySolver::calculate_max_tricks_for_declarer_with_trumps(deal, declarer, trumps);

        println!("{}", dds_result);
        assert_eq!(dds_result, expected);
    }
}
