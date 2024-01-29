use crate::dds::dds_move::DdsMove;
use crate::dds::virtual_state::VirtualState;
use bridge_buddy_core::primitives::contract::Strain;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::Deal;
use double_dummy_result::DoubleDummyResult;
use enum_iterator::all;
use strum::IntoEnumIterator;

pub mod card_manager;
mod dds_move;
mod double_dummy_result;
pub mod double_dummy_state;
mod virtual_card;
pub mod virtual_state;
// mod transposition_table;
// mod double_dummy_solver;

pub struct DoubleDummySolver<const N: usize> {}

impl<const N: usize> DoubleDummySolver<N> {
    const CHECK_QUICK_TRICKS: bool = true;

    pub fn solve(deal: Deal<N>) -> DoubleDummyResult {
        for (seat, hand) in Seat::iter().zip(deal.hands) {
            println!("{}:\n{}", seat, hand)
        }

        let mut result = DoubleDummyResult::new();

        for declarer in Seat::iter() {
            for strain in all::<Strain>() {
                let opening_leader = declarer + 1;
                let defenders_tricks = Self::solve_initial_position(deal, strain, opening_leader);
                result.set_tricks_for_declarer_in_strain(N - defenders_tricks, declarer, strain);
            }
        }

        println!("{}", result);
        result
    }

    fn solve_initial_position(deal: Deal<N>, strain: Strain, opening_leader: Seat) -> usize {
        let mut at_least = 0;
        let mut at_most = N; // at_most = b - 1;

        while at_least < at_most {
            let estimate = (at_least + at_most + 1) / 2;
            println!("------------------------");
            println!(
                "Trying to make {} tricks for {} as opening leader and {:?}.",
                estimate, opening_leader, strain
            );

            let trumps = match strain {
                Strain::Trump(suit) => Some(suit),
                _ => None,
            };

            let mut start_state = VirtualState::new(deal.hands, opening_leader, trumps);

            let score = Self::score_node(&mut start_state, estimate);
            println!("Scored {} tricks for defenders", score);
            if score >= estimate {
                at_least = score;
            } else {
                at_most = score;
            }
        }
        at_least
    }

    fn score_node(state: &mut VirtualState<N>, estimate: usize) -> usize {
        if let Some(early_score) = Self::try_early_node_score(state, estimate) {
            return early_score;
        }

        if state.is_last_trick() {
            return Self::score_terminal_node(state);
        }

        // println!("generating possible moves!");
        let available_moves = Self::generate_moves(state);
        let mut highest_score = 0;
        for candidate_move in available_moves {
            // println!("trying card {} for {}!", candidate_move, state.next_to_play());
            let current_player = state.next_to_play();

            state.play(candidate_move.card).unwrap();
            let new_player = state.next_to_play();
            let score = if current_player.same_axis(&new_player) {
                Self::score_node(state, estimate)
            } else {
                N - Self::score_node(state, N + 1 - estimate)
            };
            state.undo();

            if score >= estimate {
                return score;
            } else if score > highest_score {
                // if we cannot reach estimate, we need the highest score found
                highest_score = score
            }
        }
        highest_score
    }

    fn try_early_node_score(state: &mut VirtualState<N>, estimate: usize) -> Option<usize> {
        let current_tricks = Self::current_tricks(state);
        if current_tricks >= estimate {
            // println!("Already won enough tricks!");
            return Some(current_tricks);
        };
        let maximum_tricks = Self::maximum_achievable_tricks(state);
        if maximum_tricks < estimate {
            // println!("Not enough tricks left!");
            return Some(maximum_tricks);
        };
        if Self::CHECK_QUICK_TRICKS && state.player_is_leading() {
            let total_with_quick_tricks =
                state.tricks_won_by_axis(state.next_to_play()) + Self::quick_tricks_for_current_player(state) as usize;
            // println!("Enough quick tricks for target!");
            if estimate <= total_with_quick_tricks {
                return Some(total_with_quick_tricks);
            }
        }
        None
    }

    fn quick_tricks_for_current_player(state: &VirtualState<N>) -> u8 {
        state.quick_tricks_for_player(state.next_to_play())
    }

    fn maximum_achievable_tricks(state: &mut VirtualState<{ N }>) -> usize {
        state.tricks_left() + state.tricks_won_by_axis(state.next_to_play())
    }

    fn current_tricks(state: &mut VirtualState<{ N }>) -> usize {
        state.tricks_won_by_axis(state.next_to_play())
    }

    fn score_terminal_node(state: &mut VirtualState<N>) -> usize {
        let lead = state.next_to_play();

        Self::play_last_trick(state);

        // println!("{:?} has won the last trick!", state.last_trick_winner());

        let result = state.tricks_won_by_axis(lead);

        // if result {
        //     println!("{:?} has won the last trick!", lead.axis())
        // } else {
        //     println!("{:?} has lost the last trick!", lead.axis())
        // }

        Self::undo_last_trick(state);

        result
    }

    fn play_last_trick(state: &mut VirtualState<N>) {
        for _ in 0..4 {
            let last_card_of_player = state.valid_moves().first().unwrap().card;

            state.play(last_card_of_player).unwrap();
        }
    }

    fn undo_last_trick(state: &mut VirtualState<N>) {
        for _ in 0..4 {
            state.undo();
        }
    }

    fn generate_moves(state: &VirtualState<N>) -> Vec<DdsMove> {
        let valid_moves = state.valid_moves();
        let mut unique_moves = Self::select_one_move_per_sequence(&valid_moves);
        Self::prioritize_moves(&mut unique_moves, state);
        Self::sort_moves_by_priority_descending(&mut unique_moves);
        unique_moves
    }

    fn prioritize_moves(moves: &mut [DdsMove], state: &VirtualState<N>) {
        match state.count_cards_in_current_trick() {
            0 => Self::prioritize_moves_for_leading_hand(moves, state),
            1 => Self::prioritize_moves_for_second_hand(moves, state),
            2 => Self::prioritize_moves_for_third_hand(moves, state),
            3 => Self::prioritize_moves_for_last_hand(moves, state),
            _ => unreachable!(),
        };
    }

    fn prioritize_moves_for_leading_hand(moves: &mut [DdsMove], state: &VirtualState<N>) {
        for dds_move in moves {
            match state.trumps() {
                None => {
                    if dds_move.sequence_length >= 3 {
                        dds_move.priority += 50;
                    }
                }
                Some(trump_suit) => {
                    if dds_move.sequence_length >= 2 {
                        dds_move.priority += 50;
                    }

                    let our_trump_count = state.count_this_sides_trump_cards();
                    let opponents_trump_count = state.count_opponents_trump_cards();

                    if our_trump_count >= opponents_trump_count && dds_move.card.suit == trump_suit {
                        dds_move.priority += 100;
                    }
                }
            }
        }
    }
    fn prioritize_moves_for_second_hand(moves: &mut [DdsMove], _state: &VirtualState<N>) {
        for dds_move in moves {
            dds_move.priority += 15 - dds_move.card.rank as usize;
        }
    }
    fn prioritize_moves_for_third_hand(moves: &mut [DdsMove], state: &VirtualState<N>) {
        for dds_move in moves {
            if dds_move.card > state.currently_winning_card().unwrap() {
                dds_move.priority += dds_move.card.rank as usize;
            }
        }
    }
    fn prioritize_moves_for_last_hand(moves: &mut [DdsMove], state: &VirtualState<N>) {
        for dds_move in moves {
            if state.current_trick_winner() == state.next_to_play().partner() {
                dds_move.priority += 15 - dds_move.card.rank as usize;
            } else {
                dds_move.priority += dds_move.card.rank as usize;
            }

            if dds_move.card > state.currently_winning_card().unwrap() {
                break;
            }
        }
    }

    fn sort_moves_by_priority_descending(moves: &mut [DdsMove]) {
        moves.sort_unstable_by(|a, b| a.priority.cmp(&b.priority));
    }

    fn select_one_move_per_sequence(moves: &[DdsMove]) -> Vec<DdsMove> {
        let mut output: Vec<DdsMove> = vec![];
        for &candidate_move in moves {
            if let Some(last) = output.last_mut() {
                if candidate_move.card.touches(&last.card) {
                    last.sequence_length += 1;
                } else {
                    output.push(candidate_move);
                }
            } else {
                output.push(candidate_move);
            }
        }
        output
    }
}

#[cfg(test)]
mod test {
    use crate::dds::DoubleDummySolver;
    use bridge_buddy_core::primitives::contract::Strain;
    use bridge_buddy_core::primitives::deal::{Board, Seat};
    use bridge_buddy_core::primitives::{Deal, Hand};
    use std::str::FromStr;
    use strum::IntoEnumIterator;
    use test_case::test_case;

    #[test_case( 1u64, [1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0]; "Test A")]
    #[test_case( 2u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test B")]
    fn solve1(seed: u64, expected: [usize; 20]) {
        let deal: Deal<1> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let dds_result = DoubleDummySolver::solve(deal);
        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 1u64, [0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0]; "Test A")]
    #[test_case( 2u64, [2, 1, 0, 1, 1, 0, 1, 2, 1, 0, 2, 0, 0, 1, 0, 0, 1, 2, 1, 1]; "Test B")]
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
    #[test_case( 1u64, [0, 1, 0, 0, 0, 2, 1, 2, 1, 2, 0, 1, 0, 2, 1, 2, 1, 2, 1, 2]; "Test A")]
    #[test_case( 2u64, [3, 3, 3, 0, 2, 0, 1, 0, 3, 0, 3, 2, 3, 0, 2, 0, 0, 0, 3, 0]; "Test B")]
    fn solve3(seed: u64, expected: [usize; 20]) {
        let deal: Deal<3> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let dds_result = DoubleDummySolver::solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 1u64, [4, 3, 4, 2, 3, 1, 2, 1, 2, 2, 4, 3, 4, 2, 3, 1, 2, 1, 2, 2]; "Test A")]
    #[test_case( 2u64, [3, 0, 1, 2, 0, 2, 5, 4, 3, 4, 3, 0, 1, 2, 1, 2, 5, 4, 3, 4]; "Test B")]
    #[test_case( 20u64, [3, 0, 3, 0, 0, 0, 4, 1, 4, 3, 4, 0, 4, 0, 0, 1, 4, 1, 5, 3]; "Test C")]
    #[test_case( 38u64, [0, 1, 4, 3, 1, 4, 3, 1, 2, 1, 0, 1, 4, 3, 0, 4, 3, 1, 2, 1]; "Test D")]
    fn solve5(seed: u64, expected: [usize; 20]) {
        let deal: Deal<5> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let dds_result = DoubleDummySolver::solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 14u64, [2, 1, 5, 4, 5, 4, 4, 0, 2, 1, 2, 1, 6, 3, 2, 4, 4, 0, 2, 1]; "Test A")]
    #[test_case( 22u64, [1, 1, 6, 6, 3, 4, 4, 0, 0, 2, 1, 1, 6, 6, 3, 5, 4, 0, 0, 2]; "Test B")]
    fn solve6(seed: u64, expected: [usize; 20]) {
        let deal: Deal<6> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let dds_result = DoubleDummySolver::solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[ignore]
    #[test_case( 37u64, [4, 2, 1, 1, 1, 4, 5, 7, 7, 6, 4, 2, 1, 1, 1, 4, 5, 7, 7, 6]; "Test A")]
    #[test_case( 82u64, [6, 8, 5, 4, 5, 2, 0, 2, 3, 2, 6, 8, 5, 4, 5, 2, 0, 2, 3, 2]; "Test B")]
    fn solve8(seed: u64, expected: [usize; 20]) {
        let deal: Deal<8> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let dds_result = DoubleDummySolver::solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[ignore]
    #[test_case( 37u64, [1, 0, 2, 3, 0, 8, 9, 7, 6, 9, 1, 0, 2, 3, 0, 7, 9, 5, 6, 8]; "Test A")]
    #[test_case( 82u64, [4, 1, 4, 2, 2, 5, 8, 4, 6, 7, 4, 1, 4, 2, 2, 5, 8, 4, 6, 7]; "Test B")]
    fn solve9(seed: u64, expected: [usize; 20]) {
        let deal: Deal<9> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let dds_result = DoubleDummySolver::solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( "S:A", "H:A", "C:A", "D:A", [1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0]; "Test A")]
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
    #[test_case( "S:8654,H:J964,D:75,C:K98", "S:J92,H:KT83,D:AK64,C:AQ", "S:AQ7,H:A7,D:QJ83,C:J764", "S:KT3, H:Q52,D:T92,C:T532", Strain::NoTrump, Seat::West, 9; "Test A")]
    fn solve_single13(
        north: &str,
        east: &str,
        south: &str,
        west: &str,
        strain: Strain,
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

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let dds_result = DoubleDummySolver::solve_initial_position(deal, strain, declarer);

        // println!("{}", dds_result);
        assert_eq!(dds_result, expected);
    }
}
