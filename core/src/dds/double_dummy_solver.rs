pub struct DoubleDummySolver {
    ab_pruning: bool,
    transposition_table: bool,
    partitioning: bool,
}

// pub struct PositionNode;
//
// pub enum Color {
//     Min,
//     Max,
// }

impl DoubleDummySolver {
    // The basic assumption of the Double Dummy Solver is that we can assign fixed "score" to each "starting position"
    // defined by (deal + strain + declarer) under the assumption that both sides have perfect knowledge and perfect play.

    // Splitting every trick into 4 plys (normally half-moves, in our case quarter-moves) by the two sides, each game
    // forms a directed acyclic graph of depth 48. As there is no more choice involved in playing the last trick, the
    // result of the last trick is fixed, making ply 47 (counted from 0) "terminal" by definition. This also means
    // that each ply brings the game as whole closer to termination and all terminal nodes are in the same generation.

    // We can score any terminal node by counting the number of tricks taken by declarer's side.
    // Declarer's side want to maximize this score, while defenders try to minimize this score. Because Bridge is a
    // zero-sum game, we can also use subjective scores (#tricks taken) related by score_NS = 14 - score_EW,
    // and make both players maximizers of their own score (cmp. Negamax).

    // Perfect Knowledge means that all players know all cards. Seeing two dummies is enough for that, hence the name.

    // Perfect Play means that every player will always make a move that brings them "closer" to one of their
    // subjectively optimal terminal position (which they know about because of perfect knowledge). These moves are
    // called the "principal variation" and all other moves are "unacceptable" to the players. Using this adversarial
    // behaviour we will not reach every theoretically reachable final position. In fact, we will only reach any one
    // of a group of "equal" terminal positions, that each have the same score. We can therefore assign this score of
    // equivalent final positions to the initial position.

    // While traversing the graph, the engine will keep track of the subjectively best terminal positions reached,
    // restricting the interval that covers the "true score" of the starting position further and further.
    // The current bounds are referred to as a (lower bound, inclusive) and b (upper bound, exclusive).
    // Because our scoring function is integer, we are done once b equals a+1, in other words, we run our search while
    // a + 1 < b.

    // We allow searching to the end of the game (unlimited depth), assuming that this can be done in reasonable time.
    // We therefore do not need an explicit scoring function for non-terminal nodes. Instead, as the subjective score
    // for both sides never decreases as we progress through the game, a lower bound on the score of a position
    // of the current node is the amount of tricks already taken. An upper bound is the amount of tricks already taken
    // plus the number of tricks remaining. Therefore, for each node we have a bounded interval covering the true score.

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
    // PV-nodes: If alpha<=s<beta, we found a move that "looks good", as it improves our lower bound.
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
    // to s<alpha), we have to start the process all over again, from this node downwards, to get new estimates for
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

    // A minor complication is that plies (half-moves) are not strictly alternating (If the last player to a trick or their partner
    // wins, the same side makes two consecutive moves.
    // Therefore we always check whose turn it is and rearrange the arguments only if turn-side changes.

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
}
