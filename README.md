# Bridge Buddy

## Introduction

Bridge Buddy is a small app designed to help you evaluate Bridge Hands according to different criteria. Currently
implementation is based on the official bidding system of the German Bridge
Association [Deutscher Bridge Verband](http://www.dbv.de). In the future, other evaluation systems might be added. We
are also planning to expand Bridge Buddy's capabilities to include bidding, evaluating a partner's and maybe even
opponents' bids.

## Development Environment

Bridge Buddy is developed exclusively in Rust. To set up your development environment, we recommend
using [rustup](http://rustup.sh).

## Current Capabilities

Bridge Buddy started as an evaluation tool, but has various other capabilities

### Command-Line Interface

Bridge Buddy's CLI is the currenlty preferred tool for interacting with the various capabilities of Bridge Buddy. It's
mostly glue code, which an be found in Code can be found in `./cli`

### Game Management

Bridge Buddy includes a state machine to manage the progress of a bridge game, including both bidding and card play.
This state machine is built on the basic game primitives like cards and bids.

All of this code lives in `./core`

The code is structured so that a future server and client can reuse the same code, even though only the server is aware
of all
cards, while the client acts on partial information (the hand dealt to the player by the server). Currently the client
and server processes run in the same binary.

You can play a hand of bridge using the cli-tool by running

```shell
cargo run --bin bridge-buddy-cli play 
```

### Hand Lookup

Bridge Buddy implements two different systems for enumerating all possible bridge-hands, following the ["impossible
bridge book"](https://bridge.thomasoandrews.com/impossible/). All hands can be represented by a 96-bit integer.

The code lives in `./core/impossible-book`.

### Double Dummy Solver

Bridge Buddy is capable of analyzing the perfect play strategy for a given deal of Bridge. It is inspired
by [Bo Haglund's DDS](https://github.com/dds-bridge/dds), but is implemented independently.

Code can be found in `./dds`, although it relies on the primitives from `./core`. It contains an alternate version of
tracking remaining cards for performance reasons, which might be integrated into `core` at some point in the future.

It's basic mechanism follows that of other game engines (like e.g. chess), visiting each possible state of play and
calculating if a better outcome than the currently optimal result can be achieved. If not, the branch is pruned from the
game-tree.

Various optimizations are applied. First of all, all positions are transposed in such a way that only the relative
ordering of remaining cards matter, not their absolute rank. E.g. if only the cards "853" of a given suit remain in
play, they are treated exactly the same as "AKQ".

Secondly, all moves are treated in a subjective position, ignoring absolute seating.

Combining this with an on-the-fly lookup table that stores the result of an already
visited position for future reference, this reduces the amount of visited nodes dramatically.

Additionally, there is a simple heuristic applied to pre-order moves according to their likelihood of being a "good
move", again trying to lower visited nodes.

It currently analyzes a full hand of Bridge in roughly 10 seconds (in `--release` mode).
The result shows the number of tricks a given declarer can achieve in a given strain when all players play
perfectly. The corresponding test is normally ignored due to taking quite some time without the `--release` flag. To run
this test anyway,

```shell
cargo test --release --package bridge-buddy-dds double_dummy_solver::test::solve_explicit13::test_a -- --ignored 
```

A single strain (no-trump vs trump) can be run in about 5 seconds using

```shell
cargo test --release --package bridge-buddy-dds double_dummy_solver::test::solve_single13::test_a -- --ignored 
```

### Play Engine

Bridge Buddy's Engine is an early WIP but builds on the DDS described above, using the subjective seating and relative
ordering.

## Browser App

The Bridge Buddy Browser App is currenlty POC-only. No functionality has been implemented

To deploy our browser-app, we are using [`trunk`](https://trunkrs.dev/).

After basic toolchain-setup, install the WASM-target and `trunk` by running

```{sh}
rustup target add wasm32-unknown-unknown
cargo install trunk
```

You are now ready to clone the Bridge Buddy repository and run our app

```
git clone https://github.com/crai0n/bridge-buddy
cd bridge-buddy/webapp
trunk serve --open
```