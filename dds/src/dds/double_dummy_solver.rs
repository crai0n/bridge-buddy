use crate::dds::double_dummy_result::DoubleDummyResult;
use crate::primitives::contract::Strain;
use crate::primitives::deal::Seat;
use crate::primitives::Deal;
use enum_iterator::all;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub struct DoubleDummySolver {
    dds_config: DdsConfig,
    transposition_table: HashMap<NodeIdentifier, LookupResult>,
}

impl DoubleDummySolver {
    pub fn solve_deal<const N_TRICKS: usize>(&mut self, deal: Deal<N_TRICKS>) -> DoubleDummyResult {
        let mut result = DoubleDummyResult::new();
        for strain in all::<Strain>() {
            for declarer in Seat::iter() {
                let starting_state = DdsState::new(deal, strain, declarer);
                let defenders_tricks = self.solve_initial_position(starting_state); // defenders play first
                result.set_tricks(declarer, strain, N_TRICKS - defenders_tricks);
            }
            self.transposition_table.clear() // entries are almost useless with new trump strain
        }
        result
    }

    fn solve_initial_position<const N_TRICKS: usize>(&self, state: DdsState) -> usize {
        let mut a = self.calculate_quick_tricks(state);
        let mut b = N_TRICKS + 1 - self.calculate_losing_tricks(state);
        while b > a + 1 {
            let estimate = (a + b) / 2;
            let score = self.score_node(state, estimate);
            if score >= estimate {
                a = score
            } else {
                b = score + 1
            }
        }
        a
    }

    fn try_lookup_node(&self, state: DdsState) -> Option<LookupResult> {
        let node_id = NodeIdentifier::from_state(state);
        self.transposition_table.get(node_id)
    }

    pub fn score_node(&self, state: DdsState, alpha: usize) -> usize {
        // check if we've seen node before
        if let Some(lookup_result) = self.try_lookup_node(state, alpha) {
            match lookup_result {
                LookupResult::Exact(score) => return score,
                LookupResult::Bounded(lower, upper) => {
                    if lower >= alpha {
                        return lower;
                    } else if upper < alpha {
                        return upper;
                    }
                }
            }
        }

        // no more choice in the last trick, so we can score this node
        if state.tricks_left() == 1 {
            return self.score_terminal_node(state, alpha);
        }

        // try to return early by establishing bounds on the possible score
        if let Some(score) = self.try_early_node_score(state, alpha) {
            return score;
        }

        // try to calculate score by scoring child nodes (negamax)
        let candidate_moves = self.generate_candidate_moves(state);
        let mut high_bound = 0usize;
        for candidate in candidate_moves {
            let candidate_score = self.score_candidate(candidate, state, alpha);
            if candidate_score >= alpha {
                // early return, cause we found a move that fulfills our guess
                return candidate_score;
            } else if candidate_score > high_bound {
                // if all candidates score low, we need the "best of the worst"
                high_bound = candidate_score
            }
        }
        return high_bound;
    }

    pub fn generate_candidate_moves(&self, state: DdsState) -> Vec<Move> {}

    pub fn score_terminal_node(&self, _state: &mut DdsState) -> usize {
        unimplemented!()
    }

    fn try_early_node_score(&self, state: DdsState, alpha: usize) -> Option<usize> {
        // lower bound on score
        let taken_tricks: usize = state.count_tricks_taken_by_players_side();
        if taken_tricks >= alpha {
            // early return (futility pruning)
            return Some(taken_tricks);
        }

        // upper bound on score
        let max_tricks = taken_tricks + state.tricks_left();
        if max_tricks < alpha {
            // early return (futility pruning)
            return Some(max_tricks);
        }

        // improvements on lower bound using quick tricks
        let quick_tricks = self.calculate_quick_tricks(state);
        let min_tricks = taken_tricks + quick_tricks;
        if min_tricks >= alpha {
            // early return
            return Some(min_tricks);
        }

        // improvement on upper bound using lost tricks
        let losing_tricks = self.calculate_losing_tricks(state);
        let max_tricks = max_tricks - losing_tricks;
        if max_tricks < alpha {
            // early return
            return Some(max_tricks);
        }

        None
    }

    fn calculate_quick_tricks(&self, state: DdsState) -> usize {
        0
    }

    fn calculate_losing_tricks(&self, state: DdsState) -> usize {
        0
    }

    fn score_candidate(&self, candidate: Move, state: DdsState, alpha: usize) -> usize {
        let our_axis = state.next_to_play().axis();
        state.apply_move(candidate);

        // A minor complication is that plies (half-moves) are not strictly alternating (If the last player to a trick
        // or their partner wins, the same side makes two consecutive moves. Therefore we always check whose turn it is
        // and rearrange the arguments only if turn-side changes.
        let candidate_score = if state.next_to_play().axis() == our_axis {
            self.score_node(state, alpha)
        } else {
            N_TRICKS - self.score_node(state, N_TRICKS + 1 - alpha)
        };
        state.undo_move();

        candidate_score
    }
}



pub struct DdsConfig {
    ab_pruning: bool,
    transposition_table: bool,
    partitioning: bool,
    fail_soft: bool,
}

pub enum Bound {
    AtLeast(usize),
    LessThan(usize),
}

pub struct Interval {
    lower: usize,
    upper: usize,
}

pub struct NodeIdentifier {
    relative_remaining_cards: [Vec<Cards>; 4],
    trumps: Option<Suit>,
}

pub enum LookupResult {
    Exact(usize),
    Bounded(usize, usize),
}
// pub struct PositionNode;
//
// pub enum Color {
//     Min,
//     Max,
// }

// pub fn negaMax(node: PositionNode, depth: usize, color: Color) {
//     if last_trick() {
//         return Self::evaluate_last_trick(node);
//     }
//     let mut value = 0;
//     for
// }
//
// pub fn evaluate_terminal_node(node: PositionNode) {
//     unimplemented!()
// }
