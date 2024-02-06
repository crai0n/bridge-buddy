use crate::dds_config::DdsConfig;
use crate::move_generator::MoveGenerator;
use crate::primitives::DoubleDummyResult;
use crate::state::VirtualState;
use crate::transposition_table::TranspositionTable;
use crate::trick_estimations::losing_tricks_for_player;
use crate::trick_estimations::quick_tricks_for_player;
use bridge_buddy_core::engine::hand_evaluation::ForumDPlus2015Evaluator;
use bridge_buddy_core::primitives::contract::Strain;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::Deal;
use enum_iterator::all;
use std::cmp::min;
use strum::IntoEnumIterator;

// mod transposition_table;
// mod double_dummy_solver;

pub struct DoubleDummySolver<const N: usize> {
    config: DdsConfig,
    transposition_table: TranspositionTable,
}

impl<const N: usize> Default for DoubleDummySolver<N> {
    fn default() -> Self {
        Self::new(DdsConfig::default())
    }
}

impl<const N: usize> DoubleDummySolver<N> {
    pub fn new(config: DdsConfig) -> Self {
        Self {
            config,
            transposition_table: TranspositionTable::new(),
        }
    }

    pub fn solve(&mut self, deal: Deal<N>) -> DoubleDummyResult {
        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut result = DoubleDummyResult::new();

        for strain in all::<Strain>() {
            self.transposition_table.clear();
            for declarer in Seat::iter() {
                let opening_leader = declarer + 1;
                let defenders_tricks = self.solve_initial_position(deal, strain, opening_leader);
                result.set_tricks_for_declarer_in_strain(N - defenders_tricks, declarer, strain);
            }
        }

        // println!("{}", result);
        result
    }

    fn get_initial_estimate(deal: Deal<N>, strain: Strain, opening_leader: Seat) -> usize {
        let my_hand = deal.hand_of(opening_leader);
        let partners_hand = deal.hand_of(opening_leader + 2);
        match strain {
            Strain::NoTrump => {
                let my_ptc = ForumDPlus2015Evaluator::playing_trick_count(my_hand) as usize;
                let partners_ptc = ForumDPlus2015Evaluator::playing_trick_count(partners_hand) as usize;
                min(N, my_ptc + partners_ptc)
            }
            Strain::Trump(_) => {
                let my_ltc = ForumDPlus2015Evaluator::losing_trick_count(my_hand) as usize;
                let partners_ltc = ForumDPlus2015Evaluator::losing_trick_count(my_hand) as usize;
                let min_ltc = min(my_ltc, partners_ltc);
                N - min_ltc
            }
        }
    }

    fn solve_initial_position(&mut self, deal: Deal<N>, strain: Strain, opening_leader: Seat) -> usize {
        let mut at_least = 0;
        let mut at_most = N; // at_most = b - 1;

        let mut first_round = true;
        while at_least < at_most {
            let estimate = if self.config.pre_estimate && first_round {
                first_round = false;
                Self::get_initial_estimate(deal, strain, opening_leader)
            } else {
                (at_least + at_most + 1) / 2
            };

            let trumps = match strain {
                Strain::Trump(suit) => Some(suit),
                _ => None,
            };

            let mut start_state = VirtualState::new(deal.hands, opening_leader, trumps);

            let score = self.score_node(&mut start_state, estimate);
            // println!("Scored {} tricks for defenders", score);

            if score >= estimate {
                at_least = score;
            } else {
                at_most = score;
            }
        }
        at_least
    }

    fn score_node(&mut self, state: &mut VirtualState<N>, estimate: usize) -> usize {
        if let Some(early_score) = self.try_early_node_score(state, estimate) {
            return early_score;
        }

        if state.is_last_trick() {
            return self.score_terminal_node(state);
        }

        // println!("generating possible moves!");
        let available_moves = MoveGenerator::generate_moves(state, self.config.move_ordering);
        let mut highest_score = 0;
        for candidate_move in available_moves {
            // println!("trying card {} for {}!", candidate_move, state.next_to_play());
            let current_player = state.next_to_play();

            state.play(candidate_move.card).unwrap();
            let new_player = state.next_to_play();
            let score = if current_player.same_axis(&new_player) {
                self.score_node(state, estimate)
            } else {
                N - self.score_node(state, N + 1 - estimate)
            };
            state.undo();

            if score >= estimate {
                if self.config.use_transposition_table && state.player_is_leading() {
                    let add_tricks = score - state.tricks_won_by_axis(state.next_to_play());
                    self.store_lower_bound_in_tt(state, add_tricks);
                }
                return score;
            } else if score > highest_score {
                // if we cannot reach estimate, we need the highest score found
                highest_score = score;
            }
        }

        if self.config.use_transposition_table && state.player_is_leading() {
            let add_tricks = highest_score - state.tricks_won_by_axis(state.next_to_play());
            self.store_upper_bound_in_tt(state, add_tricks);
        }

        highest_score
    }

    fn try_find_node_in_tt(&self, state: &VirtualState<N>, estimate: usize) -> Option<usize> {
        let tt_key = state.generate_tt_key();
        match self.transposition_table.lookup(&tt_key) {
            None => None,
            Some(tt_value) => {
                let current_tricks = state.tricks_won_by_axis(state.next_to_play());
                let lower = current_tricks + tt_value.at_least;
                let upper = current_tricks + tt_value.at_most;
                if lower >= estimate {
                    Some(lower)
                } else if upper < estimate {
                    Some(upper)
                } else {
                    None
                }
            }
        }
    }

    fn try_early_node_score(&mut self, state: &mut VirtualState<N>, estimate: usize) -> Option<usize> {
        let current_tricks = Self::current_tricks(state);
        if current_tricks >= estimate {
            // storing in TT doesn't make sense as we can never improve lower bound here
            return Some(current_tricks);
        };

        let maximum_tricks = Self::maximum_achievable_tricks(state);
        if maximum_tricks < estimate {
            // storing in TT doesn't make sense as we can never improve upper bound here
            return Some(maximum_tricks);
        };

        if self.config.use_transposition_table && state.player_is_leading() {
            if let Some(tt_score) = self.try_find_node_in_tt(state, estimate) {
                return Some(tt_score);
            }
        }

        if self.config.check_quick_tricks && state.player_is_leading() {
            if let Some(qt_score) = self.try_score_using_quick_tricks(state, estimate) {
                return Some(qt_score);
            }
        }

        if self.config.check_losing_tricks && state.player_is_leading() {
            if let Some(lt_score) = self.try_score_using_losing_tricks(state, estimate) {
                return Some(lt_score);
            }
        }

        None
    }

    fn try_score_using_losing_tricks(&mut self, state: &VirtualState<N>, estimate: usize) -> Option<usize> {
        let losing_tricks = Self::losing_tricks_for_current_player(state);
        let total_with_losing_tricks = Self::maximum_achievable_tricks(state) - losing_tricks;
        if total_with_losing_tricks < estimate {
            if self.config.use_transposition_table {
                self.store_upper_bound_in_tt(state, total_with_losing_tricks - Self::current_tricks(state));
            }
            return Some(total_with_losing_tricks);
        }
        None
    }

    fn try_score_using_quick_tricks(&mut self, state: &VirtualState<N>, estimate: usize) -> Option<usize> {
        let quick_tricks = Self::quick_tricks_for_current_player(state);
        let total_with_quick_tricks = state.tricks_won_by_axis(state.next_to_play()) + quick_tricks;
        if total_with_quick_tricks >= estimate {
            if self.config.use_transposition_table {
                self.store_lower_bound_in_tt(state, quick_tricks);
            }
            return Some(total_with_quick_tricks);
        }
        None
    }

    fn store_lower_bound_in_tt(&mut self, state: &VirtualState<N>, bound: usize) {
        let tt_key = state.generate_tt_key();
        self.transposition_table.update_lower_bound(&tt_key, bound)
    }

    fn store_upper_bound_in_tt(&mut self, state: &VirtualState<N>, bound: usize) {
        let tt_key = state.generate_tt_key();
        self.transposition_table.update_upper_bound(&tt_key, bound)
    }

    fn quick_tricks_for_current_player(state: &VirtualState<N>) -> usize {
        quick_tricks_for_player(state, state.next_to_play())
    }

    fn losing_tricks_for_current_player(state: &VirtualState<N>) -> usize {
        losing_tricks_for_player(state, state.next_to_play())
    }

    fn maximum_achievable_tricks(state: &VirtualState<{ N }>) -> usize {
        state.tricks_left() + state.tricks_won_by_axis(state.next_to_play())
    }

    fn current_tricks(state: &VirtualState<{ N }>) -> usize {
        state.tricks_won_by_axis(state.next_to_play())
    }

    fn score_terminal_node(&mut self, state: &mut VirtualState<N>) -> usize {
        let lead = state.next_to_play();

        Self::play_last_trick(state);

        let score = state.tricks_won_by_axis(lead);
        let winner_of_last_trick = state.last_trick_winner().unwrap();

        Self::undo_last_trick(state);

        if self.config.use_transposition_table && state.player_is_leading() {
            self.store_terminal_node_in_tt(state, winner_of_last_trick);
        }

        score
    }

    fn store_terminal_node_in_tt(&mut self, state: &VirtualState<N>, winner_of_last_trick: Seat) {
        if winner_of_last_trick.same_axis(&state.next_to_play()) {
            self.store_lower_bound_in_tt(state, 1);
        } else {
            self.store_upper_bound_in_tt(state, 0);
        }
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
}

#[cfg(test)]
mod test {
    use super::DoubleDummySolver;
    use bridge_buddy_core::primitives::contract::Strain;
    use bridge_buddy_core::primitives::deal::{Board, Seat};
    use bridge_buddy_core::primitives::{Deal, Hand, Suit};
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case( 30u64, [1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0]; "Test A")]
    #[test_case( 31u64, [1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0]; "Test B")]
    #[test_case( 32u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test C")]
    #[test_case( 33u64, [1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0]; "Test D")]
    #[test_case( 34u64, [0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0]; "Test E")]
    #[test_case( 35u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test F")]
    #[test_case( 36u64, [1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0]; "Test G")]
    #[test_case( 37u64, [1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0]; "Test H")]
    #[test_case( 38u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test I")]
    #[test_case( 39u64, [1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0]; "Test J")]
    #[test_case( 40u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test K")]
    #[test_case( 41u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test L")]
    #[test_case( 42u64, [1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0]; "Test M")]
    #[test_case( 43u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test N")]
    #[test_case( 44u64, [1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0]; "Test O")]
    #[test_case( 45u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test P")]
    #[test_case( 46u64, [0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0]; "Test Q")]
    #[test_case( 47u64, [0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0]; "Test R")]
    #[test_case( 48u64, [1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0]; "Test S")]
    #[test_case( 49u64, [1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0]; "Test T")]
    fn solve1(seed: u64, expected: [usize; 20]) {
        let deal: Deal<1> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);
        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 30u64, [1, 2, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0]; "Test A")]
    #[test_case( 31u64, [1, 2, 1, 0, 1, 0, 0, 0, 2, 0, 1, 2, 1, 0, 1, 0, 0, 0, 2, 0]; "Test B")]
    #[test_case( 32u64, [0, 2, 2, 0, 0, 2, 0, 0, 2, 0, 0, 2, 2, 0, 0, 2, 0, 0, 2, 0]; "Test C")]
    #[test_case( 33u64, [0, 0, 2, 1, 0, 1, 2, 0, 1, 0, 1, 0, 2, 1, 1, 1, 2, 0, 0, 0]; "Test D")]
    #[test_case( 34u64, [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0]; "Test E")]
    #[test_case( 35u64, [0, 1, 1, 2, 0, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0]; "Test F")]
    #[test_case( 36u64, [2, 1, 0, 1, 1, 0, 1, 2, 1, 0, 2, 1, 0, 1, 0, 0, 0, 2, 1, 0]; "Test G")]
    #[test_case( 37u64, [0, 0, 0, 2, 0, 1, 1, 2, 0, 1, 0, 0, 0, 2, 0, 1, 1, 2, 0, 1]; "Test H")]
    #[test_case( 38u64, [0, 1, 1, 2, 1, 2, 0, 1, 0, 0, 0, 1, 1, 2, 1, 2, 0, 0, 0, 0]; "Test I")]
    #[test_case( 39u64, [2, 2, 0, 2, 2, 0, 0, 2, 0, 0, 2, 1, 0, 1, 0, 0, 0, 2, 0, 0]; "Test J")]
    #[test_case( 40u64, [0, 2, 0, 0, 0, 1, 0, 1, 2, 1, 0, 2, 0, 0, 0, 1, 0, 1, 2, 1]; "Test K")]
    #[test_case( 41u64, [1, 0, 0, 2, 0, 0, 2, 1, 0, 0, 1, 0, 1, 2, 1, 1, 2, 1, 0, 0]; "Test L")]
    #[test_case( 42u64, [1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 2, 1, 2, 2, 1, 0, 1, 0, 0]; "Test M")]
    #[test_case( 43u64, [1, 1, 0, 2, 1, 0, 1, 2, 0, 0, 1, 0, 0, 2, 0, 1, 1, 2, 0, 0]; "Test N")]
    #[test_case( 44u64, [2, 1, 0, 1, 1, 0, 1, 2, 1, 1, 2, 1, 0, 1, 0, 0, 0, 2, 1, 0]; "Test O")]
    #[test_case( 45u64, [0, 2, 0, 1, 0, 1, 0, 2, 1, 1, 1, 2, 0, 0, 0, 1, 0, 2, 1, 1]; "Test P")]
    #[test_case( 46u64, [0, 2, 0, 0, 0, 2, 0, 1, 1, 1, 0, 2, 0, 0, 0, 2, 0, 1, 1, 1]; "Test Q")]
    #[test_case( 47u64, [0, 2, 0, 0, 0, 2, 0, 1, 1, 1, 0, 2, 1, 0, 0, 2, 0, 1, 1, 1]; "Test R")]
    #[test_case( 48u64, [1, 2, 1, 0, 1, 0, 0, 1, 2, 0, 1, 2, 1, 0, 0, 1, 0, 1, 2, 1]; "Test S")]
    #[test_case( 49u64, [2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0]; "Test T")]
    fn solve2(seed: u64, expected: [usize; 20]) {
        let deal: Deal<2> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);
        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    // #[ignore]
    #[test_case( 30u64, [1, 3, 3, 1, 1, 2, 0, 0, 0, 0, 1, 3, 3, 1, 1, 2, 0, 0, 2, 0]; "Test A")]
    #[test_case( 31u64, [2, 3, 0, 0, 0, 0, 0, 3, 2, 0, 2, 3, 0, 0, 0, 0, 0, 3, 2, 0]; "Test B")]
    #[test_case( 32u64, [0, 3, 0, 0, 0, 3, 0, 2, 3, 1, 0, 3, 0, 0, 0, 2, 0, 2, 2, 1]; "Test C")]
    #[test_case( 33u64, [1, 2, 3, 2, 2, 0, 1, 0, 0, 0, 1, 1, 2, 2, 2, 2, 1, 0, 0, 0]; "Test D")]
    #[test_case( 34u64, [0, 1, 1, 2, 0, 3, 2, 2, 1, 3, 0, 1, 1, 0, 0, 3, 2, 2, 1, 3]; "Test E")]
    #[test_case( 35u64, [2, 2, 1, 1, 1, 0, 0, 1, 1, 0, 2, 3, 2, 1, 3, 0, 0, 1, 1, 0]; "Test F")]
    #[test_case( 36u64, [1, 0, 0, 2, 0, 2, 2, 2, 1, 3, 1, 0, 0, 0, 0, 2, 3, 3, 1, 3]; "Test G")]
    #[test_case( 37u64, [0, 2, 0, 2, 0, 3, 1, 3, 1, 2, 0, 2, 0, 0, 0, 3, 1, 3, 1, 2]; "Test H")]
    #[test_case( 38u64, [0, 2, 0, 1, 0, 3, 1, 2, 2, 3, 0, 2, 0, 1, 0, 3, 1, 3, 1, 1]; "Test I")]
    #[test_case( 39u64, [2, 1, 2, 1, 1, 1, 1, 0, 1, 1, 2, 1, 2, 1, 1, 1, 2, 0, 2, 1]; "Test J")]
    #[test_case( 40u64, [0, 0, 2, 3, 0, 2, 3, 1, 0, 1, 0, 0, 2, 3, 0, 2, 3, 1, 0, 1]; "Test K")]
    #[test_case( 41u64, [3, 0, 1, 3, 0, 0, 3, 2, 0, 0, 3, 0, 1, 3, 0, 0, 3, 2, 0, 0]; "Test L")]
    #[test_case( 42u64, [3, 0, 0, 2, 0, 0, 3, 3, 0, 0, 3, 0, 0, 2, 0, 0, 3, 3, 0, 0]; "Test M")]
    #[test_case( 43u64, [2, 3, 1, 3, 2, 1, 0, 2, 0, 0, 2, 2, 1, 3, 2, 1, 0, 0, 0, 0]; "Test N")]
    #[test_case( 44u64, [2, 1, 0, 3, 1, 1, 0, 2, 0, 0, 2, 1, 0, 3, 1, 1, 2, 3, 0, 0]; "Test O")]
    #[test_case( 45u64, [2, 3, 1, 1, 1, 0, 0, 1, 2, 0, 3, 2, 2, 1, 3, 0, 0, 1, 2, 0]; "Test P")]
    #[test_case( 46u64, [0, 0, 2, 2, 1, 2, 2, 0, 0, 0, 0, 0, 3, 2, 1, 2, 2, 0, 0, 0]; "Test Q")]
    #[test_case( 47u64, [1, 3, 1, 1, 1, 2, 0, 2, 2, 1, 1, 3, 1, 1, 1, 0, 0, 2, 1, 0]; "Test R")]
    #[test_case( 48u64, [2, 1, 3, 1, 2, 0, 1, 0, 2, 1, 2, 1, 2, 1, 2, 0, 2, 0, 2, 0]; "Test S")]
    #[test_case( 49u64, [3, 1, 2, 2, 2, 0, 2, 1, 1, 0, 2, 1, 2, 1, 2, 0, 1, 1, 1, 1]; "Test T")]
    fn solve3(seed: u64, expected: [usize; 20]) {
        let deal: Deal<3> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 30u64, [2, 5, 5, 1, 4, 2, 0, 0, 4, 0, 3, 5, 5, 1, 4, 3, 0, 0, 4, 0]; "Test A")]
    #[test_case( 31u64, [1, 1, 1, 0, 1, 3, 4, 4, 4, 4, 2, 1, 1, 1, 1, 2, 4, 4, 4, 4]; "Test B")]
    #[test_case( 32u64, [1, 4, 1, 3, 1, 3, 0, 3, 2, 0, 1, 5, 1, 3, 2, 3, 0, 3, 1, 3]; "Test C")]
    #[test_case( 33u64, [4, 2, 2, 2, 3, 0, 3, 3, 2, 0, 4, 2, 2, 2, 3, 0, 3, 3, 3, 0]; "Test D")]
    #[test_case( 34u64, [3, 3, 2, 2, 3, 1, 1, 2, 3, 1, 2, 3, 2, 1, 3, 1, 1, 2, 3, 1]; "Test E")]
    #[test_case( 35u64, [4, 1, 1, 3, 2, 1, 3, 2, 1, 2, 4, 1, 2, 3, 2, 1, 3, 2, 1, 2]; "Test F")]
    #[test_case( 36u64, [2, 3, 1, 3, 2, 1, 1, 2, 1, 2, 1, 3, 1, 2, 2, 1, 1, 3, 1, 2]; "Test G")]
    #[test_case( 37u64, [0, 0, 1, 1, 0, 5, 5, 3, 3, 5, 0, 0, 1, 1, 0, 5, 4, 3, 3, 5]; "Test H")]
    #[test_case( 38u64, [0, 1, 4, 3, 1, 4, 3, 1, 2, 1, 0, 1, 4, 3, 0, 4, 3, 1, 2, 1]; "Test I")]
    #[test_case( 39u64, [0, 0, 2, 2, 0, 3, 4, 2, 2, 4, 0, 0, 3, 1, 0, 4, 4, 2, 2, 4]; "Test J")]
    #[test_case( 40u64, [3, 1, 2, 3, 2, 1, 4, 3, 2, 2, 3, 1, 2, 2, 2, 1, 4, 2, 2, 2]; "Test K")]
    #[test_case( 41u64, [2, 1, 1, 3, 1, 3, 2, 2, 2, 3, 2, 1, 1, 2, 1, 2, 2, 4, 1, 3]; "Test L")]
    #[test_case( 42u64, [3, 0, 2, 1, 1, 1, 3, 2, 3, 3, 4, 0, 2, 1, 1, 1, 3, 2, 3, 3]; "Test M")]
    #[test_case( 43u64, [1, 0, 5, 2, 1, 3, 4, 0, 2, 0, 1, 0, 5, 2, 1, 3, 4, 0, 2, 3]; "Test N")]
    #[test_case( 44u64, [3, 3, 3, 0, 0, 1, 1, 2, 5, 3, 4, 3, 3, 0, 0, 1, 0, 2, 5, 3]; "Test O")]
    #[test_case( 45u64, [2, 0, 3, 1, 1, 2, 4, 2, 2, 3, 3, 0, 2, 2, 1, 2, 4, 2, 2, 3]; "Test P")]
    #[test_case( 46u64, [3, 3, 1, 3, 1, 2, 1, 4, 2, 2, 3, 3, 1, 3, 1, 2, 2, 4, 2, 2]; "Test Q")]
    #[test_case( 47u64, [2, 0, 0, 1, 0, 2, 3, 3, 4, 3, 2, 2, 0, 1, 2, 3, 3, 5, 4, 3]; "Test R")]
    #[test_case( 48u64, [3, 2, 3, 2, 2, 2, 3, 2, 3, 2, 3, 2, 3, 2, 2, 2, 3, 2, 3, 2]; "Test S")]
    #[test_case( 49u64, [0, 1, 3, 0, 1, 4, 4, 2, 4, 2, 0, 1, 2, 0, 1, 4, 4, 2, 4, 2]; "Test T")]
    fn solve5(seed: u64, expected: [usize; 20]) {
        let deal: Deal<5> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 30u64, [1, 3, 3, 1, 1, 5, 3, 3, 5, 4, 1, 3, 3, 1, 1, 5, 3, 3, 5, 4]; "Test A")]
    #[test_case( 31u64, [1, 4, 1, 3, 2, 3, 2, 2, 3, 4, 1, 4, 1, 3, 2, 4, 2, 3, 2, 3]; "Test B")]
    #[test_case( 32u64, [2, 4, 6, 3, 3, 4, 0, 0, 3, 0, 2, 5, 6, 3, 3, 4, 0, 0, 3, 0]; "Test C")]
    #[test_case( 33u64, [5, 3, 4, 4, 5, 1, 2, 1, 1, 1, 5, 3, 4, 4, 5, 0, 2, 1, 0, 0]; "Test D")]
    #[test_case( 34u64, [6, 5, 4, 4, 5, 0, 1, 2, 1, 0, 6, 5, 4, 4, 5, 0, 1, 2, 1, 0]; "Test E")]
    #[test_case( 35u64, [6, 5, 4, 4, 4, 0, 1, 1, 2, 2, 6, 5, 4, 4, 4, 0, 1, 1, 2, 2]; "Test F")]
    #[test_case( 36u64, [5, 6, 5, 6, 6, 1, 0, 1, 0, 0, 5, 6, 5, 6, 6, 1, 0, 1, 0, 0]; "Test G")]
    #[test_case( 37u64, [3, 1, 1, 1, 1, 3, 4, 4, 4, 5, 3, 1, 1, 1, 1, 2, 4, 4, 4, 5]; "Test H")]
    #[test_case( 38u64, [2, 4, 5, 2, 4, 3, 0, 0, 3, 0, 2, 5, 5, 2, 4, 3, 0, 0, 3, 0]; "Test I")]
    #[test_case( 39u64, [5, 3, 3, 4, 3, 1, 3, 3, 2, 1, 5, 3, 3, 4, 3, 0, 3, 3, 2, 3]; "Test J")]
    #[test_case( 40u64, [1, 1, 3, 2, 2, 5, 4, 2, 4, 2, 1, 1, 3, 2, 2, 5, 4, 2, 4, 2]; "Test K")]
    #[test_case( 41u64, [4, 3, 0, 1, 1, 2, 2, 4, 4, 4, 4, 3, 0, 1, 1, 2, 3, 4, 4, 4]; "Test L")]
    #[test_case( 42u64, [4, 3, 2, 6, 3, 2, 2, 3, 0, 0, 4, 3, 2, 6, 3, 2, 2, 3, 0, 3]; "Test M")]
    #[test_case( 43u64, [2, 4, 3, 3, 4, 4, 1, 1, 2, 1, 2, 3, 3, 3, 4, 4, 1, 1, 2, 1]; "Test N")]
    #[test_case( 44u64, [1, 3, 2, 3, 3, 4, 2, 3, 3, 3, 1, 3, 2, 3, 3, 4, 2, 3, 3, 3]; "Test O")]
    #[test_case( 45u64, [1, 4, 2, 5, 3, 3, 1, 4, 1, 3, 2, 4, 2, 5, 3, 3, 1, 4, 1, 3]; "Test P")]
    #[test_case( 46u64, [0, 4, 2, 1, 1, 6, 2, 4, 4, 3, 0, 4, 2, 1, 0, 5, 2, 4, 4, 3]; "Test Q")]
    #[test_case( 47u64, [2, 3, 2, 3, 2, 3, 3, 3, 3, 3, 2, 3, 2, 3, 2, 3, 3, 3, 3, 3]; "Test R")]
    #[test_case( 48u64, [5, 2, 2, 0, 1, 1, 3, 3, 4, 4, 5, 2, 2, 0, 1, 1, 3, 3, 4, 4]; "Test S")]
    #[test_case( 49u64, [3, 0, 4, 2, 1, 1, 6, 1, 4, 4, 3, 0, 4, 2, 1, 1, 6, 1, 4, 4]; "Test T")]
    fn solve6(seed: u64, expected: [usize; 20]) {
        let deal: Deal<6> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 30u64, [3, 2, 3, 5, 2, 4, 6, 5, 3, 6, 4, 2, 3, 5, 2, 4, 6, 5, 3, 6]; "Test A")]
    #[test_case( 31u64, [6, 6, 1, 4, 4, 2, 2, 7, 3, 3, 6, 6, 1, 4, 4, 2, 2, 7, 3, 3]; "Test B")]
    #[test_case( 32u64, [1, 1, 5, 3, 1, 6, 6, 3, 5, 6, 1, 1, 5, 3, 1, 6, 7, 3, 5, 6]; "Test C")]
    #[test_case( 33u64, [4, 2, 1, 3, 2, 4, 6, 6, 5, 6, 4, 2, 1, 3, 2, 4, 6, 6, 5, 6]; "Test D")]
    #[test_case( 34u64, [4, 2, 1, 0, 1, 3, 6, 7, 8, 6, 4, 2, 1, 0, 1, 3, 6, 6, 8, 6]; "Test E")]
    #[test_case( 35u64, [1, 0, 5, 5, 1, 7, 6, 2, 3, 4, 1, 0, 5, 5, 0, 6, 6, 2, 3, 4]; "Test F")]
    #[test_case( 36u64, [3, 6, 3, 6, 4, 5, 2, 4, 2, 2, 3, 6, 3, 6, 4, 4, 2, 4, 2, 2]; "Test G")]
    #[test_case( 37u64, [4, 2, 1, 1, 1, 4, 5, 7, 7, 6, 4, 2, 1, 1, 1, 4, 5, 7, 7, 6]; "Test H")]
    #[test_case( 38u64, [1, 3, 3, 4, 2, 7, 5, 4, 4, 5, 1, 3, 3, 4, 2, 7, 5, 4, 4, 5]; "Test I")]
    #[test_case( 39u64, [5, 5, 1, 3, 3, 2, 2, 6, 4, 4, 6, 6, 1, 3, 3, 2, 2, 6, 4, 4]; "Test J")]
    #[test_case( 40u64, [5, 5, 5, 4, 5, 1, 1, 2, 3, 3, 5, 5, 5, 4, 5, 1, 1, 2, 3, 3]; "Test K")]
    #[test_case( 41u64, [6, 5, 5, 8, 7, 1, 2, 3, 0, 1, 6, 5, 5, 8, 7, 1, 3, 3, 0, 1]; "Test L")]
    #[test_case( 42u64, [7, 3, 8, 7, 7, 1, 5, 0, 0, 0, 7, 3, 8, 6, 7, 1, 5, 0, 0, 0]; "Test M")]
    #[test_case( 43u64, [5, 6, 6, 6, 6, 2, 1, 1, 2, 1, 5, 6, 6, 6, 6, 2, 2, 1, 2, 1]; "Test N")]
    #[test_case( 44u64, [1, 1, 0, 0, 0, 7, 7, 8, 8, 8, 1, 1, 0, 0, 0, 7, 7, 8, 8, 8]; "Test O")]
    #[test_case( 45u64, [3, 5, 4, 1, 2, 3, 2, 2, 5, 2, 3, 5, 4, 1, 2, 3, 2, 2, 5, 2]; "Test P")]
    #[test_case( 46u64, [3, 4, 1, 6, 3, 5, 3, 6, 2, 3, 3, 4, 1, 6, 3, 4, 3, 6, 2, 3]; "Test Q")]
    #[test_case( 47u64, [1, 0, 0, 4, 0, 7, 8, 7, 4, 7, 1, 0, 0, 4, 0, 7, 8, 7, 3, 3]; "Test R")]
    #[test_case( 48u64, [2, 3, 7, 4, 4, 6, 5, 1, 4, 3, 2, 3, 7, 4, 4, 6, 5, 1, 4, 3]; "Test S")]
    #[test_case( 49u64, [6, 5, 3, 2, 6, 2, 3, 5, 5, 2, 5, 5, 3, 1, 3, 2, 3, 5, 5, 2]; "Test T")]
    fn solve8(seed: u64, expected: [usize; 20]) {
        let deal: Deal<8> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 30u64, [2, 6, 4, 3, 3, 6, 3, 4, 5, 4, 2, 6, 4, 3, 3, 6, 3, 4, 5, 4]; "Test A")]
    #[test_case( 31u64, [2, 3, 2, 3, 3, 6, 5, 5, 5, 5, 2, 3, 2, 3, 3, 7, 6, 6, 6, 6]; "Test B")]
    #[test_case( 32u64, [0, 0, 5, 4, 0, 8, 8, 4, 5, 8, 0, 0, 5, 4, 0, 8, 8, 4, 5, 6]; "Test C")]
    #[test_case( 33u64, [8, 6, 4, 8, 5, 1, 3, 4, 0, 1, 7, 6, 4, 8, 5, 1, 3, 4, 1, 1]; "Test D")]
    #[test_case( 34u64, [4, 4, 4, 4, 4, 3, 4, 5, 5, 5, 4, 4, 4, 4, 4, 3, 4, 5, 5, 5]; "Test E")]
    #[test_case( 35u64, [5, 6, 6, 8, 7, 3, 3, 3, 0, 2, 5, 6, 6, 8, 7, 4, 3, 3, 0, 2]; "Test F")]
    #[test_case( 36u64, [0, 3, 2, 3, 0, 6, 6, 6, 6, 8, 0, 3, 2, 3, 0, 7, 6, 6, 6, 8]; "Test G")]
    #[test_case( 37u64, [1, 0, 2, 3, 0, 8, 9, 7, 6, 9, 1, 0, 2, 3, 0, 7, 9, 5, 6, 8]; "Test H")]
    #[test_case( 38u64, [6, 7, 7, 8, 8, 3, 1, 2, 1, 1, 6, 7, 7, 8, 8, 3, 1, 2, 1, 1]; "Test I")]
    #[test_case( 39u64, [3, 2, 0, 0, 0, 6, 7, 9, 9, 9, 3, 2, 0, 0, 0, 6, 7, 9, 9, 9]; "Test J")]
    #[test_case( 40u64, [2, 1, 3, 3, 1, 6, 7, 6, 6, 7, 2, 1, 3, 3, 1, 7, 8, 6, 6, 8]; "Test K")]
    #[test_case( 41u64, [2, 1, 1, 1, 1, 7, 8, 8, 7, 8, 2, 1, 1, 1, 1, 7, 8, 8, 7, 8]; "Test L")]
    #[test_case( 42u64, [4, 8, 4, 8, 8, 4, 0, 5, 0, 0, 4, 8, 4, 8, 8, 4, 0, 5, 0, 0]; "Test M")]
    #[test_case( 43u64, [2, 4, 4, 3, 2, 7, 5, 5, 5, 5, 2, 4, 4, 3, 2, 7, 5, 5, 5, 5]; "Test N")]
    #[test_case( 44u64, [5, 2, 2, 4, 3, 4, 7, 7, 5, 6, 5, 2, 2, 4, 3, 4, 7, 7, 5, 6]; "Test O")]
    #[test_case( 45u64, [0, 5, 1, 4, 1, 9, 3, 8, 4, 5, 0, 6, 1, 4, 1, 9, 3, 8, 4, 5]; "Test P")]
    #[test_case( 46u64, [5, 7, 5, 5, 4, 4, 2, 4, 4, 5, 4, 7, 5, 4, 4, 4, 2, 4, 4, 5]; "Test Q")]
    #[test_case( 47u64, [1, 1, 2, 3, 1, 8, 8, 7, 5, 5, 1, 1, 2, 3, 1, 8, 8, 7, 5, 5]; "Test R")]
    #[test_case( 48u64, [6, 3, 4, 7, 5, 0, 6, 5, 0, 0, 6, 3, 4, 7, 5, 0, 6, 5, 0, 0]; "Test S")]
    #[test_case( 49u64, [5, 4, 8, 7, 4, 3, 5, 1, 1, 3, 5, 3, 8, 7, 4, 3, 5, 1, 1, 3]; "Test T")]
    fn solve9(seed: u64, expected: [usize; 20]) {
        let deal: Deal<9> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

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

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 2u64, Strain::Trump(Suit::Spades), Seat::North, 2; "Test B")]
    fn solve_single5(seed: u64, strain: Strain, declarer: Seat, expected: usize) {
        let deal: Deal<5> = Deal::from_u64_seed(seed);

        let mut dds = DoubleDummySolver::default();

        let dds_result = dds.solve_initial_position(deal, strain, declarer);

        // println!("{}", dds_result);
        assert_eq!(dds_result, expected);
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

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
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

        let mut dds = DoubleDummySolver::default();

        let dds_result = dds.solve_initial_position(deal, strain, declarer);

        // println!("{}", dds_result);
        assert_eq!(dds_result, expected);
    }
}
