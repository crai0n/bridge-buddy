use crate::dds_config::DdsConfig;
use crate::double_dummy_solver::dds_statistics::DdsStatistics;
use crate::move_generator::MoveGenerator;
use crate::state::VirtualState;
use crate::transposition_table::TranspositionTable;
use crate::trick_estimations::{losing_tricks_for_leader, quick_tricks_for_leader, quick_tricks_for_second_hand};
use bridge_buddy_core::engine::hand_evaluation::ForumDPlus2015Evaluator;
use bridge_buddy_core::primitives::contract::Strain;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::Deal;
use std::cmp::min;

#[derive(Default)]
pub struct DoubleDummyRunner {
    config: DdsConfig,
    transposition_table: TranspositionTable,
    statistics: DdsStatistics,
}

impl DoubleDummyRunner {
    pub fn with_config(config: DdsConfig) -> Self {
        Self {
            config,
            transposition_table: TranspositionTable::default(),
            statistics: DdsStatistics::default(),
        }
    }

    pub fn get_statistics(&self) -> DdsStatistics {
        self.statistics
    }

    pub fn solve_initial_position<const N: usize>(
        &mut self,
        deal: Deal<N>,
        strain: Strain,
        opening_leader: Seat,
    ) -> usize {
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

    fn get_initial_estimate<const N: usize>(deal: Deal<N>, strain: Strain, opening_leader: Seat) -> usize {
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

    fn score_node<const N: usize>(&mut self, state: &mut VirtualState<N>, estimate: usize) -> usize {
        self.statistics.node_count += 1;
        if let Some(early_score) = self.try_early_node_score(state, estimate) {
            return early_score;
        }

        if state.is_last_trick() {
            return self.score_terminal_node(state);
        }

        // println!("generating possible moves!");
        let available_moves = MoveGenerator::generate_moves(state, self.config.move_ordering);
        let mut highest_score = 0;
        let mut first_move_is_best = true;
        for (moves_tried, candidate_move) in available_moves.into_iter().enumerate() {
            if moves_tried == 0 {
                self.statistics.n_first_moves += 1;
            }

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
                if moves_tried == 0 {
                    self.statistics.n_first_move_is_best += 1;
                }
                return score;
            } else if score > highest_score {
                // if we cannot reach estimate, we need the highest score found
                highest_score = score;
                if moves_tried >= 1 {
                    first_move_is_best = false;
                }
            }
        }

        if self.config.use_transposition_table && state.player_is_leading() {
            let add_tricks = highest_score - state.tricks_won_by_axis(state.next_to_play());
            self.store_upper_bound_in_tt(state, add_tricks);
        }

        if first_move_is_best {
            self.statistics.n_first_move_is_best += 1;
        }

        highest_score
    }

    fn try_find_node_in_tt<const N: usize>(&self, state: &VirtualState<N>, estimate: usize) -> Option<usize> {
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

    fn try_early_node_score<const N: usize>(&mut self, state: &mut VirtualState<N>, estimate: usize) -> Option<usize> {
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

        if self.config.check_losing_tricks && state.player_is_leading() {
            if let Some(lt_score) = self.try_score_using_losing_tricks(state, estimate) {
                return Some(lt_score);
            }
        }

        if self.config.check_quick_tricks {
            if let Some(qt_score) = self.try_score_using_quick_tricks(state, estimate) {
                return Some(qt_score);
            }
        }

        None
    }

    fn try_score_using_losing_tricks<const N: usize>(
        &mut self,
        state: &VirtualState<N>,
        estimate: usize,
    ) -> Option<usize> {
        let losing_tricks = losing_tricks_for_leader(state);
        let total_with_losing_tricks = Self::maximum_achievable_tricks(state) - losing_tricks;
        if total_with_losing_tricks < estimate {
            if self.config.use_transposition_table {
                self.store_upper_bound_in_tt(state, total_with_losing_tricks - Self::current_tricks(state));
            }
            return Some(total_with_losing_tricks);
        }
        None
    }

    fn try_score_using_quick_tricks<const N: usize>(
        &mut self,
        state: &VirtualState<N>,
        estimate: usize,
    ) -> Option<usize> {
        let quick_tricks = match state.count_cards_in_current_trick() {
            0 => quick_tricks_for_leader(state),
            1 if self.config.quick_tricks_in_second_hand => quick_tricks_for_second_hand(state),
            _ => return None,
        };
        let total_with_quick_tricks = state.tricks_won_by_axis(state.next_to_play()) + quick_tricks;
        if total_with_quick_tricks >= estimate {
            if self.config.use_transposition_table {
                self.store_lower_bound_in_tt(state, quick_tricks);
            }
            return Some(total_with_quick_tricks);
        }
        None
    }

    fn store_lower_bound_in_tt<const N: usize>(&mut self, state: &VirtualState<N>, bound: usize) {
        let tt_key = state.generate_tt_key();
        self.transposition_table.update_lower_bound(&tt_key, bound)
    }

    fn store_upper_bound_in_tt<const N: usize>(&mut self, state: &VirtualState<N>, bound: usize) {
        let tt_key = state.generate_tt_key();
        self.transposition_table.update_upper_bound(&tt_key, bound)
    }

    fn maximum_achievable_tricks<const N: usize>(state: &VirtualState<N>) -> usize {
        state.tricks_left() + state.tricks_won_by_axis(state.next_to_play())
    }

    fn current_tricks<const N: usize>(state: &VirtualState<N>) -> usize {
        state.tricks_won_by_axis(state.next_to_play())
    }

    fn score_terminal_node<const N: usize>(&mut self, state: &mut VirtualState<N>) -> usize {
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

    fn store_terminal_node_in_tt<const N: usize>(&mut self, state: &VirtualState<N>, winner_of_last_trick: Seat) {
        if winner_of_last_trick.same_axis(&state.next_to_play()) {
            self.store_lower_bound_in_tt(state, 1);
        } else {
            self.store_upper_bound_in_tt(state, 0);
        }
    }

    fn play_last_trick<const N: usize>(state: &mut VirtualState<N>) {
        for _ in 0..4 {
            let last_card_of_player = state.valid_moves().next().unwrap();
            state.play(last_card_of_player).unwrap();
        }
    }

    fn undo_last_trick<const N: usize>(state: &mut VirtualState<N>) {
        for _ in 0..4 {
            state.undo();
        }
    }
}
