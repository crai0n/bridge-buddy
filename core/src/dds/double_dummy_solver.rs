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

    // The basic assumption of the Double Dummy Solver is that there is a fixed "score", i.e. a maximum number of tricks
    // possible for a given declarer and trump strain for each deal of the hands. This score is calculated for
    // each "starting position" defined by deal + strain + declarer under the assumption that both sides have perfect
    // knowledge and perfect play.

    // Perfect Knowledge means that all players know all cards. Seeing two dummies (and their own hand) is enough for
    // that, hence the name. This way, we turn bridge into a game of perfect information, which we then try to solve by
    // methods already established in engines developed for chess, checkers and others.

    // Splitting every trick into 4 plys (normally half-moves, in our case quarter-moves) by the two sides, each game
    // forms a directed acyclic graph of depth 48. As there is no more choice involved in playing the last trick, the
    // result of the last trick is fixed, making ply 47 (counted from 0) "terminal" by definition. This also means
    // that each ply brings the game as a whole closer to termination and all terminal nodes are in the same generation.
    // Compared to chess, which is basically open-ended (as long as you have pawn-moves), the game-tree is therefore
    // comparatively small.

    // Perfect Play then means that every player will always make a move that brings them "closer" to one of their
    // subjectively optimal terminal position (which they know about because of perfect knowledge). These moves are
    // called the "principal variation" and all other moves are "unacceptable" to the players. Using this adversarial
    // behaviour not every final position theoretically possible given an initial position will be reached. In fact, we
    // will only reach any one of a group of "equal" terminal positions, that each have the same score (otherwise one of
    // the players would make a different move earlier). We can then propagate the score of these terminal positions
    // backwards up the tree, at every node neglecting all but the "optimal" child, until we reach the initial position,
    // this way assigning the score of these equivalent "optimal" final positions to the initial position.

    // We can score any terminal node by counting the number of tricks taken by declarer's side.
    // Declarer's side want to maximize this score, while defenders try to minimize this score. Because Bridge is a
    // zero-sum game, we can also use subjective scores (#tricks taken) related by score_NS = N_TRICKS - score_EW,
    // and make both players maximizers of their own score (cmp. Negamax). This way, all scores are always positive.

    // While traversing the graph, the engine will keep track of the subjectively best terminal positions already reached,
    // restricting the interval that covers the "true score" of the starting position further and further.
    // The current bounds are referred to as "a" (lower bound, inclusive) and "b" (upper bound, exclusive).
    // Because our scoring function is integer, we are done once b equals a+1, in other words, we run our search while
    // a + 1 < b.

    // We allow searching to the end of the game (unlimited depth), assuming that this can be done in reasonable time.
    // We therefore do not need an explicit depth parameter or a real scoring function for non-terminal nodes.

    // Instead, as the subjective score for both sides never decreases as we progress through the game, a lower bound on
    // the score of a position of the current node is the amount of tricks already taken. An upper bound is the amount
    // of tricks already taken plus the number of tricks remaining. Therefore, for each node we have a bounded interval
    // covering the true score.

    // These two estimates can both be improved upon by calculating "quick tricks" and "losing tricks", which are
    // related, but somewhat opposite concepts:
    // Quick Tricks are the number of tricks a side can take without giving up the lead, e.g. the number of high trumps
    // they hold. These can be added to the lower estimate of possible tricks if player either has the lead or could
    // take the lead no matter what opponents do.
    // Losing tricks occur when player has no card to play (either now or later) that would keep opponents from winning
    // the trick (should they want to). This occurs e.g. if the opponents have the winner in all suits that player
    // could lead, or if the opponents have winning trumps. Losing tricks must be subtracted from the upper bound of
    // possible tricks.

    // Because traversing the whole graph to find the solution is unfeasible, we need to include several optimizations,
    // starting with alpha-beta-pruning. Alpha-Beta-Pruning skips the evaluation of nodes (prunes the tree) if their
    // score (or the interval covering their true score) lies outside previously established bounds of the
    // minimax-score. Alpha refers to the lower, inclusive bound, while beta refers to the higher, exclusive bound.
    // Because our scoring functions are subjective, beta and alpha will also be subjective and "alternate" between
    // nodes for the different players: alpha_defenders = 14 - beta_declarer and beta_defenders = 14 - alpha_declarer.

    // Let's look at an example:
    // We are at trick 10, declarer having already taken 7 tricks. The maximum they can make from this position is
    // therefore 10 tricks (beta_declarer=11). We have already tried one ply and all possible
    // variations after that. We found that this ply results in at least 8, not 7 tricks. We therefore have found a new
    // lower bound on the score of this node with alpha=8. As this is currently our best move, we would take it if
    // we were forced to decide right now, keeping defenders to 5 tricks. Therefore the score of defenders last ply
    // is bounded from above by beta_defenders=6=14-alpha_declarer.

    // We now try an alternative ply for declarer. For this ply, defenders find one re-ply that makes 6 tricks for defenders
    // (keeping declarer to 7 tricks). This ply and all childs can then be ignored immediately, as it is obviously worse than our
    // previous attempt for declarer. In mathematical terms, the defenders found a ply with s>=7>=beta_defenders, called a fail-high.
    // A single fail-high is always enough to prune the parent node from the tree, we found a "refutation". This in turn
    // means that declarer's ply "failed low" and we need to look at more sibling nodes if we want to find a better move.

    // To be more general, while searching the tree, we will find nodes of three types (cmp Marsland and Popowich):
    // Cut-nodes: If s>= beta, we found a "refutation", or "failed high", proving that the last ply was weak.
    // A single fail high is enough to stop searching the subtree.
    // All-nodes: If we fail low (s<alpha), we must search all sibling nodes (Try to find a better reply). If all
    // siblings fail low, the parent node fails high.
    // PV-nodes: If alpha<=s<beta, we found a move that "looks good", as it keeps or improves our lower bound.
    // We continue searching the childs of this node.

    // To prune a subtree, we do not have to evaluate the terminal node of the tree. Whenever the estimates given above
    // (already taken tricks + quick tricks and taken tricks + remaining tricks - losing tricks) for any node lie
    // outside the interval [alpha, beta), we can return early and skip evaluating the subtree. More explicitly, if we
    // know that no move could make us reach alpha, we immediately fail-low, and by the same logic: If we can assure our
    // side of at least beta tricks, we immediately fail-high. Cmp. Futility pruning

    // The effect of alpha-beta-pruning depends heavily on the order in which we try moves. If we try best moves early,
    // we cut a lot, if we try best moves late, we will traverse most of the graph.

    // Assuming we have a good heuristic to order moves, we can do even better than alpha-beta-pruning by using what is
    // called a "zero-window-search". Assuming we are very good at guessing the best ply for both sides, we can find a
    // reasonable estimate of the true score of a node by just applying the best plies in turn until we can evaluate the
    // score.
    // Assuming for now that this estimate is accurate, we set alpha=estimate, beta=estimate+1 (integer score).
    // We now search every alternative move from the leaf to the root using this "zero-sized" window. As our first try
    // most likely was the best move, we will find refutations for most alternatives we try, proving them to be worse
    // than our initial guess. If we however find a better move (i.e. a move that has no refutation and that evaluates
    // to s>alpha), we have to start the process all over again, from this node downwards, to get new estimates for
    // alpha and beta.

    // Example 2:
    // Continuing the example from above, we have found an initial estimate of 9 tricks for declarer's side. Using this
    // estimate for a ZWS, we have retraced the tree until declarer's first ply at trick 8 without finding a "better"
    // move for any side. We now try an alternate ply for declarer at trick 8, which looked uninteresting to us, but we
    // realize that no matter what defenders do, we arrive at trick 11 with 10 tricks. There is no reply by defenders
    // that can refute this line. We have now found that our guess of the principal variation was
    // incorrect and this new play is better. We now have to search this sub-tree again without a zero-window to get a
    // new estimate of the real score.

    // If we are good at guessing, this algorithm (known as Principle-Variation-Search or Nega-Scout) performs better
    // than alpha-beta. If we are bad at guessing, it performs quite a bit worse, however.

    // Without any idea of the true number of tricks possible, we can pin-point the exact score with just four runs of
    // ZWS (also called Scout-Search) in any case, because our score function, always returns integer values in the
    // range [0,N_TRICKS] and log_2(N_TRICKS+1) < 4.

    // Without any heuristic to estimate the number of possible tricks, we start out by guessing that declarer can take
    // alpha_declarer=(N_TRICKS+1)/2=7 tricks (Meaning beta_declarer=8, alpha_defenders=14-8=6 and beta_defenders=7).

    // We then run the search until we prove this estimate to be too high or until we find a line that forces at least this estimate.
    // let mut a = 0, mut b = N_TRICKS+1;
    // while a + 1 < b {
    //     let test_val = (a + b)/2;
    //     test(testvalue), if success a = testvalue, else b = testvalue;
    // }
    // return a;
    // This results in
    // gen 0: test(7) -> if success test(10) else test(3),
    // gen 1: test(10)-> if success test(12), else test(8),
    //        test(3) -> if success test(5), else test(1),
    // gen 2: test(12) -> if success test(13), else test(11),
    //        test(8) -> if success test(9), else return 7
    //        test(5) -> if success test(6), else test(4),
    //        test(1) -> if success test(2), else return 0,
    // gen 3: test(13) -> if success return 13, else return 12.
    //        test(11) -> if success return 11, else return 10.
    //        test(9) -> if success return 9, else return 8
    //        test(6) -> if success return 6, else return 5,
    //        test(4) -> if success return 4, else return 3,
    //        test(2) -> if success return 2, else return 1,

    // This approach can be refined by making use of our quick-tricks- and losing-tricks-estimation. Using these, we
    // will sometimes "overshoot" our estimate. If quick-tricks bring us to a score higher than our estimate, we
    // do not need to test this value again, but can immediately improve our lower bound to this score.
    // By the same logic, if we "undershoot" our estimate in all cases (never even getting close to our estimate), our
    // upper bound can be lowered to the highest score found (as long as we really exhausted all possible moves).

    // let mut a = 0, mut b = N_TRICKS+1;
    // while a + 1 < b {
    //     let estimate = (a + b)/2;
    //     let score = score_starting_position(estimate);
    //     if score >= a {
    //         a = score;
    //     } else {
    //         b = score
    //     }
    // }
    // return a;

    // As these estimates are also the first to run at the beginning of a node-evaluation, we can quickly triangulate the range of
    // possible tricks, before delving deeper than the opening lead.

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

pub struct Move {
    card: Card,
    priority: usize,
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
